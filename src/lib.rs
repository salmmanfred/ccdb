// ! Olive core
pub mod bcore;

// ! Banana core
pub mod acore;

// ! libs
#[path = "libs/keyin.rs"]pub mod keyin;
#[path = "libs/keycode.rs"]pub mod keycode;
#[path = "libs/cursor.rs"]pub mod cursor;
#[path = "libs/terminal.rs"]pub mod terminal;


// ! map
#[path = "map/loader.rs"]pub mod loader;
#[path = "map/sprite.rs"]pub mod sprite;

// ! misc
#[path = "misc/colour.rs"]pub mod colour;
#[path = "misc/check.rs"]pub mod check;
// ! physics
#[path = "physics/physics.rs"]pub mod physics;
#[path = "physics/collision.rs"]pub mod collision;

// ! UI
#[path = "UI/ui.rs"] pub mod ui;