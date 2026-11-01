//! Resizable panes and columns in the TAW look ([`hifi`](crate::hifi)): for
//! the Library's lists and the Player's.
//!
//! - [`pane`]: a pane beside the rest of a view, its edge dragged to resize
//!   it (double-click puts it back).
//! - [`header`]: a list's column titles, with dividers dragged to resize the
//!   columns, a double-click on a divider to fit the column to what it shows,
//!   and a right-click to show or hide columns. One flexible column takes the
//!   room the others leave. It returns a [`Table`], where each column is.
//! - [`Layout`]: pane widths, column widths and hidden columns, to save with
//!   an app's settings (serde, every field defaulted).

use crate::hifi::{self, Tokens};
use egui::{Align2, Color32, CursorIcon, FontId, Id, Painter, Pos2, Rect, Sense, Stroke, Ui, Vec2};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// The room either side of a column's content.
pub const PAD: f32 = 10.0;
/// How wide a divider is to the mouse.
const GRIP: f32 = 8.0;

// ---------------------------------------------------------------- saving

/// Pane widths and list columns, by name, for an app to keep in its saved
/// settings. Anything not in it uses its default, so it can start empty.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Layout {
    /// Pane widths in points.
    pub panes: BTreeMap<String, f32>,
    /// Each list's columns.
    pub lists: BTreeMap<String, Columns>,
}

impl Layout {
    /// A pane's saved width: 0 until it's dragged, which means its default.
    pub fn pane_mut(&mut self, name: &str) -> &mut f32 {
        self.panes.entry(name.to_string()).or_insert(0.0)
    }

    /// A list's saved columns.
    pub fn list_mut(&mut self, name: &str) -> &mut Columns {
        self.lists.entry(name.to_string()).or_default()
    }
}

/// One list's column widths (only those that were changed) and hidden
/// columns, by column key.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Columns {
    pub widths: BTreeMap<String, f32>,
    pub hidden: BTreeSet<String>,
}

// ---------------------------------------------------------------- columns

/// A column of a list: its key (what it's saved under), its title, and its
/// default width, including [`PAD`] either side.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Col {
    pub key: &'static str,
    pub title: &'static str,
    pub width: f32,
    pub min: f32,
    /// Its content and title sit at the right (numbers, times).
    pub right: bool,
    /// It takes the room the other columns leave. One per list; it can't be
    /// hidden or dragged, but the columns beside it can.
    pub flex: bool,
    /// It can be hidden with the header's right-click menu.
    pub can_hide: bool,
    /// Its width can be dragged.
    pub resizable: bool,
}

impl Col {
    pub const fn new(key: &'static str, title: &'static str, width: f32) -> Self {
        Col {
            key,
            title,
            width,
            min: 36.0,
            right: false,
            flex: false,
            can_hide: true,
            resizable: true,
        }
    }

    /// Content and title at the right.
    pub const fn right(mut self) -> Self {
        self.right = true;
        self
    }

    /// Takes the spare room; always shown.
    pub const fn flex(mut self) -> Self {
        self.flex = true;
        self.can_hide = false;
        self
    }

    /// The narrowest it can be dragged or squeezed to.
    pub const fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    /// Always shown.
    pub const fn always(mut self) -> Self {
        self.can_hide = false;
        self
    }

    /// Always its default width, and always shown (artwork, a status light).
    pub const fn fixed(mut self) -> Self {
        self.resizable = false;
        self.can_hide = false;
        self.min = self.width;
        self
    }
}

/// Where a column landed: from `left` to `right`, with [`PAD`] inside each
/// edge for its content.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    pub key: &'static str,
    pub left: f32,
    pub right: f32,
    /// Content at the right.
    pub right_aligned: bool,
}

impl Span {
    /// Where content is anchored: inside the left edge, or the right edge
    /// for a right-aligned column.
    pub fn x(&self) -> f32 {
        if self.right_aligned {
            self.right - PAD
        } else {
            self.left + PAD
        }
    }

