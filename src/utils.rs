use colored::Colorize;
use std::env;
use std::iter;

pub fn print_welcome() {
    const MSG: &str = "Welcome to the `init42` CLI, a 42 project kickstarter by qjungo";
    let left: String = String::from("*  ");

    let right: String = left.chars().rev().collect::<String>();
    let mut welcome = left.clone();
    let mut line = left.clone();
    let mut blank_line = left.clone();
    let middle: String = iter::repeat('-').take(MSG.len()).collect::<String>();
    let blank_middle: String = iter::repeat(' ').take(MSG.len()).collect::<String>();

    welcome.push_str(MSG);
    welcome.push_str(&right);
    line.push_str(&middle);
    line.push_str(&right);
    blank_line.push_str(&blank_middle);
    blank_line.push_str(&right);

    let final_msg = format!(
        "\n\n{}\n{}\n{}\n{}\n{}\n\n",
        line, blank_line, welcome, blank_line, line
    );
    println!("{}", final_msg.bold().green());
}

pub fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "Failed".to_string(),
    }
}
