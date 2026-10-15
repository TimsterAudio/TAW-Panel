//! The TAW look for the hi-fi apps (Library, Player, Equaliser, Analyser):
//! a calm, modern faceplate in a choice of finishes, each with a light and a
//! dark build, and the controls drawn in it. The Jupiter look in [`theme`]
//! and [`widgets`] stays for the Box instruments.
//!
//! - [`Look`]: the user's choice of finish, build and accent (saved by apps).
//!   The default is Spool's look: its warm panels and seven colour themes.
//! - [`Tokens`]: the colours a look resolves to, for drawing
//! - [`install_fonts`]: Hanken Grotesk for text, Chivo Mono for numbers,
//!   Michroma for the logo
//! - the nameplate (badge, logo, model line), window buttons, dragging and
//!   resizing a window without a system title bar
//! - buttons, switches, segmented controls, menus, knobs, sliders, faders,
//!   display windows, wells and meters
//!
//! [`theme`]: crate::theme
//! [`widgets`]: crate::widgets

use crate::theme::{F_LOGO, F_MAKER, F_MAKER_BOLD};
use crate::widgets::{slanted_text, text, Spec};
use egui::epaint::Mesh;
use egui::text::{LayoutJob, TextFormat};
use egui::viewport::ResizeDirection;
use egui::{
    Align2, Color32, CornerRadius, CursorIcon, FontData, FontDefinitions, FontFamily, FontId, Id,
    Painter, Pos2, Rect, Response, Sense, Shape, Stroke, StrokeKind, Ui, Vec2, ViewportCommand,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ---------------------------------------------------------------- looks

/// The faceplate's material.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Finish {
    /// Spool's look (the default): warm panels tinted by the colour theme,
    /// as in Spool's Dark and Light modes. Settings saved while this was
    /// the brushed-aluminium "Anodised" finish load as Spool.
    #[default]
    #[serde(alias = "Anodised")]
    Spool,
    /// Matte white, after Braun; Graphite in the dark build.
    Porcelain,
}

impl Finish {
    pub const ALL: [Finish; 2] = [Finish::Spool, Finish::Porcelain];
    pub fn name(self) -> &'static str {
        match self {
            Finish::Spool => "Spool",
            Finish::Porcelain => "Porcelain",
        }
    }
}

/// Light, dark, or following the computer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Build {
    #[default]
    Light,
    Dark,
    /// Follows the computer's light or dark setting.
    System,
}

impl Build {
    pub const ALL: [Build; 3] = [Build::Light, Build::Dark, Build::System];
    pub fn name(self) -> &'static str {
        match self {
            Build::Light => "Light",
            Build::Dark => "Dark",
            Build::System => "Automatic",
        }
    }
}

/// The accent colour: the finish's own, or one of Spool's colour themes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Accent {
    /// The finish's own: Porcelain's orange, or Spool's amber.
    Finish,
    Neutral,
    #[default]
    Amber,
    Green,
    Blue,
    Red,
    Purple,
    Teal,
}

impl Accent {
    pub const ALL: [Accent; 8] = [
        Accent::Amber,
        Accent::Green,
        Accent::Blue,
        Accent::Red,
        Accent::Purple,
        Accent::Teal,
        Accent::Neutral,
        Accent::Finish,
    ];
    /// The choices that make sense with `finish`: Spool's seven, plus
    /// Porcelain's orange on Porcelain.
    pub fn choices(finish: Finish) -> &'static [Accent] {
        match finish {
            Finish::Spool => &Accent::ALL[..7],
            Finish::Porcelain => &Accent::ALL,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Accent::Finish => "Orange",
            Accent::Neutral => "Neutral",
            Accent::Amber => "Amber",
            Accent::Green => "Green",
            Accent::Blue => "Blue",
            Accent::Red => "Red",
            Accent::Purple => "Purple",
            Accent::Teal => "Teal",
        }
    }
    /// (dark build, light build), from Spool's colour themes.
    fn spool(self) -> Option<(Color32, Color32)> {
        Some(match self {
            Accent::Finish => return None,
            Accent::Neutral => (hex(0xbdb9b0), hex(0x6d6a63)),
            Accent::Amber => (hex(0xe8a33d), hex(0xb8791f)),
            Accent::Green => (hex(0x6bbf6b), hex(0x3f8f46)),
            Accent::Blue => (hex(0x5a9fe6), hex(0x3568b0)),
            Accent::Red => (hex(0xe2685f), hex(0xb8392f)),
            Accent::Purple => (hex(0xa684e8), hex(0x7050b0)),
            Accent::Teal => (hex(0x4fbdb2), hex(0x23827a)),
        })
    }
}

/// One of Spool's palettes: background, panel, raised panel, line, text,
/// dim text, accent and the accent's dim tint.
struct SpoolPalette([u32; 8]);

impl Accent {
    /// Spool's palette for this colour theme, (dark, light).
    fn palette(self) -> (SpoolPalette, SpoolPalette) {
        let p = SpoolPalette;
        match self {
            Accent::Amber | Accent::Finish => (
                p([
                    0x26221d, 0x2c2a25, 0x39342d, 0x524636, 0xece6db, 0x929090, 0xe8a33d, 0x6b5427,
                ]),
                p([
                    0xeee6d4, 0xf6efe2, 0xe8ddc5, 0xd6c3a1, 0x27281f, 0x7c735b, 0xb8791f, 0xe3cf9e,
                ]),
            ),
            Accent::Green => (
                p([
                    0x1c2420, 0x222c29, 0x2b3732, 0x394c3f, 0xe8e7dc, 0x899293, 0x6bbf6b, 0x2f4a30,
                ]),
                p([
                    0xe5e7d7, 0xecf1e5, 0xdbdfca, 0xbdc8a9, 0x242820, 0x74755d, 0x3f8f46, 0xcfe8cf,
                ]),
            ),
            Accent::Blue => (
                p([
                    0x1b222a, 0x212933, 0x293340, 0x354558, 0xe8e6e0, 0x88909c, 0x5a9fe6, 0x2c4a66,
                ]),
                p([
                    0xe4e4e0, 0xebedee, 0xdadbd5, 0xbbc0be, 0x242723, 0x737265, 0x3568b0, 0xcddcf0,
                ]),
            ),
            Accent::Red => (
                p([
                    0x251d20, 0x2c2528, 0x382d31, 0x503a3d, 0xece4dc, 0x918c92, 0xe2685f, 0x5a2c29,
                ]),
                p([
                    0xeee0d6, 0xf6eae3, 0xe8d6c7, 0xd6b7a5, 0x27261f, 0x7c6f5c, 0xb8392f, 0xf0d2cd,
                ]),
            ),
            Accent::Purple => (
                p([
                    0x21202a, 0x272733, 0x313040, 0x444058, 0xeae5e0, 0x8d8e9c, 0xa684e8, 0x453263,
                ]),
                p([
                    0xe9e2e0, 0xf0ebee, 0xe0d8d5, 0xc7bbbe, 0x252623, 0x777165, 0x7050b0, 0xe2d7f5,
                ]),
            ),
            Accent::Teal => (
                p([
                    0x1a2526, 0x202c2e, 0x28373a, 0x334c4d, 0xe7e7df, 0x879398, 0x4fc2b0, 0x254843,
                ]),
                p([
                    0xe3e7db, 0xebf0e9, 0xd9decf, 0xbac6b3, 0x232822, 0x737461, 0x2e8577, 0xc9ece6,
                ]),
            ),
            Accent::Neutral => (
                p([
                    0x232529, 0x2a2d31, 0x35383d, 0x4b4e53, 0xebe7e0, 0x8f939a, 0xc6cad0, 0x484b50,
                ]),
                p([
                    0xe7e3da, 0xeeece8, 0xded9cd, 0xc2beaf, 0x252721, 0x76715f, 0x585c64, 0xd6d9dd,
                ]),
            ),
        }
    }
}

