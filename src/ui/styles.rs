use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::AQUAMARINE;
pub const PRESSED_BUTTON_COLOR: Color = Color::RED;
pub const NONE_BUTTON_COLOR: Color = Color::GOLD;

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.column_gap = Val::Px(10.0);
    style.row_gap = Val::Px(10.0);
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style
};
