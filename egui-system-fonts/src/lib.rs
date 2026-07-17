//! System font helpers for `egui`.
//!
//! This crate resolves platform-installed font families and applies them to an `egui::Context`.
//!
//! # Quick start
//!
//! Replace `egui`'s default fonts using the current system locale:
//!
//! ```no_run
//! # use egui_system_fonts::{set_auto, FontStyle};
//! # fn demo(ctx: &egui::Context) {
//! set_auto(ctx, FontStyle::Sans);
//! # }
//! ```
//!
//! Add system fonts as fallback only (keeps existing font priority):
//!
//! ```no_run
//! # use egui_system_fonts::{add_auto, FontStyle};
//! # fn demo(ctx: &egui::Context) {
//! add_auto(ctx, FontStyle::Sans);
//! # }
//! ```
//!
use egui::{
    epaint::text::{FontInsert, FontPriority, InsertFontFamily},
    FontData, FontFamily,
};
pub use system_fonts::{FontPreset, FontRegion, FontStyle};

#[cfg(not(target_arch = "wasm32"))]
use egui::FontDefinitions;
#[cfg(not(target_arch = "wasm32"))]
use std::collections::BTreeMap;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use system_fonts::FoundFontSource;

#[cfg(target_arch = "wasm32")]
mod wasm_defaults {
    use super::FontPreset;
    use crate::FontStyle;

    // Noto CJK SubsetOTF via jsDelivr GitHub CDN.
    // egui requires raw OTF/TTF bytes — WOFF/WOFF2 container formats are not supported.
    // SubsetOTF files contain glyphs for one region only (~3-6 MB each).
    const NOTO_SANS_KR: &str = concat!(
        "https://cdn.jsdelivr.net/gh/notofonts/noto-cjk@Sans2.004/Sans/SubsetOTF",
        "/KR/NotoSansKR-Regular.otf"
    );
    const NOTO_SANS_JP: &str = concat!(
        "https://cdn.jsdelivr.net/gh/notofonts/noto-cjk@Sans2.004/Sans/SubsetOTF",
        "/JP/NotoSansJP-Regular.otf"
    );
    const NOTO_SANS_SC: &str = concat!(
        "https://cdn.jsdelivr.net/gh/notofonts/noto-cjk@Sans2.004/Sans/SubsetOTF",
        "/SC/NotoSansSC-Regular.otf"
    );
    const NOTO_SANS_TC: &str = concat!(
        "https://cdn.jsdelivr.net/gh/notofonts/noto-cjk@Sans2.004/Sans/SubsetOTF",
        "/TC/NotoSansTC-Regular.otf"
    );
    const NOTO_SERIF_KR: &str = concat!(
        "https://cdn.jsdelivr.net/gh/notofonts/noto-cjk@Serif2.003/Serif/SubsetOTF",
        "/KR/NotoSerifKR-Regular.otf"
    );
    const NOTO_SERIF_JP: &str = concat!(
        "https://cdn.jsdelivr.net/gh/notofonts/noto-cjk@Serif2.003/Serif/SubsetOTF",
        "/JP/NotoSerifJP-Regular.otf"
    );
    const NOTO_SERIF_SC: &str = concat!(
        "https://cdn.jsdelivr.net/gh/notofonts/noto-cjk@Serif2.003/Serif/SubsetOTF",
        "/SC/NotoSerifSC-Regular.otf"
    );
    const NOTO_SERIF_TC: &str = concat!(
        "https://cdn.jsdelivr.net/gh/notofonts/noto-cjk@Serif2.003/Serif/SubsetOTF",
        "/TC/NotoSerifTC-Regular.otf"
    );

