from setuptools import find_packages, setup


with open("README.md", "r") as file_:
    long_description = file_.read()

production_requirements = []

setup(
    name="slackblocks",
    version="0.2.3",
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
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "License :: OSI Approved :: BSD License",
        "Operating System :: OS Independent",
        "Typing :: Typed",
        "Topic :: Communications :: Chat"
    ]
)