    /// The anchor at height `y`.
    pub fn pos(&self, y: f32) -> Pos2 {
        Pos2::new(self.x(), y)
    }

    /// The alignment to draw content with at [`Span::pos`].
    pub fn align(&self) -> Align2 {
        if self.right_aligned {
            Align2::RIGHT_CENTER
        } else {
            Align2::LEFT_CENTER
        }
    }

    /// The room for content, inside the padding.
    pub fn width(&self) -> f32 {
        (self.right - self.left - 2.0 * PAD).max(0.0)
    }
}

/// Lays the shown columns out from `left` across `width`: each at its saved
/// width (or default), at least its minimum; the flexible column takes the
/// rest. When there isn't room, the columns after the flexible one, then
/// those before it, give way down to their minimums, right to left.
pub fn spans(cols: &[Col], saved: &Columns, left: f32, width: f32) -> Vec<Span> {
    let shown: Vec<&Col> = cols
        .iter()
        .filter(|c| !c.can_hide || !saved.hidden.contains(c.key))
        .collect();
    let mut w: Vec<f32> = shown
        .iter()
        .map(|c| {
            if c.flex {
                c.min
            } else if c.resizable {
                saved
                    .widths
                    .get(c.key)
                    .copied()
                    .unwrap_or(c.width)
                    .max(c.min)
            } else {
                c.width
            }
        })
        .collect();
    if let Some(f) = shown.iter().position(|c| c.flex) {
        let others: f32 = w
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != f)
            .map(|(_, v)| v)
            .sum();
        let mut over = others + shown[f].min - width;
        let order = (f + 1..shown.len()).rev().chain((0..f).rev());
        for i in order {
            if over <= 0.0 {
                break;
            }
            let give = (w[i] - shown[i].min).max(0.0).min(over);
            w[i] -= give;
            over -= give;
        }
        let others: f32 = w
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != f)
            .map(|(_, v)| v)
            .sum();
        w[f] = (width - others).max(shown[f].min);
    }
    let mut x = left;
    shown
        .iter()
        .zip(w)
        .map(|(c, cw)| {
            let s = Span {
                key: c.key,
                left: x,
                right: x + cw,
                right_aligned: c.right,
            };
            x += cw;
            s
        })
        .collect()
}

/// Which column the divider after shown column `i` resizes, so that the
/// divider follows the mouse: before the flexible column, the one to its
/// left; from the flexible column on (where columns are held to the right
/// edge), the one to its right, which grows as the divider moves left.
/// `None` if that column can't be dragged.
fn divider_target(cols: &[&Col], i: usize) -> Option<(usize, f32)> {
    let after_flex = cols.iter().position(|c| c.flex).is_some_and(|f| i >= f);
    let (j, sign) = if after_flex { (i + 1, -1.0) } else { (i, 1.0) };
    cols.get(j)
        .filter(|c| c.resizable && !c.flex)
        .map(|_| (j, sign))
}

/// Content widths seen in a list, by column key.
type Seen = HashMap<&'static str, f32>;

/// A list's columns, as [`header`] laid them out this frame.
pub struct Table {
    id: Id,
    pub spans: Vec<Span>,
}

impl Table {
    /// Where a column is, if it's shown.
    pub fn get(&self, key: &str) -> Option<Span> {
        self.spans.iter().find(|s| s.key == key).copied()
    }

    /// Tells the list how wide a column's content is in a row it drew (not
    /// counting [`PAD`]), so a double-click on its divider can fit it.
    /// [`Table::text`] does this itself.
    pub fn note(&self, ui: &Ui, key: &'static str, w: f32) {
        ui.ctx().data_mut(|d| {
            let seen = d.get_temp_mut_or_default::<Seen>(self.id.with("seen"));
            let m = seen.entry(key).or_insert(0.0);
            *m = m.max(w);
        });
    }

