# Terminal3d
Terminal3d (`t3d`) is a tool for viewing 3d `.obj` files, right in your terminal! 🦀
![](./media/readme/demo-suzanne.gif)
*Suzanne the monkey - Find this example under [`examples/suzanne.obj`](./examples/suzanne.obj)*

## Features
- Reads any renders `.obj` file to the terminal.
- Render with both **braile** (`⡟`) and **block** (`▛`) charecters.
- Choose between wireframe and vertices modes.
- Use mouse controls to view your model, just like any other 3d software.


## Usage
```
t3d: Visualize .obj files in the terminal!

Usage:
    "t3d <filepath.obj>": Interactively view the provided .obj file.
    "t3d --h", "t3d --help", "t3d -h", "t3d -help", "t3d": Help and info.

Controls:
    Scroll down to zoom out, scroll up to zoom in.
    Click and drag the mouse to rotate around the model.
    Click and drag the mouse while holding [shift] to pan.

    Press [b] to toggle block mode. 
    Press [p] to toggle vertices mode. 
```
*Obtained from `t3d -h`*

## Author
(c) [Liam Ilan](https://www.liamilan.com/)