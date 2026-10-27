//! Every piece of the TAW look (`hifi`) on one page, for checking it:
//!
//!     cargo run --example gallery
//!     cargo run --example gallery -- porcelain dark
//!
//! The gear opens the look settings, as in the apps. The Jupiter look, for
//! the Box instruments, has its own page: `cargo run --example jupiter`.

use egui::{Align2, Id, Pos2, Rect, Vec2};
use taw_panel::hifi::{self, Build, Finish, Seg, Tokens};
use taw_panel::widgets::Spec;

const DESIGN: Vec2 = Vec2::new(1120.0, 680.0);

const LEVEL: Spec = Spec {
    min: -24.0,
    max: 12.0,
    default: 0.0,
    log: false,
    step: 0.1,
};
const FREQ: Spec = Spec {
    min: 20.0,
    max: 20_000.0,
    default: 1000.0,
    log: true,
    step: 0.0,
};
const Q: Spec = Spec {
    min: 0.1,
    max: 20.0,
    default: 0.7,
    log: true,
    step: 0.0,
};
const UNIT: Spec = Spec {
    min: 0.0,
    max: 1.0,
    default: 0.5,
    log: false,
    step: 0.0,
};

struct Gallery {
    look: hifi::Look,
    settings: bool,
    simple: bool,
    view: usize,
    on: bool,
    ink: bool,
    menu: usize,
    gain: f32,
    freq: f32,
    q: f32,
    fader: f32,
    position: f32,
    volume: f32,
    sel: usize,
}

const ROWS: [(&str, &str, &str); 5] = [
    ("Low Tide", "Mara Quell", "4:12"),
    ("Harbour Lights", "Mara Quell", "5:03"),
    ("Night Ferry", "Mara Quell", "3:47"),
    ("Salt Air", "Mara Quell feat. Ivo Lane", "4:29"),
    ("Lantern Row", "Mara Quell", "6:10"),
];

impl eframe::App for Gallery {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        taw_panel::frame::fit_to_window(&ctx, DESIGN, 1.5);
        let t = Tokens::from_ctx(&ctx, &self.look);
        hifi::apply_visuals(&ctx, &t);
        let screen = ui.max_rect();
        hifi::faceplate(ui.painter(), screen, &t);
        let o = screen.min.to_vec2();
        let at = |x: f32, y: f32| Pos2::new(x, y) + o;

        // The nameplate, window buttons, family switcher, view and gear.
        hifi::drag_to_move(
            ui,
            Rect::from_min_size(screen.min, Vec2::new(screen.width(), 96.0)),
        );
        hifi::nameplate(
            ui,
            at(24.0, 14.0),
            &t,
            "Panel",
            &format!("Gallery · the TAW look · V{}", env!("CARGO_PKG_VERSION")),
        );
        hifi::window_buttons(
            ui,
            Pos2::new(screen.right() - 10.0, screen.top() + 10.0),
            &t,
        );
        let row = screen.top() + 66.0;
        let gear = Rect::from_min_size(
            Pos2::new(screen.right() - 54.0, row - 15.0),
            Vec2::splat(30.0),
        );
        if hifi::gear_button(ui, gear, Id::new("gear"), &t, self.settings).clicked() {
            self.settings = !self.settings;
        }
        let (seg, picked) = hifi::segmented(
            ui,
            Pos2::new(gear.left() - 12.0, row),
            true,
            Id::new("simple"),
            &t,
            &["Simple", "Detailed"],
            if self.simple { 0 } else { 1 },
            Seg::Plain,
            &[],
            &[],
        );
        if let Some(i) = picked {
            self.simple = i == 0;
        }
        hifi::family_switch(
            ui,
            Pos2::new(seg.left() - 14.0, row),
            &t,
            1,
            [true, true, false, true],
        );

        // Left: the controls.
        let p = ui.painter();
        let head = |p: &egui::Painter, x: f32, y: f32, s: &str| {
            hifi::label(
                p,
                at(x, y),
                Align2::LEFT_TOP,
                &s.to_uppercase(),
                hifi::semi(10.5),
                t.dim,
            );
        };
        head(p, 24.0, 116.0, "Buttons");
        let mut x = 24.0;
        for (i, (s, primary)) in [
            ("Play in Player", true),
            ("Show folder", false),
            ("Clear", false),
        ]
        .into_iter()
        .enumerate()
        {
            let w = hifi::button_width(ui, s);
            let r = Rect::from_min_size(at(x, 138.0), Vec2::new(w, 30.0));
            hifi::button(ui, r, Id::new(("button", i)), &t, s, primary);
            x += w + 10.0;
        }

