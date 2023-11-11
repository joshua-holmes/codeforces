import shutil
import subprocess

from language_support.base import SetupProblem


class SetupRust(SetupProblem):
    def __init__(self, title, problem_html) -> None:
        super().__init__(title, problem_html)
        rust_dir = self.problem_dir / "rust"
        if rust_dir.exists():
            shutil.rmtree(rust_dir)
        subprocess.run(["cargo", "new", "rust"], cwd=self.problem_dir, text=True, capture_output=True)
        with open(self.problem_dir / "rust/src/main.rs", "w") as f:
            f.write(
"""\
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        // write code here
    }

    Ok(())
}
"""
            )

