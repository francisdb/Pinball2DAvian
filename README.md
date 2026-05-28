# Pinball2DAvian
A port of https://github.com/gunstein/Pinball2D to the Avian physics engine.

Using the [Bevy](https://bevy.org/) game engine and [Avian](https://github.com/Jondolf/avian) physics engine for a simple 2D pinball game.

## Running the game with Rust
[Cargo](https://github.com/rust-lang/cargo) is a prerequisite.
```Bash
git clone https://github.com/francisdb/Pinball2DAvian.git
cargo run --release
```

## Controls

| Key | Action |
| --- | --- |
| Left Shift / Left Arrow | Left flipper |
| Right Shift / Right Arrow | Right flipper |
| Enter | Hold to pull the plunger back, release to launch |
| Z | Nudge left |
| / | Nudge right |
| Space | Nudge (center) |
| Escape | Quit |

Nudge keys mirror Visual Pinball's defaults.

<img src="/Screenshot_pinball2d.png?raw=true" width="200">

https://user-images.githubusercontent.com/5881978/135753714-fb8e7d7a-0752-43d7-84ed-c8a90e5d85c7.mov