    pub fn url(preset: &FontPreset, style: FontStyle) -> Option<&'static str> {
        match (preset, style) {
            (FontPreset::Korean, FontStyle::Sans) => Some(NOTO_SANS_KR),
            (FontPreset::Korean, FontStyle::Serif) => Some(NOTO_SERIF_KR),
            (FontPreset::Japanese, FontStyle::Sans) => Some(NOTO_SANS_JP),
            (FontPreset::Japanese, FontStyle::Serif) => Some(NOTO_SERIF_JP),
            (FontPreset::SimplifiedChinese, FontStyle::Sans) => Some(NOTO_SANS_SC),
            (FontPreset::SimplifiedChinese, FontStyle::Serif) => Some(NOTO_SERIF_SC),
            (FontPreset::TraditionalChinese, FontStyle::Sans) => Some(NOTO_SANS_TC),
            (FontPreset::TraditionalChinese, FontStyle::Serif) => Some(NOTO_SERIF_TC),
            // Latin and Cyrillic are covered by egui's built-in font.
            _ => None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static FETCHING_URLS: std::cell::RefCell<std::collections::HashSet<&'static str>> =
        std::cell::RefCell::new(std::collections::HashSet::new());
}

/// Replaces `egui` font definitions with system fonts detected from the current system locale.
///
/// # Examples
///
/// ```no_run
/// # use egui_system_fonts::{set_auto, FontStyle};
/// # fn demo(ctx: &egui::Context) {
/// set_auto(ctx, FontStyle::Sans);
/// # }
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub fn set_auto(ctx: &egui::Context, style: FontStyle) -> Vec<String> {
    let (locale, region, fonts) = system_fonts::find_for_system_locale(style);
    log::info!(
        "Detected locale: {:?}, region: {:?}, style: {:?}, candidates: {}",
        locale,
        region,
        style,
        fonts.len()
    );
    set_found_fonts(ctx, fonts)
}

#[cfg(target_arch = "wasm32")]
pub fn set_auto(ctx: &egui::Context, style: FontStyle) -> Vec<String> {
    let locale = system_fonts::system_locale();
    let region = locale
        .as_deref()
        .map(system_fonts::region_from_locale)
        .unwrap_or(FontRegion::Latin);
    set_with_region(ctx, region, style)
}

/// Replaces `egui` font definitions with system fonts for the given region.
///
/// # Examples
///
/// ```no_run
/// # use egui_system_fonts::{set_with_region, FontRegion, FontStyle};
/// # fn demo(ctx: &egui::Context) {
/// set_with_region(ctx, FontRegion::Korean, FontStyle::Sans);
/// # }
/// ```
pub fn set_with_region(ctx: &egui::Context, region: FontRegion, style: FontStyle) -> Vec<String> {
    let presets = system_fonts::presets_for_region(region);
    set_with_presets(ctx, presets, style)
}

/// Replaces `egui` font definitions with system fonts resolved from the given presets.
///
/// # Examples
///
/// ```no_run
/// # use egui_system_fonts::{set_with_presets, FontPreset, FontStyle};
/// # fn demo(ctx: &egui::Context) {
/// let presets = [FontPreset::Korean, FontPreset::Latin];
/// set_with_presets(ctx, presets, FontStyle::Sans);
/// # }
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub fn set_with_presets<I>(ctx: &egui::Context, presets: I, style: FontStyle) -> Vec<String>
where
    I: IntoIterator<Item = FontPreset>,
{
    let fonts = system_fonts::find_from_presets(presets, style);
    set_found_fonts(ctx, fonts)
}

#[cfg(target_arch = "wasm32")]
pub fn set_with_presets<I>(ctx: &egui::Context, presets: I, style: FontStyle) -> Vec<String>
where
    I: IntoIterator<Item = FontPreset>,
{
    for preset in presets {
        if let Some(url) = wasm_defaults::url(&preset, style) {
            fetch_and_set_font(ctx.clone(), url, FontPriority::Highest);
        }
    }
    vec![]
}

/// Appends system fonts as fallback to the existing fonts in `ctx`.
///
/// # Examples
///
/// ```no_run
/// # use egui_system_fonts::{add_auto, FontStyle};
/// # fn demo(ctx: &egui::Context) {
/// add_auto(ctx, FontStyle::Sans);
/// # }
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub fn add_auto(ctx: &egui::Context, style: FontStyle) {
    let (locale, region, fonts) = system_fonts::find_for_system_locale(style);
    log::info!(
        "Detected locale: {:?}, region: {:?}, style: {:?}, candidates: {}",
        locale,
        region,
        style,
        fonts.len()
    );
    for f in fonts {
        let Some(bytes) = read_font_bytes(f.source) else {
            continue;
        };
        ctx.add_font(FontInsert {
            name: f.key,
            data: FontData::from_owned(bytes),
            families: vec![
                InsertFontFamily {
                    family: FontFamily::Proportional,
                    priority: FontPriority::Lowest,
                },
                InsertFontFamily {
                    family: FontFamily::Monospace,
                    priority: FontPriority::Lowest,
                },
            ],
        });
    }
}

#[cfg(target_arch = "wasm32")]
pub fn add_auto(ctx: &egui::Context, style: FontStyle) {
    let locale = system_fonts::system_locale();
    let region = locale
        .as_deref()
        .map(system_fonts::region_from_locale)
        .unwrap_or(FontRegion::Latin);
    add_with_region(ctx, region, style);
}

/// Appends system fonts for the given region as fallback to the existing fonts in `ctx`.
///
/// # Examples
///
/// ```no_run
/// # use egui_system_fonts::{add_with_region, FontRegion, FontStyle};
/// # fn demo(ctx: &egui::Context) {
/// add_with_region(ctx, FontRegion::Japanese, FontStyle::Sans);
/// # }
/// ```
pub fn add_with_region(ctx: &egui::Context, region: FontRegion, style: FontStyle) {
    let presets = system_fonts::presets_for_region(region);
    add_with_presets(ctx, presets, style);
}

/// Appends system fonts resolved from the given presets as fallback to the existing fonts in `ctx`.
///
/// # Examples
///
/// ```no_run
/// # use egui_system_fonts::{add_with_presets, FontPreset, FontStyle};
/// # fn demo(ctx: &egui::Context) {
/// let presets = [FontPreset::TraditionalChinese, FontPreset::Latin];
/// add_with_presets(ctx, presets, FontStyle::Serif);
/// # }
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub fn add_with_presets<I>(ctx: &egui::Context, presets: I, style: FontStyle)
where
    I: IntoIterator<Item = FontPreset>,
{
    let fonts = system_fonts::find_from_presets(presets, style);
    for f in fonts {
        let Some(bytes) = read_font_bytes(f.source) else {
            continue;
        };
        ctx.add_font(FontInsert {
            name: f.key,
            data: FontData::from_owned(bytes),
            families: vec![
                InsertFontFamily {
                    family: FontFamily::Proportional,
                    priority: FontPriority::Lowest,
                },
                InsertFontFamily {
                    family: FontFamily::Monospace,
                    priority: FontPriority::Lowest,
                },
            ],
        });
    }
}

#[cfg(target_arch = "wasm32")]
pub fn add_with_presets<I>(ctx: &egui::Context, presets: I, style: FontStyle)
where
    I: IntoIterator<Item = FontPreset>,
{
    for preset in presets {
        if let Some(url) = wasm_defaults::url(&preset, style) {
            fetch_and_set_font(ctx.clone(), url, FontPriority::Lowest);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn set_found_fonts(ctx: &egui::Context, fonts: Vec<system_fonts::FoundFont>) -> Vec<String> {
    let mut defs = FontDefinitions::default();
    let mut installed_names: Vec<String> = Vec::new();
    let mut keys_in_priority: Vec<String> = Vec::new();

    for f in fonts {
        let Some(bytes) = read_font_bytes(f.source) else {
            continue;
        };

        defs.font_data
            .insert(f.key.clone(), Arc::new(FontData::from_owned(bytes)));
        keys_in_priority.push(f.key.clone());
        installed_names.push(f.family);
    }

    if installed_names.is_empty() {
        log::warn!("No matching system fonts found.");
        return vec![];
    }

    for key in keys_in_priority.into_iter().rev() {
        insert_front(&mut defs.families, FontFamily::Proportional, key.clone());
        insert_front(&mut defs.families, FontFamily::Monospace, key);
    }

    ctx.set_fonts(defs);
    log::info!("Set fonts (family names): {:?}", installed_names);
    installed_names
}

#[cfg(not(target_arch = "wasm32"))]
fn read_font_bytes(source: FoundFontSource) -> Option<Vec<u8>> {
    match source {
        FoundFontSource::Path(path) => match std::fs::read(&path) {
            Ok(b) => Some(b),
            Err(e) => {
                log::debug!("Failed to read font file {:?}: {}", path, e);
                None
            }
        },
        FoundFontSource::Bytes(b) => Some(b.as_ref().to_vec()),
    }
}

#[cfg(target_arch = "wasm32")]
fn fetch_and_set_font(ctx: egui::Context, url: &'static str, priority: FontPriority) {
    if FETCHING_URLS.with(|s| !s.borrow_mut().insert(url)) {
        return; // fetch already in flight
    }
    let request = ehttp::Request::get(url);
    ehttp::fetch(request, move |response| {
        FETCHING_URLS.with(|s| {
            s.borrow_mut().remove(url);
        });
        let Ok(response) = response else {
            log::error!("Failed to download font: {url}");
            return;
        };
        if !response.ok {
            log::error!("Failed to download font: HTTP {} {url}", response.status);
            return;
        }
        let name = url.rsplit('/').next().unwrap_or(url).to_string();
        ctx.add_font(FontInsert {
            name,
            data: FontData::from_owned(response.bytes),
            families: vec![
                InsertFontFamily {
                    family: FontFamily::Proportional,
                    priority: priority.clone(),
                },
                InsertFontFamily {
                    family: FontFamily::Monospace,
                    priority,
                },
            ],
        });
        ctx.request_repaint();
        log::info!("Font loaded: {url}");
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn insert_front(families: &mut BTreeMap<FontFamily, Vec<String>>, family: FontFamily, key: String) {
    let list = families.entry(family).or_default();
    if list.iter().any(|k| k == &key) {
        return;
    }
    list.insert(0, key);
}
