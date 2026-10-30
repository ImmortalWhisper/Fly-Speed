# Fly Speed

A native Horizon mod: `/flyspeed` changes how fast you fly, from chat or from
Options > Mods > Fly Speed > Settings.

## Commands

| Command | What it does |
| --- | --- |
| `/flyspeed` | Says the current multiplier |
| `/flyspeed 2` | Flies twice as fast |
| `/flyspeed 1.5` | Any multiplier from 0.1 to 20 |
| `/flyspeed +1` / `/flyspeed -0.5` | Steps from the current one |
| `/flyspeed reset` | Back to the game's own speed (1x) |

The settings page has one button that steps through 0.5x, 1x, 2x, 4x, 8x and
16x. The choice is kept per mod in
`%APPDATA%\horizon\horizon-mods\.horizon-mod-data\fly_speed\multiplier`.

## How it works

The mod is native, so it is compiled into the game rather than loaded as data.
It uses five stable hook points (version 1):

- `native_tick` sets the local player's `PlayerMotion::flying_speed` to
  `0.05 * multiplier` each tick, skipping spectators.
- `native_command` handles `/flyspeed`.
- `native_mods` adds the row under Options > Mods.
- `native_settings` and `native_settings_click` draw and handle the button.

It only touches the player's own motion, so it is client-side: other players see
whatever the server says your speed is.

## Building

Native mods are source; the launcher compiles them.

1. Put `fly_speed-0.1.0.hnmod` in
   `%APPDATA%\horizon\instances\native-mods\horizon-native-mods\`.
2. In the launcher: Settings > Native mods (experimental) > Build, and confirm
   you trust the mod.

Or from a checkout, with the bundle directory that holds `horizon-native.json`
and `horizon-mixin.toml`:

```
horizon-launcher.exe native-build "<bundle>" --trust-native-code
```

## Tests

```
cargo test
```

Six tests cover the clamp, the multiplier-to-speed conversion, the labels, the
presets, `/flyspeed`'s arguments and the stored setting.

## Files

```
horizon-native.json                  the .hnmod manifest
mixins/fly_speed/horizon-mixin.toml  the module and its five injections
mixins/fly_speed/runtime.rs          the logic and its tests
mixins/fly_speed/{tick,command,list,settings,click}.rs   the hook helpers
compatibility.json                   game 26.3, hooks 1, pinned revisions
```
