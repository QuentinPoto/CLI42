//use crate::get_current_working_dir;

pub struct GitInit {
    //pub inf: bool, // aucune idee de quoi mettre ??
}

pub struct FilesInit {}

pub enum BinaryType {
    Librairy,
    Function,
    Programme,
}
pub struct Makefile {
    pub binary_name: String,
    pub binary_type: BinaryType,
    pub test: bool,
}

pub struct Modules {
    pub git: Option<GitInit>,
    pub files: Option<FilesInit>,
    pub makefile: Option<Makefile>,
}

pub struct Project {
    pub name: String,
    pub directory: String,
    pub modules: Option<Modules>,
}

impl Makefile {
    pub fn new() -> Makefile {
        Makefile {
            binary_name: String::new(),
            binary_type: BinaryType::Function,
            test: false,
        }
    }
}

impl Project {
    pub fn new() -> Project {
        Project {
            name: String::new(),
            directory: String::new(),
            modules: Some(Modules {
                git: None,
                files: None,
                makefile: None,
            }),
        }
    }
    /*
    pub fn prg_config(project_name: String) -> Project {
        let project_directory = get_current_working_dir();
        project_directory.push_str(&project_name);
        Project {
            name: project_name,
            directory: project_directory,
            modules: Modules {
                git: None,
                files: true,
                makefile: true,
            },
        }
    }*/
}
