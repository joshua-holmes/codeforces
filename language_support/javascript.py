from language_support.base import SetupProblem


class SetupJavascript(SetupProblem):
    def __init__(self, title, problem_html) -> None:
        super().__init__(title, problem_html)
        with open(self.problem_dir / "javascript.js", mode="w") as f:
            f.write(
"""\
"use strict";
process.stdin.resume();
process.stdin.setEncoding("utf-8");
 
function print(x) {
  console.log(x);
}
let inputString = "";
let currentLine = 0;
 
process.stdin.on("data", (inputStdin) => {
  inputString += inputStdin;
});
process.stdin.on("end", () => {
  inputString = inputString.split("\\n");
  main();
});
function readline() {
  return inputString[currentLine++];
}
 
// ********** Code Start **********

function main() {
  let testCases = parseInt(readline());

  for (let _ = 0; _ < testCases; _++) {
    // write code here
  }
}
"""
            )

