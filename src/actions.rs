use crate::components::GridPosition;

#[derive(Debug, Clone)]
pub enum Action {
    // Teleports player to a certain location
    Teleport(GridPosition),
    // shows a message with a basic dialog box
    ShowDialog(String),
    // Prints a message (used for debugging or testing triggers)
    PrintMessage(String),
}
