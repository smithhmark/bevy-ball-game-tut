# Ball Game Tutorial #

This is a working through of the Ball Game Tutorial by YouTuber [Jaques]( https://www.youtube.com/@jacques-dev). See the play list: [here](https://www.youtube.com/playlist?list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd)

**NOTE:** Jacques' original tutorial is for Bevy v0.10. I am updating the example to v0.12 as I go.


## Running ##

I have been executing: 
`cargo run --features "bevy/dynamic_linking"`
to get very fast development cycles.

## Todo items ##
- Official examples put audio assets in Resources, rather than use the asset_server over and over again
- Episode 8 of the tutorial [here](https://youtu.be/i-Wczghlmxc?si=VngRYpFSBJu9jmWr&t=554) uses explicit `SystemsSet`'s to order player movement and then confining the movement to the playarea. That capability is deprecated as it is described. See the `TODO` in src/enemy/mod.rs

# Differences from the Videos #
- Video 9 uses `State<...>`, rather than `NextState<...>`
- adding systems is different:
  - now: `.add_systems(<Schedule>, <Systems>)`
  - before: `.add_system(<Systems>)` &  `.add_startup_system(<Systems>)` 
