# Roadmap

TAW-Panel moves when the apps need it. Right now the priority is getting each app to a usable state (the Equaliser tested on Windows, the Player running); the look waits until then.

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
