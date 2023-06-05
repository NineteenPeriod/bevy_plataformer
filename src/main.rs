use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResolution;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::{Collider, RigidBody};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use bevy_platformer::models::platform_bundle::PlatformBundle;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

const BACKGROUD_COLOR: Color = Color::rgb(0.03, 0.18, 0.28);
const PLATFORM_COLOR: Color = Color::rgb(0.3, 0.56, 0.69);
const PLAYER_COLOR: Color = Color::rgb(0.65, 0.54, 0.98);

const FLOOR_THICKNESS: f32 = 10.0;
const FLOOR_COLOR: Color = Color::rgb(0.3, 0.56, 0.69);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUD_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup).run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Hello, World!");

    commands.spawn(PlatformBundle::new(-100.0, Vec3::new(75.0, 100.0, 1.0), PLATFORM_COLOR));
    commands.spawn(PlatformBundle::new(100.0, Vec3::new(100.0, 200.0, 1.0), PLATFORM_COLOR));
    commands.spawn(PlatformBundle::new(300.0, Vec3::new(50.0, 150.0, 1.0), PLATFORM_COLOR));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: FLOOR_COLOR,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, WINDOW_BOTTOM_Y + (FLOOR_THICKNESS / 2.0), 0.0),
            scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
            ..Default::default()
        },
        ..Default::default()
    }).insert(RigidBody::Fixed).insert(Collider::cuboid(0.5, 0.5));

    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::default().into()).into(),
        material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
        transform: Transform {
            translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
            scale: Vec3::new(30.0, 30.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}