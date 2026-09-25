use std::{path::Path, process::exit};

use serde::{Deserialize, Serialize};
use config_checker::*;

#[derive(Debug, Deserialize, Serialize, Check)]
struct Root {
    name: String,
    value: f32,
    #[check]
    child: Option<Child>,
}

impl Check for Root {
    fn do_check(&self) -> CheckResult {
        let mut result = CheckResult::new();
        if let Some(child) = &self.child {
            match &child.gender {
                Gender::Male => {
                    if self.name != "Papy" && self.name != "Papa" {
                        result.add_error(format!("Since the child is a male, the name should be `Papy` or `Papa`, but found `{}`", self.name));
                    }
                },
                Gender::Female => {
                    if self.name != "Mamie" && self.name != "Maman" {
                        result.add_error(format!("Since the child is a female, the name should be `Mamie` or `Maman`, but found `{}`", self.name));
                    }
                },
                _ => {},
            }
            if self.value >= child.value {
                result.add_error(format!("The value should be less than the child's value ({}), but found `{}`", child.value, self.value));
            }
        }
        if self.value < 0. {
            result.add_error(format!("The value should be positive, but found `{}`", self.value));
        }
        if self.name.is_empty() {
            result.add_warning("The name is empty".to_string());
        }
        result
    }
}

impl Default for Root {
    fn default() -> Self {
        Self {
            name: "root".to_string(),
            value: 2.,
            child: Some(Child::default()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Check)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug, Deserialize, Serialize, Check)]
struct Car {
    weight: f32,
}

impl Check for Car {
    fn do_check(&self) -> CheckResult {
        let mut result = CheckResult::new();
        if self.weight <= 0. {
            result.add_error(format!("The weight should be greater than 0, but found `{}`", self.weight));
        } else if self.weight >= 3500. {
            result.add_error(format!("The weight should be less than 3500, but found `{}`", self.weight));
        }
        result
    }
}

#[derive(Debug, Deserialize, Serialize, Check)]
struct Truck {
    weight: f32,
    wheels: u8,
}

impl Check for Truck {
    fn do_check(&self) -> CheckResult {
        let mut result = CheckResult::new();
        if self.weight <= 3500. {
            result.add_error(format!("The truck should have a weight greater than 3500, but found `{}`", self.weight));
        }
        if self.wheels < 4 {
            result.add_warning(format!("The truck has less than 4 wheels, found `{}`", self.wheels));
        }
        result
    }
}

#[derive(Debug, Deserialize, Serialize, Check)]
pub enum Toy {
    #[check]
    Car(Car),
    #[check]
    Truck(Truck),
    None,
}

#[derive(Debug, Deserialize, Serialize, Check)]
#[serde(default)]
struct Child {
    value: f32,
    #[check]
    gender: Gender,
    #[check]
    toy: Toy,
    child: GreatChild,
}

impl Check for Child {
    fn do_check(&self) -> CheckResult {
        let mut result = CheckResult::new();
        if self.value >= 0. && self.value < 1. {
            result.add_error(format!("The value should be outside of the range ]0, 1], but found `{}`", self.value));
        }
        result
    }
}

impl Default for Child {
    fn default() -> Self {
        Self {
            value: 4.,
            gender: Gender::Other,
            toy: Toy::Car(Car {
                weight: 2.
            }),
            child: GreatChild::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Check)]
#[serde(default)]
struct GreatChild {
    name: String,
    value: f32,
}

impl Default for GreatChild {
    fn default() -> Self {
        Self {
            name: "baby".to_string(),
            value: 6.,
        }
    }
}

fn main() {
    println!("====== config1.yaml ======");
    let config_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/config1.yaml"));
    let config: Root = match confy::load_path(config_path) {
        Ok(config) => config,
        Err(error) => {
            println!("Error from Confy while loading the config file : {:?}", error);
            exit(-1);
        }
    };
    let config_check = config.check();
    println!("{}", config_check);
    println!("Config check: {}", config_check.is_ok());

    println!("Loaded configuration: \n{:#?}", config);

    println!("====== config2.yaml ======");
    let config_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/config2.yaml"));
    let config: Root = match confy::load_path(config_path) {
        Ok(config) => config,
        Err(error) => {
            println!("Error from Confy while loading the config file : {:?}", error);
            exit(-1);
        }
    };
    let config_check = config.check();
    print!("{}", config_check);
    println!("Config check: {}", config_check.is_ok());

    println!("Loaded configuration: \n{:#?}", config);
}
