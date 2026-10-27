# Roadmap

TAW-Panel moves when the apps need it. All four apps are released on 0.3.0; Player 0.2.0 is built on 0.3.2. Each new version below is driven by an app release that needs it. [docs/TRACKER.md](docs/TRACKER.md) has where every project stands.

## Pipeline (proposed 2026-10-14)

Sizes: S = one session, M = two or three, L = a release of its own.

| Release | What | For | Size |
|---|---|---|---|
| 0.5.0 | **The resizable list:** drag a pane edge or column divider, double-click to fit, one flexible column takes the spare room, header right-click shows or hides columns, widths saved per view (serde, with defaults) | Library 0.4.0, then the Player | M |
| 0.6.0 | **Meters, part 2:** Spool's meter scale (20 segments, 1 dB a step at the top) with four peak LEDs and hold; an analogue VU meter widget | Player's meters and VU window; the Equaliser and Analyser | M |
| parked | More finishes (Bronze, Cassette, Walnut, Studio), then skins (below) | When the apps are settled | L |
| proposed | **Looks beyond the hi-fi:** Isobar and Waypoint's look in the panel, plus Glass, Phosphor, Departures, Signalbox, Blueprint and Folio, all filling one set of colour roles; a page frame beside the faceplate; sparklines, scrolling graphs, pills, lamps and flap figures. Mock-ups in [`docs/looks-review.html`](docs/looks-review.html) (2026-10-27) | Net Meter, Isobar, Waypoint, Split-Flap Clock, Folder Sync, SPL Calculator | L |

The Library, Equaliser and Analyser should move their pin to 0.3.2 in their next release, for the corner-resize fix.

## Parked: skins

Reviewed on 2026-10-13 and parked until the Equaliser is tested and the Player runs.

- **The idea:** a skin changes materials, colours, knob style and display technology (red LED, fluorescent, LCD, analogue VU). The type, layout, badge and controls stay, so every skin still reads as the family. Black and Silver become two of the skins, and each app gets a skin selector.
- **The candidates (18):** see [`docs/skins-review.html`](docs/skins-review.html) (open it in a browser) or the review page in the family session. Skin names are generic; brand and model names stay in the notes, never in the apps.

  | Group | Skins (inspired by) |
  |---|---|
  | Synths | Panel, the current one (Roland Jupiter-8) · Blue Line (Oberheim OB-Xa/OB-8) · Walnut (Sequential Prophet-5) · Mocha (Yamaha DX7) · Rocker (Moog Minimoog) · Patchbay (Korg MS-20) |
  | Studio | Hammertone (Fairchild 670) · Console (Neve 1073) · Desk (SSL 4000) |
  | Tape | Studio Tape (Otari MX-5050) · Reel (Revox A77/B77) · Cassette (Nakamichi Dragon) · Teak (Tandberg) |
  | Hi-fi | Blue Meter (McIntosh) · Silverface (Technics/Pioneer) · Champagne (Marantz/Sansui) · Bronze (Quad 33/44) · Anodised (Schiit Audio) |

- **What building them needs in TAW-Panel:**
  - [ ] `Theme` grows from two panels into a skin: face colour or texture (hammertone, brushed, glass, wood cheeks), legend colours, accent (stripe, rules, none), key style, knob style, display style, meter style
  - [ ] Display styles beyond red LED: fluorescent (cyan, blue, green), LCD, amber, white
  - [ ] An analogue VU meter widget (also useful to the Equaliser and Analyser on any skin)
  - [ ] Key styles: square, membrane, toggle, rocker, lit, transport; knob styles: skirted, aluminium, bakelite, chrome, coloured caps
  - [ ] The gallery shows every skin
  - [ ] Each app: a skin selector, saved with its settings

## Done

- [x] 0.1.0: the shared look from the Equaliser plus the Library's list pieces
- [x] 0.2.0: spectrum drawing and the analyser (feature `analyser`)
- [x] 0.3.0: `hifi`, the TAW look (Spool and Porcelain; light, dark, automatic), used by all four apps
- [x] 0.3.1: `hifi::meter_scale`, a dB scale for the meters, for Player 0.2.0
- [x] 0.3.2: resizing frameless windows from the corners, and after a resize or move
- [x] 0.3.3: `hifi::badge` and `hifi::logo`, for a centred badge
- [x] 0.4.0: the hi-fi nameplate without the big name; one tighter corner radius; chosen segments stand out; meter colours (classic or the display's); drawer sections (from the Player's feedback, 2026-10-15)
