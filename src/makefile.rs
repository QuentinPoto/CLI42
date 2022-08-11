use crate::{data::Makefile, inputs};
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn create_makefile(infos: &Makefile) {
    let binary_name = &infos.binary_name;
    let mut makefile = File::create("Makefile").unwrap(); // TODO handle s'il existe deja..
    let mut content = format!(
        "CC =\t\tgcc\n\
        CFLAGS =\t-Wall -Wextra -Werror\n\
        RM =\t\trm -f\n\
        SRCS =\t\t??\n\
        OBJS =\t\t${{SRCS:.c=.o}}\n\
        NAME =\t\t{binary_name}\n\n\
        all: ${{NAME}}\n\n\
        clean:\n\
        \t${{RM}} ${{OBJS}}\n\n\
        fclean: clean\n\
        \t${{RM}} ${{NAME}}\n\n\
        re: fclean all\n\n\
        .PHONY: all clean fclean re\n\n"
    );
    if infos.test {
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

pub fn input_makefile() -> Makefile {
    Makefile {
        binary_name: inputs::binary_name(),
        binary_type: inputs::binary_type(),
        test: inputs::make_test(),
    }
}
