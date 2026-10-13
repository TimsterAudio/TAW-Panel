//! The panel itself: its background, and fitting it to the window.

use crate::theme::Theme;
use egui::{Color32, Pos2, Rect, Stroke, Vec2};

/// The panel face: plain black with a lit top edge, or silver with a faint
/// brushed texture.
pub fn draw_panel_bg(p: &egui::Painter, rect: Rect, th: &Theme) {
    p.rect_filled(rect, 0, th.panel);
    if th.silver {
        let mut x = rect.left();
        while x < rect.right() {
            p.line_segment(
                [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
                Stroke::new(1.0, Color32::from_white_alpha(14)),
            );
            x += 3.0;
        }
    } else {
        p.rect_filled(
            Rect::from_min_size(rect.min, Vec2::new(rect.width(), 2.0)),
            0,
            Color32::from_white_alpha(8),
        );
    }
}

/// Scales the whole panel so `design` fits the window, as hardware would
/// rather than re-wrapping. `max_zoom` 1.0 means "never larger than
/// designed" (lists then get the extra room); larger values let a fixed
/// panel grow with the window.
pub fn fit_to_window(ctx: &egui::Context, design: Vec2, max_zoom: f32) {
    let zoom = ctx.zoom_factor();
    let screen = ctx.content_rect().size() * zoom;
    let want = (screen.x / design.x)
        .min(screen.y / design.y)
        .clamp(0.3, max_zoom);
    if (want - zoom).abs() > 0.002 {
        ctx.set_zoom_factor(want);
        ctx.request_repaint();
    }
}