/// What the user chose. Apps keep this in their saved settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Look {
    pub finish: Finish,
    pub build: Build,
    pub accent: Accent,
}

/// The colours a [`Look`] resolves to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tokens {
    pub dark: bool,
    pub finish: Finish,
    /// The faceplate, top and bottom of its gradient.
    pub face_top: Color32,
    pub face_bottom: Color32,
    /// Fine vertical brushing (no finish uses it at present).
    pub brushed: bool,
    pub text: Color32,
    pub dim: Color32,
    /// Buttons and the selected part of a segmented control.
    pub raised: Color32,
    /// The recessed background of lists and fields.
    pub well: Color32,
    pub line: Color32,
    pub line_soft: Color32,
    pub accent: Color32,
    pub on_accent: Color32,
    /// A selected row.
    pub select: Color32,
    /// Slider and switch tracks.
    pub groove: Color32,
    /// Slider thumbs.
    pub thumb: Color32,
    /// The transport/control bar along the bottom.
    pub bar: Color32,
    /// The thin line round the window.
    pub edge: Color32,
    /// The display window: background, ink, and the ink's glow.
    pub display_bg: Color32,
    pub ink: Color32,
    pub glow: Color32,
    pub ok: Color32,
    pub warn: Color32,
}

const fn hex(v: u32) -> Color32 {
    Color32::from_rgb((v >> 16) as u8, (v >> 8) as u8, v as u8)
}

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(r, g, b, (a * 255.0).round() as u8)
}

impl Tokens {
    /// `system_dark` is the computer's setting, used by [`Build::System`].
    pub fn new(look: &Look, system_dark: bool) -> Tokens {
        let dark = match look.build {
            Build::Light => false,
            Build::Dark => true,
            Build::System => system_dark,
        };
        let black = |a| rgba(0, 0, 0, a);
        let white = |a| rgba(255, 255, 255, a);
        let oled = (hex(0x050607), hex(0xf2f5f7), white(0.22));
        let mut t = match (look.finish, dark) {
            (Finish::Spool, _) => {
                let (d, l) = look.accent.palette();
                let SpoolPalette(c) = if dark { d } else { l };
                let [bg, panel, raised, line, text, dim, accent, accent_dim] = c.map(hex);
                Tokens {
                    dark,
                    finish: look.finish,
                    face_top: bg,
                    face_bottom: bg,
                    brushed: false,
                    text,
                    dim,
                    raised,
                    well: panel,
                    line,
                    line_soft: line.gamma_multiply(0.45),
                    accent,
                    on_accent: if dark { bg } else { Color32::WHITE },
                    select: accent_dim,
                    groove: accent_dim,
                    thumb: if dark { text } else { Color32::WHITE },
                    bar: panel,
                    edge: line,
                    // Spool's display: a panel with the readout in the accent.
                    display_bg: panel,
                    ink: accent,
                    glow: Color32::TRANSPARENT,
                    ok: hex(if dark { 0x6bbf6b } else { 0x3f8f46 }),
                    warn: hex(if dark { 0xe3a043 } else { 0xb0492f }),
                }
            }
            (Finish::Porcelain, false) => Tokens {
                dark,
                finish: look.finish,
                face_top: hex(0xf2f1ed),
                face_bottom: hex(0xf2f1ed),
                brushed: false,
                text: hex(0x1f1f1d),
                dim: hex(0x6c6b66),
                raised: Color32::WHITE,
                well: hex(0xfbfaf7),
                line: black(0.10),
                line_soft: black(0.05),
                accent: hex(0xd9542b),
                on_accent: Color32::WHITE,
                select: Color32::TRANSPARENT,
                groove: black(0.10),
                thumb: Color32::WHITE,
                bar: Color32::TRANSPARENT,
                edge: black(0.26),
                display_bg: hex(0xe3e2dc),
                ink: hex(0x262624),
                glow: Color32::TRANSPARENT,
                ok: hex(0x23955a),
                warn: hex(0xc07a18),
            },
            (Finish::Porcelain, true) => Tokens {
                dark,
                finish: look.finish,
                face_top: hex(0x1d1e1f),
                face_bottom: hex(0x1d1e1f),
                brushed: false,
                text: hex(0xecebe7),
                dim: hex(0x9a9893),
                raised: hex(0x2a2b2d),
                well: hex(0x161718),
                line: white(0.09),
                line_soft: white(0.045),
                accent: hex(0xff7043),
                on_accent: hex(0x1f0b04),
                select: Color32::TRANSPARENT,
                groove: white(0.10),
                thumb: hex(0xecebe7),
                bar: Color32::TRANSPARENT,
                edge: white(0.16),
                display_bg: oled.0,
                ink: oled.1,
                glow: oled.2,
                ok: hex(0x2fbf71),
                warn: hex(0xe0a040),
            },
        };
        if look.finish == Finish::Porcelain {
            if let Some((d, l)) = look.accent.spool() {
                t.accent = if dark { d } else { l };
                t.on_accent = if dark { hex(0x15130f) } else { Color32::WHITE };
            }
            let a = t.accent;
            t.select = rgba(a.r(), a.g(), a.b(), if dark { 0.18 } else { 0.14 });
        }
        t
    }

    /// The look's tokens, following the computer for [`Build::System`].
    pub fn from_ctx(ctx: &egui::Context, look: &Look) -> Tokens {
        let system_dark = ctx.system_theme() == Some(egui::Theme::Dark);
        Tokens::new(look, system_dark)
    }

    /// Grid lines and other faint marks in the display window.
    pub fn ink_faint(&self, a: f32) -> Color32 {
        self.ink.gamma_multiply(a)
    }
}

// ---------------------------------------------------------------- type

pub const F_UI: &str = "ui";
pub const F_UI_SEMI: &str = "ui-semibold";
pub const F_UI_BOLD: &str = "ui-bold";
pub const F_NUM: &str = "num";

pub fn ui_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(F_UI.into()))
}
pub fn semi(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(F_UI_SEMI.into()))
}
pub fn bold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(F_UI_BOLD.into()))
}
pub fn num(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(F_NUM.into()))
}

/// The hi-fi faces, plus the logo and maker faces the badge uses. Plain
/// egui text (menus, tooltips) uses Hanken Grotesk.
pub fn install_fonts(ctx: &egui::Context) {
    let mut defs = FontDefinitions::default();
    let faces: [(&str, &'static [u8]); 7] = [
        (
            F_UI,
            include_bytes!("../assets/fonts/HankenGrotesk-Regular.ttf"),
        ),
        (
            F_UI_SEMI,
            include_bytes!("../assets/fonts/HankenGrotesk-SemiBold.ttf"),
        ),
        (
            F_UI_BOLD,
            include_bytes!("../assets/fonts/HankenGrotesk-Bold.ttf"),
        ),
        (
            F_NUM,
            include_bytes!("../assets/fonts/ChivoMono-Regular.ttf"),
        ),
        (
            F_LOGO,
            include_bytes!("../assets/fonts/Michroma-Regular.ttf"),
        ),
        (
            F_MAKER,
            include_bytes!("../assets/fonts/SpaceGrotesk-Medium.ttf"),
        ),
        (
            F_MAKER_BOLD,
            include_bytes!("../assets/fonts/SpaceGrotesk-Bold.ttf"),
        ),
    ];
    for (name, bytes) in faces {
        defs.font_data
            .insert(name.to_owned(), Arc::new(FontData::from_static(bytes)));
        let mut chain = vec![name.to_owned()];
        chain.extend(defs.families[&FontFamily::Proportional].iter().cloned());
        defs.families.insert(FontFamily::Name(name.into()), chain);
    }
    defs.families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, F_UI.to_owned());
    defs.families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, F_NUM.to_owned());
    ctx.set_fonts(defs);
}

