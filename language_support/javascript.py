from language_support.base import SetupProblem


class SetupJavascript(SetupProblem):
    def __init__(self, title, problem_html) -> None:
        super().__init__(title, problem_html)
        with open(self.problem_dir / "javascript.js", mode="w") as f:
            f.write(
"""\
const readline = require("readline").createInterface({
    input: process.stdin,
    output: process.stdout
});

readline.question("", s => {
    const num_of_tests = parseInt(s.trim());

    for (let _ = 0; _ < num_of_tests; _++) {
        // write code here
    }

    readline.close();
});
"""
            )

