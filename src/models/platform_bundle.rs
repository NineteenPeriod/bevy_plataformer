use bevy::prelude::{Bundle, Color, Sprite, SpriteBundle, Transform, Vec3};
use bevy_rapier2d::prelude::{Collider, RigidBody};

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const PLATFORM_COLOR: Color = Color::rgb(0.3, 0.56, 0.69);

const FLOOR_THICKNESS: f32 = 10.0;


#[derive(Bundle)]
pub struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl PlatformBundle {
    pub fn new(x: f32, scale: Vec3, color: Color) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, WINDOW_BOTTOM_Y + (scale.y / 2.0) + FLOOR_THICKNESS, 0.0),
                    scale,
                    ..Default::default()
                },
                ..Default::default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}