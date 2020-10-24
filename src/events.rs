use specs::Entity;
use crate::Direction;
use crate::GridPosition;

#[derive(Debug, Copy, Clone)]
pub enum Event {
    // Fired when an entity tries to move in a particular direction
    // EntityTriesMove(Entity, Direction),
    EntityTriesMove(Direction),

    // Fired when player presses Use button while facing a particular direction
    PlayerTriesUse(Direction),

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
