use crate::get_current_working_dir;

struct GitInit {
    inf: bool, // aucune idee de quoi mettre ??
}

struct FilesInit {
    a: bool, 
}

struct Makefile {
    
}

struct Modules {
    git: Option<GitInit>,
    files: Option<FilesInit>,
    makefile: Option<Makefile>,
}

pub struct Project {
    pub name: String,
    pub directory: String,
    pub modules: Modules,
}

impl Project {
    pub fn new() -> Project {
        Project {
            name: String::new(),
            directory: String::new(),
            modules: Modules { git: None, files: None, makefile: None },
        }
    }
    pub fn prg_config(project_name: String) -> Project {
        let project_directory = get_current_working_dir();
        project_directory.push_str(&project_name);
        Project {
            name: project_name,
            directory: project_directory,
            modules: Modules { git: None, files: true, makefile: true },
        }
    }
}
