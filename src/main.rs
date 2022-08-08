mod data;
mod utils;
mod inputs;
mod makefile;
use utils::{get_current_working_dir, print_welcome};
use data::{Project};
use makefile::makefile;

use inquire::{MultiSelect, Select, Text};
use std::{env, fs, process};

// Idee : avoir un "config.json" avec des trucs preconfigurer
//          et si le fichier n'existe pas, proposer de le remplir !!


fn main() {
    let mut project = Project::new();    
    
    /* Getting the inputs */
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        input_err(1)
    } else if args.len() == 3 {
        if args[1] == "-p" {
            // p comme simple programme
            project.name = args[2].clone();
            //TODO lance mes preset
            process::exit(1);
            //let project_name = args[2].clone();
        }
    } else {
         print_welcome();
         project.name = inputs::project_name();
         project.directory = inputs::project_directory(&project.name);
         project.modules = inputs::project_modules();
    }
   
    /* Creating the shit */
    fs::create_dir_all(&project.directory).unwrap();
   
    /* Modules */
    match project.modules {
        Some(modules) => {
            if modules.contains(&"Makefile".to_string()) {
                println!("make");
                //makefile();
            }
            if modules.contains(&"Some c files".to_string()) {
                println!("aure");
            }
            if modules.contains(&"GitInit".to_string()) {
                println!("init");
            }
        },
        None => {},
    }

}

fn c_init(directory: &String) {
    println!("c init {}", directory);
}

fn git_init(directory: &String) {
    println!("git init {}", directory);
    //  sys send git init
}
fn input_err(code: u8) {
    if code == 1 {
        println!("not enough args... --help mes couilles");
    }
}
