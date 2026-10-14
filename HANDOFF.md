# Handoff: TAW-Panel and the family

Written 2026-10-14 when the family's work moved from one shared chat to a chat per app (Library, Player, Equaliser, Analyser each get their own; each repo has a HANDOFF.md). Read CLAUDE.md and FAMILY.md, then this.

## Where it stands

- `main` is 0.2.0 (commit `41a95d4`): the Jupiter look plus spectrum and analyser.
- Branch `claude/focused-ritchie-veg4m9`, unmerged:
  - **`hifi`**, the TAW look: Spool finish (default, Light, Amber; Spool's seven colour themes tint the whole panel) and Porcelain; Light, Dark, Automatic; Hanken Grotesk and Chivo Mono; nameplate, window buttons, drag and resize for frameless windows; buttons, switches, segmented controls, family switcher, menus, knobs, faders, sliders, display, wells, meters, settings drawer. An Anodised finish was tried and dropped; "Anodised" in saved settings loads as Spool.
  - `docs/finishes-review.html` (v2) and `-v1.html` (all seven finishes): the mock-ups the user reviewed (also published as an artifact).
  - FAMILY.md updated for the look and the Library → Player link.
- The Analyser and Equaliser branches pin `ce561c2` (this branch's head).

## Next

1. When the user approves the look: release TAW-Panel 0.3.0 (CHANGELOG version section, Cargo.toml, record the commit), then move each app's pin to it.
2. A resizable list (panes and columns) for the Library and Player.
3. Keep FAMILY.md current; parked ideas live there.
