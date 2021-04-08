// ! Olive core
pub mod bcore;

// ! Banana core
pub mod acore;

// ! libs
#[path = "libs/keyin.rs"]pub mod keyin;
#[path = "libs/cursor.rs"]pub mod cursor;

// ! map
#[path = "map/loader.rs"]pub mod loader;
#[path = "map/sprite.rs"]pub mod sprite;

// ! misc
#[path = "misc/colour.rs"]pub mod colour;
#[path = "misc/check.rs"]pub mod check;
// ! physics
#[path = "physics/physics.rs"]pub mod physics;
#[path = "physics/col.rs"]pub mod col;