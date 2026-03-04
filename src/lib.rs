pub extern crate colored;
pub extern crate config_checker_macros as macros;
pub use macros::Check;

pub trait Check {
    fn do_check(&self) -> Result<(), Vec<String>>;
}

pub trait __MarkerCheck<T> {
    fn call_do_check(_: &T) -> Result<(), Vec<String>> {
        Ok(())
    }
}
pub struct __CheckBranching<T>(T);

impl<T: Check> __CheckBranching<T> {
    pub fn call_do_check(t: &T) -> Result<(), Vec<String>> {
        t.do_check()
    }
}
impl<T> __MarkerCheck<T> for __CheckBranching<T>{}


pub trait ConfigCheckable {
    fn check(&self) -> Result<(), String>;
    fn __tree_check(&self, depth: usize) -> Result<(), String>;
}

pub fn __check_config<T: ConfigCheckable>(item: &T, depth: usize) -> Result<(), String> {
    item.__tree_check(depth)
}

impl<T: ConfigCheckable> ConfigCheckable for Box<T>  {
    fn check(&self) -> Result<(), String> {
        self.as_ref().__tree_check(0)
    }
    fn __tree_check(&self, depth: usize) -> Result<(), String> {
        self.as_ref().__tree_check(depth)
    }
}

impl<T: ConfigCheckable> ConfigCheckable for Vec<T>  {
    fn check(&self) -> Result<(), String> {
        self.__tree_check(0)
    }
    fn __tree_check(&self, depth: usize) -> Result<(), String> {
        let mut ret = Ok(());
        let depth_space = vec!["| "; depth].join("");
        let mut i = 0;
        for item in self {
            if let Err(e) = item.__tree_check(depth+1) {
                if ret.is_ok() {
                    ret = Err(String::new());
                }
                ret = Err(ret.err().unwrap() + format!("{e}{} {depth_space}From item number `{i}`:\n{e}", ::colored::Colorize::blue("NOTE :")).as_str());
            }
            i+= 1;
        }
        ret
    }
}

impl<T: ConfigCheckable> ConfigCheckable for Option<T>  {
    fn check(&self) -> Result<(), String> {
        self.__tree_check(0)
    }
    fn __tree_check(&self, depth: usize) -> Result<(), String> {
        if let Some(t) = self {
            t.__tree_check(depth)
        } else {
            Ok(())
        }
    }
}