//! Every piece of the panel on one page, for checking the look:
//!
//!     cargo run --example gallery

use egui::{Align2, Id, Pos2, Rect, Vec2};
use taw_panel::frame::{draw_panel_bg, fit_to_window};
use taw_panel::lists::*;
use taw_panel::theme::*;
use taw_panel::widgets::*;

const DESIGN: Vec2 = Vec2::new(1100.0, 560.0);

#[derive(Default)]
struct Gallery {
    panel: PanelKind,
    knobs: KnobStyle,
    on: bool,
    lit: bool,
    level: f32,
    freq: f32,
    q: f32,
    sel: usize,
}

const ROWS: [(&str, &str, Light); 6] = [
    ("Radiohead", "OK Computer", Light::Ok),
    ("Portishead", "Dummy", Light::Ok),
    ("Kate Bush", "The Sensual World", Light::Warn),
    ("Massive Attack", "Protection", Light::Warn),
    ("Beatles", "The White Album", Light::Bad),
    ("Old iPod", "(offline)", Light::Off),
];

impl eframe::App for Gallery {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        fit_to_window(ui.ctx(), DESIGN, 4.0);
        let th = Theme::new(self.panel, self.knobs);
        let panel = Rect::from_min_size(ui.max_rect().min, DESIGN);
        draw_panel_bg(ui.painter(), ui.max_rect(), &th);
        let at = |x: f32, y: f32| panel.min + Vec2::new(x, y);

        slanted_text(
            ui,
            at(24.0, 18.0),
            "TAW PANEL",
            font(F_LOGO, 30.0),
            th.legend,
            3.0,
            12.0,
        );
        text(
            ui.painter(),
            at(22.0, 60.0),
            Align2::LEFT_TOP,
            "GALLERY · EVERY PIECE OF THE PANEL",
            font(F_LEGEND, 10.0),
            th.legend_dim,
            2.4,
        );
        lcd(
            ui.painter(),
            Rect::from_min_size(at(300.0, 18.0), Vec2::new(300.0, 52.0)),
            &[
                ("Timster's Audio Works".into(), "Panel".into()),
                ("Last touched".into(), format!("{:.1} dB", self.level)),
            ],
        );
        let small = |x: f32| Rect::from_min_size(at(x, 40.0), Vec2::new(50.0, 24.0));
        for (i, (k, label)) in [(PanelKind::Black, "Black"), (PanelKind::Silver, "Silver")]
            .into_iter()
            .enumerate()
        {
            if key(
                ui,
                small(660.0 + i as f32 * 58.0),
                Id::new(("panel", i)),
                K_GREY,
                Some(self.panel == k),
                label,
                &th,
            )
            .clicked()
            {
                self.panel = k;
            }
        }
        for (i, (k, label)) in [
            (KnobStyle::Colour, "Colour"),
            (KnobStyle::Pointer, "Pointer"),
        ]
        .into_iter()
        .enumerate()
        {
            if key(
                ui,
                small(800.0 + i as f32 * 58.0),
                Id::new(("knobs", i)),
                K_GREY,
                Some(self.knobs == k),
                label,
                &th,
            )
            .clicked()
            {
                self.knobs = k;
            }
        }
        if key(
            ui,
            Rect::from_min_size(at(960.0, 38.0), Vec2::new(96.0, 30.0)),
            Id::new("run"),
            K_RED,
            Some(self.on),
            "Run / Stop",
            &th,
        )
        .clicked()
        {
            self.on = !self.on;
        }
        stripe(
            ui.painter(),
            Rect::from_min_size(at(22.0, 100.0), Vec2::new(DESIGN.x - 44.0, 6.0)),
        );

        // Keys
        let keys = Rect::from_min_size(at(22.0, 130.0), Vec2::new(330.0, 120.0));
        section(ui.painter(), keys, "Keys", &th, false);
        for (i, (c, n)) in [
            (K_RED, "Red"),
            (K_ORANGE, "Orange"),
            (K_YELLOW, "Yellow"),
            (K_CREAM, "Cream"),
            (K_GREY, "Grey"),
        ]
        .into_iter()
        .enumerate()
        {
            key(
                ui,
                Rect::from_min_size(
                    keys.min + Vec2::new(16.0 + i as f32 * 62.0, 34.0),
                    Vec2::new(50.0, 24.0),
                ),
                Id::new(("k", i)),
                c,
                Some(i == 0 && self.on),
                n,
                &th,
            );
        }
        if lit_key(
            ui,
            Rect::from_min_size(keys.min + Vec2::new(16.0, 88.0), Vec2::new(40.0, 18.0)),
            Id::new("lit"),
            ON_GREEN,
            self.lit,
            "",
            &th,
        )
        .clicked()
        {
            self.lit = !self.lit;
        }
        text(
            ui.painter(),
            keys.min + Vec2::new(70.0, 97.0),
            Align2::LEFT_CENTER,
            "LIT KEY",
            font(F_LEGEND, 11.0),
            th.legend,
            1.3,
        );