    /// One line of text in a column at height `y`, cut short with "…" if it
    /// doesn't fit. Does nothing if the column is hidden.
    #[allow(clippy::too_many_arguments)]
    pub fn text(
        &self,
        ui: &Ui,
        p: &Painter,
        key: &'static str,
        y: f32,
        s: &str,
        font: FontId,
        c: Color32,
    ) -> Option<Rect> {
        let span = self.get(key)?;
        let natural = p.layout_no_wrap(s.to_string(), font.clone(), c).size().x;
        self.note(ui, key, natural);
        Some(hifi::label_fit(
            p,
            span.pos(y),
            span.align(),
            s,
            font,
            c,
            span.width(),
        ))
    }
}

/// A list's column titles along the top of its well (`r`, the full width of
/// the list), with the dividers between them. Drag a divider to resize a
/// column, double-click it to fit the column to what the list shows, and
/// right-click the titles to show or hide columns or reset the widths.
/// `id` names the list (one per list in the window).
pub fn header(
    ui: &mut Ui,
    r: Rect,
    id: &str,
    cols: &[Col],
    saved: &mut Columns,
    t: &Tokens,
) -> Table {
    let id = Id::new(("taw-table", id));
    // Last frame's content widths, for fitting; this frame's start afresh.
    let seen: Seen = ui
        .ctx()
        .data_mut(|d| d.remove_temp::<Seen>(id.with("seen")))
        .unwrap_or_default();

    // Right-click anywhere on the titles: show or hide, and reset.
    let strip = ui.interact(r, id.with("strip"), Sense::click());
    strip.context_menu(|ui| {
        ui.style_mut().override_font_id = Some(hifi::ui_font(12.5));
        let mut any = false;
        for c in cols.iter().filter(|c| c.can_hide) {
            any = true;
            let mut on = !saved.hidden.contains(c.key);
            if ui.checkbox(&mut on, title_case(c.title)).changed() {
                if on {
                    saved.hidden.remove(c.key);
                } else {
                    saved.hidden.insert(c.key.to_string());
                }
            }
        }
        if any {
            ui.separator();
        }
        if ui.button("Reset column widths").clicked() {
            saved.widths.clear();
            ui.close();
        }
    });

    let mut spans = spans(cols, saved, r.left(), r.width());
    let shown: Vec<&Col> = cols
        .iter()
        .filter(|c| !c.can_hide || !saved.hidden.contains(c.key))
        .collect();

    // The dividers.
    let mut changed = false;
    for i in 0..spans.len() {
        let x = spans[i].right;
        if i + 1 == spans.len() && x >= r.right() - 1.0 {
            continue;
        }
        let Some((target, sign)) = divider_target(&shown, i) else {
            continue;
        };
        let zone = Rect::from_center_size(Pos2::new(x, r.center().y), Vec2::new(GRIP, r.height()));
        let resp = ui
            .interact(zone, id.with(("divider", i)), Sense::click_and_drag())
            .on_hover_cursor(CursorIcon::ResizeColumn);
        let c = shown[target];
        let now = spans[target].right - spans[target].left;
        if resp.dragged() {
            let w = (now + sign * resp.drag_delta().x).max(c.min);
            saved.widths.insert(c.key.to_string(), w);
            changed = true;
        }
        if resp.double_clicked() {
            let title = ui
                .painter()
                .layout_no_wrap(c.title.to_uppercase(), hifi::semi(10.5), t.dim)
                .size()
                .x;
            let content = seen.get(c.key).copied().unwrap_or(0.0).max(title);
            saved
                .widths
                .insert(c.key.to_string(), (content + 2.0 * PAD).ceil().max(c.min));
            changed = true;
        }
        let ink = if resp.dragged() {
            t.accent
        } else if resp.hovered() {
            t.dim
        } else {
            t.line_soft
        };
        ui.painter().line_segment(
            [
                Pos2::new(x.round() - 0.5, r.top() + 8.0),
                Pos2::new(x.round() - 0.5, r.bottom() - 8.0),
            ],
            Stroke::new(1.0, ink),
        );
    }
    if changed {
        spans = self::spans(cols, saved, r.left(), r.width());
    }

    let p = ui.painter();
    for s in &spans {
        let c = cols
            .iter()
            .find(|c| c.key == s.key)
            .expect("a shown column");
        if c.title.is_empty() {
            continue;
        }
        hifi::label_fit(
            p,
            s.pos(r.center().y),
            s.align(),
            &c.title.to_uppercase(),
            hifi::semi(10.5),
            t.dim,
            s.width().max(1.0),
        );
    }
    p.line_segment(
        [
            Pos2::new(r.left() + 1.0, r.bottom()),
            Pos2::new(r.right() - 1.0, r.bottom()),
        ],
        Stroke::new(1.0, t.line),
    );
    Table { id, spans }
}

