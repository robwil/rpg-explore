#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum GameState {
    AwaitingInput {
        player_facing: Direction,
    },
    PlayerMoving {
        delta_x: f32,
        delta_y: f32,
        direction: Direction,
    },
}
