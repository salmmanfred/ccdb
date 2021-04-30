// ! Olive core
pub mod bcore;

// ! Banana core
pub mod acore;

// ! libs
#[path = "libs/cursor.rs"]
pub mod cursor;
#[path = "libs/escape/escape.rs"]
pub mod escape;
#[path = "libs/keycode.rs"]
pub mod keycode;
#[path = "libs/keyin.rs"]
pub mod keyin;
#[path = "libs/sys/sys.rs"]
mod sys;
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
#[path = "physics/particle.rs"]
pub mod particle;
#[path = "physics/physics.rs"]
pub mod physics;

// ! UI
#[path = "UI/ui.rs"]
pub mod ui;

#[path = "test/test.rs"]
mod test;
#[cfg(test)]
mod tests {
    use crate::test;
    #[test]
    fn test() {
        test::t();
    }
    #[test]
    fn test2() {
        test::check_const();
    }
}
