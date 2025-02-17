pub extern crate colored;
pub extern crate config_checker_macros;
pub trait ConfigCheckable {
    fn check(&self) -> bool;
}

pub fn check_config<T: ConfigCheckable>(item: &T) -> bool {
    item.check()
}