/// "WHERE IT IS" → "Where it is", for the menu.
fn title_case(s: &str) -> String {
    let lower = s.to_lowercase();
    let mut c = lower.chars();
    match c.next() {
        Some(f) => f.to_uppercase().chain(c).collect(),
        None => String::new(),
    }
}

// ---------------------------------------------------------------- panes

/// Which side of the area a [`pane`] sits on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

/// How wide a pane is: its saved width (0 means its default), kept between
/// `min` and `max`.
pub fn pane_width(saved: f32, default: f32, min: f32, max: f32) -> f32 {
    let w = if saved > 0.0 { saved } else { default };
    w.clamp(min, max.max(min))
}

/// Splits `area` into a pane on one side, `gap` apart from the rest, and
/// lets the gap be dragged to resize the pane (a double-click puts it back
/// to `default`). `saved` is the pane's width as kept in [`Layout::panes`]:
/// 0 until it's dragged. Returns the pane and the rest.
#[allow(clippy::too_many_arguments)]
pub fn pane(
    ui: &mut Ui,
    area: Rect,
    id: &str,
    side: Side,
    saved: &mut f32,
    default: f32,
    (min, max): (f32, f32),
    gap: f32,
    t: &Tokens,
) -> (Rect, Rect) {
    let w = pane_width(*saved, default, min, max);
    let edge = match side {
        Side::Left => area.left() + w + gap / 2.0,
        Side::Right => area.right() - w - gap / 2.0,
    };
    let zone = Rect::from_center_size(
        Pos2::new(edge, area.center().y),
        Vec2::new(gap.max(GRIP), area.height()),
    );
    let resp = ui
        .interact(zone, Id::new(("taw-pane", id)), Sense::click_and_drag())
        .on_hover_cursor(CursorIcon::ResizeHorizontal);
    let mut w = w;
    if resp.dragged() {
        let dx = resp.drag_delta().x;
        w = pane_width(
            w + if side == Side::Left { dx } else { -dx },
            default,
            min,
            max,
        );
        *saved = w;
    }
    if resp.double_clicked() {
        *saved = 0.0;
        w = pane_width(0.0, default, min, max);
    }
    if resp.hovered() || resp.dragged() {
        let x = match side {
            Side::Left => area.left() + w + gap / 2.0,
            Side::Right => area.right() - w - gap / 2.0,
        };
        let ink = if resp.dragged() { t.accent } else { t.line };
        ui.painter().line_segment(
            [
                Pos2::new(x.round() - 0.5, area.top() + 20.0),
                Pos2::new(x.round() - 0.5, area.bottom()),
            ],
            Stroke::new(2.0, ink),
        );
    }
    match side {
        Side::Left => (
            Rect::from_min_max(area.min, Pos2::new(area.left() + w, area.bottom())),
            Rect::from_min_max(Pos2::new(area.left() + w + gap, area.top()), area.max),
        ),
        Side::Right => (
            Rect::from_min_max(Pos2::new(area.right() - w, area.top()), area.max),
            Rect::from_min_max(area.min, Pos2::new(area.right() - w - gap, area.bottom())),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COLS: [Col; 4] = [
        Col::new("n", "#", 40.0).right().fixed(),
        Col::new("title", "Title", 200.0).flex().min(120.0),
        Col::new("artist", "Artist", 160.0).min(60.0),
        Col::new("time", "Time", 70.0).right().min(50.0),
    ];

    fn widths(s: &[Span]) -> Vec<f32> {
        s.iter().map(|s| s.right - s.left).collect()
    }

    #[test]
    fn the_flexible_column_takes_the_rest() {
        let s = spans(&COLS, &Columns::default(), 10.0, 600.0);
        assert_eq!(widths(&s), vec![40.0, 330.0, 160.0, 70.0]);
        assert_eq!(s[0].left, 10.0);
        assert_eq!(s[3].right, 610.0);
        assert_eq!(s[3].x(), 600.0);
        assert_eq!(s[1].x(), 60.0);
    }

    #[test]
    fn saved_widths_and_hidden_columns() {
        let mut c = Columns::default();
        c.widths.insert("artist".into(), 240.0);
        c.widths.insert("time".into(), 10.0); // under its minimum
        let s = spans(&COLS, &c, 0.0, 600.0);
        assert_eq!(widths(&s), vec![40.0, 270.0, 240.0, 50.0]);
        c.hidden.insert("artist".into());
        c.hidden.insert("title".into()); // the flexible column can't hide
        c.hidden.insert("n".into()); // nor a fixed one
        let s = spans(&COLS, &c, 0.0, 600.0);
        assert_eq!(
            s.iter().map(|s| s.key).collect::<Vec<_>>(),
            ["n", "title", "time"]
        );
        assert_eq!(widths(&s), vec![40.0, 510.0, 50.0]);
    }

    #[test]
    fn narrow_lists_squeeze_from_the_right() {
        // 40 + 120 (title's min) + 160 + 70 = 390: 90 too many in 300.
        let s = spans(&COLS, &Columns::default(), 0.0, 300.0);
        assert_eq!(widths(&s), vec![40.0, 120.0, 90.0, 50.0]);
        // Too narrow even at the minimums: everything at its minimum.
        let s = spans(&COLS, &Columns::default(), 0.0, 100.0);
        assert_eq!(widths(&s), vec![40.0, 120.0, 60.0, 50.0]);
    }

    #[test]
    fn dividers_follow_the_mouse() {
        let shown: Vec<&Col> = COLS.iter().collect();
        assert_eq!(divider_target(&shown, 0), None); // the fixed "#"
        assert_eq!(divider_target(&shown, 1), Some((2, -1.0)));
        assert_eq!(divider_target(&shown, 2), Some((3, -1.0)));
        assert_eq!(divider_target(&shown, 3), None);
        // Before the flexible column, the one to the divider's left.
        let cols = [Col::new("a", "A", 100.0), Col::new("b", "B", 100.0).flex()];
        let shown: Vec<&Col> = cols.iter().collect();
        assert_eq!(divider_target(&shown, 0), Some((0, 1.0)));
    }

    #[test]
    fn panes_keep_within_their_limits() {
        assert_eq!(pane_width(0.0, 210.0, 150.0, 400.0), 210.0);
        assert_eq!(pane_width(500.0, 210.0, 150.0, 400.0), 400.0);
        assert_eq!(pane_width(90.0, 210.0, 150.0, 400.0), 150.0);
        // A window too small for the maximum still gives the minimum.
        assert_eq!(pane_width(300.0, 210.0, 150.0, 100.0), 150.0);
    }

    #[test]
    fn layouts_save_and_load() {
        let mut l = Layout::default();
        *l.pane_mut("artists") = 260.0;
        l.list_mut("tracks").hidden.insert("format".into());
        let json = serde_json::to_string(&l).unwrap();
        let back: Layout = serde_json::from_str(&json).unwrap();
        assert_eq!(back, l);
        // Older or partial settings load with defaults.
        let empty: Layout = serde_json::from_str("{}").unwrap();
        assert_eq!(empty, Layout::default());
        let partial: Layout = serde_json::from_str(r#"{"lists":{"a":{"hidden":["x"]}}}"#).unwrap();
        assert!(partial.lists["a"].widths.is_empty());
    }

    #[test]
    fn menu_titles() {
        assert_eq!(title_case("WHERE IT IS"), "Where it is");
        assert_eq!(title_case(""), "");
    }
}
