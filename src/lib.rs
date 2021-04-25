// ! Olive core
pub mod bcore;

// ! Banana core
pub mod acore;

// ! libs
#[path = "libs/cursor.rs"]
pub mod cursor;
#[path = "libs/keycode.rs"]
pub mod keycode;
#[path = "libs/keyin.rs"]
pub mod keyin;
#[path = "libs/terminal.rs"]
pub mod terminal;

// ! map
#[path = "map/loader.rs"]
pub mod loader;
#[path = "map/sprite.rs"]
pub mod sprite;

// ! misc
#[path = "misc/check.rs"]
pub mod check;
#[path = "misc/colour.rs"]
pub mod colour;
// ! physics
#[path = "physics/collision.rs"]
pub mod collision;
#[path = "physics/physics.rs"]
pub mod physics;

// ! UI
#[path = "UI/ui.rs"]
pub mod ui;

pub enum orders {
    rend,
    ui,
    keyboard,
    misc,
}

pub struct order {
    ex: i64,
}
impl order {
    pub fn new() -> order {
        order { ex: 0 }
    }
    pub fn next(&mut self) -> orders {
        if self.ex == 0 {
            return orders::rend;
        }
        if self.ex == 1 {
            return orders::ui;
        }
        if self.ex == 2 {
            return orders::keyboard;
        }
        if self.ex == 4 {
            self.ex = 0;
        }
        self.ex += 1;

        return orders::misc;
    }
}
