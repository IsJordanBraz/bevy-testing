use bevy::prelude::*;

use crate::ui::{components::{MainMenu, PlayButton, QuitButton}, styles::{NORMAL_BUTTON_COLOR, BUTTON_STYLE}};

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity {
    let main_menu_entity = commands.spawn(
        (NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::RED.into(),
            ..Default::default()
        },
        MainMenu {}
    )).with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..Default::default()
            },
            PlayButton {}
        ));
        // BUTTON PLAY
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(200.0),
                    ..Default::default()
                },
                background_color: Color::GREEN.into(),
                ..Default::default()
            },
            PlayButton {}
        )).with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Play",
                                TextStyle { 
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"), 
                                    font_size: 32.0, 
                                    color: Color::BLACK.into() 
                                }
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        });
        // button quit
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(200.0),
                    ..Default::default()
                },
                background_color: Color::GRAY.into(),
                ..Default::default()
            },
            QuitButton {}
        )).with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "QUIT",
                                TextStyle { 
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"), 
                                    font_size: 32.0, 
                                    color: Color::BLACK.into() 
                                }
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        });
    })
    .id();
    main_menu_entity
}