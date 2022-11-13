mod recipe;

use bevy::prelude::*;
use ron::ser::PrettyConfig;
use crate::recipe::{CraftingRecipe, Item, ItemType};

fn main() {
    let r = CraftingRecipe {
        input: vec![Item{ item_type: ItemType::Wood }],
        output: vec![Item{ item_type: ItemType::Stone }]
    };
    
    let s = ron::to_string(&r).unwrap();
    println!("{}", s);
    
    
    // App::build()
    //     .add_plugins(DefaultPlugins)
    //     .add_startup_system(hello_world.system())
    //     .run();
}

fn hello_world() {
    println!("Hello, world!");
}