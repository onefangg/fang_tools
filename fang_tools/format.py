"""
Module for formatting text
"""
from rich.console import Console
from rich.text import Text


def starry_text(text: str) -> str:
    """Wraps text with 4 '*'

    Args:
        text (str): Input text

    Returns:
        str: Text wrapped asteriks
    """
    return f"****{text}****"


def pprint_starry_text(text: str, color: str ="bold magenta"):
    """Pretty prints text with color. Defaults to color if not specified.

    Args:
        text (str): Input text
        color (str, optional): Color to format input text. Defaults to "bold magenta".
    """
    console = Console()
    fmt_text = Text(text)
    fmt_text.stylize(color, 0, 6)
    console.print(fmt_text)
