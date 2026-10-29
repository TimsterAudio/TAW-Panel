# Family tracker

Where each project stands, one line each, plus the handoffs between the family session and the app sessions. The detail lives in each repo's own `ROADMAP.md` (the release pipeline) and `HANDOFF.md` (where its session left off). TAW-Panel is public and the apps are private, so this file names releases and themes, not app internals.

Last checked: **2026-10-16** by the family session, against `main` of all five repos: every commit an app pins is on TAW-Panel's `main`, and CI is green on every `main`.

## Projects

| Project | Released | Panel pin | Now | Waiting on | Next release |
|---|---|---|---|---|---|
| TAW-Panel | 0.4.0 | — | 0.3.2: corner-resize fix (every app until its pin moves); 0.3.3: `badge` and `logo`; 0.4.0: the hi-fi look tightened from the Player's feedback (nameplate without the big name, one corner radius, chosen segments stand out, meter colours, drawer sections) | — | 0.5.0: resizable list (renumbered from 0.4.0) |
| Library (LB-001) | 0.3.0 | 0.3.0 | Released in the TAW look | The user's review of the look | 0.3.x: review fixes; pin to Panel 0.4.0 |
| Player (PL-001) | 0.2.0 | 0.4.0 | 0.2.0 released: the Mint feedback brief plus two rounds of look feedback | The user's test of 0.2.0 (Windows media keys untested; MP3 gapless, M3U8) | 0.3.0: WASAPI exclusive mode |
| Equaliser (EQ-001) | 0.3.0 | 0.3.0 | README screenshots updated (PR #3) | The user's test on real Windows | 0.3.x: Windows fixes; pin to Panel 0.4.0 |
| Analyser (SP-001) | 0.2.0 | 0.3.0 | README screenshots updated | The user's feedback | 0.2.x: feedback fixes; pin to Panel 0.4.0 |

## Handoffs

Newest first. A handoff is a brief from the family session to an app's session. The brief is kept in the app's repo, under `docs/briefs/`.

| Date | To | For | Brief | State |
|---|---|---|---|---|
| 2026-10-14 | Player | 0.2.0, from the 0.1.0 test on Linux Mint | `docs/briefs/2026-10-14-0.2.0.md` in TAW-Player | Released 2026-10-15 as 0.2.0: all seven items, plus the user's look feedback (2026-10-15, given in the Player session), on TAW-Panel 0.4.0 |

## Test rounds

| Date | App | Version | Where | Result |
|---|---|---|---|---|
| 2026-10-14 | Player | 0.1.0 | Linux Mint | Mostly passed; seven items of feedback, all in the 0.2.0 brief. Still to test: MP3 gapless, M3U8, Windows |

## Open with the user

- Library and Analyser (next session each): "one of each at a time" (FAMILY.md). Clicking them in a family switcher while they're open starts a second copy. The Player and the Equaliser have it on their branches `claude/kind-johnson-du69z8` (2026-10-29), not yet released; each app's `instance.rs` there is the pattern to copy.
- Agree or adjust each project's pipeline (in each repo's ROADMAP.md, marked "proposed").
- Test Player 0.2.0 (the v0.2.0 release), especially media keys on Windows, MP3 gapless and the Library's M3U8.
- Moving the Library, Equaliser and Analyser to TAW-Panel 0.4.0 changes their look (shorter nameplate, tighter corners): each app's session should tighten its top band and screenshot it.
- Player, next session (small hardening): with no D-Bus session bus, the media-controls thread panics inside `souvlaki` (zbus `unwrap`). The Player keeps playing, and real desktops always have a bus; check for the bus before starting media controls. Seen 2026-10-16 while screenshotting 0.2.0 under Xvfb.
- Roadmap numbers to fix in each app's next session (the panel renumbering of 2026-10-15): the Library's resizable list needs TAW-Panel **0.5.0** (its ROADMAP says 0.4.0); the Analyser's VU comes from **0.6.0** (its ROADMAP says 0.5.0).

## Decided

- 2026-10-16: every repo's default branch is `main` (the Analyser's switched by the user). The user deleted all 15 old `claude/…` branches, so every repo has only `main`; all were merged except the Equaliser's `claude/roadmap-pipeline` (ROADMAP.md only, superseded; last commit `b5b3f94`).
- 2026-10-16: all five repos now run the Equaliser's cheaper CI (the family session ported it to TAW-Panel, the Library, the Player and the Analyser). Docs-only pushes don't run CI; Windows CI runs on `main`, pull requests and manual runs only; Release runs on `main` only. **Test packages of a branch** now come from a manual run: Actions > Release > Run workflow, on that branch.
- 2026-10-15: from the user's feedback in the Player session, the hi-fi series drops the big slanted name from its nameplate (badge and model line only), uses one tighter corner radius, and offers classic or display meter colours: TAW-Panel 0.4.0, so the planned resizable list became 0.5.0 and meters part 2 0.6.0.
- 2026-10-15: the corner-resize bug was in TAW-Panel (`hifi::resize_edges`, every app); fixed in 0.3.2 by the Player session, merged to TAW-Panel `main` by the family session so the pinned commit stays on `main`.
- 2026-10-14: the meter scale is TAW-Panel 0.3.1 (a patch number, by the user's choice, though it adds a function).
- 2026-10-14: this tracker, one line per project, in TAW-Panel; the detailed pipeline in each repo's ROADMAP.md.
- 2026-10-14: Player: Enter plays the highlighted track (the arrows only move the highlight); the Library's Play key always starts playing, whatever the Player's new "Start playing when I open music" setting says.

## How it works

1. Ideas are talked through in the family session and written into the right repo's ROADMAP.md.
2. The family session writes a brief into the app's `docs/briefs/` and logs it above.
3. The app's session builds and releases it, and updates its CHANGELOG, ROADMAP and HANDOFF.
4. Back in the family session, "status" re-reads all five repos and brings this file up to date.

Only one session works in a repo at a time. The family session writes to an app repo only when that app's session isn't working in it.
