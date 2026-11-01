# Handoff: TAW-Panel and the family

Written 2026-10-14 when the family's work moved from one shared chat to a chat per app (Library, Player, Equaliser, Analyser each get their own; each repo has a HANDOFF.md). Read CLAUDE.md and FAMILY.md, then this.

## Where it stands

- **0.3.0 released on 2026-10-14** (commit `1b8162d`, merged to `main`): adds **`hifi`**, the TAW look. Spool finish (default, Light, Amber; Spool's seven colour themes tint the whole panel) and Porcelain; Light, Dark, Automatic; Hanken Grotesk and Chivo Mono; nameplate, window buttons, drag and resize for frameless windows; buttons, switches, segmented controls, family switcher, menus, knobs, faders, sliders, display, wells, meters, settings drawer. An Anodised finish was tried and dropped; "Anodised" in saved settings loads as Spool. The Jupiter look is unchanged.
- `docs/finishes-review.html` (v2) and `-v1.html` (all seven finishes): the mock-ups the user reviewed.
- All four apps pin `1b8162d` and are released in the TAW look: Library 0.3.0, Player 0.1.0, Equaliser 0.3.0, Analyser 0.2.0.
- **0.3.1 released on 2026-10-14**: `hifi::meter_scale`, a dB scale under the meters, for Player 0.2.0. Nothing else changed.
- **0.3.2 released on 2026-10-15** (from the Player's 0.2.0 session, merged to `main` by the family session): frameless windows resize from every corner, and every other resize or move no longer does nothing (see CHANGELOG). The Library, Equaliser and Analyser have the same bug until they move their pin.
- **0.3.3 released on 2026-10-15** (from the Player's session): `hifi::badge` and `hifi::logo`, the nameplate in two halves.
- **0.4.0 released on 2026-10-15** (from the Player's session, merged to `main` with 0.3.3): from the user's feedback on the Player, the hi-fi nameplate is the badge and model line (no big name), one corner radius (`RADIUS`, 6), chosen segments get a heavier edge, `meter_in` with classic or display colours, drawer sections with a hairline between them. Player 0.2.0 pins it; the other apps take it when they move their pin (their top band can then be tightened, since the nameplate is shorter). The resizable list and meters part 2 are now 0.5.0 and 0.6.0.
- `cargo run --example gallery` shows every `hifi` piece (add `porcelain dark` and the like to start on a look); `cargo run --example jupiter` shows the Box instruments' Jupiter look.

- **0.6.0 made on 2026-11-01** (from the Library's session, on branch `claude/resizable-list`, commit `efbded4`): `table`, the resizable list. Library 0.4.0 pins it (its branch of the same name). Merge it to `main` with a merge commit or fast-forward, never a squash, so the pinned commit stays on `main`. The Player takes it when it moves its pin: its playlist's columns (#, Title, Artist, Album, Time, Type) map straight onto `table::Col`.

## Next

1. The Player: resizable playlist columns on `table` (0.6.0), in the Player's session.
2. 0.7.0: meters, part 2 (see ROADMAP.md).
3. Keep FAMILY.md and docs/TRACKER.md current; parked ideas live in FAMILY.md.
