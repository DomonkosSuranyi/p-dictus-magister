use amethyst::input::{InputHandler, StringBindings};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, WriteStorage, prelude::Join};
use amethyst::core::Transform;
use amethyst::core::math::{Vector2, Rotation2};

use crate::components::{Player, Velocity};

pub(crate) const ACTION_FORWARD: &str = "forward";
pub(crate) const ACTION_BACKWARD: &str = "backward";
pub(crate) const ACTION_STRAFE_LEFT: &str = "strafe_left";
pub(crate) const ACTION_STRAFE_RIGHT: &str = "strafe_right";

const MOVE_ACTIONS: &'static [&'static str] = &[
    ACTION_FORWARD,
    ACTION_BACKWARD,
    ACTION_STRAFE_LEFT,
    ACTION_STRAFE_RIGHT,
];

#[derive(SystemDesc)]
pub struct PlayerMovementSystem;

/// The move direction relative to facing
#[derive(Copy, Clone)]
pub enum MoveDirection {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
}

impl MoveDirection {
    pub fn from_action(key: &str) -> Option<MoveDirection> {
        match key {
            ACTION_FORWARD => Some(MoveDirection::Forward),
            ACTION_BACKWARD => Some(MoveDirection::Backward),
            ACTION_STRAFE_LEFT => Some(MoveDirection::StrafeLeft),
            ACTION_STRAFE_RIGHT => Some(MoveDirection::StrafeRight),
            _ => None,
        }
    }
}

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, mut velocities, players, input): Self::SystemData) {
        for (_player, mut velocity, mut transform) in (&players, &mut velocities, &mut transforms).join() {
            if let Some(mouse_pos) = input.mouse_position() {
                rotate_toward_mouse(&mut transform, mouse_pos);
            }
            update_velocity(&transform, &input, &mut velocity);
        }
    }

}

fn rotate_toward_mouse(
    transform: &mut Transform,
    (mouse_x, mouse_y): (f32, f32),
) {
    // Calculate the vector from player position to mouse cursor
    let mouse_direction = Vector2::new(
        mouse_x - transform.translation().x,
        transform.translation().y - mouse_y
    );

    let base_vector = Vector2::new(0.0, -1.0);
    let mut angle = base_vector.angle(&mouse_direction);

    if mouse_direction.x < 0.0 {
        angle = 2.0 * std::f32::consts::PI - angle;
    }
    transform.set_rotation_2d(angle);
}

const PLAYER_MAX_WALK_SPEED: f32 = 64.0;

fn update_velocity(
    transform: &Transform,
    input: &InputHandler<StringBindings>,
    velocity: &mut Velocity
) {
    let velocities: Vec<Vector2<f32>> = MOVE_ACTIONS.iter()
        .filter(|s| input.action_is_down(&s.to_string()).unwrap_or(false))
        .map(|&s| MoveDirection::from_action(s))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .map(as_vector2)
        .collect();

    if velocities.is_empty() {
        *velocity = Velocity::default();
    } else {
        let rot = Rotation2::new(transform.rotation().angle());

        *velocity = Velocity(rot * vector_avg(&velocities));
    }

}

fn vector_avg<'a, I>(velocities: I) -> Vector2<f32>
    where I: IntoIterator<Item=&'a Vector2<f32>> {
    let mut x = 0_f32;
    let mut y = 0_f32;
    let mut len = 0;

    for &vel in velocities {
        x += vel.x;
        y += vel.y;
        len += 1;
    }

    Vector2::new(x/len as f32, y/len as f32)

}

// TODO I couldn't manage to create valid rustdoc links :(
/// Gives the corresponding `Vector2` to the given `MoveDirection` element.
/// In te case of `Forward` the length of the returned vector will be the max walk speed
/// and the halt of that in any other cases
fn as_vector2(move_dir: MoveDirection) -> Vector2<f32> {
    match move_dir {
        MoveDirection::Forward => Vector2::new(0.0, -PLAYER_MAX_WALK_SPEED),
        MoveDirection::Backward => Vector2::new(0.0, PLAYER_MAX_WALK_SPEED / 2.0),
        MoveDirection::StrafeLeft => Vector2::new(PLAYER_MAX_WALK_SPEED / 2.0, 0.0),
        MoveDirection::StrafeRight => Vector2::new(-PLAYER_MAX_WALK_SPEED / 2.0, 0.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f32::consts::PI;
    use amethyst::core::Transform;
    use amethyst::core::math::Vector3;

    fn assert_f32_equals(exp: f32, act:f32, threshold: f32) {
        let diff = (exp - act).abs();


        assert!(diff < threshold, "Expected: {}, Actual: {}", exp, act);
    }

    mod test_rotate_toward_mouse {
        use super::*;

        macro_rules! test_rotate_toward_mouse {
            ($($name:ident: $player_coord:expr, $cursor_coord:expr, $expected:expr,)*) => {
                $(
                    #[test]
                    fn $name() {
                        let player = $player_coord;
                        let transform = &mut Transform::default();
                        transform.set_translation_x(player.0);
                        transform.set_translation_y(player.1);

                        rotate_toward_mouse(transform, $cursor_coord);

                        let angle = transform.rotation().axis().map(|vec| vec.z).unwrap_or(1.0) * transform.rotation().angle();

                        // sin is being called to normalize the angles (e.g. -PI = PI)
                        assert_f32_equals(f32::sin($expected), angle.sin(), 0.000001);
                    }
                )*
            }
        }

        test_rotate_toward_mouse! {
            cursor_up: (3.0, 3.0), (3.0, 2.0), PI,
            cursor_left: (3.0, 3.0), (2.0, 3.0), -PI/2.0,
            cursor_down: (3.0, 3.0), (3.0, 4.0), 0.0,
            cursor_right: (3.0, 3.0), (4.0, 3.0), PI/2.0,
            cursor_upright_45deg: (3.0, 3.0), (4.0, 2.0), 3.0*PI/4.0,
        }
    }
}
