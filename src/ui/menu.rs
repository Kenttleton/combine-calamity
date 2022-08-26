use super::{despawn_screen, GameState, OnMenuScreen};
use bevy::{app::AppExit, prelude::*};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems to handle the main menu screen
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Menu).with_system(despawn_screen::<OnMenuScreen>),
            )
            // Common systems to all screens that handles buttons behaviour
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(menu_action)
                    .with_system(button_system),
            );
    }
    fn name(&self) -> &str {
        "MenuPlugin"
    }
}

fn menu_setup(mut commands: Commands, assets: Res<AssetServer>) {
    let combine_image: Handle<Image> = assets.load("images/combine-harvester-nobackground.png");
    let font: Handle<Font> = assets.load("fonts/Monthoers.ttf");
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            color: Color::rgb(128.0 / 255.0, 5.0 / 255.0, 0.0).into(),
            ..default()
        })
        .insert(OnMenuScreen)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(60.0)),
                    ..default()
                },
                text: Text::from_section(
                    "Combine Calamity",
                    TextStyle {
                        font: font.clone().into(),
                        font_size: 76.0,
                        color: Color::ANTIQUE_WHITE.into(),
                    },
                ),
                ..default()
            });
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Px(450.0), Val::Px(450.0)),
                    ..default()
                },
                image: combine_image.into(),
                ..default()
            });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: Color::YELLOW.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            size: Size::new(Val::Percent(20.0), Val::Px(20.0)),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        text: Text::from_section(
                            "Play",
                            TextStyle {
                                font: font.clone().into(),
                                font_size: 36.0,
                                color: Color::BLACK.into(),
                            },
                        ),
                        ..default()
                    });
                });
        });
}

fn menu_action() {}

fn button_system() {}
