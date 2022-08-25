#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use bevy::{
    prelude::*,
    render::texture::ImageSettings,
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

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

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
        .add_system(animate_sprite)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player/tux_big.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(65.0, 96.0), 8, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(-530., -300., 10.).with_scale(Vec3::splat(1.)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.10, true)));

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        texture: asset_server.load("hill/hill1.png"),
        // transform: Transform::from_xyz(0.0, 100.0, 1.0),
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        texture: asset_server.load("hill/hill2.png"),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        texture: asset_server.load("hill/hill3.png"),
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        texture: asset_server.load("hill/hill4.png"),
        transform: Transform::from_xyz(0.0, 0.0, 3.0),
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        texture: asset_server.load("hill/hill5.png"),
        transform: Transform::from_xyz(0.0, 0.0, 4.0),
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        texture: asset_server.load("hill/hill6.png"),
        transform: Transform::from_xyz(0.0, 0.0, 20.0),
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(300.0, 300.0)),
            ..default()
        },
        texture: asset_server.load("images/combine-harvester-parted.png"),
        transform: Transform::from_xyz(-490.0, 200.0, 1.0),
        ..default()
    });

    // commands.spawn_bundle(SpriteBundle {
    //     sprite: Sprite {
    //         custom_size: Some(Vec2::new(96.0, 96.0)),
    //         ..default()
    //     },
    //     texture: asset_server.load("player/farmera1.png"),
    //     transform: Transform::from_xyz(-530.0, -300.0, 10.0),
    //     ..default()
    // });

    /*
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(512.0, 384.0)),
            ..default()
        },
        texture: asset_server.load("images/FreeTileset.png"),
        transform: Transform::from_xyz(0.0, -230.0, 10.0),
        ..default()
    });
    */
}

fn change_clear_color(input: Res<Input<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::Space) {
        clear_color.0 = Color::PURPLE;
    }
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = 64 + (sprite.index % 7); // texture_atlas.textures.len());
        }
    }
}
