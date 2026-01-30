# egui-system-fonts (workspace)

System font loader helpers for [`egui`](https://github.com/emilk/egui).

This repository is a small workspace containing:

- **`egui-system-fonts`** — the library crate published on crates.io
- **`demo-egui`** — a small native demo app (not published) to test fonts and fallbacks

The core, UI-agnostic font discovery logic lives in a separate repository:

- **`system-fonts`**: https://github.com/yijehyung/system-fonts

## Crate

- crates.io: `egui-system-fonts`
- docs.rs: https://docs.rs/egui-system-fonts

Add to your project:

```toml
[dependencies]
egui-system-fonts = "0.1"
```

Minimal usage:

```rust,no_run
use egui_system_fonts::{set_auto, FontStyle};

fn setup_fonts(ctx: &egui::Context) {
    set_auto(ctx, FontStyle::Sans);
}
```

## Demo app

From the workspace root:

```bash
cargo run -p demo-egui
```

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license
