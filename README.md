# rpg-explore

This will be my scratch project where I follow along with the Part 1 (Explore) of the [How to make an RPG book](http://howtomakeanrpg.com/a/how-to-make-an-rpg-release.html).
Except I'm doing it in Rust with [macroquad](https://github.com/not-fl3/macroquad) library.

You can follow progress on my blog: https://robwilliams.me/categories/Game-Development/

## WASM notes

```
$ ./scripts/build_wasm.sh
$ basic-http-server .
```

Then navigate to http://127.0.0.1:4000/rpg-explore.html to see the WASM in action.

## Dev notes

Here I am going to write various notes of things that were non-obvious in implementing the game.

## Using Tiled maps

The RPG book provides a lot of Tiled maps for the [Tiled Map Editor](https://www.mapeditor.org/). However, it uses the LUA export and enables compression. The `macroquad-tiled` subcrate
(only available in the `0.3` branch, by the way) seems to only support the uncompressed CSV format of tile layer data. Therefore, for each map I want to load from the RPG book, I am needing
to open it in Tiled Map Editor, change the tile layer format to CSV, and then export as JSON.

That being said, Tiled maps are used in this game for the following things:
- all of the graphics for the maps / levels
- collision details for the map itself
- layers of the map, currently only used for foreground/background rendering but in the future could be used for multi-level maps containing ladders or stairs

## Trigger architecture

A "trigger" in an RPG like this is some action that gets triggered based on the player entering, exiting, or "using" a particular map tile. There were two tricky aspects to handling this in an ECS system.

First of all, how do we keep track of the player entering, exiting, or using particular tiles? The most natural place for this is the Player Movement System, which is where we handle the animation and actual movement on screen of the player going from one tile to the next. On any valid movement, this is where we know a player is exiting or entering a tile. For the "using" case, that can be handled in the Input System by simply checking for the use key. The actual tracking of this can be handled by emitting an Event, which will be stored as a global Resource in the ECS world.

How to actually represent event state within the app? For simplicity, I am just using a global `EventQueue` containing two vecs: current events, and new events. At the end of each frame, the current events are cleared and replaced by the new events. This allows multiple systems to read the current events in a decoupled fashion, while also allowing multiple systems to emit new events. The decoupled nature could have downsides in the future if things get too complex because the relationships between producers and subscribers is not explicit, but it seems like the best way to handle things.

The second issue to consider is how to represent the actual trigger points, e.g. at map position (5, 2) there is a door that should bring the player to the next map. It makes sense to represent these as Entities, using my existing `GridPosition` component. We will have some triggers that take place on map elements that come from the Tiled map, and therefore have no `SpriteDrawable` component. Other triggers might be drawn separately from the map, e.g. appearing after another trigger happened. That's fine, and is exactly what ECS empowers us to do. The actual trigger dimension can be captured with components like `OnEnterTrigger { action: ActionEnum }` or `OnUseTrigger { actions: ActionEnum }`. The systems responsible for such events would be 1) iterating through all the events from the appropriate event queue, 2) joining GridPosition with the appropriate trigger component, and iterating all those components, 3) if any of the incoming events' positions match the positions of the triggers, we execute the action. For now, I am doing all of that in a single system `ActionSystem`, but in the future this could be split out to many systems based on which events and actions they deal with. We could even completely decouple the event handling from the action execution if desired, by creating an `ActionQueue`.

One important note is that both `Event`s and `Action`s are modeled as enums. I didn't want `Action`s to have arbitrary code/lambdas attached to them because that would break the ECS paradigm. By keeping Actions as strictly data, we keep the logic in the Systems.