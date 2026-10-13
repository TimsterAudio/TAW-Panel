# TAW Panel

The panel look shared by Timster's Audio Works' native apps: the **Library**, **Player**, **Equaliser** and **Analyser**. One crate, so a change to the look is made once.

![The gallery example on the Black panel](docs/gallery.png)

It draws Drone Box's panel language with egui's painter: Black and Silver panels, Jupiter-style coloured keys with LEDs, knurled knobs, faders, red LED displays and readouts, meters, outlined sections and the red, orange, yellow and cream stripe. The four typefaces (Space Grotesk, Michroma, Barlow Semi Condensed, Share Tech Mono, all under the OFL) are built in.

| Module | What's in it |
|---|---|
| `theme` | `Theme` (Black or Silver), `KnobStyle`, the key colours (`K_RED` … `K_GREY`), lights and display colours, `install_fonts` and `font` |
| `widgets` | `key`, `lit_key`, `knob`, `fader`, `lcd`, `lcd_combo`, `readout`, `meter`, `ladder`, `section`, `frame_tab`, `stripe`, `led`, text helpers |
| `lists` | `well`, `rows` (a scrolling list that only draws what's on screen), `row_bg`, `header`, `chip`, `light`, `fit`, `thousands`, `ago` |
| `frame` | `draw_panel_bg` and `fit_to_window` |

## Using it

```toml
[dependencies]
taw-panel = { git = "https://github.com/TimsterAudio/TAW-Panel", tag = "v0.1.0" }
```

```rust
taw_panel::theme::install_fonts(&cc.egui_ctx);   // once, at start-up
let th = taw_panel::theme::Theme::panel(PanelKind::Black);
```

Apps pin a tag, so a new version of the panel only reaches an app when that app moves to it.

## Seeing every piece

```sh
cargo run --example gallery
```