/// egui's own widgets (menus, tooltips, text fields) in the look's colours.
pub fn apply_visuals(ctx: &egui::Context, t: &Tokens) {
    let mut v = if t.dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    v.panel_fill = t.face_top;
    v.window_fill = t.raised;
    v.window_stroke = Stroke::new(1.0, t.line);
    v.extreme_bg_color = t.well;
    v.faint_bg_color = t.well;
    v.override_text_color = Some(t.text);
    v.selection.bg_fill = t.select;
    v.selection.stroke = Stroke::new(1.0, t.accent);
    v.hyperlink_color = t.accent;
    v.widgets.hovered.weak_bg_fill = t.select;
    v.widgets.hovered.bg_fill = t.select;
    v.widgets.active.weak_bg_fill = t.select;
    v.window_corner_radius = CornerRadius::same(8);
    v.menu_corner_radius = CornerRadius::same(8);
    if ctx.global_style().visuals != v {
        ctx.set_visuals(v);
    }
}

/// Plain text in a face and colour.
pub fn label(p: &Painter, pos: Pos2, align: Align2, s: &str, font: FontId, c: Color32) -> Rect {
    text(p, pos, align, s, font, c, 0.0)
}

/// Text cut to `max_w` with "…".
pub fn label_fit(
    p: &Painter,
    pos: Pos2,
    align: Align2,
    s: &str,
    font: FontId,
    c: Color32,
    max_w: f32,
) -> Rect {
    crate::lists::fit(p, pos, align, s, font, c, max_w)
}

fn text_width(p: &Painter, s: &str, font: FontId) -> f32 {
    p.layout_no_wrap(s.to_string(), font, Color32::WHITE)
        .size()
        .x
}

// ---------------------------------------------------------------- surfaces

fn vgradient(p: &Painter, r: Rect, top: Color32, bottom: Color32) {
    let mut mesh = Mesh::default();
    mesh.colored_vertex(r.left_top(), top);
    mesh.colored_vertex(r.right_top(), top);
    mesh.colored_vertex(r.right_bottom(), bottom);
    mesh.colored_vertex(r.left_bottom(), bottom);
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    p.add(Shape::mesh(mesh));
}

/// The faceplate: the finish's surface, and the thin edge round it.
pub fn faceplate(p: &Painter, r: Rect, t: &Tokens) {
    vgradient(p, r, t.face_top, t.face_bottom);
    if t.brushed {
        let (hi, lo) = if t.dark {
            (rgba(255, 255, 255, 0.025), rgba(0, 0, 0, 0.05))
        } else {
            (rgba(255, 255, 255, 0.10), rgba(0, 0, 0, 0.025))
        };
        let mut x = r.left();
        let mut i = 0u32;
        while x < r.right() {
            // A little irregularity reads as brushing rather than stripes.
            let c = if (i * 7 + i / 3) % 5 < 2 { hi } else { lo };
            p.line_segment(
                [Pos2::new(x, r.top()), Pos2::new(x, r.bottom())],
                Stroke::new(1.0, c),
            );
            x += 2.0;
            i += 1;
        }
        // Light catching the top.
        vgradient(
            p,
            Rect::from_min_size(r.min, Vec2::new(r.width(), r.height() * 0.3)),
            rgba(255, 255, 255, if t.dark { 0.03 } else { 0.10 }),
            rgba(255, 255, 255, 0.0),
        );
    }
    p.rect_stroke(r, 0, Stroke::new(1.0, t.edge), StrokeKind::Inside);
}

/// The control bar along the bottom of an app, above `r`'s bottom edge.
pub fn bar(p: &Painter, r: Rect, t: &Tokens) {
    p.rect_filled(r, 0, t.bar);
    p.line_segment([r.left_top(), r.right_top()], Stroke::new(1.0, t.line));
}

/// A recessed area for lists and cards.
pub fn well(p: &Painter, r: Rect, t: &Tokens) {
    p.rect_filled(r, CornerRadius::same(8), t.well);
    p.rect_stroke(
        r,
        CornerRadius::same(8),
        Stroke::new(1.0, t.line),
        StrokeKind::Inside,
    );
}

/// A display window (graph, spectrum, time): dark glass on most finishes,
/// paper on light Porcelain.
pub fn display(p: &Painter, r: Rect, t: &Tokens) {
    p.rect_filled(r, CornerRadius::same(8), t.display_bg);
    // The glass sits slightly below the faceplate.
    p.rect_filled(
        Rect::from_min_size(r.min, Vec2::new(r.width(), 3.0)),
        CornerRadius {
            nw: 8,
            ne: 8,
            sw: 0,
            se: 0,
        },
        rgba(0, 0, 0, if t.dark { 0.30 } else { 0.06 }),
    );
    p.rect_stroke(
        r,
        CornerRadius::same(8),
        Stroke::new(1.0, t.line),
        StrokeKind::Outside,
    );
}

// ---------------------------------------------------------------- nameplate

/// The badge, logo and model line, as on every Timster's Audio Works unit:
/// "Timster's Audio Works - Analyser", ANALYSER, "SP-001 · SPECTRUM
/// ANALYSER · V0.2.0". Returns the space it took.
pub fn nameplate(ui: &Ui, at: Pos2, t: &Tokens, name: &str, model_line: &str) -> Rect {
    let badge = badge_at(ui.painter(), at, t, name);
    let l = logo(
        ui,
        Pos2::new(at.x, badge.bottom() + 4.0),
        t,
        name,
        model_line,
    );
    Rect::from_min_max(at, Pos2::new(badge.right().max(l.right()), l.bottom()))
}

/// The "Timster's Audio Works - Player" badge on its own, centred on
/// `centre_top` (the middle of its top edge): for a nameplate whose badge
/// sits in the middle of the window's top edge, with [`logo`] at the left.
/// Returns the space it took.
pub fn badge(p: &Painter, centre_top: Pos2, t: &Tokens, name: &str) -> Rect {
    let w = badge_galley(p, t, name).size().x + 18.0;
    badge_at(p, Pos2::new(centre_top.x - w / 2.0, centre_top.y), t, name)
}

/// The logo and model line without the badge (ANALYSER, "SP-001 ·
/// SPECTRUM ANALYSER · V0.2.0"), from `at`. Returns the space it took.
pub fn logo(ui: &Ui, at: Pos2, t: &Tokens, name: &str, model_line: &str) -> Rect {
    let p = ui.painter();
    let logo_at = Pos2::new(at.x + 3.0, at.y);
    slanted_text(
        ui,
        logo_at,
        &name.to_uppercase(),
        crate::theme::font(F_LOGO, 21.0),
        t.text,
        3.5,
        12.0,
    );
    let ml = text(
        p,
        Pos2::new(at.x, logo_at.y + 33.0),
        Align2::LEFT_TOP,
        &model_line.to_uppercase(),
        semi(9.5),
        t.dim,
        1.9,
    );
    Rect::from_min_max(at, Pos2::new(ml.right().max(at.x), ml.bottom()))
}

fn badge_galley(p: &Painter, t: &Tokens, name: &str) -> std::sync::Arc<egui::Galley> {
    let mut job = LayoutJob::default();
    job.append(
        "Timster's Audio Works - ",
        0.0,
        TextFormat {
            font_id: ui_font(12.0),
            color: t.dim,
            ..Default::default()
        },
    );
    job.append(
        name,
        0.0,
        TextFormat {
            font_id: bold(12.0),
            color: t.text,
            ..Default::default()
        },
    );
    p.layout_job(job)
}

fn badge_at(p: &Painter, at: Pos2, t: &Tokens, name: &str) -> Rect {
    let g = badge_galley(p, t, name);
    let badge = Rect::from_min_size(at, g.size() + Vec2::new(18.0, 6.0));
    p.rect_stroke(
        badge,
        CornerRadius::same(4),
        Stroke::new(1.0, t.line),
        StrokeKind::Inside,
    );
    p.galley(badge.min + Vec2::new(9.0, 3.0), g, t.text);
    badge
}

