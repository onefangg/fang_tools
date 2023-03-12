from setuptools import setup, find_packages


setup(
    name="fang_tools",
    version="0.0.2",
    python_requires=">=3.7",
    packages=find_packages(),
    install_requires=[
        "rich>=13.0.0"
    ]
)
