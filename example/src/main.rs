use std::{path::Path, process::exit};

use config_checker::*;
use config_checker_macros::Check;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Check)]
struct Root {
    #[convert(as_str)]
    #[check(if(is_enum(self.child.gender, Gender::Male(_)), inside("Papy", "Papa"), if(is_enum(self.child.gender, Gender::Female(_)), inside("Mamie", "Maman"))))]
    name: String,
    #[check(and(ge(0.), lt(self.child.value)))]
    value: f32,
    #[check]
    child: Child,
}

impl Default for Root {
    fn default() -> Self {
        Self {
            name: "root".to_string(),
            value: 2.,
            child: Child::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Gender {
    Male(i32),
    Female(f32),
    Other(usize),
}

#[derive(Debug, Serialize, Deserialize, Clone, Check)]
struct Child {
    #[check(or(ge(1.), lt(0.)))]
    value: f32,
    gender: Gender,
    child: GreatChild,
}

impl Default for Child {
    fn default() -> Self {
        Self {
            value: 4.,
            gender: Gender::Other(0),
            child: GreatChild::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    let config_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/config1.yaml"));
    let config: Root = match confy::load_path(config_path) {
        Ok(config) => config,
        Err(error) => {
            println!("Error from Confy while loading the config file : {}", error);
            exit(-1);
        }
    };
    println!("Config check: {}", config.check());

    println!("Loaded configuration: \n{:#?}", config);
}
