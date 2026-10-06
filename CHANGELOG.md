# Changelog

All notable changes to this project are documented here.

## [0.1.0] - 2026-10-06

### Added

- HUD-style iced widget library with dark tokens, chamfered geometry, panels, cards,
  buttons, inputs, lists, tables, tabs, navigation, title bars, modal dialogs,
  toasts, loading indicators, status panels, data readouts, and progress bars.
- Gallery application covering the released widgets and their visual states.
- Sample application with dashboard, node list and detail, settings, and dialog-flow pages.
- Debug theme loading and hot reload from `assets/theme/dark.toml`.
- Headless tiny-skia snapshot rendering for Gallery pages at 100% and 150% scale.
- Pixel-level PNG comparison in CI through `gallery --compare-snapshot`.
- GitHub Actions checks for formatting, Clippy, tests, MSRV, cargo-deny, visual regression,
  and Windows release builds.

### Changed

- Project license is Apache License 2.0.
- `iced` is pinned to 0.14.0 and `iced_tiny_skia` to 0.14.1 to avoid the known
  multi-canvas clipping regression in 0.14.0.
- Windows demo binaries are distributed through GitHub Releases rather than Git history.

### Known limitations

- Windows hardware rendering, IME behavior, borderless-window system integration,
  multi-monitor DPI transitions, and reduced-motion system settings still require
  real Windows hardware validation.
- The first release does not promise API stability for the 0.x series.
- P2 widgets such as ContextMenu, Breadcrumb, Avatar/Image, and SplitPane are not part
  of the 0.1.0 component set.

[0.1.0]: https://github.com/madlaxcb/Iced-Arknights/releases/tag/v0.1.0
