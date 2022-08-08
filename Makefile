CC =		gcc
CFLAGS =	-Wall -Wextra -Werror
RM =		rm -f
SRCS =		??
OBJS =		${SRCS:.c=.o}
NAME =		salut.exe

all: ${NAME}

clean:
	${RM} ${OBJS}

fclean: clean
	${RM} ${NAME}

re: fclean all

.PHONY: all clean fclean re

${NAME}: ${OBJS}
	${CC} ${CFLAGS} ${OBJS} -o ${NAME}
LEAKS =	valgrind -q --leak-check=full --track-origins=yes

bon je vais pas faire les tests pour vous quand meme
