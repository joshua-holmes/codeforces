import re
from selenium import webdriver
from selenium.webdriver.chrome.webdriver import WebDriver
from selenium.webdriver.common.by import By
from typing import Tuple


from language_support import SUPPORTED_LANGUAGES


# selenium constants (selenium is the web scraping tool)
DRIVER = None
HEADLESS = True


def get_driver() -> WebDriver:
    global DRIVER
    if not DRIVER:
        options = webdriver.ChromeOptions()
        options.add_argument("--headless")
        DRIVER = webdriver.Chrome(options=options)
    return DRIVER


def scrape_problem(url: str) -> Tuple[str, str]:
    """
    Scrapes codeforces website for title and problem description. Also adds URL to bottom of description.

    Arguments:
        url -- URL of codeforces problem to scrape

    Returns:
        tuple -- Tuple with 2 string values, 1st is title of problem, 2nd is the whole problem in html
    """
    driver = get_driver()
    driver.get(url)

    # get problem data
    problem_obj = driver.find_element(By.CLASS_NAME, "problem-statement")
    problem_html = problem_obj.get_attribute("innerHTML")
    if not problem_html:
        raise Exception("Cannot get problem description when from url")
    title = problem_obj.find_element(By.CLASS_NAME, "title").text

    # example: `title` looks like this -> "C1. A Title". find "C1. " and remove it from `title`
    re_match = re.match("^[A-Z]+[0-9]*\. ", title)
    if re_match:
        title = title[re_match.end():]

    # inject url at bottom of problem html
    problem_html += f'\n<a href={url} target="_blank">{url}</a>'

    return title, problem_html


def generate_problem_info() -> Tuple[str, str]:
    """
    Ask for title from user (stdin), then returns that along with generic description.

    Returns:
        tuple -- Tuple with 2 string values, 1st is title of problem, 2nd is description
    """
    print("Please enter title of problem or name of directory to generate:")
    title = input()
    print()
    if not title:
        raise Exception("No title/directory name provided!")
    problem_html = f"""\
<h1>{title}</h1>
<p>URL of codeforces problem was not provided, so this generic description is provided instead</p>
"""

    return title, problem_html


def main():
    print("Enter URL of codeforces problem (optional):")
    url = input().strip()
    print()

    print("Enter programming language you'd like to use. Supported language are:")
    for l in SUPPORTED_LANGUAGES:
        print(f"\t{l}")
    language = input().strip().lower()
    print()
    if language not in SUPPORTED_LANGUAGES:
        raise Exception(f'Language "{language}" is not supported. Please run the program again.')

    if url:
        title, problem_html = scrape_problem(url)
    else:
        print("No URL provided")
        title, problem_html = generate_problem_info()

    SetupClass = SUPPORTED_LANGUAGES[language]
    problem = SetupClass(title, problem_html)

    print(f"Done with process {problem.__class__.__name__}. Problem can be found here:")
    print(problem.problem_dir)

if __name__ == "__main__":
    main()

