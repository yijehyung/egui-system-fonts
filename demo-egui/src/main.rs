use eframe::egui;
use egui_system_fonts::{
    extend_auto, extend_with_region, set_auto, set_with_region, FontRegion, FontStyle,
};

fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "System Fonts Fallback Demo",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    code_text: String,
    logs: Vec<String>,
    selected_region: Option<FontRegion>,
    selected_style: FontStyle,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            code_text: r#"// 1. Latin
The quick brown fox jumps over the lazy dog.
0123456789 !@#$%^&*()

// 2. Korean
다람쥐 헌 쳇바퀴에 타고파.
별 헤는 밤, 계절이 지나가는 하늘에는 가을로 가득 차 있습니다.

// 3. Japanese
いろはにほへと ちりぬるを
リュックサック詰め込んで、気ままな旅に出よう。

// 4. Chinese
敏捷的棕色狐狸跳过懒狗。 (Simplified)
敏捷的棕色狐狸跳過懶狗。 (Traditional)

// 5. Cyrillic
Съешь же ещё этих мягких французских булок, да выпей чаю."#
                .to_owned(),
            logs: vec!["Ready. Select options and click Set/Extend.".to_owned()],

            selected_region: None,
            selected_style: FontStyle::Sans,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("log_panel")
            .resizable(true)
            .min_height(100.0)
            .default_height(150.0)
            .show(ctx, |ui| {
                ui.heading("Logs");
                egui::ScrollArea::vertical()
                    .id_salt("log_scroll")
                    .stick_to_bottom(true)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for log in &self.logs {
                            ui.monospace(log);
                        }
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.heading("Font Controls");
                ui.add_space(5.0);

                egui::Grid::new("controls_grid")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Target Region:");
                        egui::ComboBox::from_id_salt("region_combo")
                            .selected_text(match self.selected_region {
                                None => "Auto (System Locale)",
                                Some(FontRegion::Korean) => "Korean",
                                Some(FontRegion::Japanese) => "Japanese",
                                Some(FontRegion::SimplifiedChinese) => "Chinese (Simplified)",
                                Some(FontRegion::TraditionalChinese) => "Chinese (Traditional)",
                                Some(FontRegion::Latin) => "Latin / English",
                                Some(FontRegion::Cyrillic) => "Cyrillic",
                                _ => "Unknown",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.selected_region,
                                    None,
                                    "Auto (System Locale)",
                                );
                                ui.selectable_value(
                                    &mut self.selected_region,
                                    Some(FontRegion::Korean),
                                    "Korean",
                                );
                                ui.selectable_value(
                                    &mut self.selected_region,
                                    Some(FontRegion::Japanese),
                                    "Japanese",
                                );
                                ui.selectable_value(
                                    &mut self.selected_region,
                                    Some(FontRegion::SimplifiedChinese),
                                    "Chinese (Simplified)",
                                );
                                ui.selectable_value(
                                    &mut self.selected_region,
                                    Some(FontRegion::TraditionalChinese),
                                    "Chinese (Traditional)",
                                );
                            });
                        ui.end_row();

                        ui.label("Font Style:");
                        egui::ComboBox::from_id_salt("style_combo")
                            .selected_text(match self.selected_style {
                                FontStyle::Sans => "Sans-serif",
                                FontStyle::Serif => "Serif",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.selected_style,
                                    FontStyle::Sans,
                                    "Sans-serif",
                                );
                                ui.selectable_value(
                                    &mut self.selected_style,
                                    FontStyle::Serif,
                                    "Serif",
                                );
                            });
                        ui.end_row();
                    });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("Set (Replace All)").clicked() {
                        self.add_log(format!("LANG={:?}", std::env::var("LANG")));
                        self.add_log(format!("LC_ALL={:?}", std::env::var("LC_ALL")));
                        self.add_log(format!("LC_CTYPE={:?}", std::env::var("LC_CTYPE")));
                        let installed = match self.selected_region {
                            None => set_auto(ctx, self.selected_style),
                            Some(region) => set_with_region(ctx, region, self.selected_style),
                        };

                        let region_text = match self.selected_region {
                            None => "Auto (System Locale)".to_string(),
                            Some(r) => format!("{r:?}"),
                        };

                        self.add_log(format!(
                            "Set Fonts: Region={}, Style={:?}, Installed={}",
                            region_text,
                            self.selected_style,
                            installed.len()
                        ));
                    }

                    if ui.button("Extend (Fallback Only)").clicked() {
                        self.add_log(format!("LANG={:?}", std::env::var("LANG")));
                        self.add_log(format!("LC_ALL={:?}", std::env::var("LC_ALL")));
                        self.add_log(format!("LC_CTYPE={:?}", std::env::var("LC_CTYPE")));
                        let mut defs = egui::FontDefinitions::default();

                        let installed = match self.selected_region {
                            None => extend_auto(ctx, &mut defs, self.selected_style),
                            Some(region) => {
                                extend_with_region(ctx, &mut defs, region, self.selected_style)
                            }
                        };

                        let region_text = match self.selected_region {
                            None => "Auto (System Locale)".to_string(),
                            Some(r) => format!("{r:?}"),
                        };

                        self.add_log(format!(
                            "Extend Fonts: Region={}, Style={:?}, Added={}",
                            region_text,
                            self.selected_style,
                            installed.len()
                        ));
                    }

                    if ui.button("Reset (Default)").clicked() {
                        ctx.set_fonts(egui::FontDefinitions::default());
                        self.add_log("Reset to egui defaults.".to_string());
                    }
                });
            });

            ui.add_space(8.0);

            ui.columns(2, |columns| {
                columns[0].vertical(|ui| {
                    ui.heading("Label (Proportional)");
                    ui.label("Plain text rendering:");

                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("left_scroll")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(&self.code_text)
                                        .size(15.0)
                                        .line_height(Some(22.0)),
                                );
                            });
                    });
                });

                columns[1].vertical(|ui| {
                    ui.heading("Code Editor (Monospace)");
                    ui.label("Editable code view:");
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("right_scroll")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut self.code_text)
                                        .font(egui::TextStyle::Monospace)
                                        .code_editor()
                                        .desired_width(f32::INFINITY)
                                        .lock_focus(true),
                                );
                            });
                    });
                });
            });
        });
    }
}

impl MyApp {
    fn add_log(&mut self, msg: String) {
        println!("{}", msg);
        self.logs.push(msg);
    }
}
