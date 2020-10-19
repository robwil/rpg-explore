use crate::components::GridPosition;

#[derive(Debug, Clone)]
pub enum Action {
    // Teleports player to a certain location
    Teleport(GridPosition),
    // Prints a message (used for debugging or testing triggers)
    PrintMessage(String),
}
