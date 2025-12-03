use std::{path::Path, process::exit};

use config_checker::*;
use config_checker::macros::Check;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Check)]
struct Root {
    #[convert(as_str)]
    #[check(if(is_enum(self.child.as_ref().unwrap().gender, Gender::Male), inside("Papy", "Papa"), if(is_enum(self.child.as_ref().unwrap().gender, Gender::Female), inside("Mamie", "Maman"))))]
    name: String,
    #[check(and(ge(0.), lt(self.child.as_ref().unwrap().value)))]
    value: f32,
    #[check]
    child: Option<Child>,
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
    #[check(and(gt(0.), lt(3500.)))]
    weight: f32,
}

#[derive(Debug, Deserialize, Serialize, Check)]
struct Truck {
    #[check(gt(3500.))]
    weight: f32,
    wheels: u8,
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
    #[check(or(ge(1.), lt(0.)))]
    value: f32,
    #[check]
    gender: Gender,
    #[check]
    toy: Toy,
    child: GreatChild,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    let config_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/config2.yaml"));
    let config: Root = match confy::load_path(config_path) {
        Ok(config) => config,
        Err(error) => {
            println!("Error from Confy while loading the config file : {:?}", error);
            exit(-1);
        }
    };
    println!("Config check: {}", match config.check() {
        Ok(_) => "OK".to_string(),
        Err(e) => "\n".to_string()+e.as_str(),
    });

    println!("Loaded configuration: \n{:#?}", config);
}
