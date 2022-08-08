use inquire::{Select, Text};
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn makefile(_directory: &String) {
    const LIBRAIRY_MSG: &str = "A Static Librairy";
    const FUNCTION_MSG: &str = "A Function";
    const PROGRAMME_MSG: &str = "A Programme";

    let make_render = match Select::new(
        "What do you want to render ?",
        vec![PROGRAMME_MSG, FUNCTION_MSG, LIBRAIRY_MSG],
    )
    .prompt()
    {
        Ok(a) => a,
        Err(_) => " ",
    };

    let make_name: String = match Text::new("What's the name of the final binairy ?").prompt() {
        Ok(name) => name.trim().to_string(),
        Err(_) => "Default_Name".to_string(),
    };

    let test: bool = match Select::new("Do you want a test rule ?", vec!["[YES]", "[NO]"]).prompt()
    {
        Ok(a) => {
            if a == "[YES]" {
                true
            } else {
                false
            }
        }

        Err(_) => false,
    };

    let mut makefile = File::create("Makefile").unwrap(); // TODO handle s'il existe deja..
    let mut content = format!(
        "CC =\t\tgcc\n\
        CFLAGS =\t-Wall -Wextra -Werror\n\
        RM =\t\trm -f\n\
        SRCS =\t\t??\n\
        OBJS =\t\t${{SRCS:.c=.o}}\n\
        NAME =\t\t{make_name}\n\n\
        all: ${{NAME}}\n\n\
        clean:\n\
        \t${{RM}} ${{OBJS}}\n\n\
        fclean: clean\n\
        \t${{RM}} ${{NAME}}\n\n\
        re: fclean all\n\n\
        .PHONY: all clean fclean re\n\n"
    );

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
    if test {
        let os = env::consts::OS;
        content.push_str(if os == "linux" {
            "LEAKS =\tvalgrind -q --leak-check=full --track-origins=yes\n\n"
        } else if os == "darwin" {
            "LEAKS =\t??\n"
        } else {
            "windows c'est pas le top avec gcc..."
        });
        content.push_str("#bon je vais pas faire les tests pour vous quand meme\n");
    }
    makefile.write_all(content.as_bytes()).unwrap();
}
