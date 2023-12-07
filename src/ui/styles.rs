use bevy::{prelude::*, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}};

use crate::ui::components::FpsText;

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

pub fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(if cfg!(feature = "default_font") {
                TextStyle {
                    font_size: 60.0,
                    color: Color::GOLD,
                    ..default()
                }
            } else {
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                }
            }),
        ]),
        FpsText,
    ));
}
