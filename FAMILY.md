# The Timster's Audio Works family

The one place that describes how the apps fit together. Every app's CLAUDE.md points here; keep it current when something family-wide changes.

## The hi-fi separates

Four plain names under one maker's badge, like a stack of hi-fi units. Each has a model code.

| App | Model | What it does | Repo | Program |
|---|---|---|---|---|
| **Library** | LB-001 | Finds, checks and (later) organises your music. Read-only until a plan you review. | TAW-Library | `taw-library` |
| **Player** | PL-001 | Plays it: gapless, bit-perfect where it can. The native successor to Spool. | TAW-Player | `taw-player` |
| **Equaliser** | EQ-001 | System-wide parametric EQ (PipeWire sink on Linux, an audio-engine effect on Windows). | TAW-Equaliser | `taw-equaliser` |
| **Analyser** | SP-001 | Spectrum analyser. Listens only. | TAW-Analyser | `taw-analyser` |

Instruments keep "Box" names: **Drone Box**, **Drum Box** (browser apps, their own repos), and **EQ Box** (TimsterAudio/EQ-Box, native, restored from its 0.2.0 on 2026-10-14 as the Box family's equaliser with Drone Box's panel and the SP-10 analyser built in). TAW Equaliser grew out of EQ Box; the two share one Windows engine (same CLSID), so only one engine can be installed. **Spool** (the browser player) stays as it is while the Player is built.

## The shared panel

All four use **TAW-Panel** (this repo) for the look. Apps pin a TAW-Panel commit in Cargo.toml; a new panel version only reaches an app when its pin moves. TAW-Panel is public; the apps are private.

- **The TAW look** (`hifi`, from 2026-10-14) is for the hi-fi apps: a calm, modern faceplate. **Spool** (Spool's warm panels, tinted by its seven colour themes) is the default, in **Light** with **Amber**; **Porcelain** (matte white, paper display; Graphite when dark) is the alternative. Light, Dark or Automatic; an Anodised (brushed aluminium) finish was tried on 2026-10-14 and dropped as looking fake; saved "Anodised" settings load as Spool. The badge ("Timster's Audio Works - Player"), slanted logo and model line stay from the original panel as the family's tie-in. Apps draw their own title strip and window buttons, with "Use the system title bar" in Settings. Each app has a Simple and a Detailed view, and a family switcher along the top. `docs/finishes-review.html` is the reviewed mock-up (version 1, with all seven finishes, is `finishes-review-v1.html`).
- **The Jupiter look** (`theme`, `widgets`, `lists`, `frame`) is Drone Box's panel language: Black or Silver, coloured keys, knobs, faders, red displays. It stays for the **Box** instruments: Drone Box, Drum Box and EQ Box (TimsterAudio/EQ-Box, which has the SP-10 analyser built in).
- Both share the spectrum drawing and FFT (feature `analyser`).

Where each app is: all four were released in the TAW look on 2026-10-14, on TAW-Panel 0.3.0: Library 0.3.0, Player 0.1.0, Equaliser 0.3.0 and Analyser 0.2.0.

## How they fit together

```
Library ──(album or playlist)──► Player ──► system output ──► Equaliser ──► speakers
                                                                  └──► Analyser listens
```

- **Library → Player:** the Library's Play key writes an `.m3u8` playlist and runs `taw-player --play <file>` when it finds the Player (next to the Library, in a folder beside it named `taw-player…`, or on the PATH); otherwise it opens the playlist in the default player. A Player that's already open takes the playlist and the second launch exits (files in the Player's data folder, not a socket). Later, the Player may read the Library's database (read-only) to browse the collection.
- **The family switcher:** along the top of every app, it starts the others when they're found (next to the app, in a sibling folder named after the program, on the PATH, plus `Program Files\Timster's Audio Works\<App>` on Windows for the Player); apps that weren't found are dimmed. Every app works on its own: none needs another installed.
- **Equaliser:** works at the system level, so it sits after any player with no coupling.
- **Analyser:** listens to an output or an input; needs nothing from the others.

## Names on disk

The short name is on the panel; the full name is everywhere technical, so "Library" or "Player" never collides with anything else on a computer.

| | Windows | Linux |
|---|---|---|
| Window title, menu entry | TAW Library, TAW Player, TAW Equaliser, TAW Analyser | same |
| Program | `taw-library.exe` … | `taw-library` … |
| App data | `%APPDATA%\Timster's Audio Works\<App>\` | `~/.local/share/timsters-audio-works/<app>/` |
| Installed system parts (Equaliser engine) | `%ProgramFiles%\Timster's Audio Works\Equaliser\`, settings in `%ProgramData%\Timster's Audio Works\Equaliser\` | — |

## Shared conventions

- Native Rust and egui; no web views. No network requests unless the user switches a feature on (only the Library's Lookups, so far).
- Saved settings use `#[serde(default)]`: add fields with defaults, never rename one.
- Each repo: feature branches, `main` always works, CHANGELOG.md with an Unreleased section, and a release workflow that publishes Windows and Linux downloads when `main` carries a new version.
- The command line is part of an app's contract once another app uses it: `taw-player [--play] [FILE | FOLDER | PLAYLIST ...]`.
- Checks before pushing: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and a Windows cross-compile (`--target x86_64-pc-windows-gnu`).
- Work across apps happens in the family session; deep work on one app in a session for that app. Only one session works in a repo at a time. `docs/TRACKER.md` (in this repo) tracks where every project stands and the handoffs between the family session and the app sessions; each repo's ROADMAP.md holds its own release pipeline, and briefs to an app's session go in that app's `docs/briefs/`.

## Order of work

1. The TAW look for all four (done 2026-10-14; all released).
2. Each app usable day to day, in its own chat: the Equaliser tested on Windows (the EQ Box to TAW Equaliser switch), the Player's listening test, the Library's resizable panes and columns, the Analyser's feedback.
3. Later finishes and skins (parked in TAW-Panel's ROADMAP.md).

## Parked ideas

Ideas suggested along the way, kept here for review later. Nothing here is scheduled.

- **More finishes** for the TAW look: Bronze, Cassette, Walnut, Studio and Spool (in `docs/finishes-review-v1.html`).
- **Resizable columns and panes** (the Library first): drag a column divider or pane edge to resize, double-click to fit the contents, widths remembered per view; one flexible column (the title) takes the spare room.
- **Panel skins**: 18 candidates reviewed 2026-10-13; see TAW-Panel's ROADMAP.md and `docs/skins-review.html`.
