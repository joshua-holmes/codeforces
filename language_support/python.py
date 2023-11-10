from language_support.base import SetupProblem


class SetupPython(SetupProblem):
    def __init__(self, title, description) -> None:
        super().__init__(title, description)
        with open(self.problem_dir / "python.py", mode="w") as f:
            f.write(
"""\
def main():
    num_of_tests = input()
    for _ in num_of_tests:
        # write code here
        pass

if __name__ == "__main__":
    main()
"""
            )

