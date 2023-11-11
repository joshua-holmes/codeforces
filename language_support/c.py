from language_support.base import SetupProblem


class SetupC(SetupProblem):
    def __init__(self, title, description) -> None:
        super().__init__(title, description)
        cpp_dir = self.problem_dir / "c"
        cpp_dir.mkdir(exist_ok=True)

        # main cpp file
        with open(cpp_dir / "main.c", mode="w") as f:
            f.write(
            """\
#include <stdio.h>
#include <stdlib.h>

int main()
{
    char num_of_tests_str[10240] = ""; // allows 10,240 chars (10KB), adjust if needed
    scanf("%s", num_of_tests_str);
    int num_of_tests = atoi(num_of_tests_str);

    for (int _ = 0; _ < num_of_tests; _++) {
        // write code here
    }
}
"""
            )

        # for quick and easy testing, just run `make` to compile, run and cleanup
        with open(cpp_dir / "Makefile", mode="w") as f:
            f.write(
                """\
run: main.c
	@gcc *.c && ./a.out && rm a.out
"""
            )

        print("Run `make` in `c` directory to compile and run code for testing.\n")

