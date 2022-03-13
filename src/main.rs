pub mod menus;
pub mod util;

fn main() {
    util::init_prefs();
    menus::run();
}
