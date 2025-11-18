pub extern crate colored;
pub extern crate config_checker_macros as macros;

pub trait ConfigCheckable {
    fn check(&self) -> Result<(), String>;
    fn __check(&self, depth: usize) -> Result<(), String>;
}

pub fn __check_config<T: ConfigCheckable>(item: &T, depth: usize) -> Result<(), String> {
    item.__check(depth)
}

impl<T: ConfigCheckable> ConfigCheckable for Box<T>  {
    fn check(&self) -> Result<(), String> {
        self.as_ref().__check(0)
    }
    fn __check(&self, depth: usize) -> Result<(), String> {
        self.as_ref().__check(depth)
    }
}

impl<T: ConfigCheckable> ConfigCheckable for Vec<T>  {
    fn check(&self) -> Result<(), String> {
        self.__check(0)
    }
    fn __check(&self, depth: usize) -> Result<(), String> {
        let mut ret = Ok(());
        let depth_space = String::from_utf8(vec![b' '; depth*2]).unwrap();
        let mut i = 0;
        for item in self {
            if let Err(e) = item.__check(depth) {
                if ret.is_ok() {
                    ret = Err(String::new());
                }
                ret = Err(ret.err().unwrap() + format!("{e}{} {depth_space}  {} From item number `{i}`\n", ::colored::Colorize::blue("NOTE: "), "\u{21b3}").as_str())
                ;
            }
            i+= 1;
        }
        ret
    }
}

impl<T: ConfigCheckable> ConfigCheckable for Option<T>  {
    fn check(&self) -> Result<(), String> {
        self.__check(0)
    }
    fn __check(&self, depth: usize) -> Result<(), String> {
        let mut ret = Ok(());
        let depth_space = String::from_utf8(vec![b' '; depth*2]).unwrap();
        if let Some(t) = self {
            if let Err(e) = t.__check(depth) {
                if ret.is_ok() {
                    ret = Err(String::new());
                }
                ret = Err(ret.err().unwrap() + format!("{e}{} {depth_space}  {} From option\n", ::colored::Colorize::blue("NOTE: "), "\u{21b3}").as_str());
            }
        }
        ret
    }
}