// ---------------------------------------------------------------- window

/// How far in from the window's edge a drag resizes it.
pub const RESIZE_MARGIN: f32 = 6.0;

/// Minimise, maximise and close, drawn in the finish, with their top right
/// corner at `top_right`. Returns the space they took.
pub fn window_buttons(ui: &mut Ui, top_right: Pos2, t: &Tokens) -> Rect {
    let size = Vec2::new(30.0, 24.0);
    let gap = 4.0;
    let maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    let all = Rect::from_min_max(
        Pos2::new(top_right.x - size.x * 3.0 - gap * 2.0, top_right.y),
        Pos2::new(top_right.x, top_right.y + size.y),
    );
    for i in 0..3 {
        let r = Rect::from_min_size(
            Pos2::new(all.left() + i as f32 * (size.x + gap), all.top()),
            size,
        );
        let resp = ui
            .interact(r, Id::new(("taw-window", i)), Sense::click())
            .on_hover_text(match i {
                0 => "Minimise",
                1 if maximized => "Restore",
                1 => "Maximise",
                _ => "Close",
            });
        let p = ui.painter();
        let close = i == 2;
        if resp.hovered() {
            let bg = if close { hex(0xd9443a) } else { t.raised };
            p.rect_filled(r, CornerRadius::same(6), bg);
        }
        let c = if close && resp.hovered() {
            Color32::WHITE
        } else if close {
            t.text
        } else {
            t.dim
        };
        let s = Stroke::new(1.4, c);
        let m = r.center();
        match i {
            0 => {
                p.line_segment([m + Vec2::new(-5.0, 0.5), m + Vec2::new(5.0, 0.5)], s);
            }
            1 => {
                if maximized {
                    p.rect_stroke(
                        Rect::from_center_size(m + Vec2::new(-1.5, 1.5), Vec2::splat(8.0)),
                        CornerRadius::same(1),
                        s,
                        StrokeKind::Middle,
                    );
                    p.line_segment([m + Vec2::new(-1.5, -4.5), m + Vec2::new(4.5, -4.5)], s);
                    p.line_segment([m + Vec2::new(4.5, -4.5), m + Vec2::new(4.5, 1.5)], s);
                } else {
                    p.rect_stroke(
                        Rect::from_center_size(m, Vec2::splat(9.0)),
                        CornerRadius::same(1),
                        s,
                        StrokeKind::Middle,
                    );
                }
            }
            _ => {
                p.line_segment([m + Vec2::new(-4.5, -4.5), m + Vec2::new(4.5, 4.5)], s);
                p.line_segment([m + Vec2::new(4.5, -4.5), m + Vec2::new(-4.5, 4.5)], s);
            }
        }
        if resp.clicked() {
            let cmd = match i {
                0 => ViewportCommand::Minimized(true),
                1 => ViewportCommand::Maximized(!maximized),
                _ => ViewportCommand::Close,
            };
            ui.ctx().send_viewport_cmd(cmd);
        }
    }
    all
}

/// Makes `r` move the window when dragged, and maximise it when
/// double-clicked. Call it before drawing the controls on top of `r`, so
/// they still get their clicks.
pub fn drag_to_move(ui: &mut Ui, r: Rect) {
    let resp = ui.interact(r, Id::new("taw-window-drag"), Sense::click_and_drag());
    if resp.drag_started_by(egui::PointerButton::Primary) {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
        // As in `resize_edges`: the window manager keeps the release.
        ui.ctx().stop_dragging();
    }
    if resp.double_clicked() {
        let maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!maximized));
    }
}

/// Lets the window's edges and corners resize it, when it has no system
/// frame. Call it last, so the edges win over anything drawn beneath.
pub fn resize_edges(ui: &mut Ui, r: Rect) {
    let (maximized, fullscreen) = ui.input(|i| {
        (
            i.viewport().maximized.unwrap_or(false),
            i.viewport().fullscreen.unwrap_or(false),
        )
    });
    if maximized || fullscreen {
        return;
    }
    let e = RESIZE_MARGIN;
    let c = e * 2.5;
    use ResizeDirection::*;
    // Edges first, then the corners over them: egui gives a press to the
    // last zone added within reach, so a corner must come after the two
    // edges it overlaps or only one edge would move.
    let zones = [
        (
            Rect::from_min_max(r.min, Pos2::new(r.right(), r.top() + e)),
            North,
            CursorIcon::ResizeVertical,
        ),
        (
            Rect::from_min_max(Pos2::new(r.left(), r.bottom() - e), r.max),
            South,
            CursorIcon::ResizeVertical,
        ),
        (
            Rect::from_min_max(r.min, Pos2::new(r.left() + e, r.bottom())),
            West,
            CursorIcon::ResizeHorizontal,
        ),
        (
            Rect::from_min_max(Pos2::new(r.right() - e, r.top()), r.max),
            East,
            CursorIcon::ResizeHorizontal,
        ),
        (
            Rect::from_min_size(r.min, Vec2::splat(c)),
            NorthWest,
            CursorIcon::ResizeNorthWest,
        ),
        (
            Rect::from_min_size(Pos2::new(r.right() - c, r.top()), Vec2::splat(c)),
            NorthEast,
            CursorIcon::ResizeNorthEast,
        ),
        (
            Rect::from_min_size(Pos2::new(r.left(), r.bottom() - c), Vec2::splat(c)),
            SouthWest,
            CursorIcon::ResizeSouthWest,
        ),
        (
            Rect::from_min_size(r.max - Vec2::splat(c), Vec2::splat(c)),
            SouthEast,
            CursorIcon::ResizeSouthEast,
        ),
    ];
    for (i, (zone, dir, cursor)) in zones.into_iter().enumerate() {
        let resp = ui
            .interact(zone, Id::new(("taw-resize", i)), Sense::drag())
            .on_hover_cursor(cursor);
        if resp.drag_started_by(egui::PointerButton::Primary) {
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::BeginResize(dir));
            // The window manager takes the pointer from here and keeps the
            // button's release, so egui would think this zone was still
            // being dragged and ignore the next press on any zone.
            ui.ctx().stop_dragging();
        }
    }
}

// ---------------------------------------------------------------- controls

/// A text button; `primary` fills it with the accent.
pub fn button(ui: &mut Ui, r: Rect, id: Id, t: &Tokens, label_s: &str, primary: bool) -> Response {
    let resp = ui
        .interact(r, id, Sense::click())
        .on_hover_cursor(CursorIcon::PointingHand);
    let p = ui.painter();
    let down = resp.is_pointer_button_down_on();
    let (bg, fg) = if primary {
        (t.accent, t.on_accent)
    } else {
        (t.raised, t.text)
    };
    let bg = if resp.hovered() && !down {
        crate::theme::lerp_colour(
            bg,
            if t.dark {
                Color32::WHITE
            } else {
                Color32::BLACK
            },
            0.06,
        )
    } else {
        bg
    };
    p.rect_filled(r, CornerRadius::same(8), bg);
    if !primary {
        p.rect_stroke(
            r,
            CornerRadius::same(8),
            Stroke::new(1.0, t.line),
            StrokeKind::Inside,
        );
    }
    if resp.has_focus() {
        p.rect_stroke(
            r.expand(2.0),
            CornerRadius::same(9),
            Stroke::new(2.0, t.accent),
            StrokeKind::Outside,
        );
    }
    let font = if primary { semi(12.5) } else { ui_font(12.5) };
    label(p, r.center(), Align2::CENTER_CENTER, label_s, font, fg);
    resp
}

/// How wide [`button`] wants to be for a label.
pub fn button_width(ui: &Ui, label_s: &str) -> f32 {
    text_width(ui.painter(), label_s, semi(12.5)) + 24.0
}

