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

Instruments keep "Box" names: **Drone Box**, **Drum Box** (browser apps, their own repos). **Spool** (the browser player) stays as it is while the Player is built.

## The shared panel

All four use **TAW-Panel** (this repo) for the look: Drone Box's Jupiter-style panel, Black or Silver, keys, knobs, faders, red displays, lists, and the spectrum drawing and FFT (feature `analyser`). Apps pin a TAW-Panel commit in Cargo.toml; a new panel version only reaches an app when its pin moves. TAW-Panel is public; the apps are private.

## How they fit together

```
Library ──(album or playlist)──► Player ──► system output ──► Equaliser ──► speakers
                                                                  └──► Analyser listens
```

- **Library → Player:** the Library's Play key writes an `.m3u8` playlist and runs `taw-player --play <file>` when it finds the Player (next to the Library, in a folder beside it named `taw-player…`, or on the PATH); otherwise it opens the playlist in the default player. A Player that's already open takes the playlist and the second launch exits (files in the Player's data folder, not a socket). Later, the Player may read the Library's database (read-only) to browse the collection.
- **Player → the others:** the Player's Family keys start the Library, Equaliser and Analyser when they're found (the same places, plus `Program Files\Timster's Audio Works\<App>` on Windows); the LED over each key shows whether it was. Every app works on its own: none needs another installed.
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
- Work across apps happens in the family session; deep work on one app in a session for that app. Only one session works in a repo at a time.

## Order of work

1. Get each app to a usable state: the Equaliser tested on Windows (the EQ Box to TAW Equaliser switch), then the Player running (first build 2026-10-13, on its branch for review).
2. Then the look: skins (parked in TAW-Panel's ROADMAP.md).

## Parked ideas

Ideas suggested along the way, kept here for review later. Nothing here is scheduled.

- **TAW finishes (proposed 2026-10-13)**: a modern hi-fi look for the TAW apps (Anodised, Bronze, Cassette, Walnut, Porcelain, Studio, Spool; each light and dark), Simple/Detailed views, Separates/Now Playing/Mini layouts. The Jupiter look would stay with the Box family (Drone Box, Drum Box, and a Box EQ and Analyser). See `docs/finishes-review.html`.
- **Panel skins**: 18 candidates reviewed 2026-10-13; see TAW-Panel's ROADMAP.md and `docs/skins-review.html`.
