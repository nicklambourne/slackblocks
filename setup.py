from setuptools import find_packages, setup


development_requirements = [
    "pytest"
]

with open("README.md", "r") as file_:
    long_description = file_.read()

setup(
    name="slackblocks",
    version="0.0.1",
    author="Nicholas Lambourne",
    author_email="",
    description="Python wrapper for the Slack Blocks API",
    #  install_requires=[],
    extras_requires={
        "dev": development_requirements
    },
    long_description=long_description,
    long_description_content_time="text/markdown",
    url="https://github.com/nicklambourne/slackblocks",
    packages=find_packages(),
    classifiers=[
        "Programming Language :: Python :: 3.6",
        "License :: OSI Approved :: BSD License",
        "Operating System :: OS Independent",
        "Typing :: Typed",
        "Topic :: Communications :: Chat"
    ]
)
