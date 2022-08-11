use crate::data::{BinaryType, FilesInit, GitInit, Makefile, Modules};
use crate::{get_current_working_dir, MultiSelect, Select, Text};

pub fn project_name() -> String {
    Text::new("Let's begin, what is the name of your project ?")
        .prompt()
        .expect("You need a project name")
}

pub fn project_directory(project_name: &String) -> String {
    Select::new(
        &format!("Where do you want {} to be in ?", project_name),
        vec![
            get_current_working_dir(),
            format!("{}/{}", get_current_working_dir(), project_name),
        ],
    )
    .prompt()
    .expect("You need a directory")
}

pub fn project_modules() -> Option<Modules> {
    const MAKEFILE: &str = "Makefile";
    const FILES: &str = "Some c files";
    const GIT: &str = "GitInit";

    let starter_options = vec![MAKEFILE, FILES, GIT];
    match MultiSelect::new("What do you want to initialize ?", starter_options).prompt() {
        Ok(a) => {
            let mut module = Modules {
                git: None,
                files: None,
                makefile: None,
            };
            if a.contains(&MAKEFILE) {
                module.makefile = Some(Makefile::new());
            }
            if a.contains(&FILES) {
                module.files = Some(FilesInit {});
            }
            if a.contains(&GIT) {
                module.git = Some(GitInit {});
            }
            Some(module)
        }
        Err(_) => None,
    }
}

pub fn make_test() -> bool {
    const YES: &str = "YES";
    const NO: &str = "NO";
    match Select::new("Do you want a test rule ?", vec![YES, NO]).prompt()//.expect("bug when choosing if test")
    {
        Ok(a) => {
            match a {
                YES=> true,
                NO => false,
                _  => false,
            }
        }
        Err(_) => false,
    }
}

pub fn binary_type() -> BinaryType {
    const LIBRAIRY_MSG: &str = "A Static Librairy";
    const FUNCTION_MSG: &str = "A Function";
    const PROGRAMME_MSG: &str = "A Programme";

    match Select::new(
        "What do you want to render ?",
        vec![PROGRAMME_MSG, FUNCTION_MSG, LIBRAIRY_MSG],
    )
    .prompt()
    {
        Ok(a) => match a {
            PROGRAMME_MSG => BinaryType::Programme,
            LIBRAIRY_MSG => BinaryType::Librairy,
            FUNCTION_MSG => BinaryType::Function,
            _ => BinaryType::Programme,
        },
        Err(_) => BinaryType::Programme,
    }
}

/*
   if make_render == PROGRAMME_MSG {
       content.push_str(
           "${NAME}: ${OBJS}\n\
                       \t${CC} ${CFLAGS} ${OBJS} -o ${NAME}\n",
       );
   } else if make_render == LIBRAIRY_MSG {
       content.push_str(
           "${NAME}: ${OBJS}\n\
                       \tar -rcs ${NAME} ${OBJS}\n\n",
       );
   }

*/

pub fn binary_name() -> String {
    match Text::new("What's the name of the final binairy ?").prompt() {
        Ok(name) => name.trim().to_string(),
        Err(_) => "Default_Name".to_string(),
    }
}
