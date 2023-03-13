# Demo


## Branch Navigation

0.0.1 -> barebones libary with setup.py build method

0.0.2 -> library with dependencies

0.0.3 -> add pyproject.toml for build alternative

0.0.4 -> using project manager (hatch) and adding docs

## Setting up

Creating virtual environments

`python -m venv .venv`

Activating them

`source .venv/bin/activate`

## Build

`python setup.py bdist_wheel`

- install `wheel` 

`pip install build`

- recommendation from [pypa](https://packaging.python.org/en/latest/tutorials/packaging-projects/#generating-distribution-archives)



`pipx install hatch`

`hatch new --init`
-> updates existing pyproject.toml (if any)


create docs

enter sub-environment specified

`hatch -e docs shell`

run command to quick start setup

`sphinx-quickstart docs/ --ext-autodoc`