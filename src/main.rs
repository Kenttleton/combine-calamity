#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

const TIME_STEP: f32 = 1.0 / 60.0;
const BACKGROUND_COLOR: Color = Color::rgb(0., 0.71, 0.9);

// player
const PLAYER_WALK: f32 = 500.0;
const PLAYER_RUN: f32 = 250.0;
const PLAYER_JUMP: f32 = 250.0;

// enemies
const ENEMY_WALK: f32 = 500.0;
const ENEMY_JUMP: f32 = 500.0;

// boss
const BOSS_WALK: f32 = 250.0;
const BOSS_JUMP: f32 = 500.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        // .add_event::<CollisionEvent>()
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
        //         .with_system(check_for_collision)
        //         .with_system(move_player.before(check_for_collisions))
        //         .with_system(apply_velocity.before(check_for_collisions))
        //         .with_system(play_collision_sound.after(check_for_collisions)),
        // )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(300.0, 300.0)),
            ..default()
        },
        texture: asset_server.load("images/combine-harvester-parted.png"),
        transform: Transform::from_xyz(0.0, 100.0, 1.0),
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        texture: asset_server.load("player/farmera1.png"),
        transform: Transform::from_xyz(200.0, 0.0, 1.0),
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/FreeTileset.png"),
        transform: Transform::from_xyz(0.0, -100.0, 1.0),
        ..default()
    });
}

fn change_clear_color(input: Res<Input<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::Space) {
        clear_color.0 = Color::PURPLE;
    }
}
