#![allow(unused)]

use egui::style::{Selection, WidgetVisuals, Widgets};
use egui::{Color32, Shadow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
    pub font_size: f32,
    pub margin: Margin,
    pub query_corner_radius: CornerRadius,
    pub query_corner_radius_with_results: CornerRadius,
    pub max_height: f32,

    // egui Visual
    pub dark_mode: bool,
    pub hyperlink_color: RgbaUnmultiplied,
    pub faint_bg_color: RgbaUnmultiplied,
    pub extreme_bg_color: RgbaUnmultiplied,
    pub code_bg_color: RgbaUnmultiplied,
    pub warn_fg_color: RgbaUnmultiplied,
    pub error_fg_color: RgbaUnmultiplied,
    pub window_fill: RgbaUnmultiplied,
    pub panel_fill: RgbaUnmultiplied,
    pub window_stroke: Stroke,
    pub noninteractive_bg_fill: RgbaUnmultiplied,
    pub noninteractive_weak_bg_fill: RgbaUnmultiplied,
    pub noninteractive_bg_stroke: Stroke,
    pub noninteractive_fg_stroke: Stroke,
    pub noninteractive_corner_radius: CornerRadius,
    pub noninteractive_expansion: f32,
    pub inactive_bg_fill: RgbaUnmultiplied,
    pub inactive_weak_bg_fill: RgbaUnmultiplied,
    pub inactive_bg_stroke: Stroke,
    pub inactive_fg_stroke: Stroke,
    pub inactive_corner_radius: CornerRadius,
    pub inactive_expansion: f32,
    pub hovered_bg_fill: RgbaUnmultiplied,
    pub hovered_weak_bg_fill: RgbaUnmultiplied,
    pub hovered_bg_stroke: Stroke,
    pub hovered_fg_stroke: Stroke,
    pub hovered_corner_radius: CornerRadius,
    pub hovered_expansion: f32,
    pub active_bg_fill: RgbaUnmultiplied,
    pub active_weak_bg_fill: RgbaUnmultiplied,
    pub active_bg_stroke: Stroke,
    pub active_fg_stroke: Stroke,
    pub active_corner_radius: CornerRadius,
    pub active_expansion: f32,
    pub open_bg_fill: RgbaUnmultiplied,
    pub open_weak_bg_fill: RgbaUnmultiplied,
    pub open_bg_stroke: Stroke,
    pub open_fg_stroke: Stroke,
    pub open_corner_radius: CornerRadius,
    pub open_expansion: f32,
    pub selection_bg_fill: RgbaUnmultiplied,
    pub selection_stroke: Stroke,
    pub window_shadow_color: RgbaUnmultiplied,
    pub window_shadow_offset: [i8; 2],
    pub window_shadow_blur: u8,
    pub window_shadow_spread: u8,
    pub popup_shadow_color: RgbaUnmultiplied,
    pub popup_shadow_offset: [i8; 2],
    pub popup_shadow_blur: u8,
    pub popup_shadow_spread: u8,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Margin {
    pub left: i8,
    pub right: i8,
    pub top: i8,
    pub bottom: i8,
}

impl Margin {
    pub const NONE: Margin = Margin::splat(0);

    pub const fn symmetric(x: i8, y: i8) -> Margin {
        Margin {
            left: x,
            right: y,
            top: y,
            bottom: y,
        }
    }
    pub const fn splat(v: i8) -> Margin {
        Margin {
            left: v,
            right: v,
            top: v,
            bottom: v,
        }
    }

    pub const fn x(self) -> i8 {
        self.left + self.right
    }

    pub const fn y(self) -> i8 {
        self.top + self.bottom
    }

    pub const fn to(self) -> egui::Margin {
        let Self {
            left,
            right,
            top,
            bottom,
        } = self;

        egui::Margin {
            left,
            right,
            top,
            bottom,
        }
    }
}

impl Into<egui::Margin> for Margin {
    fn into(self) -> egui::Margin {
        self.to()
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct CornerRadius {
    pub ne: f32,
    pub nw: f32,
    pub se: f32,
    pub sw: f32,
}

impl CornerRadius {
    pub const NONE: CornerRadius = CornerRadius::splat(0.);

    const fn splat(v: f32) -> Self {
        CornerRadius {
            ne: v,
            nw: v,
            se: v,
            sw: v,
        }
    }

    pub fn to(self) -> egui::CornerRadius {
        let CornerRadius { ne, nw, se, sw } = self;
        egui::epaint::CornerRadiusF32 { ne, nw, se, sw }.into()
    }

    pub const fn flip_y(self) -> Self {
        Self {
            ne: self.se,
            nw: self.sw,
            se: self.ne,
            sw: self.nw,
        }
    }

    pub const fn flip_x(self) -> Self {
        Self {
            ne: self.nw,
            nw: self.ne,
            se: self.sw,
            sw: self.se,
        }
    }
}

impl Into<egui::CornerRadius> for CornerRadius {
    fn into(self) -> egui::CornerRadius {
        self.to()
    }
}

pub trait CornerRadiusExt {
    fn to(&self) -> CornerRadius;
}

impl CornerRadiusExt for egui::epaint::CornerRadiusF32 {
    fn to(&self) -> CornerRadius {
        CornerRadius {
            ne: self.ne,
            nw: self.nw,
            se: self.se,
            sw: self.sw,
        }
    }
}

impl CornerRadiusExt for egui::CornerRadius {
    fn to(&self) -> CornerRadius {
        egui::epaint::CornerRadiusF32::from(*self).to()
    }
}

impl Default for CornerRadius {
    fn default() -> Self {
        CornerRadius::splat(0.)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Stroke(f32, RgbaUnmultiplied);

impl Stroke {
    pub const NONE: Self = Stroke(0., RgbaUnmultiplied::TRANSPARENT);

    pub fn thickness(self) -> f32 {
        self.0
    }

    pub fn color(self) -> RgbaUnmultiplied {
        self.1
    }

    pub fn to(self) -> egui::Stroke {
        egui::Stroke {
            width: self.0,
            color: self.1.to(),
        }
    }
}

impl Into<egui::Stroke> for Stroke {
    fn into(self) -> egui::Stroke {
        self.to()
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(untagged)]
pub enum RgbaUnmultiplied {
    Struct { r: u8, g: u8, b: u8, a: u8 },
    Tuple(u8, u8, u8, u8),
    List([u8; 4]),
}

impl RgbaUnmultiplied {
    pub const TRANSPARENT: Self = RgbaUnmultiplied::Struct {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const BLACK: Self = RgbaUnmultiplied::Struct {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const RED: Self = RgbaUnmultiplied::Struct {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Self = RgbaUnmultiplied::Struct {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Self = RgbaUnmultiplied::Struct {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };

    pub fn to(self) -> Color32 {
        match self {
            RgbaUnmultiplied::Struct { r, g, b, a } => Color32::from_rgba_unmultiplied(r, g, b, a),
            RgbaUnmultiplied::Tuple(r, g, b, a) => Color32::from_rgba_unmultiplied(r, g, b, a),
            RgbaUnmultiplied::List([r, g, b, a]) => Color32::from_rgba_unmultiplied(r, g, b, a),
        }
    }
}

impl Into<Color32> for RgbaUnmultiplied {
    fn into(self) -> Color32 {
        self.to()
    }
}

pub trait Color32Ext {
    fn to(&self) -> RgbaUnmultiplied;
}

impl Color32Ext for Color32 {
    fn to(&self) -> RgbaUnmultiplied {
        RgbaUnmultiplied::List(self.to_srgba_unmultiplied())
    }
}

impl Default for Theme {
    fn default() -> Self {
        #![allow(unused_variables)]

        let rosewater = Color32::from_rgb(245, 224, 220).to();
        let flamingo = Color32::from_rgb(242, 205, 205).to();
        let pink = Color32::from_rgb(245, 194, 231).to();
        let mauve = Color32::from_rgb(203, 166, 247).to();
        let red = Color32::from_rgb(243, 139, 168).to();
        let maroon = Color32::from_rgb(235, 160, 172).to();
        let peach = Color32::from_rgb(250, 179, 135).to();
        let yellow = Color32::from_rgb(249, 226, 175).to();
        let green = Color32::from_rgb(166, 227, 161).to();
        let teal = Color32::from_rgb(148, 226, 213).to();
        let sky = Color32::from_rgb(137, 220, 235).to();
        let sapphire = Color32::from_rgb(116, 199, 236).to();
        let blue = Color32::from_rgb(137, 180, 250).to();
        let lavender = Color32::from_rgb(180, 190, 254).to();
        let text = Color32::from_rgb(205, 214, 244).to();
        let subtext1 = Color32::from_rgb(186, 194, 222).to();
        let subtext0 = Color32::from_rgb(166, 173, 200).to();
        let overlay2 = Color32::from_rgb(147, 153, 178).to();
        let overlay1 = Color32::from_rgb(127, 132, 156).to();
        let overlay0 = Color32::from_rgb(108, 112, 134).to();
        let surface2 = Color32::from_rgb(88, 91, 112).to();
        let surface1 = Color32::from_rgb(69, 71, 90).to();
        let surface0 = Color32::from_rgb(49, 50, 68).to();
        let base = Color32::from_rgb(30, 30, 46).to();
        let mantle = Color32::from_rgb(24, 24, 37).to();
        let crust = Color32::from_rgb(17, 17, 27).to();

        Theme {
            font_size: 16.0,
            margin: Margin::symmetric(20, 10),
            query_corner_radius: CornerRadius::splat(20.),
            query_corner_radius_with_results: CornerRadius {
                ne: 20.,
                nw: 20.,
                se: 0.,
                sw: 0.,
            },
            max_height: 500.0,

            // egui Visual
            dark_mode: true,
            hyperlink_color: rosewater,
            faint_bg_color: surface0,
            extreme_bg_color: crust,
            code_bg_color: mantle,
            warn_fg_color: peach,
            error_fg_color: maroon,
            window_fill: base.to().linear_multiply(0.9).to(),
            panel_fill: base,
            window_stroke: Stroke(0.0, overlay1),
            noninteractive_bg_fill: base,
            noninteractive_weak_bg_fill: base,
            noninteractive_bg_stroke: Stroke(0.0, overlay1),
            noninteractive_fg_stroke: Stroke(1.0, text),
            noninteractive_corner_radius: CornerRadius::splat(2.),
            noninteractive_expansion: 0.0,
            inactive_bg_fill: surface0,
            inactive_weak_bg_fill: surface0,
            inactive_bg_stroke: Stroke::NONE,
            inactive_fg_stroke: Stroke(1.0, text),
            inactive_corner_radius: CornerRadius::splat(2.),
            inactive_expansion: 0.0,
            hovered_bg_fill: surface2,
            hovered_weak_bg_fill: surface2,
            hovered_bg_stroke: Stroke(1.0, overlay1),
            hovered_fg_stroke: Stroke(1.0, text),
            hovered_corner_radius: CornerRadius::splat(3.),
            hovered_expansion: 0.0,
            active_bg_fill: surface1,
            active_weak_bg_fill: surface1,
            active_bg_stroke: Stroke(1.0, overlay1),
            active_fg_stroke: Stroke(1.0, text),
            active_corner_radius: CornerRadius::splat(2.),
            active_expansion: 0.0,
            open_bg_fill: surface0,
            open_weak_bg_fill: surface0,
            open_bg_stroke: Stroke(1.0, overlay1),
            open_fg_stroke: Stroke(1.0, text),
            open_corner_radius: CornerRadius::splat(2.),
            open_expansion: 0.0,
            selection_bg_fill: blue.to().linear_multiply(0.2).to(),
            selection_stroke: Stroke(1.0, text),
            window_shadow_color: Color32::from_black_alpha(96).to(),
            window_shadow_offset: [10, 20],
            window_shadow_blur: 15,
            window_shadow_spread: 0,
            popup_shadow_color: Color32::from_black_alpha(96).to(),
            popup_shadow_offset: [6, 10],
            popup_shadow_blur: 8,
            popup_shadow_spread: 0,
        }
    }
}

impl Theme {
    pub fn update(&self, style: &mut egui::Style) {
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::proportional(self.font_size),
        );

        style.text_styles.insert(
            egui::TextStyle::Monospace,
            egui::FontId::monospace(self.font_size),
        );

        let old_visual = style.visuals.clone();
        style.visuals = egui::Visuals {
            dark_mode: self.dark_mode,
            hyperlink_color: self.hyperlink_color.to(),
            faint_bg_color: self.faint_bg_color.to(),
            extreme_bg_color: self.extreme_bg_color.to(),
            code_bg_color: self.code_bg_color.to(),
            warn_fg_color: self.warn_fg_color.to(),
            error_fg_color: self.error_fg_color.to(),
            window_fill: self.window_fill.to(),
            panel_fill: self.panel_fill.to(),
            window_stroke: self.window_stroke.to(),
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: self.noninteractive_bg_fill.to(),
                    weak_bg_fill: self.noninteractive_weak_bg_fill.to(),
                    bg_stroke: self.noninteractive_bg_stroke.to(),
                    fg_stroke: self.noninteractive_fg_stroke.to(),
                    corner_radius: self.noninteractive_corner_radius.to(),
                    expansion: self.noninteractive_expansion,
                },
                inactive: WidgetVisuals {
                    bg_fill: self.inactive_bg_fill.to(),
                    weak_bg_fill: self.inactive_weak_bg_fill.to(),
                    bg_stroke: self.inactive_bg_stroke.to(),
                    fg_stroke: self.inactive_fg_stroke.to(),
                    corner_radius: self.inactive_corner_radius.to(),
                    expansion: self.inactive_expansion,
                },
                hovered: WidgetVisuals {
                    bg_fill: self.hovered_bg_fill.to(),
                    weak_bg_fill: self.hovered_weak_bg_fill.to(),
                    bg_stroke: self.hovered_bg_stroke.to(),
                    fg_stroke: self.hovered_fg_stroke.to(),
                    corner_radius: self.hovered_corner_radius.to(),
                    expansion: self.hovered_expansion,
                },
                active: WidgetVisuals {
                    bg_fill: self.active_bg_fill.to(),
                    weak_bg_fill: self.active_weak_bg_fill.to(),
                    bg_stroke: self.active_bg_stroke.to(),
                    fg_stroke: self.active_fg_stroke.to(),
                    corner_radius: self.active_corner_radius.to(),
                    expansion: self.active_expansion,
                },
                open: WidgetVisuals {
                    bg_fill: self.open_bg_fill.to(),
                    weak_bg_fill: self.open_weak_bg_fill.to(),
                    bg_stroke: self.open_bg_stroke.to(),
                    fg_stroke: self.open_fg_stroke.to(),
                    corner_radius: self.open_corner_radius.to(),
                    expansion: self.open_expansion,
                },
            },
            selection: Selection {
                bg_fill: self.selection_bg_fill.to(),
                stroke: self.selection_stroke.to(),
            },
            window_shadow: Shadow {
                color: self.window_shadow_color.to(),
                offset: self.window_shadow_offset,
                blur: self.window_shadow_blur,
                spread: self.window_shadow_spread,
            },
            popup_shadow: Shadow {
                color: self.popup_shadow_color.to(),
                offset: self.popup_shadow_offset,
                blur: self.popup_shadow_blur,
                spread: self.popup_shadow_spread,
            },
            ..old_visual
        };
    }
}
