# egui_system_fonts

System font loader helpers for `egui`.

- Auto-detects the system locale and picks a reasonable font fallback chain
- Can either replace `egui` fonts (set) or append fallback fonts only (extend)
- Supports region presets (Korean/Japanese/Chinese/Cyrillic/Latin)

## Installation

```toml
[dependencies]
egui_system_fonts = "0.1"
```

## Usage

### Replace all egui fonts (auto-detect locale)

```rust,no_run
use egui_system_fonts::{set_auto, FontStyle};

fn setup_fonts(ctx: &egui::Context) {
    set_auto(ctx, FontStyle::Sans);
}
```

### Fallback only (keep existing priorities)

```rust,no_run
use egui_system_fonts::{extend_auto, FontStyle};

fn setup_fonts(ctx: &egui::Context) {
    let mut defs = egui::FontDefinitions::default();
    extend_auto(ctx, &mut defs, FontStyle::Sans);
}
```

### Force a region

```rust,no_run
use egui_system_fonts::{set_with_region, FontRegion, FontStyle};

fn setup_fonts(ctx: &egui::Context) {
    set_with_region(ctx, FontRegion::Korean, FontStyle::Sans);
}
```

### Use custom presets

```rust,no_run
use egui_system_fonts::{set_with_presets, FontPreset, FontStyle};

fn setup_fonts(ctx: &egui::Context) {
    let presets = [FontPreset::Korean, FontPreset::Latin];
    set_with_presets(ctx, presets, FontStyle::Sans);
}
```

## Notes

- If no matching system fonts are found, the functions return an empty list.
- `extend_*` only applies updated definitions when at least one font was added.
- `set_*` overwrites the default `egui` fonts.

## License

MIT OR Apache-2.0
