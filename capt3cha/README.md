# CAPT3CHA

The Mindless Cyberspace Hyvemind is proud to present CAPT3CHA!

While IA are exceptionaly nimble at navigating their native environnement, the Cyberspace, they struggle when this comes to navigating the human domain: 3D Space.

CAPT3CHA is a brand new captcha system that takes advantage of the fact by taking captcha to the next dimension ! The third dimension !

## Controls

After executing the binary, you should be presented with a window. Inside this window you should see 6 letters and digits, this is the captcha.
You can rotate around the captcha by pressing the left button on your mouse and moving it.

To type the captcha, simply type the letter or digit with you keyboard. 
Due to a technical limitation, there is no case handling, and the captcha is case insensitive, so do not press shift!

If the captcha is valid, a green screen should appear immediately and the window should close after 5 seconds.

## Compilation

Should you want to compile CAPT3CHA yourself, you will need:
- Cmake and a C compiler (for SDL), on Windows, prefer using MSVC from a recent Visual Studio.
- Blender to generate the 3D Assets.
- A recent Stable Rust Toolchain, use rustup to get one if you don't have one.
- Python >3.6

## 1. Generating the 3D Assets

In the `obj` folder, you should find a `glyph.blend`. Open it with Blender.
You should be in Scripting mode and have a script open. Run it by clicking the run arrow.
You should now see an `.obj` file for every glyph between a-z and 0-9 in the same folder.

You know need to turn them to a single `glyph.rs` file.
To do this, you need the execute the Python script `convert.py` on each file.
The simplest way to do this is to use `fd-find/fd` with the following argument: `fd -e obj -j 1 -x python convert.py {} glyph.rs`
The move/copy the glyph.rs to `../peglrs/src/mesh/glyph.rs`

You are done!

## 2. Compile it !

Just run `cargo build` at the root of this project. It should build both `peglrs` (the 3d engine) and `sdl_backend` (the SDL backend).

The first build might be long due to:
- Automatically compiling and linking SDL
- The huge `glyph.rs` file we generated

## 3. Run it !

`cargo run` and see!


Your dear friends at the Mindless Cyberspace,
Maël Naccache Tüfekçi