        // Controls
        let ctl = Rect::from_min_size(at(22.0, 272.0), Vec2::new(330.0, 260.0));
        section(ui.painter(), ctl, "Controls", &th, false);
        let gain = Spec {
            min: -24.0,
            max: 12.0,
            default: 0.0,
            log: false,
            step: 0.1,
        };
        let freq = Spec {
            min: 20.0,
            max: 20_000.0,
            default: 1000.0,
            log: true,
            step: 0.0,
        };
        let q = Spec {
            min: 0.1,
            max: 20.0,
            default: 0.7,
            log: true,
            step: 0.0,
        };
        if self.freq == 0.0 {
            self.freq = 1000.0;
            self.q = 0.7;
        }
        fader(
            ui,
            Rect::from_min_size(ctl.min + Vec2::new(20.0, 26.0), Vec2::new(36.0, 200.0)),
            Id::new("fader"),
            &mut self.level,
            &gain,
            None,
            &th,
        );
        readout(
            ui.painter(),
            ctl.min + Vec2::new(38.0, 230.0),
            &format!("{:+.1} dB", self.level),
            60.0,
        );
        knob(
            ui,
            ctl.min + Vec2::new(130.0, 80.0),
            22.0,
            Id::new("freq"),
            &mut self.freq,
            &freq,
            Some(FAM_TONE),
            &th,
        );
        legend(
            ui.painter(),
            ctl.min + Vec2::new(130.0, 112.0),
            Align2::CENTER_TOP,
            "Freq",
            &th,
        );
        readout(
            ui.painter(),
            ctl.min + Vec2::new(130.0, 128.0),
            &format!("{:.0} Hz", self.freq),
            60.0,
        );
        knob(
            ui,
            ctl.min + Vec2::new(230.0, 80.0),
            22.0,
            Id::new("q"),
            &mut self.q,
            &q,
            Some(FAM_TYPE),
            &th,
        );
        legend(
            ui.painter(),
            ctl.min + Vec2::new(230.0, 112.0),
            Align2::CENTER_TOP,
            "Q",
            &th,
        );
        readout(
            ui.painter(),
            ctl.min + Vec2::new(230.0, 128.0),
            &format!("{:.2}", self.q),
            60.0,
        );
        meter(
            ui.painter(),
            Rect::from_min_size(ctl.min + Vec2::new(90.0, 180.0), Vec2::new(220.0, 22.0)),
            [self.level - 6.0, self.level - 9.0],
        );
        ladder(
            ui.painter(),
            Rect::from_min_size(ctl.min + Vec2::new(90.0, 214.0), Vec2::new(220.0, 14.0)),
            0.71,
            AMBER,
        );

        // Lists
        let lr = Rect::from_min_max(at(372.0, 130.0), at(DESIGN.x - 22.0, 532.0));
        section(ui.painter(), lr, "Lists", &th, false);
        let inner = Rect::from_min_max(
            lr.min + Vec2::new(10.0, 16.0),
            lr.max - Vec2::new(10.0, 10.0),
        );
        well(ui.painter(), inner, &th);
        let head = Rect::from_min_size(inner.min, Vec2::new(inner.width(), 28.0));
        header(
            ui.painter(),
            head,
            &[
                (inner.left() + 30.0, "Artist", Align2::LEFT_CENTER),
                (inner.left() + 230.0, "Album", Align2::LEFT_CENTER),
                (inner.left() + 470.0, "Needs", Align2::LEFT_CENTER),
            ],
            &th,
        );
        let list = Rect::from_min_max(Pos2::new(inner.left(), head.bottom() + 1.0), inner.max);
        let mut clicked = None;
        rows(
            ui,
            list,
            "rows",
            34.0,
            ROWS.len(),
            None,
            |ui, i, r, resp| {
                let (a, b, l) = ROWS[i];
                let p = ui.painter();
                row_bg(p, r, resp, i == self.sel, &th);
                light(p, Pos2::new(r.left() + 16.0, r.center().y), l, &th);
                fit(
                    p,
                    Pos2::new(r.left() + 30.0, r.center().y),
                    Align2::LEFT_CENTER,
                    a,
                    font(F_LEGEND_BOLD, 13.5),
                    th.legend,
                    190.0,
                );
                fit(
                    p,
                    Pos2::new(r.left() + 230.0, r.center().y),
                    Align2::LEFT_CENTER,
                    b,
                    font(F_LEGEND, 13.5),
                    th.legend,
                    220.0,
                );
                let x = r.left() + 470.0;
                match l {
                    Light::Ok => {
                        chip(p, Pos2::new(x, r.center().y), "Complete", ON_GREEN, false);
                    }
                    Light::Warn => {
                        chip(p, Pos2::new(x, r.center().y), "Art small", AMBER, false);
                    }
                    Light::Bad => {
                        let w = chip(p, Pos2::new(x, r.center().y), "Art", LED, false);
                        chip(
                            p,
                            Pos2::new(x + w + 4.0, r.center().y),
                            "Merge?",
                            AMBER,
                            true,
                        );
                    }
                    Light::Off => {}
                }
                if resp.clicked() {
                    clicked = Some(i);
                }
            },
        );
        if let Some(i) = clicked {
            self.sel = i;
        }
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "TAW Panel gallery",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size(DESIGN),
            ..Default::default()
        },
        Box::new(|cc| {
            install_fonts(&cc.egui_ctx);
            Ok(Box::new(Gallery::default()))
        }),
    )
}
