# Terminal3d
Terminal3d (`t3d`) is a tool for viewing 3d `.obj` files, right in your terminal! 🦀

---

![](./media/readme/demo-suzanne.gif)
*Suzanne the monkey - Find this example under [`examples/suzanne.obj`](./examples/suzanne.obj)*

---

## Features
- Reads any renders `.obj` file to the terminal.
- Render with both **braile** (`⡟`) and **block** (`▛`) charecters.
- Choose between wireframe and vertices modes.
- Use mouse controls to view your model, just like any other 3d software.

## Installation
Installing Terminal3d can be done via brew, crates.io, or from source.

### From brew
To install Terminal3d with brew, install from [this tap](https://github.com/liam-ilan/homebrew-terminal3d).
```sh
brew install liam-ilan/terminal3d/terminal3d
``` 

You will be able to invoke the binary as `t3d`. Render a `.obj` file with
```sh
t3d <filepath.obj>
```

### From crates.io
Terminal3d is published as a crate on [crates.io](https://crates.io/crates/terminal3d). If you have Cargo, you can install it with
```sh
cargo install terminal3d
```

You will be able to invoke the binary as `t3d`. Render a `.obj` file with
```sh
t3d <filepath.obj>
```

### From Source
If you don't want to install a Rust crate, but do have Rust installed, you can build and run Terminal3d directly from source.

Clone this repository,
```sh
git clone https://github.com/liam-ilan/terminal3d.git
```

To render a `.obj` file, navigate to the root of the repo directory, and run
```sh
cargo run --release <filepath.obj>
```

## Usage
```
t3d: Visualize .obj files in the terminal!

Usage:
    "t3d <filepath.obj>": Interactively view the provided .obj file.
    "t3d --h", "t3d --help", "t3d -h", "t3d -help", "t3d": Help and info.
    "t3d --v", "t3d --version", "t3d -v", "t3d -version": Get version info.

Controls:
    Scroll down to zoom out, scroll up to zoom in.
    Click and drag the mouse to rotate around the model.
    Click and drag the mouse while holding [shift] to pan.

    Press [b] to toggle block mode. 
    Press [p] to toggle vertices mode. 
```
*Obtained from `t3d -h`*

## Publishing
Notes for the maintainer on publishing Terminal3d can be found in [`PUBLISHING.md`](PUBLISHING.md).

## Author
(c) [Liam Ilan](https://www.liamilan.com/)