pub extern crate colored;
pub extern crate config_checker_macros as macros;

pub trait ConfigCheckable {
    fn check(&self) -> bool;
    fn __check(&self, depth: usize) -> bool;
}

pub fn __check_config<T: ConfigCheckable>(item: &T, depth: usize) -> bool {
    item.__check(depth)
}

impl<T: ConfigCheckable> ConfigCheckable for Box<T>  {
    fn check(&self) -> bool {
        self.as_ref().check()
    }
    fn __check(&self, depth: usize) -> bool {
        self.as_ref().__check(depth)
    }
}

impl<T: ConfigCheckable> ConfigCheckable for Vec<T>  {
    fn check(&self) -> bool {
        let mut ret = true;
        for item in self{
            ret &= item.check();
        }
        ret
    }
    fn __check(&self, depth: usize) -> bool {
        let mut ret = true;
        let depth_space = String::from_utf8(vec![b' '; depth*2]).unwrap();
        let mut i = 0;
        for item in self {
            if !item.__check(depth) {
                ret = false;
                println!("{} {depth_space}  {} From item number `{i}`", ::colored::Colorize::blue("NOTE: "), "\u{21b3}");
            }
            i+= 1;
        }
        ret
    }
}
