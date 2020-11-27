use crate::components::GridPosition;
use crate::ui::DialogBoxConf;

#[derive(Debug, Clone)]
pub enum Action {
    // Teleports player to a certain location
    Teleport(GridPosition),
    // shows a message with a basic dialog box
    ShowSimpleDialog(String),
    // shows a full dialog box, including optional character name and portrait
    ShowDialog(DialogBoxConf),
    // Prints a message (used for debugging or testing triggers)
    PrintMessage(String),
}
