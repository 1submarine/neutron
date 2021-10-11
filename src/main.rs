#![allow(dead_code)]
mod astronomical;
mod display;
mod ident;
mod input;
mod name;
mod naval;
mod save_game;
mod world;

use std::fs::File;

use directories::ProjectDirs;
use handlebars::Handlebars;
use nanorand::WyRand;
use rustyline::Editor;
use serde_json::json;

use crate::{save_game::SaveGame, world::WorldBuilder};

fn main() -> Result<(), String> {
    // Setup
    // RNG
    let mut rng = WyRand::new();
    // Readline
    let mut rl = Editor::<()>::new();
    // Save vector
    let mut saves = Vec::new();
    // Templating
    let reg: Handlebars = handlebars_init();

    // Program Directories
    let proj_dirs = ProjectDirs::from("star", "Neutron", "Neutron Star").unwrap();

    // Ensure integrity of configuration
    // TODO use if let?
    let config_dir = proj_dirs.config_dir();
    if !config_dir.is_dir() {
        std::fs::create_dir_all(config_dir).unwrap();
    }
    let cache_dir = proj_dirs.cache_dir();
    if !cache_dir.is_dir() {
        std::fs::create_dir_all(cache_dir).unwrap();
    }
    let conf_file = proj_dirs.config_dir().join("config");
    if !conf_file.is_file() {
        File::create(&conf_file).unwrap();
    }
    let hist_file = proj_dirs.cache_dir().join("history.txt");
    if !hist_file.is_file() {
        File::create(&hist_file).unwrap();
    }

    // Revise Names
    let configuration = {
        let mut ret = config::Config::default();

        // Load Config File
        ret.merge(config::File::with_name(
            proj_dirs.config_dir().join("config.toml").to_str().unwrap(),
        ))
        .unwrap();

        ret
    };

    let player_name: String = {
        let key = "player_name";
        let error = json!({ "key": key });
        configuration
            .get(key)
            .expect(&reg.render("config_error", &error).unwrap())
    };

    // Load Readline history
    rl.load_history(&hist_file).unwrap();

    let world = WorldBuilder::new(&mut rng).build(&mut rng);

    // Print Greeting
    println!("Welcome {}", player_name);

    'running: loop {
        let readline = rl.readline("(+ ").unwrap();
        rl.add_history_entry(readline.as_str());
        // TODO unicode_segmentation
        match readline.as_str() {
            "save" => saves.push(SaveGame::new(vec![(
                world.id.uuidv4.to_string(),
                world.save().unwrap(),
            )])),
            "write" => {
                saves.last().unwrap().write(cache_dir).unwrap();
                rl.save_history(&hist_file).unwrap();
            }
            "exit" => break 'running,
            "map" => display::map(&world.galaxy),
            _ => (),
        }
    }

    Ok(())
}

fn handlebars_init<'a>() -> Handlebars<'a> {
    let mut reg = Handlebars::new();
    reg.register_template_string(
        "config_error",
        "Configuration Error key `{{ key }}` not found or has no value",
    )
    .unwrap();
    reg
}
