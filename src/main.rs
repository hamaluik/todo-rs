extern crate clap;
extern crate home;
extern crate todo_txt;

use clap::{App, Arg, SubCommand};

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
    let config: String = match matches.value_of("config") {
        Some(c) => String::from(c),
        None => {
            match home::home_dir() {
                Some(v) => String::from(v.to_str().unwrap()),
                None => {
                    panic!("No config path supplied and couldn't find home!");
                }
            }
        }
    };
    println!("Using config: {}", config);
    
    if let Some(_) = matches.subcommand_matches("add") {
        // TODO: add
    }
    else if let Some(_) = matches.subcommand_matches("search") {
        // TODO: search
    }
    else {
        // TODO: list all our tasks!
    }
}
