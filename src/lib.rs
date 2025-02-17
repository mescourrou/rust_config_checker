pub extern crate colored;
pub extern crate config_checker_macros as macros;
pub trait ConfigCheckable {
    fn check(&self) -> bool;
    fn __check(&self, depth: usize) -> bool;
}

pub fn __check_config<T: ConfigCheckable>(item: &T, depth: usize) -> bool {
    item.__check(depth)
}
