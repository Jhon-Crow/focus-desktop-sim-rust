//! UI module for sidebar menus using egui
//!
//! Implements:
//! - Left sidebar: Object palette with categories (like the reference Electron app)
//! - Right sidebar: Object customization panel (colors, delete)

use crate::desk_object::ObjectType;
use egui::{Color32, RichText, Vec2};

/// Palette category for organizing object types
#[derive(Debug, Clone)]
pub struct PaletteCategory {
    pub name: &'static str,
    pub icon: &'static str,
    pub variants: Vec<PaletteVariant>,
    pub expanded: bool,
}

/// A variant within a palette category
#[derive(Debug, Clone)]
pub struct PaletteVariant {
    pub object_type: ObjectType,
    pub name: &'static str,
    pub icon: &'static str,
}

/// Color presets for object customization
pub const COLOR_PRESETS: &[(u32, &str)] = &[
    (0xEF4444, "Red"),
    (0xF97316, "Orange"),
    (0xEAB308, "Yellow"),
    (0x22C55E, "Green"),
    (0x3B82F6, "Blue"),
    (0x8B5CF6, "Purple"),
    (0xEC4899, "Pink"),
    (0xFFFFFF, "White"),
    (0x64748B, "Gray"),
    (0x1E293B, "Dark"),
];

pub const ACCENT_COLOR_PRESETS: &[(u32, &str)] = &[
    (0xFBBF24, "Amber"),
    (0xA3E635, "Lime"),
    (0x2DD4BF, "Teal"),
    (0x60A5FA, "Light Blue"),
    (0xC084FC, "Lavender"),
    (0xF472B6, "Rose"),
    (0xFB923C, "Peach"),
    (0xD4D4D4, "Silver"),
    (0x000000, "Black"),
    (0xFFFFFF, "White"),
];

/// UI state for menus
pub struct UiState {
    /// Whether the left sidebar (palette) is open
    pub left_sidebar_open: bool,
    /// Whether the right sidebar (customization) is open
    pub right_sidebar_open: bool,
    /// Palette categories
    pub categories: Vec<PaletteCategory>,
    /// Currently selected object for customization
    pub selected_object_id: Option<u64>,
    /// Current main color for selected object
    pub current_main_color: u32,
    /// Current accent color for selected object
    pub current_accent_color: u32,
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

impl UiState {
    pub fn new() -> Self {
        let categories = vec![
            PaletteCategory {
                name: "Clocks",
                icon: "🕐",
                variants: vec![
                    PaletteVariant {
                        object_type: ObjectType::Clock,
                        name: "Clock",
                        icon: "🕐",
                    },
                    PaletteVariant {
                        object_type: ObjectType::Hourglass,
                        name: "Hourglass",
                        icon: "⏳",
                    },
                ],
                expanded: false,
            },
            PaletteCategory {
                name: "Lighting",
                icon: "💡",
                variants: vec![PaletteVariant {
                    object_type: ObjectType::Lamp,
                    name: "Desk Lamp",
                    icon: "💡",
                }],
                expanded: false,
            },
            PaletteCategory {
                name: "Writing",
                icon: "📝",
                variants: vec![
                    PaletteVariant {
                        object_type: ObjectType::Notebook,
                        name: "Notebook",
                        icon: "📓",
                    },
                    PaletteVariant {
                        object_type: ObjectType::Paper,
                        name: "Paper",
                        icon: "📄",
                    },
                    PaletteVariant {
                        object_type: ObjectType::PenHolder,
                        name: "Pen Holder",
                        icon: "🖊️",
                    },
                ],
                expanded: false,
            },
            PaletteCategory {
                name: "Books",
                icon: "📚",
                variants: vec![
                    PaletteVariant {
                        object_type: ObjectType::Books,
                        name: "Books",
                        icon: "📕",
                    },
                    PaletteVariant {
                        object_type: ObjectType::Magazine,
                        name: "Magazine",
                        icon: "📰",
                    },
                ],
                expanded: false,
            },
            PaletteCategory {
                name: "Audio",
                icon: "🎵",
                variants: vec![PaletteVariant {
                    object_type: ObjectType::Metronome,
                    name: "Metronome",
                    icon: "🎵",
                }],
                expanded: false,
            },
            PaletteCategory {
                name: "Trinkets",
                icon: "🎁",
                variants: vec![
                    PaletteVariant {
                        object_type: ObjectType::Coffee,
                        name: "Coffee Mug",
                        icon: "☕",
                    },
                    PaletteVariant {
                        object_type: ObjectType::Plant,
                        name: "Plant",
                        icon: "🌱",
                    },
                    PaletteVariant {
                        object_type: ObjectType::Globe,
                        name: "Globe",
                        icon: "🌍",
                    },
                    PaletteVariant {
                        object_type: ObjectType::Trophy,
                        name: "Trophy",
                        icon: "🏆",
                    },
                ],
                expanded: false,
            },
            PaletteCategory {
                name: "Frames",
                icon: "🖼️",
                variants: vec![PaletteVariant {
                    object_type: ObjectType::PhotoFrame,
                    name: "Photo Frame",
                    icon: "🖼️",
                }],
                expanded: false,
            },
            PaletteCategory {
                name: "Tech",
                icon: "💻",
                variants: vec![PaletteVariant {
                    object_type: ObjectType::Laptop,
                    name: "Laptop",
                    icon: "💻",
                }],
                expanded: false,
            },
        ];

        Self {
            left_sidebar_open: false,
            right_sidebar_open: false,
            categories,
            selected_object_id: None,
            current_main_color: 0xFFFFFF,
            current_accent_color: 0x1E293B,
        }
    }

