use std::{path::Path, process::exit};

use config_checker_macros::Check;
use config_checker::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, Check)]
struct Root {
    #[inside("Papy", "Mamie")]
    name: String,
    #[ge =0.]#[lt=1.]
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

#[derive(Debug, Serialize, Deserialize, Clone, Check)]
struct Child {
    #[ge =1.]
    value: f32,
    child: GreatChild,
}

impl Default for Child {
    fn default() -> Self {
        Self {
            value: 4.,
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
            value: 6.
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

    // let c = std::str::from_utf8(&(0x21B3 as u32).to_be_bytes()).unwrap();
    // match Box::new(&config.child).try_into() {
    //     Ok(c) => println!("Coucou"),
    //     Err(_) => println!("Not coucou"),
    // };
    
    println!("Loaded configuration: \n{:#?}", config);
}
