# gameboy_display

A desktop frontend for `gameboy_core` built using Rust and SDL2. It handles video output, keyboard input polling, battery save file management, and includes a built-in command-line debugger for ROM development and testing.

## Dependencies

You need the SDL2 development libraries installed on your system to build this project. 

* **Debian/Ubuntu:** `sudo apt-get install libsdl2-dev`
* **Arch Linux:** `sudo pacman -S sdl2`
* **macOS:** `sudo pacman -S sdl2` (or via brew)

And the gameboy core as well.
```bash
git clone https://github.com/riteshco/gameboy_core.git
```

## Building and Running

Compile and run the emulator by passing the path to a Game Boy ROM:

```bash
cargo run --release -- path/to/game.gb
