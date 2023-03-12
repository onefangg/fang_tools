"""
Module for formatting text
"""
from rich.console import Console
from rich.text import Text


def starry_text(text):
    return f"****{text}****"


def pprint_starry_text(text, color="bold magenta"):
    console = Console()
    fmt_text = Text(text)
    fmt_text.stylize(color, 0, 6)
    console.print(fmt_text)
