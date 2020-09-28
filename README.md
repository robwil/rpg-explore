# rpg-explore

This will be my scratch project where I follow along with the Part 1 (Explore) of the [How to make an RPG book](http://howtomakeanrpg.com/a/how-to-make-an-rpg-release.html).
Except I'm doing it in Rust with [macroquad](https://github.com/not-fl3/macroquad) library.

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