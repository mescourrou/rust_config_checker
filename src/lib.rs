pub extern crate colored;

pub trait ConfigCheckable {
    fn check(&self) -> bool;
}

pub fn check_config<T:ConfigCheckable>(item: &T) -> bool {
    item.check()
}