/// An on/off switch. Returns its response; the caller flips the value when
/// it's clicked.
pub fn switch(ui: &mut Ui, r: Rect, id: Id, t: &Tokens, on: bool) -> Response {
    let resp = ui
        .interact(r, id, Sense::click())
        .on_hover_cursor(CursorIcon::PointingHand);
    let p = ui.painter();
    let h = r.height();
    let rad = (h / 2.0) as u8;
    p.rect_filled(
        r,
        CornerRadius::same(rad),
        if on { t.accent } else { t.groove },
    );
    let d = h - 4.0;
    let x = if on {
        r.right() - 2.0 - d / 2.0
    } else {
        r.left() + 2.0 + d / 2.0
    };
    let c = Pos2::new(x, r.center().y);
    p.circle_filled(c + Vec2::new(0.0, 0.8), d / 2.0, rgba(0, 0, 0, 0.25));
    p.circle_filled(c, d / 2.0, t.thumb);
    if resp.has_focus() {
        p.rect_stroke(
            r.expand(2.0),
            CornerRadius::same(rad + 2),
            Stroke::new(2.0, t.accent),
            StrokeKind::Outside,
        );
    }
    resp
}

/// A switch with its label to the right. Flips `on` when clicked.
pub fn labelled_switch(ui: &mut Ui, at: Pos2, id: Id, t: &Tokens, on: &mut bool, s: &str) -> Rect {
    let sw = Rect::from_min_size(at, Vec2::new(32.0, 18.0));
    let w = text_width(ui.painter(), s, ui_font(12.0));
    let hit = Rect::from_min_size(at, Vec2::new(40.0 + w, 18.0));
    let resp = switch(ui, sw, id, t, *on);
    let lr = ui.interact(hit, id.with("label"), Sense::click());
    if resp.clicked() || lr.clicked() {
        *on = !*on;
    }
    label(
        ui.painter(),
        Pos2::new(sw.right() + 8.0, sw.center().y),
        Align2::LEFT_CENTER,
        s,
        ui_font(12.0),
        t.dim,
    );
    hit
}

/// How a segmented control shows its choice.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Seg {
    /// The chosen part is raised (Simple / Detailed).
    Plain,
    /// The chosen part is filled with the accent (Curve / Bars / LEDs).
    Accent,
    /// Round, with a coloured dot before each item (the family switcher).
    Pill,
}

/// A row of choices. `at` is the left (or, with `right`, the right) end of
/// the control's vertical middle. Returns its rect and a newly picked index.
#[allow(clippy::too_many_arguments)]
pub fn segmented(
    ui: &mut Ui,
    at: Pos2,
    right: bool,
    id: Id,
    t: &Tokens,
    items: &[&str],
    selected: usize,
    kind: Seg,
    dots: &[Color32],
    dimmed: &[bool],
) -> (Rect, Option<usize>) {
    let font = ui_font(12.5);
    let pad = if kind == Seg::Pill { 12.0 } else { 10.0 };
    let dot_w = if dots.is_empty() { 0.0 } else { 13.0 };
    let h = if kind == Seg::Pill { 30.0 } else { 26.0 };
    let widths: Vec<f32> = items
        .iter()
        .map(|s| text_width(ui.painter(), s, semi(12.5)) + pad * 2.0 + dot_w)
        .collect();
    let inset = if kind == Seg::Pill { 3.0 } else { 0.0 };
    let total: f32 = widths.iter().sum::<f32>() + inset * 2.0;
    let left = if right { at.x - total } else { at.x };
    let r = Rect::from_min_size(Pos2::new(left, at.y - h / 2.0), Vec2::new(total, h));
    let rad = if kind == Seg::Pill {
        (h / 2.0) as u8
    } else {
        8
    };
    {
        let p = ui.painter();
        p.rect_filled(r, CornerRadius::same(rad), t.well);
        p.rect_stroke(
            r,
            CornerRadius::same(rad),
            Stroke::new(1.0, t.line),
            StrokeKind::Inside,
        );
    }
    let mut picked = None;
    let mut x = r.left() + inset;
    for (i, (s, w)) in items.iter().zip(&widths).enumerate() {
        let cell = Rect::from_min_size(
            Pos2::new(x, r.top() + inset),
            Vec2::new(*w, h - inset * 2.0),
        );
        let resp = ui
            .interact(cell, id.with(i), Sense::click())
            .on_hover_cursor(CursorIcon::PointingHand);
        let p = ui.painter();
        let on = i == selected;
        let (bg, fg) = match (on, kind) {
            (true, Seg::Accent) => (Some(t.accent), t.on_accent),
            (true, _) => (Some(t.raised), t.text),
            (false, _) => (
                resp.hovered().then_some(t.line_soft),
                if dimmed.get(i).copied().unwrap_or(false) {
                    t.dim.gamma_multiply(0.6)
                } else {
                    t.dim
                },
            ),
        };
        if let Some(bg) = bg {
            let cr = if kind == Seg::Pill {
                CornerRadius::same(((h - inset * 2.0) / 2.0) as u8)
            } else {
                // Only the outer corners of the end cells are rounded.
                let first = i == 0;
                let last = i + 1 == items.len();
                CornerRadius {
                    nw: if first { 8 } else { 0 },
                    sw: if first { 8 } else { 0 },
                    ne: if last { 8 } else { 0 },
                    se: if last { 8 } else { 0 },
                }
            };
            if on && kind == Seg::Pill {
                p.rect_filled(cell.translate(Vec2::new(0.0, 1.0)), cr, rgba(0, 0, 0, 0.12));
            }
            p.rect_filled(cell, cr, bg);
        }
        let mut tx = cell.left() + pad;
        if let Some(c) = dots.get(i) {
            p.circle_filled(Pos2::new(tx + 3.5, cell.center().y), 3.5, *c);
            tx += dot_w;
        }
        let f = if on { semi(12.5) } else { font.clone() };
        label(
            p,
            Pos2::new(tx, cell.center().y),
            Align2::LEFT_CENTER,
            s,
            f,
            fg,
        );
        if resp.clicked() && !on {
            picked = Some(i);
        }
        x += w;
    }
    (r, picked)
}

/// The family switcher: Library, Player, Equaliser, Analyser. The current
/// app is raised; apps that aren't installed are dimmed. Returns the one
/// clicked.
pub fn family_switch(
    ui: &mut Ui,
    right_middle: Pos2,
    t: &Tokens,
    current: usize,
    installed: [bool; 4],
) -> (Rect, Option<usize>) {
    let dots = [hex(0x2aa79b), t.accent, hex(0xe0a030), hex(0x9b7be0)];
    let dimmed = installed.map(|i| !i);
    let (r, picked) = segmented(
        ui,
        right_middle,
        true,
        Id::new("taw-family"),
        t,
        &["Library", "Player", "Equaliser", "Analyser"],
        current,
        Seg::Pill,
        &dots,
        &dimmed,
    );
    (r, picked)
}

/// A square icon button with a gear, for settings.
pub fn gear_button(ui: &mut Ui, r: Rect, id: Id, t: &Tokens, active: bool) -> Response {
    let resp = ui
        .interact(r, id, Sense::click())
        .on_hover_cursor(CursorIcon::PointingHand)
        .on_hover_text("Settings");
    let p = ui.painter();
    p.rect_filled(
        r,
        CornerRadius::same(8),
        if active {
            t.raised
        } else if resp.hovered() {
            t.line_soft
        } else {
            Color32::TRANSPARENT
        },
    );
    p.rect_stroke(
        r,
        CornerRadius::same(8),
        Stroke::new(1.0, t.line),
        StrokeKind::Inside,
    );
    let c = r.center();
    let col = if active { t.text } else { t.dim };
    for k in 0..8 {
        let a = k as f32 * std::f32::consts::TAU / 8.0;
        let d = Vec2::new(a.cos(), a.sin());
        p.line_segment([c + d * 4.0, c + d * 7.0], Stroke::new(2.4, col));
    }
    p.circle_stroke(c, 4.6, Stroke::new(2.0, col));
    resp
}

