pub mod components;
pub mod iced;
use components::Library;

pub fn get_hibiki_dirs() -> Vec<String> {
    use std::env;
    for (key, value) in env::vars() {
        if key == "HIBIKI_DIRS" {
            return value.split(':').map(String::from).collect::<Vec<_>>();
        }
    }
    panic!("You should set an environmental variable 'HIBIKI_DIRS'")
}
