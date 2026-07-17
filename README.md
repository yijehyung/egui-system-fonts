# egui-system-fonts

System font loader helpers for [`egui`](https://github.com/emilk/egui).

This repository is a small workspace containing:

- **`egui-system-fonts`** — the library crate published on crates.io
- **`demo-egui`** — a demo app to test fonts and fallbacks (native + WASM)

## Crate

- crates.io: `egui-system-fonts`
- docs.rs: https://docs.rs/egui-system-fonts

```toml
[dependencies]
egui-system-fonts = "0.35.0"
```

```rust,no_run
use egui_system_fonts::{set_auto, FontStyle};

fn setup_fonts(ctx: &egui::Context) {
    set_auto(ctx, FontStyle::Sans);
}
```

## Demo app

```bash
# Native
cargo run -p demo-egui

# WASM (requires trunk)
cd demo-egui
trunk serve
```

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license
