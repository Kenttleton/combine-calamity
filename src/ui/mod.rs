use super::GameState;
//pub mod game;
mod menu;
mod splash;
use bevy::prelude::*;

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMenuScreen;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(splash::SplashPlugin)
            .add_plugin(menu::MenuPlugin);
        //.add_plugin(game::GameOverlayPlugin);
    }
    fn name(&self) -> &str {
        "UIPlugins"
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

// struct UiAssets {
//     root: Color,
//     font: Handle<Font>,
//     menu: Color,
//     border: Color,
//     button: Color,
//     button_hovered: Color,
//     button_pressed: Color,
//     button_hovered_pressed: Color,
//     button_text: Color,
//     image: Handle<Image>,
//     button_style: Style,
//     button_icon_style: Style,
// }

// fn get_styles(assets: Res<AssetServer>) -> UiAssets {
//     UiAssets {
//         root: Color::NONE,
//         font: assets.load("fonts/Monthoers.ttf"),
//         image: assets.load("images/combine-harvester.png"),
//         menu: Color::BLACK,
//         border: Color::YELLOW,
//         button: Color::NONE,
//         button_hovered: Color::rgba(0.9, 0.9, 0.9, 0.25),
//         button_pressed: Color::BISQUE,
//         button_hovered_pressed: Color::BISQUE,
//         button_text: Color::ANTIQUE_WHITE,
//         button_style: Style {
//             size: Size::new(Val::Px(250.0), Val::Px(65.0)),
//             margin: UiRect::all(Val::Px(20.0)),
//             justify_content: JustifyContent::Center,
//             align_items: AlignItems::Center,
//             ..default()
//         },
//         button_icon_style: Style {
//             size: Size::new(Val::Px(30.0), Val::Auto),
//             // This takes the icons out of the flexbox flow, to be positioned exactly
//             position_type: PositionType::Absolute,
//             // The icon will be close to the left border of the button
//             position: UiRect {
//                 left: Val::Px(10.0),
//                 right: Val::Auto,
//                 top: Val::Auto,
//                 bottom: Val::Auto,
//             },
//             ..default()
//         },
//     }
// }
