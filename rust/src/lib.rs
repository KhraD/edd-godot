use godot::prelude::*;

mod my_character_body_2d;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}