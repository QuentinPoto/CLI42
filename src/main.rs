mod data;
mod inputs;
mod makefile;
mod utils;
use data::Project;
use makefile::{create_makefile, input_makefile};
use utils::{get_current_working_dir, print_welcome};

use inquire::{MultiSelect, Select, Text};
use std::{
    env, fs,
    process::{exit, Command},
};

// Idee : avoir un "config.json" avec des trucs preconfigurer
//          et si le fichier n'existe pas, proposer de le remplir !!

// TODO : faire un i42 -um : update makefile, en fonction des fichiers,
//          pareil pour le .h du projet
//          si je suis la meme structure a chaque fois, il peut devenier commen
//          updapter tout ce qu'il faut pour ca
//
// idee : mettre un flag devant les fonctions que je veut deplacer
//      "/// nom_du_fichier.c - autre infos"
//      ca permettrai de repartrir dans des .c, update le .h et le makefile...
//
//  TODO: le make file dans le bon folder !!

fn main() {
    let mut project = Project::new();
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => input(&mut project),
        2 => input_err(1),
        3 => {
            match &args[1][..] {
                "-p" => {
                    // p comme simple programme
                    project.name = args[2].clone(); //TODO lance mes preset
                    exit(1); //let project_name = args[2].clone();
                }
                _ => (),
            }
        }
        _ => input_err(1),
    }
    create(project);
}

fn input_err(code: u8) {
    if code == 1 {
        println!("not enough args... --help mes couilles");
    }
}

fn input(project: &mut Project) {
    print_welcome();
    project.name = inputs::project_name();
    project.directory = inputs::project_directory(&project.name);
    project.modules = inputs::project_modules();

    match project.modules {
        None => (),
        Some(ref mut modules) => {
            if !modules.makefile.is_none() {
                modules.makefile = Some(input_makefile());
            }
            if !modules.git.is_none() {
                //modules.git = Some(input_git());
            }
            if !modules.files.is_none() {
                //modules.files = Some(input_files()) ;
            }
        }
    }
}

fn create(mut project: Project) {
    fs::create_dir_all(&project.directory).unwrap();
    match project.modules {
        None => (),
        Some(ref mut modules) => {
            match modules.makefile {
                Some(ref makefile) => create_makefile(makefile),
                None => (),
            };
            match modules.files {
                Some(ref _files) => {}
                None => (),
            };
            match modules.git {
                Some(ref _git) => {
                    let pr = Command::new("sh")
                        .arg("-c")
                        .arg("git status")
                        .output()
                        .expect("caca git status");
                    let res = std::str::from_utf8(&pr.stdout).expect("caca");
                    let git_not_init_msg =
                        "fatal: not a git repository (or any of the parent directories): .git";
                    if res == git_not_init_msg {
                        let cmd_init = Command::new("sh")
                            .arg("-c")
                            .arg("git init")
                            .output()
                            .expect("git init caca");
                        println!("{}", std::str::from_utf8(&cmd_init.stdout).expect("caca2"));
                    } else {
                        println!("You already are in a git repo");
                    }
                }
                None => (),
            };
        }
    }
}
