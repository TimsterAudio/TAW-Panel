# Changelog

## Unreleased

- `hifi`: the TAW look for the hi-fi apps. Finishes (Anodised, Porcelain), light, dark and automatic builds, accents (the finish's own or Spool's colours), Hanken Grotesk and Chivo Mono, the nameplate (badge, logo, model line), window buttons, drag-to-move and edge resizing for windows without a system title bar, and its controls: buttons, switches, segmented controls, the family switcher, menus, knobs, faders, sliders, display windows, wells, meters and a settings drawer. The Jupiter look (`theme`, `widgets`) is unchanged, for the Box instruments.

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