/// A drop-down menu. Returns the newly picked index.
pub fn select(
    ui: &mut Ui,
    r: Rect,
    id: &str,
    t: &Tokens,
    items: &[String],
    selected: usize,
) -> Option<usize> {
    let mut picked = None;
    let current = items.get(selected).cloned().unwrap_or_else(|| "—".into());
    ui.scope_builder(egui::UiBuilder::new().max_rect(r), |ui| {
        let style = ui.style_mut();
        for w in [
            &mut style.visuals.widgets.inactive,
            &mut style.visuals.widgets.hovered,
            &mut style.visuals.widgets.active,
            &mut style.visuals.widgets.open,
        ] {
            w.bg_fill = t.well;
            w.weak_bg_fill = t.well;
            w.fg_stroke = Stroke::new(1.0, t.text);
            w.bg_stroke = Stroke::new(1.0, t.line);
            w.corner_radius = CornerRadius::same(8);
            w.expansion = 0.0;
        }
        style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, t.dim);
        style.override_font_id = Some(ui_font(12.5));
        style.spacing.button_padding = Vec2::new(10.0, 5.0);
        egui::ComboBox::from_id_salt(id)
            .width(r.width())
            .truncate()
            .selected_text(egui::RichText::new(current).color(t.text))
            .show_ui(ui, |ui| {
                for (i, item) in items.iter().enumerate() {
                    if ui.selectable_label(i == selected, item).clicked() && i != selected {
                        picked = Some(i);
                    }
                }
            });
    });
    picked
}

/// Drag, wheel and keyboard adjustment shared by knobs and sliders.
/// `along` is the drag direction that raises the value, in points per full
/// travel.
fn adjust(ui: &Ui, resp: &mut Response, v: &mut f32, spec: &Spec, along: Vec2) {
    let before = *v;
    let fine = ui.input(|i| i.modifiers.shift);
    if resp.dragged() {
        let d = resp.drag_delta();
        let travel = along.length().max(1.0);
        let dir = along / travel;
        let mut dt = (d.x * dir.x + d.y * dir.y) / travel;
        if fine {
            dt *= 0.15;
        }
        let tn = (spec.norm(*v) + dt).clamp(0.0, 1.0);
        *v = spec.denorm(tn);
        if spec.step > 0.0 {
            // Snap only on release-ish granularity: keep it responsive.
            *v = spec.snap(*v);
        }
    }
    if resp.hovered() {
        let wheel = crate::widgets::wheel(ui);
        if wheel != 0.0 {
            let step = if fine { 0.005 } else { 0.03 };
            let tn = (spec.norm(*v) + wheel.signum() * step).clamp(0.0, 1.0);
            *v = spec.snap(spec.denorm(tn));
        }
    }
    if resp.double_clicked() {
        *v = spec.default;
    }
    if resp.has_focus() {
        let (up, down) = ui.input(|i| {
            (
                i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::ArrowRight),
                i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::ArrowLeft),
            )
        });
        let d = up as i32 - down as i32;
        if d != 0 {
            let tn = (spec.norm(*v) + d as f32 * if fine { 0.005 } else { 0.02 }).clamp(0.0, 1.0);
            *v = spec.snap(spec.denorm(tn));
        }
    }
    if *v != before {
        resp.mark_changed();
    }
}

/// A small knob: an arc in the accent, a value under it and a label.
#[allow(clippy::too_many_arguments)]
pub fn knob(
    ui: &mut Ui,
    centre: Pos2,
    id: Id,
    t: &Tokens,
    v: &mut f32,
    spec: &Spec,
    value_s: &str,
    label_s: &str,
) -> Response {
    let r = 16.0;
    let hit = Rect::from_center_size(centre, Vec2::splat(r * 2.0 + 6.0));
    let mut resp = ui
        .interact(hit, id, Sense::click_and_drag())
        .on_hover_cursor(CursorIcon::ResizeVertical);
    adjust(ui, &mut resp, v, spec, Vec2::new(0.0, -160.0));
    let p = ui.painter();
    let tn = spec.norm(*v);
    let a0 = 135f32.to_radians();
    let a1 = 405f32.to_radians();
    let arc = |from: f32, to: f32| -> Vec<Pos2> {
        let n = ((to - from).abs() / 0.08).ceil().max(1.0) as usize;
        (0..=n)
            .map(|k| {
                let a = from + (to - from) * k as f32 / n as f32;
                centre + Vec2::new(a.cos(), a.sin()) * r
            })
            .collect()
    };
    p.add(Shape::line(arc(a0, a1), Stroke::new(3.0, t.groove)));
    let a = a0 + (a1 - a0) * tn;
    if tn > 0.002 {
        p.add(Shape::line(arc(a0, a), Stroke::new(3.0, t.accent)));
    }
    p.circle_filled(centre + Vec2::new(0.0, 1.0), 10.5, rgba(0, 0, 0, 0.18));
    p.circle_filled(centre, 10.5, t.raised);
    p.circle_stroke(centre, 10.5, Stroke::new(1.0, t.line));
    let tip = centre + Vec2::new(a.cos(), a.sin()) * 8.0;
    p.line_segment([centre, tip], Stroke::new(2.0, t.text));
    if resp.has_focus() {
        p.circle_stroke(centre, r + 4.0, Stroke::new(2.0, t.accent));
    }
    label(
        p,
        Pos2::new(centre.x, centre.y + r + 5.0),
        Align2::CENTER_TOP,
        value_s,
        num(11.0),
        t.text,
    );
    label(
        p,
        Pos2::new(centre.x, centre.y + r + 19.0),
        Align2::CENTER_TOP,
        label_s,
        ui_font(10.5),
        t.dim,
    );
    resp
}

/// A vertical fader: the track fills from `zero` (0 dB) to the value.
pub fn vfader(
    ui: &mut Ui,
    r: Rect,
    id: Id,
    t: &Tokens,
    v: &mut f32,
    spec: &Spec,
    zero: f32,
) -> Response {
    let mut resp = ui
        .interact(r.expand2(Vec2::new(8.0, 6.0)), id, Sense::click_and_drag())
        .on_hover_cursor(CursorIcon::ResizeVertical);
    adjust(ui, &mut resp, v, spec, Vec2::new(0.0, -r.height()));
    let p = ui.painter();
    let track = Rect::from_center_size(r.center(), Vec2::new(6.0, r.height()));
    p.rect_filled(track, CornerRadius::same(3), t.groove);
    let y_of = |val: f32| r.bottom() - spec.norm(val) * r.height();
    let (yv, yz) = (y_of(*v), y_of(zero));
    p.line_segment(
        [
            Pos2::new(track.left() - 6.0, yz),
            Pos2::new(track.right() + 6.0, yz),
        ],
        Stroke::new(1.0, t.dim.gamma_multiply(0.6)),
    );
    let fill = Rect::from_min_max(
        Pos2::new(track.left(), yv.min(yz)),
        Pos2::new(track.right(), yv.max(yz)),
    );
    p.rect_filled(fill, CornerRadius::same(3), t.accent);
    let thumb = Rect::from_center_size(Pos2::new(r.center().x, yv), Vec2::new(24.0, 12.0));
    p.rect_filled(
        thumb.translate(Vec2::new(0.0, 1.0)),
        CornerRadius::same(4),
        rgba(0, 0, 0, 0.25),
    );
    p.rect_filled(thumb, CornerRadius::same(4), t.thumb);
    p.rect_stroke(
        thumb,
        CornerRadius::same(4),
        Stroke::new(1.0, t.line),
        StrokeKind::Inside,
    );
    if resp.has_focus() {
        p.rect_stroke(
            thumb.expand(3.0),
            CornerRadius::same(6),
            Stroke::new(2.0, t.accent),
            StrokeKind::Outside,
        );
    }
    resp
}

