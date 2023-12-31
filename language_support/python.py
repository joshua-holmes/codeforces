from language_support.base import SetupProblem


class SetupPython(SetupProblem):
    def __init__(self, title, problem_html) -> None:
        super().__init__(title, problem_html)
        with open(self.problem_dir / "python.py", mode="w") as f:
            f.write(
"""\
def main():
    num_of_tests = int(input())
    for _ in range(num_of_tests):
        # write code here
        pass

if __name__ == "__main__":
    main()
"""
            )

