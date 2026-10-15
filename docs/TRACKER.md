# Family tracker

Where each project stands, one line each, plus the handoffs between the family session and the app sessions. The detail lives in each repo's own `ROADMAP.md` (the release pipeline) and `HANDOFF.md` (where its session left off). TAW-Panel is public and the apps are private, so this file names releases and themes, not app internals.

Last checked: **2026-10-15**, against `main` of all five repos and the Player's 0.2.0 branch.

## Projects

| Project | Released | Panel pin | Now | Waiting on | Next release |
|---|---|---|---|---|---|
| TAW-Panel | 0.3.2 | — | 0.3.1: meter scale; 0.3.2: corner-resize fix (all apps have the bug until their pin moves) | — | 0.4.0: resizable list |
| Library (LB-001) | 0.3.0 | 0.3.0 | Released in the TAW look | The user's review of the look | 0.3.x: review fixes; pin to Panel 0.3.2 |
| Player (PL-001) | 0.1.0 | 0.3.0 (0.3.2 on the 0.2.0 branch) | 0.2.0 built on branch `claude/kind-johnson-du69z8`, CI green; test packages in its Actions run | The user's test of 0.2.0, then merge to `main` to publish it | 0.2.0: feedback release |
| Equaliser (EQ-001) | 0.3.0 | 0.3.0 | README screenshots updated (PR #3) | The user's test on real Windows | 0.3.x: Windows fixes; pin to Panel 0.3.2 |
| Analyser (SP-001) | 0.2.0 | 0.3.0 | README screenshots updated | The user's feedback | 0.2.x: feedback fixes; pin to Panel 0.3.2 |

## Handoffs

Newest first. A handoff is a brief from the family session to an app's session. The brief is kept in the app's repo, under `docs/briefs/`.

| Date | To | For | Brief | State |
|---|---|---|---|---|
| 2026-10-14 | Player | 0.2.0, from the 0.1.0 test on Linux Mint | `docs/briefs/2026-10-14-0.2.0.md` in TAW-Player | Built 2026-10-15: all seven items, on TAW-Panel 0.3.2 (0.3.1 plus the corner fix the Player session found and made). Not yet merged to `main` |

## Test rounds

| Date | App | Version | Where | Result |
|---|---|---|---|---|
| 2026-10-14 | Player | 0.1.0 | Linux Mint | Mostly passed; seven items of feedback, all in the 0.2.0 brief. Still to test: MP3 gapless, M3U8, Windows |

## Open with the user

- Agree or adjust each project's pipeline (in each repo's ROADMAP.md, marked "proposed").
- Test Player 0.2.0 (the packages from its branch's Release run in GitHub Actions), then say whether to merge it to `main`, which publishes the release.
- GitHub: set the Analyser's default branch to `main`; delete the merged `claude/focused-ritchie-veg4m9` branches in the app repos; delete the Equaliser's `claude/roadmap-pipeline` (its plan is now in ROADMAP.md).

## Decided

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
