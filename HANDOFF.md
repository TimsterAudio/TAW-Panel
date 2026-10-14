# Handoff: TAW-Panel and the family

Written 2026-10-14 when the family's work moved from one shared chat to a chat per app (Library, Player, Equaliser, Analyser each get their own; each repo has a HANDOFF.md). Read CLAUDE.md and FAMILY.md, then this.

## Where it stands

- **0.3.0 released on 2026-10-14** (commit `1b8162d`, merged to `main`): adds **`hifi`**, the TAW look. Spool finish (default, Light, Amber; Spool's seven colour themes tint the whole panel) and Porcelain; Light, Dark, Automatic; Hanken Grotesk and Chivo Mono; nameplate, window buttons, drag and resize for frameless windows; buttons, switches, segmented controls, family switcher, menus, knobs, faders, sliders, display, wells, meters, settings drawer. An Anodised finish was tried and dropped; "Anodised" in saved settings loads as Spool. The Jupiter look is unchanged.
- `docs/finishes-review.html` (v2) and `-v1.html` (all seven finishes): the mock-ups the user reviewed.
- All four apps pin `1b8162d` and are released in the TAW look: Library 0.3.0, Player 0.1.0, Equaliser 0.3.0, Analyser 0.2.0.
- `cargo run --example gallery` shows every `hifi` piece (add `porcelain dark` and the like to start on a look); `cargo run --example jupiter` shows the Box instruments' Jupiter look.

## Next

1. 0.4.0: a dB scale for the meters, for Player 0.2.0 (see ROADMAP.md).
2. 0.5.0: a resizable list (panes and columns) for the Library and Player.
3. Keep FAMILY.md and docs/TRACKER.md current; parked ideas live in FAMILY.md.
