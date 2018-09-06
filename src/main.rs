#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate clap;
extern crate home;
extern crate todo_txt;
extern crate crossterm;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::BufRead;
use clap::{App, Arg, SubCommand};
use todo_txt::Task;
use crossterm::{Screen, Crossterm, Color};

#[derive(Deserialize)]
struct Config {
    path: String,
}

fn ensure_config_and_get_contents(path: &PathBuf) -> String {
    let display = path.display();

    if !path.exists() {
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        let todo_loc = match home::home_dir() {
            Some(v) => v.join("todo.txt"),
            None => panic!("Couldn't find home!")
        };

        if let Err(why) = file.write_all(format!("path = '{}'", todo_loc.to_str().unwrap()).as_bytes()) {
            panic!("couldn't write to {}: {}", display, why.description())
        }
    }

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut contents: String = String::new();
    if let Err(why) = file.read_to_string(&mut contents) {
        panic!("couldn't read from {}: {}", display, why.description())
    }
    contents
}

fn ensure_todo_file_exists(path: &PathBuf) {
    if !path.exists() {
        let display = path.display();
        if let Err(why) = File::create(&path) {
            panic!("couldn't create {}: {}", display, why.description())
        }
    }
}

fn parse_todo_txt(path: &PathBuf) -> Vec<Task> {
    let display = path.display();
    let f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let file = BufReader::new(&f);
    file.lines().map(|line| todo_txt::parser::task(&line.unwrap()).unwrap()).collect()
}

fn print_task(screen: &Screen, crossterm: &Crossterm, task: &Task) {
    let style = crossterm.style(format!("{}\n", task.subject)).with(Color::Black).on(Color::White);
    style.paint(&screen);
}

fn main() {
    let app: App = App::new("todo.txt")
        .version("0.1.0")
        .about("todo.txt CLI for everywhere")
        .author("Kenton Hamaluik")
        .subcommand(SubCommand::with_name("add")
            .about("add a new todo item")
            .arg(Arg::with_name("task description")
                .required(true)
                .help("the description of the task to use, including +projects and @contexts")
            )
        )
        .subcommand(SubCommand::with_name("search")
            .about("search for a todo item")
            .arg(Arg::with_name("search terms")
                .required(true)
                .help("the search query you want to use")
            )
        )
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("file")
            .help("set a custom config file")
            .takes_value(true)
        )
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .value_name("path")
            .help("override the todo.txt path from the config")
            .takes_value(true)
        );
    let matches = app.get_matches();

    let screen = Screen::default();
    let crossterm = Crossterm::new(&screen);

    // parse the config
    let config: PathBuf = match matches.value_of("config") {
        Some(c) => PathBuf::from(c),
        None => {
            match home::home_dir() {
                Some(v) => v.join(".todo.toml"),
                None => {
                    panic!("No config path supplied and couldn't find home!");
                }
            }
        }
    };
    let config = ensure_config_and_get_contents(&config);
    let config: Config = match toml::from_str(&config) {
        Ok(c) => c,
        Err(why) => panic!("couldn't parse config: {}", why.description())
    };

    // parse the path override
    let path: PathBuf = match matches.value_of("path") {
        Some(p) => PathBuf::from(p),
        None => PathBuf::from(config.path)
    };
    ensure_todo_file_exists(&path);
    
    if let Some(_) = matches.subcommand_matches("add") {
        // TODO: add
    }
    else if let Some(_) = matches.subcommand_matches("search") {
        // TODO: search
    }
    else {
        // list all our tasks by default
        let tasks = parse_todo_txt(&path);
        for task in tasks {
            print_task(&screen, &crossterm, &task);
        }
    }
}
