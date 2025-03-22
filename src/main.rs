#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::ecolor;
use qmk_via_api::api::KeyboardApi;

const PRODUCT_VID: u16 = 0x7372;
const PRODUCT_PID: u16 = 0x0002;
const USAGE_PAGE: u16 = 0xff60;

struct ViaController {
    api: std::sync::Arc<KeyboardApi>,
    backlight_brightness: Option<u8>,
    backlight_effect: Option<u8>,
    rgblight_brightness: Option<u8>,
    rgblight_effect: Option<u8>,
    rgblight_effect_speed: Option<u8>,
    rgblight_color: Option<(u8, u8)>,
    rgb_matrix_brightness: Option<u8>,
    rgb_matrix_effect: Option<u8>,
    rgb_matrix_effect_speed: Option<u8>,
    rgb_matrix_color: Option<(u8, u8)>,
    led_matrix_brightness: Option<u8>,
    led_matrix_effect: Option<u8>,
    led_matrix_effect_speed: Option<u8>,
    protocol_version: Option<u16>,
    layer_count: Option<u8>,
    macro_count: Option<u8>,
    audio_enabled: Option<bool>,
    audio_clicky_enabled: Option<bool>,
}

impl Default for ViaController {
    fn default() -> Self {
        let api =
            std::sync::Arc::new(KeyboardApi::new(PRODUCT_VID, PRODUCT_PID, USAGE_PAGE).unwrap());
        let api_clone = api.clone();
        Self {
            api,
            backlight_brightness: api_clone.get_backlight_brightness(),
            backlight_effect: api_clone.get_backlight_effect(),
            rgblight_brightness: api_clone.get_rgblight_brightness(),
            rgblight_effect: api_clone.get_rgblight_effect(),
            rgblight_effect_speed: api_clone.get_rgblight_effect_speed(),
            rgblight_color: api_clone.get_rgblight_color(),
            rgb_matrix_brightness: api_clone.get_rgb_matrix_brightness(),
            rgb_matrix_effect: api_clone.get_rgb_matrix_effect(),
            rgb_matrix_effect_speed: api_clone.get_rgb_matrix_effect_speed(),
            rgb_matrix_color: api_clone.get_rgb_matrix_color(),
            led_matrix_brightness: api_clone.get_led_matrix_brightness(),
            led_matrix_effect: api_clone.get_led_matrix_effect(),
            led_matrix_effect_speed: api_clone.get_led_matrix_effect_speed(),
            protocol_version: api_clone.get_protocol_version(),
            layer_count: api_clone.get_layer_count(),
            macro_count: api_clone.get_macro_count(),
            audio_enabled: api_clone.get_audio_enabled(),
            audio_clicky_enabled: api_clone.get_audio_clicky_enabled(),
        }
    }
}

impl ViaController {
    fn render_slider(
        ui: &mut egui::Ui,
        label: &str,
        value: &mut u8,
        range: std::ops::RangeInclusive<u8>,
        on_change: impl Fn(u8),
    ) {
        if ui
            .add(egui::Slider::new(value, range).text(label))
            .changed()
        {
            on_change(*value);
        }
    }

    fn render_effect_control(
        ui: &mut egui::Ui,
        label: &str,
        value: &mut u8,
        range: std::ops::RangeInclusive<u8>,
        on_change: impl Fn(u8),
    ) {
        ui.horizontal(|ui| {
            if ui.button(" - ").clicked() {
                *value = value.saturating_sub(1);
                on_change(*value);
            }

            if ui
                .add(egui::DragValue::new(value).range(range).speed(1))
                .changed()
            {
                on_change(*value);
            }

            if ui.button(" + ").clicked() {
                *value = value.saturating_add(1);
                on_change(*value);
            }

            ui.label(label);
        });
    }

    fn render_color_picker(ui: &mut egui::Ui, color: &mut (u8, u8), on_change: impl Fn((u8, u8))) {
        if ui
            .add(egui::Slider::new(&mut color.0, 0..=255).text("Hue"))
            .changed()
        {
            on_change(*color);
        }

        if ui
            .add(egui::Slider::new(&mut color.1, 0..=255).text("Saturation"))
            .changed()
        {
            on_change(*color);
        }

        let mut display_color =
            ecolor::Hsva::new(color.0 as f32 / 255.0, color.1 as f32 / 255.0, 1.0, 1.0);
        ui.add_enabled_ui(false, |ui| {
            egui::widgets::color_picker::color_edit_button_hsva(
                ui,
                &mut display_color,
                egui::widgets::color_picker::Alpha::Opaque,
            );
        });
    }
}

impl eframe::App for ViaController {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    egui::CollapsingHeader::new("Keyboard Info")
                        .default_open(true)
                        .show(ui, |ui| {
                            if let Some(version) = self.protocol_version {
                                ui.label(format!("Protocol Version: {}", version));
                            }

                            if let Some(count) = self.layer_count {
                                ui.label(format!("Layer Count: {}", count));
                            }

                            if let Some(count) = self.macro_count {
                                ui.label(format!("Macro Count: {}", count));
                            }
                        });

                    ui.add_enabled_ui(self.audio_enabled.is_some(), |ui| {
                        egui::CollapsingHeader::new("Audio")
                            .default_open(false)
                            .show(ui, |ui| {
                                if ui
                                    .checkbox(self.audio_enabled.as_mut().unwrap(), "Audio Enabled")
                                    .changed()
                                {
                                    self.api.set_audio_enabled(self.audio_enabled.unwrap());
                                }

                                if ui
                                    .checkbox(
                                        self.audio_clicky_enabled.as_mut().unwrap(),
                                        "Audio Clicky Enabled",
                                    )
                                    .changed()
                                {
                                    self.api.set_audio_clicky_enabled(
                                        self.audio_clicky_enabled.unwrap(),
                                    );
                                }
                            });
                    });

