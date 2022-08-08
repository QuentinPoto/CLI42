use crate::{Select, get_current_working_dir, Text, MultiSelect};

pub fn project_name() -> String
{
        match Text::new("Let's begin, what is the name of your project ?").prompt() {
            Ok(name) => name.trim().to_string(),
            Err(_) => "Default_Name".to_string(),
        }
}

pub fn project_directory(project_name: &String) -> String
{
    match Select::new(
        &format!("Where do you want {} to be in ?", project_name),
        vec![
            get_current_working_dir(),
            format!("{}/{}", get_current_working_dir(), project_name),
        ],
    )
    .prompt()
    {
        Ok(d) => d,
        Err(_) => "dir".to_string(),
    }
}

pub fn project_modules() -> Option<Vec<String>>
{
    let starter_options = vec!["Makefile".to_string(), "Some c files".to_string(), "GitInit".to_string()];
    match MultiSelect::new("What do you want to initialize ?", starter_options).prompt() {
            Ok(a) => Some(a),
            Err(_) => None,
        }
}