    pub fn toggle_left_sidebar(&mut self) {
        self.left_sidebar_open = !self.left_sidebar_open;
    }

    pub fn toggle_right_sidebar(&mut self) {
        self.right_sidebar_open = !self.right_sidebar_open;
    }

    pub fn open_customization(&mut self, object_id: u64, main_color: u32, accent_color: u32) {
        self.selected_object_id = Some(object_id);
        self.current_main_color = main_color;
        self.current_accent_color = accent_color;
        self.right_sidebar_open = true;
    }

    pub fn close_customization(&mut self) {
        self.selected_object_id = None;
        self.right_sidebar_open = false;
    }
}

/// UI action that can be returned from rendering
#[derive(Debug, Clone)]
pub enum UiAction {
    /// Add an object of the specified type
    AddObject(ObjectType),
    /// Delete the currently selected object
    DeleteObject(u64),
    /// Change main color of selected object
    ChangeMainColor(u64, u32),
    /// Change accent color of selected object
    ChangeAccentColor(u64, u32),
    /// Clear all objects from the desk
    ClearAll,
    /// Close the customization panel
    CloseCustomization,
    /// No action
    None,
}

/// Render the left sidebar (object palette)
pub fn render_left_sidebar(ctx: &egui::Context, ui_state: &mut UiState) -> Vec<UiAction> {
    let mut actions = Vec::new();

    // Menu toggle button (always visible)
    egui::Area::new(egui::Id::new("menu_toggle_area"))
        .fixed_pos(egui::pos2(20.0, 20.0))
        .show(ctx, |ui| {
            let button = egui::Button::new(RichText::new("☰").size(24.0).color(Color32::WHITE))
                .fill(Color32::from_rgb(79, 70, 229))
                .min_size(Vec2::new(50.0, 50.0));

            if ui.add(button).clicked() {
                ui_state.toggle_left_sidebar();
            }
        });

    // Left sidebar panel
    if ui_state.left_sidebar_open {
        egui::SidePanel::left("palette_panel")
            .resizable(false)
            .default_width(260.0)
            .show(ctx, |ui| {
                ui.add_space(10.0);

                // Header
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.label(RichText::new("🎨 Palette").size(18.0).strong().color(Color32::WHITE));
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // Palette categories
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let mut category_clicked = None;
                    let mut variant_clicked = None;

                    for (cat_idx, category) in ui_state.categories.iter().enumerate() {
                        // Category header
                        let header_response = ui.add(
                            egui::Button::new(
                                RichText::new(format!("{} {}", category.icon, category.name))
                                    .size(14.0)
                                    .color(Color32::from_gray(220)),
                            )
                            .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 13))
                            .min_size(Vec2::new(ui.available_width(), 40.0)),
                        );

                        if header_response.clicked() {
                            category_clicked = Some(cat_idx);
                        }

                        // Expanded variants
                        if category.expanded {
                            ui.add_space(5.0);
                            for (var_idx, variant) in category.variants.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.add_space(20.0);
                                    let variant_button = egui::Button::new(
                                        RichText::new(format!("{} {}", variant.icon, variant.name))
                                            .size(12.0)
                                            .color(Color32::from_gray(200)),
                                    )
                                    .fill(Color32::from_rgba_unmultiplied(79, 70, 229, 51))
                                    .min_size(Vec2::new(ui.available_width() - 30.0, 35.0));

                                    if ui.add(variant_button).clicked() {
                                        variant_clicked = Some((cat_idx, var_idx));
                                    }
                                });
                            }
                            ui.add_space(5.0);
                        }
                    }

                    // Handle category toggle
                    if let Some(cat_idx) = category_clicked {
                        ui_state.categories[cat_idx].expanded = !ui_state.categories[cat_idx].expanded;
                    }

                    // Handle variant click (add object)
                    if let Some((cat_idx, var_idx)) = variant_clicked {
                        let object_type = ui_state.categories[cat_idx].variants[var_idx].object_type;
                        actions.push(UiAction::AddObject(object_type));
                    }

                    ui.add_space(20.0);

                    // Clear all button
                    ui.separator();
                    ui.add_space(10.0);

                    let clear_button = egui::Button::new(
                        RichText::new("🗑️ Clear All Objects")
                            .size(14.0)
                            .color(Color32::from_rgb(239, 68, 68)),
                    )
                    .fill(Color32::from_rgba_unmultiplied(239, 68, 68, 51))
                    .min_size(Vec2::new(ui.available_width() - 20.0, 40.0));

                    if ui.add(clear_button).clicked() {
                        actions.push(UiAction::ClearAll);
                    }

                    ui.add_space(20.0);

                    // Instructions
                    ui.separator();
                    ui.add_space(10.0);
                    ui.label(RichText::new("Controls:").size(12.0).color(Color32::from_gray(150)));
                    ui.label(RichText::new("• Click+Drag to move").size(11.0).color(Color32::from_gray(120)));
                    ui.label(RichText::new("• Scroll to rotate").size(11.0).color(Color32::from_gray(120)));
                    ui.label(RichText::new("• Shift+Scroll to scale").size(11.0).color(Color32::from_gray(120)));
                    ui.label(RichText::new("• Right-click to customize").size(11.0).color(Color32::from_gray(120)));
                    ui.label(RichText::new("• Delete to remove").size(11.0).color(Color32::from_gray(120)));
                });
            });
    }

    actions
}

