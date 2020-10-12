use crate::GridPosition;

#[derive(Debug, Copy, Clone)]
pub enum Event {
    // Fired when the player tries to move
    PlayerTriesMove { delta_x: f32, delta_y: f32 },

    // Fired when player tries to use a particular tile
    PlayerTriesUse(GridPosition),

    // Fired when player successfully leaves a tile position
    PlayerExit(GridPosition),

    // Fired when player successfully enters a tile position
    PlayerEntered(GridPosition),
}

// global event queue
// RW: in future, it might make sense to separate this out into separate queues based on different event types?
#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>, // possibly read many times by different systems
    pub new_events: Vec<Event>, // possibly written many times by different systems
                            // at end of each frame, `events` is cleared and `new_events` replaces it
}