/// A horizontal slider (volume, position).
pub fn hslider(ui: &mut Ui, r: Rect, id: Id, t: &Tokens, v: &mut f32, spec: &Spec) -> Response {
    let mut resp = ui
        .interact(r.expand2(Vec2::new(4.0, 8.0)), id, Sense::click_and_drag())
        .on_hover_cursor(CursorIcon::PointingHand);
    adjust(ui, &mut resp, v, spec, Vec2::new(r.width(), 0.0));
    if resp.clicked() {
        if let Some(pos) = resp.interact_pointer_pos() {
            let tn = ((pos.x - r.left()) / r.width()).clamp(0.0, 1.0);
            *v = spec.snap(spec.denorm(tn));
            resp.mark_changed();
        }
    }
    let p = ui.painter();
    let track = Rect::from_center_size(r.center(), Vec2::new(r.width(), 6.0));
    p.rect_filled(track, CornerRadius::same(3), t.groove);
    let x = r.left() + spec.norm(*v) * r.width();
    p.rect_filled(
        Rect::from_min_max(track.min, Pos2::new(x, track.bottom())),
        CornerRadius::same(3),
        t.accent,
    );
    let c = Pos2::new(x, r.center().y);
    p.circle_filled(c + Vec2::new(0.0, 1.0), 8.0, rgba(0, 0, 0, 0.25));
    p.circle_filled(c, 8.0, t.thumb);
    if resp.has_focus() {
        p.circle_stroke(c, 11.0, Stroke::new(2.0, t.accent));
    }
    resp
}

/// A one-row segmented level meter from −48 to 0 dBFS: green, amber above
/// −12 dB, red above −3 dB.
pub fn meter(p: &Painter, r: Rect, t: &Tokens, db: f32) {
    const SEGS: usize = METER_SEGMENTS;
    let (w, gap) = meter_segment(r);
    let lit = ((db - METER_FLOOR_DB) / -METER_FLOOR_DB * SEGS as f32).ceil() as i32;
    for s in 0..SEGS {
        let seg_db = METER_FLOOR_DB - METER_FLOOR_DB * (s as f32 + 1.0) / SEGS as f32;
        let base = if seg_db > -3.0 {
            hex(0xff4a30)
        } else if seg_db > -12.0 {
            hex(0xefc23b)
        } else {
            hex(0x4fd06a)
        };
        let on = (s as i32) < lit;
        let c = if on {
            base
        } else {
            base.gamma_multiply(if t.dark { 0.16 } else { 0.22 })
        };
        p.rect_filled(
            Rect::from_min_size(
                Pos2::new(r.left() + s as f32 * (w + gap), r.top()),
                Vec2::new(w, r.height()),
            ),
            CornerRadius::same(1),
            c,
        );
    }
}

/// The level at the left end of `meter`; it lights from here up to 0 dB.
pub const METER_FLOOR_DB: f32 = -48.0;
const METER_SEGMENTS: usize = 24;

/// The levels `meter_scale` marks, in dB, in the order they're kept when
/// a narrow meter has no room for all of them.
pub const METER_MARKS: [f32; 6] = [0.0, -48.0, -24.0, -12.0, -6.0, -3.0];

/// A meter segment's width and the gap after it, for a meter drawn in `r`.
fn meter_segment(r: Rect) -> (f32, f32) {
    let gap = 2.0;
    let w = (r.width() - gap * (METER_SEGMENTS - 1) as f32) / METER_SEGMENTS as f32;
    (w, gap)
}

/// Where `db` falls along a meter drawn in `r`, following its segments and
/// the gaps between them: the left edge at the floor, the right edge at 0 dB.
pub fn meter_x(r: Rect, db: f32) -> f32 {
    let (w, gap) = meter_segment(r);
    let pos = ((db - METER_FLOOR_DB) / -METER_FLOOR_DB).clamp(0.0, 1.0) * METER_SEGMENTS as f32;
    if pos >= METER_SEGMENTS as f32 {
        return r.right();
    }
    let s = pos.floor();
    r.left() + s * (w + gap) + (pos - s) * w
}

/// A dB scale for `meter`: a tick and a figure at each of `METER_MARKS`.
/// `meter_r` is the meter it labels; the scale is drawn just under it and
/// takes about 12 px. On a narrow meter, figures that would touch one
/// already placed are left out (0 dB and the floor always stay).
pub fn meter_scale(p: &Painter, meter_r: Rect, t: &Tokens) {
    let font = num(9.0);
    let c = t.ink_faint(0.7);
    let top = meter_r.bottom() + 2.0;
    let mut placed: Vec<Rect> = Vec::new();
    for db in METER_MARKS {
        let x = meter_x(meter_r, db);
        let s = if db == 0.0 {
            "0".to_string()
        } else {
            format!("-{}", -db as i32)
        };
        let w = text_width(p, &s, font.clone());
        let (left, align) = if db >= 0.0 {
            (x - w, Align2::RIGHT_TOP)
        } else if db <= METER_FLOOR_DB {
            (x, Align2::LEFT_TOP)
        } else {
            (x - w / 2.0, Align2::CENTER_TOP)
        };
        let slot = Rect::from_min_size(Pos2::new(left - 3.0, top), Vec2::new(w + 6.0, 10.0));
        if placed.iter().any(|o| o.intersects(slot)) {
            continue;
        }
        placed.push(slot);
        p.line_segment(
            [Pos2::new(x, top), Pos2::new(x, top + 2.0)],
            Stroke::new(1.0, c),
        );
        label(p, Pos2::new(x, top + 2.5), align, &s, font.clone(), c);
    }
}

/// A status light with a soft halo.
pub fn status_light(p: &Painter, c: Pos2, colour: Color32) {
    p.circle_filled(c, 7.0, colour.gamma_multiply(0.18));
    p.circle_filled(c, 4.5, colour);
}

/// Ink text in a display window, with the finish's glow.
pub fn ink_text(p: &Painter, pos: Pos2, align: Align2, s: &str, font: FontId, t: &Tokens) -> Rect {
    if t.glow != Color32::TRANSPARENT {
        for d in [
            Vec2::new(-1.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, -1.0),
            Vec2::new(0.0, 1.0),
        ] {
            text(
                p,
                pos + d,
                align,
                s,
                font.clone(),
                t.glow.gamma_multiply(0.5),
                0.6,
            );
        }
    }
    text(p, pos, align, s, font, t.ink, 0.6)
}

/// A filled spectrum or response curve in one ink colour: a soft fill down
/// to `base_y` and a line along the top.
pub fn ink_curve(p: &Painter, pts: &[Pos2], base_y: f32, ink: Color32, fill: f32, line: f32) {
    if pts.len() < 2 {
        return;
    }
    if fill > 0.0 {
        let mut mesh = Mesh::default();
        let top = ink.gamma_multiply(fill);
        let bottom = ink.gamma_multiply(fill * 0.15);
        for (i, q) in pts.iter().enumerate() {
            mesh.colored_vertex(*q, top);
            mesh.colored_vertex(Pos2::new(q.x, base_y), bottom);
            if i > 0 {
                let k = (i as u32) * 2;
                mesh.add_triangle(k - 2, k - 1, k);
                mesh.add_triangle(k - 1, k + 1, k);
            }
        }
        p.add(Shape::mesh(mesh));
    }
    if line > 0.0 {
        p.add(Shape::line(pts.to_vec(), Stroke::new(line, ink)));
    }
}

// ---------------------------------------------------------------- settings drawer