/// Render the right sidebar (object customization)
pub fn render_right_sidebar(ctx: &egui::Context, ui_state: &mut UiState, object_name: Option<&str>) -> Vec<UiAction> {
    let mut actions = Vec::new();

    if !ui_state.right_sidebar_open || ui_state.selected_object_id.is_none() {
        return actions;
    }

    let object_id = ui_state.selected_object_id.unwrap();

    egui::SidePanel::right("customization_panel")
        .resizable(false)
        .default_width(280.0)
        .show(ctx, |ui| {
            ui.add_space(10.0);

            // Header with close button
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                let title = object_name.unwrap_or("Object");
                ui.label(RichText::new(format!("Customize {}", title)).size(16.0).strong().color(Color32::WHITE));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(RichText::new("✕").size(16.0)).clicked() {
                        actions.push(UiAction::CloseCustomization);
                    }
                });
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(15.0);

            // Main color section
            ui.label(RichText::new("MAIN COLOR").size(11.0).color(Color32::from_gray(150)));
            ui.add_space(8.0);

            egui::Grid::new("main_colors")
                .spacing(Vec2::new(8.0, 8.0))
                .show(ui, |ui| {
                    for (i, (color, _name)) in COLOR_PRESETS.iter().enumerate() {
                        let r = ((color >> 16) & 0xFF) as u8;
                        let g = ((color >> 8) & 0xFF) as u8;
                        let b = (color & 0xFF) as u8;

                        let is_selected = *color == ui_state.current_main_color;
                        let button_size = if is_selected { 36.0 } else { 32.0 };

                        let button = egui::Button::new("")
                            .fill(Color32::from_rgb(r, g, b))
                            .min_size(Vec2::new(button_size, button_size))
                            .stroke(if is_selected {
                                egui::Stroke::new(2.0, Color32::WHITE)
                            } else {
                                egui::Stroke::NONE
                            });

                        if ui.add(button).clicked() {
                            ui_state.current_main_color = *color;
                            actions.push(UiAction::ChangeMainColor(object_id, *color));
                        }

                        if (i + 1) % 5 == 0 {
                            ui.end_row();
                        }
                    }
                });

            ui.add_space(20.0);

            // Accent color section
            ui.label(RichText::new("ACCENT COLOR").size(11.0).color(Color32::from_gray(150)));
            ui.add_space(8.0);

            egui::Grid::new("accent_colors")
                .spacing(Vec2::new(8.0, 8.0))
                .show(ui, |ui| {
                    for (i, (color, _name)) in ACCENT_COLOR_PRESETS.iter().enumerate() {
                        let r = ((color >> 16) & 0xFF) as u8;
                        let g = ((color >> 8) & 0xFF) as u8;
                        let b = (color & 0xFF) as u8;

                        let is_selected = *color == ui_state.current_accent_color;
                        let button_size = if is_selected { 36.0 } else { 32.0 };

                        let mut stroke = egui::Stroke::NONE;
                        if is_selected {
                            stroke = egui::Stroke::new(2.0, Color32::WHITE);
                        } else if *color == 0x000000 {
                            stroke = egui::Stroke::new(1.0, Color32::from_gray(100));
                        }

                        let button = egui::Button::new("")
                            .fill(Color32::from_rgb(r, g, b))
                            .min_size(Vec2::new(button_size, button_size))
                            .stroke(stroke);

                        if ui.add(button).clicked() {
                            ui_state.current_accent_color = *color;
                            actions.push(UiAction::ChangeAccentColor(object_id, *color));
                        }

                        if (i + 1) % 5 == 0 {
                            ui.end_row();
                        }
                    }
                });

            ui.add_space(30.0);

            // Delete button
            let delete_button = egui::Button::new(
                RichText::new("Delete Object")
                    .size(14.0)
                    .color(Color32::from_rgb(239, 68, 68)),
            )
            .fill(Color32::from_rgba_unmultiplied(239, 68, 68, 51))
            .min_size(Vec2::new(ui.available_width() - 20.0, 40.0));

            if ui.add(delete_button).clicked() {
                actions.push(UiAction::DeleteObject(object_id));
            }
        });

    actions
}

/// Helper function to convert hex color to egui Color32
pub fn hex_to_color32(hex: u32) -> Color32 {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    Color32::from_rgb(r, g, b)
}
