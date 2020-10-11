use crate::components::GridPosition;
use crate::components::Player;
use crate::constants::PLAYER_MOVEMENT_DURATION;
use crate::game_states::GameState;
use crate::SpriteDrawable;
use macroquad::get_frame_time;
use specs::Join;
use specs::ReadStorage;
use specs::System;
use specs::WriteExpect;
use specs::WriteStorage;

// This system is responsible for handling the animation that occurs when the game enters GameState::PlayerMoving.
pub struct PlayerMovingSystem;

impl<'a> System<'a> for PlayerMovingSystem {
    type SystemData = (
        WriteExpect<'a, GameState>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, GridPosition>,
        WriteStorage<'a, SpriteDrawable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let delta_time = get_frame_time();

        let (mut game_state, players, mut positions, mut drawables) = data;

        for (_player, position, drawable) in (&players, &mut positions, &mut drawables).join() {
            // TODO: The below code is pretty complicated. It'd be a lot clearer what was going on if we had a Tween class.
            // That might also allow removing the duplication because the distinct part of each of the below is mostly the directionality differences.
            if let GameState::PlayerMoving { delta_x, delta_y } = *game_state {
                if delta_y > 0. {
                    // down
                    // the difference between 1 and current delta_y is how much we've moved so far.
                    // we use that to determine the current animation frame and to tween the actual movement of position
                    let elapsed_duration = (1. - delta_y) * PLAYER_MOVEMENT_DURATION + delta_time;
                    let movement_this_frame = delta_time / PLAYER_MOVEMENT_DURATION;
                    drawable.current_frame =
                        8. + (elapsed_duration / PLAYER_MOVEMENT_DURATION * 4.).floor();
                    position.y += movement_this_frame;
                    let mut new_delta_y = delta_y - movement_this_frame;
                    if new_delta_y < 0. {
                        // finished moving
                        new_delta_y = 0.;
                        drawable.current_frame = 8.;
                        position.y = position.y.floor();
                    }
                    *game_state = GameState::PlayerMoving {
                        delta_x: 0.,
                        delta_y: new_delta_y,
                    };
                } else if delta_y < 0. {
                    // up
                    let elapsed_duration = (1. - -delta_y) * PLAYER_MOVEMENT_DURATION + delta_time;
                    let movement_this_frame = delta_time / PLAYER_MOVEMENT_DURATION;
                    drawable.current_frame =
                        0. + (elapsed_duration / PLAYER_MOVEMENT_DURATION * 4.).floor();
                    position.y -= movement_this_frame;
                    let mut new_delta_y = delta_y + movement_this_frame;
                    if new_delta_y > 0. {
                        // finished moving
                        new_delta_y = 0.;
                        drawable.current_frame = 0.;
                        position.y = position.y.ceil();
                    }
                    *game_state = GameState::PlayerMoving {
                        delta_x: 0.,
                        delta_y: new_delta_y,
                    };
                } else if delta_x > 0. {
                    // right
                    let elapsed_duration = (1. - delta_x) * PLAYER_MOVEMENT_DURATION + delta_time;
                    let movement_this_frame = delta_time / PLAYER_MOVEMENT_DURATION;
                    drawable.current_frame =
                        4. + (elapsed_duration / PLAYER_MOVEMENT_DURATION * 4.).floor();
                    position.x += movement_this_frame;
                    let mut new_delta_x = delta_x - movement_this_frame;
                    if new_delta_x < 0. {
                        // finished moving
                        new_delta_x = 0.;
                        drawable.current_frame = 4.;
                        position.x = position.x.floor();
                    }
                    *game_state = GameState::PlayerMoving {
                        delta_x: new_delta_x,
                        delta_y: 0.,
                    };
                } else if delta_x < 0. {
                    // left
                    let elapsed_duration = (1. - -delta_x) * PLAYER_MOVEMENT_DURATION + delta_time;
                    let movement_this_frame = delta_time / PLAYER_MOVEMENT_DURATION;
                    drawable.current_frame =
                        12. + (elapsed_duration / PLAYER_MOVEMENT_DURATION * 4.).floor();
                    position.x -= movement_this_frame;
                    let mut new_delta_x = delta_x + movement_this_frame;
                    if new_delta_x > 0. {
                        // finished moving
                        new_delta_x = 0.;
                        drawable.current_frame = 12.;
                        position.x = position.x.ceil();
                    }
                    *game_state = GameState::PlayerMoving {
                        delta_x: new_delta_x,
                        delta_y: 0.,
                    };
                } else {
                    // once delta_x and delta_y are 0, the movement is over
                    *game_state = GameState::AwaitingInput;
                }
            }
        }
    }
}
