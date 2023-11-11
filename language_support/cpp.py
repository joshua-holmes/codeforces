from language_support.base import SetupProblem


class SetupCpp(SetupProblem):
    def __init__(self, title, description) -> None:
        super().__init__(title, description)
        cpp_dir = self.problem_dir / "cpp"
        cpp_dir.mkdir(exist_ok=True)

        # main cpp file
        with open(cpp_dir / "main.cpp", mode="w") as f:
            f.write(
            """\
#include <iostream>
#include <string>

int main()
{
    std::string num_of_tests_str;
    std::cin >> num_of_tests_str;
    int num_of_tests = std::stoi(num_of_tests_str);

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
run: main.cpp
	@g++ *.cpp && ./a.out && rm a.out
"""
            )

        # so LSP can use c++20
        with open(cpp_dir / "compile_flags.txt", mode="w") as f:
            f.write("-std=c++20")

        print("Run `make` in `cpp` directory to compile and run code for testing.\n")
