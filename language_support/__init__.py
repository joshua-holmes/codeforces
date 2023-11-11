from language_support.c import SetupC
from language_support.javascript import SetupJavascript
from language_support.python import SetupPython
from language_support.rust import SetupRust
from language_support.cpp import SetupCpp


# To add new language support, just copy an existing setup (like the `python.py` file) and setup how you want the
# directory to look. Then import the file here and add it to the Dict below. That's it!


SUPPORTED_LANGUAGES = {
    "python": SetupPython,
    "rust": SetupRust,
    "c++": SetupCpp,
    "c": SetupC,
    "javascript": SetupJavascript
}

