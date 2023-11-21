# Codeforces Problems

## What Is This?

This repo has a script that automatically generates directory structure and boilerplate code for your codeforces problems.

This is a great tool if you want to push your problem solutions to your GitHub with minimal effort to have an organized repo.

## How Do I Set It Up?

1. Simply fork this repo, then delete the `problems` directory.
2. Next, install `python` and `poetry`. `poetry` is a dependency management tool and is found on Python's package manager, `pip`, which you get automatically after installing Python. So to install `poetry`, run `pip3 install poetry`.
3. Now run `poetry install` in the project's root directory to install the required dependencies.

After that, it's all yours!

## How Do I Use It?

To setup a codeforces problem, run `python3 new.py` in the project's root directory, which will start the script that sets up the problem for you.

The script will ask for the codeforce problem's URL. It will scrape the webpage you supplied for the problem description and title, then set up your problem under the generated `problems` directory with the problem description in the README file.

The script will also ask for your chosen language and give you boilerplate code for your selected language.

The script does support multiple languages per codeforces problem, which is helpful for those that like to solve a problem in multiple languages for practice.

## Contributing

If you don't see your language of choice in the list of supported languages, feel free to add it in the `language_support` directory by copying an existing language's implementation and modifying it for your language. You can also create a pull request for adding support for your language and I'd be happy to merge it in.

If you don't want to add support for your language, you can also create a GitHub issue and I can get support added in.

