# Changelog

## Unreleased

## 0.3.2 – 2026-10-15

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
