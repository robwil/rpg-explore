#[derive(PartialEq, Copy, Clone, Debug)]
pub enum GameState {
    AwaitingInput,
    PlayerMoving { delta_x: f32, delta_y: f32 },
}
