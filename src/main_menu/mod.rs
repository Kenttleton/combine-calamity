use super::GameState;
use bevy::{
    app::AppExit,
    prelude::*,
    ui::{widget::ImageMode, FocusPolicy},
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup)
            .add_system_set(
                SystemSet::on_enter(GameState::MainMenu)
                    .with_system(cleanup)
                    .with_system(setup),
            )
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(cleanup));
    }
}

enum MenuButton {
    Play,
    //Save,
    Quit,
}

struct UiAssets {
    root: Color,
    font: Handle<Font>,
    menu: Color,
    border: Color,
    button: Color,
    button_hovered: Color,
    button_pressed: Color,
    button_text: Color,
    image: Handle<Image>,
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let ui_assets = UiAssets {
        root: Color::NONE,
        font: assets.load("fonts/Monthoers.ttf"),
        image: assets.load("images/combine-harvester.png"),
        menu: Color::BLACK,
        border: Color::YELLOW,
        button: Color::NONE,
        button_hovered: Color::rgba(0.9, 0.9, 0.9, 0.25),
        button_pressed: Color::BISQUE,
        button_text: Color::ANTIQUE_WHITE,
    };
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(root(&ui_assets))
        .with_children(|parent| {
            parent.spawn_bundle(title_text(&ui_assets));
            parent.spawn_bundle(background_image(&ui_assets));
            parent
                .spawn_bundle(border(&ui_assets))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(menu_background(&ui_assets))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(button(&ui_assets))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(&ui_assets, "Start"));
                                });
                            //.insert(MenuButton::Play);
                            parent
                                .spawn_bundle(button(&ui_assets))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(&ui_assets, "Quit"));
                                });
                            //.insert(MenuButton::Quit);
                        });
                });
        });
}

fn background_image(ui_assets: &UiAssets) -> ImageBundle {
    ImageBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        image_mode: ImageMode::KeepAspect,
        color: Color::NONE.into(),
        image: ui_assets.image.clone().into(),
        ..Default::default()
    }
}

fn root(ui_assets: &UiAssets) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            display: Display::Flex,
            position_type: PositionType::Absolute,
            direction: Direction::LeftToRight,
            flex_direction: FlexDirection::ColumnReverse,
            flex_wrap: FlexWrap::NoWrap,
            overflow: Overflow::Hidden,
            ..Default::default()
        },
        color: ui_assets.root.clone().into(),
        image: ui_assets.image.clone().into(),
        ..Default::default()
    }
}

fn title_text(ui_assets: &UiAssets) -> TextBundle {
    TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(60.0)),
            ..Default::default()
        },
        text: Text::with_section(
            "Combine Calamity",
            TextStyle {
                font: ui_assets.font.clone(),
                font_size: 76.0,
                color: ui_assets.button_text.clone().into(),
            },
            Default::default(),
        ),
        ..Default::default()
    }
}

fn border(ui_assets: &UiAssets) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(400.0), Val::Auto),
            border: Rect::all(Val::Px(8.0)),
            margin: Rect::all(Val::Px(60.0)),
            ..Default::default()
        },
        color: ui_assets.border.clone().into(),
        ..Default::default()
    }
}

fn menu_background(ui_assets: &UiAssets) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            padding: Rect::all(Val::Px(5.0)),
            ..Default::default()
        },
        color: ui_assets.menu.clone().into(),
        ..Default::default()
    }
}

fn button(ui_assets: &UiAssets) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: ui_assets.button.clone().into(),
        ..Default::default()
    }
}

fn button_text(ui_assets: &UiAssets, text: &str) -> TextBundle {
    TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(10.0)),
            ..Default::default()
        },
        text: Text::with_section(
            text,
            TextStyle {
                font: ui_assets.font.clone(),
                font_size: 36.0,
                color: ui_assets.button_text.clone().into(),
            },
            Default::default(),
        ),
        ..Default::default()
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