        head(ui.painter(), 24.0, 190.0, "Switches and choices");
        let sw = Rect::from_min_size(at(24.0, 216.0), Vec2::new(32.0, 18.0));
        if hifi::switch(ui, sw, Id::new("on"), &t, self.on).clicked() {
            self.on = !self.on;
        }
        hifi::labelled_switch(
            ui,
            at(76.0, 216.0),
            Id::new("ink"),
            &t,
            &mut self.ink,
            "Display ink",
        );
        let (_, picked) = hifi::segmented(
            ui,
            at(24.0, 266.0),
            false,
            Id::new("view"),
            &t,
            &["Curve", "Bars", "LEDs"],
            self.view,
            Seg::Accent,
            &[],
            &[],
        );
        if let Some(i) = picked {
            self.view = i;
        }
        let items: Vec<String> = ["System default", "Speakers", "Headphones"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        if let Some(i) = hifi::select(
            ui,
            Rect::from_min_size(at(250.0, 253.0), Vec2::new(200.0, 26.0)),
            "menu",
            &t,
            &items,
            self.menu,
        ) {
            self.menu = i;
        }

        head(ui.painter(), 24.0, 306.0, "Knobs, faders and sliders");
        let gain = format!("{:+.1} dB", self.gain);
        hifi::knob(
            ui,
            at(48.0, 356.0),
            Id::new("gain"),
            &t,
            &mut self.gain,
            &LEVEL,
            &gain,
            "Gain",
        );
        let freq = if self.freq >= 1000.0 {
            format!("{:.1} kHz", self.freq / 1000.0)
        } else {
            format!("{:.0} Hz", self.freq)
        };
        hifi::knob(
            ui,
            at(118.0, 356.0),
            Id::new("freq"),
            &t,
            &mut self.freq,
            &FREQ,
            &freq,
            "Freq",
        );
        let q = format!("{:.2}", self.q);
        hifi::knob(
            ui,
            at(188.0, 356.0),
            Id::new("q"),
            &t,
            &mut self.q,
            &Q,
            &q,
            "Q",
        );
        hifi::vfader(
            ui,
            Rect::from_min_size(at(250.0, 326.0), Vec2::new(24.0, 110.0)),
            Id::new("fader"),
            &t,
            &mut self.fader,
            &LEVEL,
            0.0,
        );
        hifi::hslider(
            ui,
            Rect::from_min_size(at(300.0, 372.0), Vec2::new(170.0, 18.0)),
            Id::new("volume"),
            &t,
            &mut self.volume,
            &UNIT,
        );

        head(ui.painter(), 24.0, 460.0, "Lights");
        let p = ui.painter();
        for (i, (c, s)) in [
            (t.ok, "Native rate"),
            (t.warn, "Resampled"),
            (t.dim, "Offline"),
        ]
        .into_iter()
        .enumerate()
        {
            let y = 492.0 + i as f32 * 26.0;
            hifi::status_light(p, at(30.0, y), c);
            hifi::label(
                p,
                at(44.0, y),
                Align2::LEFT_CENTER,
                s,
                hifi::ui_font(13.0),
                t.text,
            );
        }

        // Right: a display window, meters and a list.
        let d = Rect::from_min_size(at(520.0, 116.0), Vec2::new(576.0, 150.0));
        let p = ui.painter();
        hifi::display(p, d, &t);
        hifi::ink_text(
            p,
            d.min + Vec2::new(20.0, 18.0),
            Align2::LEFT_TOP,
            "1:55",
            hifi::num(40.0),
            &t,
        );
        hifi::label(
            p,
            d.min + Vec2::new(22.0, 72.0),
            Align2::LEFT_TOP,
            "ELAPSED · TRACK 02",
            hifi::num(11.0),
            t.ink_faint(0.7),
        );
        let mx = d.left() + 220.0;
        for (c, db) in [-9.0f32, -12.0].into_iter().enumerate() {
            let m = Rect::from_min_size(
                Pos2::new(mx, d.top() + 22.0 + c as f32 * 16.0),
                Vec2::new(d.right() - 20.0 - mx, 10.0),
            );
            hifi::meter(p, m, &t, db);
            if c == 1 {
                hifi::meter_scale(p, m, &t);
            }
        }
        let base = d.bottom() - 16.0;
        let pts: Vec<Pos2> = (0..=120)
            .map(|i| {
                let f = i as f32 / 120.0;
                let h = 34.0
                    + 22.0 * (-(f - 0.18).powi(2) / 0.01).exp()
                    + 10.0 * (f * 23.0).sin().abs() * (1.0 - f);
                Pos2::new(mx + f * (d.right() - 20.0 - mx), base - h)
            })
            .collect();
        if self.view == 0 {
            hifi::ink_curve(p, &pts, base, t.ink, 0.35, 1.6);
        } else {
            for q in pts.iter().step_by(4) {
                p.rect_filled(
                    Rect::from_min_max(Pos2::new(q.x - 4.0, q.y), Pos2::new(q.x + 4.0, base)),
                    1,
                    t.ink.gamma_multiply(0.85),
                );
            }
        }

        let list = Rect::from_min_size(at(520.0, 290.0), Vec2::new(576.0, 34.0 + 30.0 * 5.0));
        hifi::well(ui.painter(), list, &t);
        let p = ui.painter();
        for (x, s) in [
            (list.left() + 44.0, "TITLE"),
            (list.left() + 300.0, "ARTIST"),
        ] {
            hifi::label(
                p,
                Pos2::new(x, list.top() + 16.0),
                Align2::LEFT_CENTER,
                s,
                hifi::semi(10.5),
                t.dim,
            );
        }
        hifi::label(
            p,
            Pos2::new(list.right() - 16.0, list.top() + 16.0),
            Align2::RIGHT_CENTER,
            "TIME",
            hifi::semi(10.5),
            t.dim,
        );
        for (i, (title, artist, time)) in ROWS.iter().enumerate() {
            let r = Rect::from_min_size(
                Pos2::new(list.left() + 1.0, list.top() + 32.0 + i as f32 * 30.0),
                Vec2::new(list.width() - 2.0, 30.0),
            );
            let resp = ui.interact(r, Id::new(("row", i)), egui::Sense::click());
            if resp.clicked() {
                self.sel = i;
            }
            let p = ui.painter();
            if self.sel == i {
                p.rect_filled(r, 0, t.select);
            } else if resp.hovered() {
                p.rect_filled(r, 0, t.line_soft);
            }
            let c = if self.sel == i { t.accent } else { t.text };
            let y = r.center().y;
            hifi::label(
                p,
                Pos2::new(r.left() + 30.0, y),
                Align2::RIGHT_CENTER,
                &(i + 1).to_string(),
                hifi::num(12.0),
                t.dim,
            );
            hifi::label(
                p,
                Pos2::new(r.left() + 43.0, y),
                Align2::LEFT_CENTER,
                title,
                hifi::ui_font(14.0),
                c,
            );
            hifi::label(
                p,
                Pos2::new(r.left() + 299.0, y),
                Align2::LEFT_CENTER,
                artist,
                hifi::ui_font(13.5),
                t.dim,
            );
            hifi::label(
                p,
                Pos2::new(r.right() - 15.0, y),
                Align2::RIGHT_CENTER,
                time,
                hifi::num(12.0),
                t.dim,
            );
        }

        // A front-panel bar along the bottom, with a position slider.
        let bar = Rect::from_min_max(Pos2::new(screen.left(), screen.bottom() - 80.0), screen.max);
        hifi::bar(ui.painter(), bar, &t);
        hifi::label(
            ui.painter(),
            Pos2::new(bar.left() + 24.0, bar.center().y),
            Align2::LEFT_CENTER,
            "1:55",
            hifi::num(12.0),
            t.dim,
        );
        hifi::hslider(
            ui,
            Rect::from_min_max(
                Pos2::new(bar.left() + 70.0, bar.center().y - 9.0),
                Pos2::new(bar.right() - 70.0, bar.center().y + 9.0),
            ),
            Id::new("position"),
            &t,
            &mut self.position,
            &UNIT,
        );
        hifi::label(
            ui.painter(),
            Pos2::new(bar.right() - 24.0, bar.center().y),
            Align2::RIGHT_CENTER,
            "5:03",
            hifi::num(12.0),
            t.dim,
        );
        ui.painter().rect_stroke(
            screen,
            0,
            egui::Stroke::new(1.0, t.edge),
            egui::StrokeKind::Inside,
        );

        let mut open = self.settings;
        let mut look = self.look;
        hifi::drawer(&ctx, &t, "Settings", &mut open, |ui| {
            hifi::look_settings(ui, &t, &mut look);
        });
        self.settings = open;
        self.look = look;
        hifi::resize_edges(ui, screen);
    }
}

fn main() -> eframe::Result {
    // `gallery porcelain dark` and the like start on that look.
    let mut look = hifi::Look::default();
    for a in std::env::args().skip(1) {
        match a.as_str() {
            "linen" | "spool" => look.finish = Finish::Linen,
            "porcelain" => look.finish = Finish::Porcelain,
            "light" => look.build = Build::Light,
            "dark" => look.build = Build::Dark,
            _ => {}
        }
    }
    eframe::run_native(
        "TAW Panel gallery",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size(DESIGN)
                .with_decorations(false),
            ..Default::default()
        },
        Box::new(move |cc| {
            hifi::install_fonts(&cc.egui_ctx);
            Ok(Box::new(Gallery {
                look,
                settings: false,
                simple: false,
                view: 0,
                on: true,
                ink: true,
                menu: 0,
                gain: 3.0,
                freq: 1000.0,
                q: 0.7,
                fader: -4.0,
                position: 0.38,
                volume: 0.8,
                sel: 1,
            }))
        }),
    )
}
