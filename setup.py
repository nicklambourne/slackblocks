from setuptools import find_packages, setup


production_requirements = []

with open("README.md", "r") as file_:
    long_description = file_.read()

setup(
    name="slackblocks",
    version="0.1.2",
    author="Nicholas Lambourne",
    author_email="nick@ndl.im",
    description="Python wrapper for the Slack Blocks API",
    install_requires=production_requirements,
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/nicklambourne/slackblocks",
    packages=find_packages(".", exclude=["test"]),
    include_package_data=True,
    setup_requires=[
        "pytest",
        "twine",
        "wheel",
        "slackclient"
    ],
    classifiers=[
        "Programming Language :: Python :: 3.6",
        "License :: OSI Approved :: BSD License",
        "Operating System :: OS Independent",
        "Typing :: Typed",
        "Topic :: Communications :: Chat"
    ]
)
