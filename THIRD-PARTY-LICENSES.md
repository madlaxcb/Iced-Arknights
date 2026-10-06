# Third-Party Licenses

This project is distributed under the Apache License 2.0. The Rust dependencies listed
below remain under their own licenses. This summary is generated from the locked Cargo
workspace dependency graph and is not a replacement for each upstream license notice.

## License families in the locked dependency graph

`cargo deny check` currently permits the following license families used by the build:

- MIT
- Apache-2.0
- Apache-2.0 WITH LLVM-exception
- BSD-2-Clause
- BSD-3-Clause
- ISC
- Zlib
- CC0-1.0
- Unlicense
- 0BSD
- BSL-1.0
- Unicode-3.0

The full package-to-license mapping is maintained by Cargo metadata and checked by
`code/deny.toml`. Run this command from `code/` when auditing a new dependency:

```bash
cargo deny check
```

## Font notice

Linux builds may include Cantarell through the `sctk-adwaita` dependency. Cantarell is
licensed under the SIL Open Font License 1.1. Redistributors must keep the applicable
copyright and license notice with the redistributed binaries.

## Project-owned resources

The project UI geometry, tokens, layout code, and original SVG resources are authored for
this project and distributed under Apache License 2.0. No game artwork, logo, trademark,
or proprietary font is bundled.

## Special dependency notes

- `self_cell` is available under `Apache-2.0 OR GPL-2.0-only`; this project uses the
  Apache-2.0 option.
- `clipboard-win` and `error-code` use the Boost Software License 1.0 (BSL-1.0).
- `paste`, `rustybuzz`, and `ttf-parser` are currently ignored as unmaintained advisory
  entries because they are transitive dependencies of the pinned iced/cosmic-text stack;
  this does not change their upstream licenses.