                    ui.add_enabled_ui(self.backlight_brightness.is_some(), |ui| {
                        egui::CollapsingHeader::new("Backlight")
                            .default_open(false)
                            .show(ui, |ui| {
                                Self::render_effect_control(
                                    ui,
                                    "Effect",
                                    self.backlight_effect.as_mut().unwrap(),
                                    0..=255,
                                    |value| {
                                        self.api.set_backlight_effect(value).unwrap_or_default()
                                    },
                                );

                                Self::render_slider(
                                    ui,
                                    "Brightness",
                                    self.backlight_brightness.as_mut().unwrap(),
                                    0..=255,
                                    |value| {
                                        self.api.set_backlight_brightness(value).unwrap_or_default()
                                    },
                                );
                            });
                    });

                    ui.add_enabled_ui(self.rgblight_brightness.is_some(), |ui| {
                        egui::CollapsingHeader::new("RGB Light")
                            .default_open(false)
                            .show(ui, |ui| {
                                Self::render_effect_control(
                                    ui,
                                    "Effect",
                                    self.rgblight_effect.as_mut().unwrap(),
                                    0..=255,
                                    |value| self.api.set_rgblight_effect(value).unwrap_or_default(),
                                );

                                Self::render_slider(
                                    ui,
                                    "Brightness",
                                    self.rgblight_brightness.as_mut().unwrap(),
                                    0..=255,
                                    |value| {
                                        self.api.set_rgblight_brightness(value).unwrap_or_default()
                                    },
                                );

                                Self::render_slider(
                                    ui,
                                    "Speed",
                                    self.rgblight_effect_speed.as_mut().unwrap(),
                                    0..=255,
                                    |value| {
                                        self.api
                                            .set_rgblight_effect_speed(value)
                                            .unwrap_or_default()
                                    },
                                );

                                Self::render_color_picker(
                                    ui,
                                    self.rgblight_color.as_mut().unwrap(),
                                    |(h, s)| {
                                        self.api.set_rgblight_color(h, s).unwrap_or_default();
                                    },
                                );
                            });
                    });

                    ui.add_enabled_ui(self.rgb_matrix_brightness.is_some(), |ui| {
                        egui::CollapsingHeader::new("RGB Matrix")
                            .default_open(false)
                            .show(ui, |ui| {
                                Self::render_effect_control(
                                    ui,
                                    "Effect",
                                    self.rgb_matrix_effect.as_mut().unwrap(),
                                    0..=255,
                                    |value| {
                                        self.api.set_rgb_matrix_effect(value).unwrap_or_default()
                                    },
                                );

                                Self::render_slider(
                                    ui,
                                    "Brightness",
                                    self.rgb_matrix_brightness.as_mut().unwrap(),
                                    0..=255,
                                    |value| {
                                        self.api
                                            .set_rgb_matrix_brightness(value)
                                            .unwrap_or_default()
                                    },
                                );

                                Self::render_slider(
                                    ui,
                                    "Speed",
                                    self.rgb_matrix_effect_speed.as_mut().unwrap(),
                                    0..=255,
                                    |value| {
                                        self.api
                                            .set_rgb_matrix_effect_speed(value)
                                            .unwrap_or_default()
                                    },
                                );

                                Self::render_color_picker(
                                    ui,
                                    self.rgb_matrix_color.as_mut().unwrap(),
                                    |(h, s)| {
                                        self.api.set_rgb_matrix_color(h, s).unwrap_or_default();
                                    },
                                );
                            });
                    });

                    ui.add_enabled_ui(self.led_matrix_brightness.is_some(), |ui| {
                        egui::CollapsingHeader::new("LED Matrix")
                            .default_open(false)
                            .show(ui, |ui| {
                                Self::render_effect_control(
                                    ui,
                                    "Effect",
                                    self.led_matrix_effect.as_mut().unwrap(),
                                    0..=255,
                                    |value| {
                                        self.api.set_led_matrix_effect(value).unwrap_or_default()
                                    },
                                );

                                Self::render_slider(
                                    ui,
                                    "Brightness",
                                    self.led_matrix_brightness.as_mut().unwrap(),
                                    0..=255,
                                    |value| {
                                        self.api
                                            .set_led_matrix_brightness(value)
                                            .unwrap_or_default()
                                    },
                                );

                                Self::render_slider(
                                    ui,
                                    "Speed",
                                    self.led_matrix_effect_speed.as_mut().unwrap(),
                                    0..=255,
                                    |value| {
                                        self.api
                                            .set_led_matrix_effect_speed(value)
                                            .unwrap_or_default()
                                    },
                                );
                            });
                    });

                    egui::CollapsingHeader::new("Advanced")
                        .default_open(false)
                        .show(ui, |ui| {
                            if ui.button("Reset EEPROM").clicked() {
                                self.api.reset_eeprom();
                            }

                            if ui.button("Reset Macros").clicked() {
                                self.api.reset_macros();
                            }

                            if ui.button("Jump to Bootloader").clicked() {
                                self.api.jump_to_bootloader();
                            }
                        });
                });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "VIA Controller",
        options,
        Box::new(|_cc| Ok(Box::<ViaController>::default())),
    )
}