/// A settings sheet sliding over the right of the window, with a title and
/// a close button. `add` fills it; return `true` from the sheet's close
/// button or a click outside to close it.
pub fn drawer(
    ctx: &egui::Context,
    t: &Tokens,
    title: &str,
    open: &mut bool,
    add: impl FnOnce(&mut Ui),
) {
    if !*open {
        return;
    }
    let screen = ctx.content_rect();
    let w = 360.0f32.min(screen.width());
    let rect = Rect::from_min_max(Pos2::new(screen.right() - w, screen.top()), screen.max);
    // Dim the app behind; a click there closes the sheet.
    egui::Area::new(Id::new("taw-drawer-shade"))
        .order(egui::Order::Middle)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let resp = ui.allocate_rect(screen, Sense::click());
            ui.painter()
                .rect_filled(screen, 0, rgba(0, 0, 0, if t.dark { 0.45 } else { 0.25 }));
            if resp.clicked() {
                *open = false;
            }
        });
    egui::Area::new(Id::new("taw-drawer"))
        .order(egui::Order::Foreground)
        .fixed_pos(rect.min)
        .show(ctx, |ui| {
            ui.set_min_size(rect.size());
            ui.set_max_size(rect.size());
            let p = ui.painter();
            p.rect_filled(rect, 0, t.raised);
            p.line_segment(
                [rect.left_top(), rect.left_bottom()],
                Stroke::new(1.0, t.line),
            );
            label(
                p,
                rect.min + Vec2::new(22.0, 22.0),
                Align2::LEFT_TOP,
                title,
                bold(18.0),
                t.text,
            );
            let close = Rect::from_min_size(
                Pos2::new(rect.right() - 46.0, rect.top() + 16.0),
                Vec2::new(30.0, 30.0),
            );
            let resp = ui
                .interact(close, Id::new("taw-drawer-close"), Sense::click())
                .on_hover_cursor(CursorIcon::PointingHand);
            let p = ui.painter();
            if resp.hovered() {
                p.rect_filled(close, CornerRadius::same(8), t.line_soft);
            }
            let m = close.center();
            let s = Stroke::new(1.6, t.text);
            p.line_segment([m + Vec2::new(-5.0, -5.0), m + Vec2::new(5.0, 5.0)], s);
            p.line_segment([m + Vec2::new(5.0, -5.0), m + Vec2::new(-5.0, 5.0)], s);
            if resp.clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                *open = false;
            }
            let body = Rect::from_min_max(
                rect.min + Vec2::new(22.0, 64.0),
                rect.max - Vec2::new(22.0, 16.0),
            );
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(body)
                    .layout(egui::Layout::top_down(egui::Align::Min)),
            );
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(&mut child, |ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(8.0, 10.0);
                    add(ui);
                });
        });
}

/// A heading inside the settings drawer.
pub fn drawer_heading(ui: &mut Ui, t: &Tokens, s: &str) {
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(s.to_uppercase())
            .font(semi(11.0))
            .color(t.dim),
    );
}

/// A row in the drawer: a label on the left, `width` × 26 for a control
/// on the right. Returns the control's rect.
pub fn drawer_row(ui: &mut Ui, t: &Tokens, s: &str, width: f32) -> Rect {
    let (row, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), Sense::hover());
    label(
        ui.painter(),
        Pos2::new(row.left(), row.center().y),
        Align2::LEFT_CENTER,
        s,
        ui_font(13.0),
        t.text,
    );
    Rect::from_min_size(
        Pos2::new(row.right() - width, row.center().y - 13.0),
        Vec2::new(width, 26.0),
    )
}

/// Finish, build and accent, for the drawer. Returns true if anything
/// changed.
pub fn look_settings(ui: &mut Ui, t: &Tokens, look: &mut Look) -> bool {
    let before = *look;
    drawer_heading(ui, t, "Look");
    let r = drawer_row(ui, t, "Finish", 190.0);
    let names: Vec<&str> = Finish::ALL.iter().map(|f| f.name()).collect();
    let sel = Finish::ALL
        .iter()
        .position(|f| *f == look.finish)
        .unwrap_or(0);
    if let (_, Some(i)) = segmented(
        ui,
        Pos2::new(r.right(), r.center().y),
        true,
        Id::new("look-finish"),
        t,
        &names,
        sel,
        Seg::Plain,
        &[],
        &[],
    ) {
        look.finish = Finish::ALL[i];
    }
    let r = drawer_row(ui, t, "Light or dark", 220.0);
    let names: Vec<&str> = Build::ALL.iter().map(|b| b.name()).collect();
    let sel = Build::ALL
        .iter()
        .position(|b| *b == look.build)
        .unwrap_or(0);
    if let (_, Some(i)) = segmented(
        ui,
        Pos2::new(r.right(), r.center().y),
        true,
        Id::new("look-build"),
        t,
        &names,
        sel,
        Seg::Plain,
        &[],
        &[],
    ) {
        look.build = Build::ALL[i];
    }
    let r = drawer_row(ui, t, "Colour", 170.0);
    let choices = Accent::choices(look.finish);
    let items: Vec<String> = choices.iter().map(|a| a.name().to_string()).collect();
    // On Spool, "the finish's own" is amber.
    let shown = if look.finish == Finish::Spool && look.accent == Accent::Finish {
        Accent::Amber
    } else {
        look.accent
    };
    let sel = choices.iter().position(|a| *a == shown).unwrap_or(0);
    if let Some(i) = select(ui, r, "look-accent", t, &items, sel) {
        look.accent = choices[i];
    }
    *look != before
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meter_x_follows_the_segments() {
        let r = Rect::from_min_size(Pos2::new(100.0, 0.0), Vec2::new(478.0, 10.0));
        assert_eq!(meter_x(r, METER_FLOOR_DB), r.left());
        assert_eq!(meter_x(r, 0.0), r.right());
        assert_eq!(meter_x(r, -90.0), r.left());
        assert_eq!(meter_x(r, 6.0), r.right());
        // 24 segments of 18 px with 2 px gaps: -24 dB is the start of the 13th.
        assert!((meter_x(r, -24.0) - (100.0 + 12.0 * 20.0)).abs() < 0.01);
        let xs: Vec<f32> = [-48.0, -24.0, -12.0, -6.0, -3.0, 0.0]
            .iter()
            .map(|&db| meter_x(r, db))
            .collect();
        assert!(xs.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn default_is_spool_light_amber() {
        let l = Look::default();
        assert_eq!(l.finish, Finish::Spool);
        assert_eq!(l.build, Build::Light);
        assert_eq!(l.accent, Accent::Amber);
        let t = Tokens::new(&l, true);
        assert!(!t.dark, "Light stays light whatever the computer says");
        assert_eq!(t.face_top, hex(0xeee6d4));
        assert_eq!(t.accent, hex(0xb8791f));
    }

    #[test]
    fn spool_themes_tint_the_whole_panel() {
        let green = Look {
            accent: Accent::Green,
            build: Build::Dark,
            ..Default::default()
        };
        let t = Tokens::new(&green, false);
        assert_eq!(t.face_top, hex(0x1c2420));
        assert_eq!(t.accent, hex(0x6bbf6b));
    }

    #[test]
    fn anodised_settings_load_as_spool() {
        let l: Look = serde_json::from_str(r#"{"finish":"Anodised","accent":"Neutral"}"#).unwrap();
        assert_eq!(l.finish, Finish::Spool);
        assert_eq!(l.accent, Accent::Neutral);
    }

    #[test]
    fn automatic_follows_the_computer() {
        let l = Look {
            build: Build::System,
            ..Default::default()
        };
        assert!(Tokens::new(&l, true).dark);
        assert!(!Tokens::new(&l, false).dark);
    }

    #[test]
    fn finish_accent_uses_the_finish_colour() {
        let l = Look {
            finish: Finish::Porcelain,
            accent: Accent::Finish,
            ..Default::default()
        };
        assert_eq!(Tokens::new(&l, false).accent, hex(0xd9542b));
    }

    #[test]
    fn saved_looks_tolerate_missing_fields() {
        let l: Look = serde_json::from_str(r#"{"finish":"Porcelain"}"#).unwrap();
        assert_eq!(l.finish, Finish::Porcelain);
        assert_eq!(l.accent, Accent::Amber);
    }
}
