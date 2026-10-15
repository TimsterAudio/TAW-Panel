# Changelog

## Unreleased

## 0.4.0 – 2026-10-15

Commit `d030ffdbc51c48e784cd74b20c5547a766461e5a` (branch `claude/kind-johnson-du69z8`).

From the user's feedback on the Player, for the whole hi-fi series:

- **The nameplate is the badge with the model line under it.** The big slanted name stays with the Box instruments: `hifi::logo` still draws it for anything that wants it. Apps that call `nameplate` get the new, shorter plate and can tighten their top band (about 36 px tall instead of 70).
- **One corner radius**, `hifi::RADIUS` (6, was 8): wells, display windows, buttons, segmented controls, the badge, menus, the gear key, and the family switcher (no longer a round-ended pill; its inner cells follow its corners).
- **The chosen part of a segmented control** (Simple / Detailed, Spool / Porcelain, the family switcher) has a slightly heavier edge, so the choice stands out.
- **Meter colours:** `hifi::meter_in` with `MeterColours::Classic` (green, yellow, red; what `meter` draws) or `MeterColours::Display` (the display's ink, with the top 3 dB still red). `MeterColours` is serde, for saving.
- **The settings drawer:** sections after the first start with a hairline and more room above, and the rows within them sit closer together.
- Callers build unchanged; the look changes where noted.

## 0.3.3 – 2026-10-15

Commit `3cf118182042752e9ec3622b6d202bf2d8f96dad` (branch `claude/kind-johnson-du69z8`).

- `hifi::badge` and `hifi::logo`: the nameplate's two halves on their own, so an app can put the "Timster's Audio Works - Player" badge in the middle of the window's top edge and the logo and model line up at the left (the Player 0.2.0's layout, by the user's choice). `nameplate` draws exactly as before, from the same pieces.
- Nothing else changed: callers build unchanged.

## 0.3.2 – 2026-10-15

Commit `5290f15aa8275851ba68e90417325e20565b1eeb` (branch `claude/kind-johnson-du69z8`).

- Frameless windows resize from every corner. `hifi::resize_edges` added the corners before the edges, so egui gave a press in a corner to the edge over it and only one side moved; the corners now come last. Found on Linux Mint (Cinnamon, X11) in the Player, and the same in every app.
- A drag that hands the window to the window manager (`resize_edges`, `drag_to_move`) now ends egui's own drag. The window manager keeps the mouse button's release, so egui went on thinking the zone was held and ignored the next press on any of them: every other resize or move did nothing.
- Nothing else changed: callers build unchanged.

## 0.3.1 – 2026-10-14

Commit `f5db7cb0eb288c656d739aa5ecc2c027b14baf00`.

- `hifi::meter_scale`: a dB scale for `hifi::meter` (0, −3, −6, −12, −24, −48), a tick and a figure at each, drawn under the meter in the display's ink; figures that won't fit on a narrow meter are left out. With it, `meter_x` (where a level falls along a meter) and the constants `METER_FLOOR_DB` and `METER_MARKS`. For the Player's meters (user feedback, 2026-10-14).
- The `gallery` example shows the TAW look (`hifi`), every piece on one page, and can start on any look (`gallery porcelain dark`). The Jupiter page is now the `jupiter` example.
- Nothing existing changed: 0.3.0 callers build unchanged, and `meter` looks the same.

## 0.3.0 – 2026-10-14

Commit `1b8162d75d10c41a8c9b021fc3d31e2a3311aeb6`.

- `hifi`: the TAW look for the hi-fi apps. Finishes (Spool, the default, with Spool's seven colour themes tinting the whole panel; and Porcelain), light, dark and automatic builds, accents (the finish's own or Spool's colours), Hanken Grotesk and Chivo Mono, the nameplate (badge, logo, model line), window buttons, drag-to-move and edge resizing for windows without a system title bar, and its controls: buttons, switches, segmented controls, the family switcher, menus, knobs, faders, sliders, display windows, wells, meters and a settings drawer. The Jupiter look (`theme`, `widgets`) is unchanged, for the Box instruments.
- Nothing existing changed: 0.2.0 callers build unchanged.

## 0.2.0 – 2026-10-13

Commit `41a95d4513d93dfbf18803508065a8c0a168ec95`.

- `spectrum`: drawing a spectrum in a colour scheme (`Scheme`) and style (`Look`, `Style`), from the Equaliser.
- `analyzer` (feature `analyser`): the 8192-point FFT with rise, fall and peak-hold ballistics, and `analyzer_channel` (the real-time-safe feed from an audio thread), from the Equaliser.
- Nothing existing changed: 0.1.0 callers build unchanged.

## 0.1.0 – 2026-10-13

Commit `29de12e21142d9f87ae85b708ed89ea18421621c`.


- The panel from EQ Box (theme, keys, knobs, faders, displays, meters, frames, stripe), plus the list pieces and completeness ladder from the Library, and the panel background and fit-to-window scaling both apps had.
- `Theme::panel` for apps without knobs; list colours (`well`, `row_hover`, `row_sel`) and the `AMBER` and `OK_GREEN` lights.
- The gallery example.
