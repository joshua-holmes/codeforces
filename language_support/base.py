from pathlib import Path


class SetupProblem:
    def __init__(self, title, description) -> None:
        # create necessary directories
        self.top_dir = Path(__file__).parent.parent
        self.problem_dir = self.top_dir / "problems" / self.format_to_snake_case(title)
        self.problem_dir.mkdir(parents=True, exist_ok=True)
        
        # write problem description html to README.md
        with open(self.problem_dir / "README.md", mode="w") as f:
            f.write(f"# {title}\n" + description)

    def format_to_snake_case(self, str_in):
        str_out = str_in.lower().replace(" ", "_")
        return str_out

