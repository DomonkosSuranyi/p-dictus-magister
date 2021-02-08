use amethyst::core::math::Point2;
use amethyst::core::Transform;
use amethyst::prelude::*;
use log::info;


use westiny_common::components::{BoundingCircle, Input, Player, Velocity, weapon::*};
use crate::resources::{SpriteId, SpriteResource};

pub fn initialize_player(world: &mut World,
                         sprite_resource: &SpriteResource,
                         start_pos: Point2<f32>
                         ) {

    let mut transform = Transform::default();
    transform.set_translation_xyz(start_pos.x, start_pos.y, 0.0);

    // TODO define these values in RON resource files.
    let revolver = WeaponDetails {
        damage: 5.0,
        distance: 120.0,
        fire_rate: 7.2,
        magazine_size: 6,
        reload_time: 1.0,
        spread: 2.0,
        shot: Shot::Single,
        bullet_speed: 200.0
    };

    world.register::<Input>();

    world
        .create_entity()
        .with(sprite_resource.sprite_render_for(SpriteId::Player))
        .with(transform)
        .with(Player)
        .with(Input::default())
        .with(Velocity::default())
        .with(Weapon::new(revolver))
        .with(BoundingCircle{radius: 8.0})
        .build();

    info!("Player created.");
}
