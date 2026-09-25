pub extern crate colored;
pub extern crate config_checker_macros as macros;

use colored::Colorize;
pub use macros::Check;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckMessageType {
    Error,
    Warning,
    Info,
}

pub struct CheckMessage {
    message: String,
    stack: Vec<String>,
    message_type: CheckMessageType,
}

impl CheckMessage {
    pub fn new(message: String, message_type: CheckMessageType) -> Self {
        Self {
            message,
            stack: Vec::new(),
            message_type,
        }
    }

    pub fn new_error(message: String) -> Self {
        Self::new(message, CheckMessageType::Error)
    }

    pub fn new_warning(message: String) -> Self {
        Self::new(message, CheckMessageType::Warning)
    }

    pub fn new_info(message: String) -> Self {
        Self::new(message, CheckMessageType::Info)
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    fn push_stack(&mut self, stack: String) {
        self.stack.push(stack);
    }
}

impl std::fmt::Display for CheckMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", match self.message_type {
            CheckMessageType::Error => "ERROR".red(),
            CheckMessageType::Warning => "WARNING".yellow(),
            CheckMessageType::Info => "INFO".blue(),
        }, self.message)?;
        for stack in &self.stack {
            write!(f, "\n\tFrom: {}", stack)?;
        }
        Ok(())
    }
}

pub struct CheckResult {
    messages: Vec<CheckMessage>,
}

impl CheckResult {
    pub fn new() -> Self {
        Self {
            messages: Vec::new()
        }
    }

    pub fn messages(&self) -> &[CheckMessage] {
        &self.messages
    }

    pub fn add_message(&mut self, message: CheckMessage) {
        self.messages.push(message);
    }

    pub fn add_error(&mut self, message: String) {
        self.add_message(CheckMessage::new_error(message));
    }

    pub fn add_warning(&mut self, message: String) {
        self.add_message(CheckMessage::new_warning(message));
    }

    pub fn add_info(&mut self, message: String) {
        self.add_message(CheckMessage::new_info(message));
    }

    pub fn __push_stack(&mut self, stack: String) {
        for message in &mut self.messages {
            message.push_stack(stack.clone());
        }
    }

    pub fn __concatenate(&mut self, other: CheckResult) {
        for message in other.messages {
            self.add_message(message);
        }
    }

    pub fn is_ok(&self) -> bool {
        !self.messages.iter().any(|m| m.message_type == CheckMessageType::Error)
    }
}

impl std::fmt::Display for CheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for message in &self.messages {
            writeln!(f, "{}\n", message)?;
        }
        Ok(())
    }
}

pub trait Check {
    fn do_check(&self) -> CheckResult;
}

pub trait __MarkerCheck<T> {
    fn call_do_check(_: &T) -> CheckResult {
        CheckResult::new()
    }
}
pub struct __CheckBranching<T>(T);

impl<T: Check> __CheckBranching<T> {
    pub fn call_do_check(t: &T) -> CheckResult {
        t.do_check()
    }
}
impl<T> __MarkerCheck<T> for __CheckBranching<T>{}


pub trait ConfigCheckable {
    fn check(&self) -> CheckResult;
    fn __tree_check(&self, depth: usize) -> CheckResult;
}

pub fn __check_config<T: ConfigCheckable>(item: &T, depth: usize) -> CheckResult {
    item.__tree_check(depth)
}

impl<T: ConfigCheckable> ConfigCheckable for Box<T>  {
    fn check(&self) -> CheckResult {
        self.as_ref().__tree_check(0)
    }
    fn __tree_check(&self, depth: usize) -> CheckResult {
        self.as_ref().__tree_check(depth)
    }
}

impl<T: ConfigCheckable> ConfigCheckable for Vec<T>  {
    fn check(&self) -> CheckResult {
        self.__tree_check(0)
    }
    fn __tree_check(&self, depth: usize) -> CheckResult {
        let mut ret = CheckResult::new();
        let mut i = 0;
        for item in self {
            let mut result = item.__tree_check(depth+1);
            result.__push_stack(format!("Item number `{i}`"));
            i+= 1;
            ret.__concatenate(result);
        }
        ret
    }
}

impl<T: ConfigCheckable> ConfigCheckable for Option<T>  {
    fn check(&self) -> CheckResult {
        self.__tree_check(0)
    }
    fn __tree_check(&self, depth: usize) -> CheckResult {
        if let Some(t) = self {
            t.__tree_check(depth)
        } else {
            CheckResult::new()
        }
    }
}