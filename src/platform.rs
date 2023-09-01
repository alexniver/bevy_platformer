use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const COLOR_PLATFORM: Color = Color::rgb(0.1, 0.3, 0.9);

#[derive(Bundle)]
pub struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
    collider: Collider,
}

impl PlatformBundle {
    pub fn new(translation: Vec3, scale: Vec3) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: COLOR_PLATFORM,
                    ..default()
                },
                transform: Transform {
                    translation,
                    scale,
                    ..default()
                },
                ..default()
            },
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}
