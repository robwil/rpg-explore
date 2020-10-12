use crate::components::GridPosition;

#[derive(Debug, Copy, Clone)]
pub enum Action {
    // Teleports player to a certain location
    Teleport(GridPosition),
}