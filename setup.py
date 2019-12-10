# -*- coding: utf-8 -*-
import distutils.util
import os
import re
import subprocess
from ast import literal_eval
from setuptools import setup
from setuptools_rust import Binding, RustExtension

REPO_URL = "https://github.com/autopilot-rs/autopy"


def module_attr_re(attr):
    return re.compile(r'__{0}__\s*=\s*(.*)'.format(attr))


def grep_attr(body, attr):
    return literal_eval(module_attr_re(attr).search(body).group(1))


def read_description():
    with open("README.md") as f:
        header = "For more information, see the [GitHub Repository]" \
                 "({0}).".format(REPO_URL)
        filter_re = re.compile(r'.*\bPyPI\b.*')
        contents = header + "\n" + filter_re.sub("", f.read())
        return contents.strip()


def parse_module_metadata():
    with open("autopy/__init__.py", "r") as f:
        body = f.read()
        return [grep_attr(body, attr) for attr in ("version", "author")]


def strtobool(string):
    return bool(distutils.util.strtobool(string))


def git_rev_count(revision):
    return subprocess.check_output(["git",
                                    "rev-list",
                                    "--count",
                                    revision]).decode("utf-8").strip()


def expand_version(version):
    env = os.environ
    is_ci = strtobool(env.get("CI", "f"))
    pr_sha = env.get("TRAVIS_PULL_REQUEST_SHA") or \
             env.get("APPVEYOR_PULL_REQUEST_HEAD_COMMIT")
    branch = env.get("APPVEYOR_REPO_BRANCH") or env.get("TRAVIS_BRANCH")
    if is_ci and not pr_sha and branch == "master":
        commit = env.get("APPVEYOR_REPO_COMMIT") or env.get("TRAVIS_COMMIT")
        rev_count = git_rev_count(commit)
        return "{}.dev{}".format(version, rev_count)
    return version


def main():
    version, author = parse_module_metadata()
    description = "A simple, cross-platform GUI automation library for Python."
    setup(
        name='autopy',
        version=expand_version(version),
        author=author,
        author_email='michael.sanders@fastmail.com',
        description=description,
        long_description=read_description(),
        long_description_content_type='text/markdown',
        url='https://www.autopy.org',
        license='MIT OR Apache-2.0',
        classifiers=[
            'Development Status :: 5 - Production/Stable',
            'Environment :: MacOS X',
            'Environment :: Win32 (MS Windows)',
            'Environment :: X11 Applications',
            'Intended Audience :: Developers',
            'License :: OSI Approved :: Apache Software License',
            'License :: OSI Approved :: MIT License',
            'Natural Language :: English',
            'Operating System :: MacOS',
            'Operating System :: Microsoft :: Windows',
            'Operating System :: POSIX :: Linux',
            'Programming Language :: Rust',
            'Programming Language :: Python :: 2.7',
            'Programming Language :: Python :: 3.5',
            'Programming Language :: Python :: 3.6',
            'Programming Language :: Python :: 3.7',
            'Programming Language :: Python :: 3.8'
        ],
        keywords=[
            "autopy",
            "autopilot",
            "GUI",
            "automation",
            "cross-platform",
            "input",
            "simulation",
        ],
        project_urls={
            "Documentation": "https://www.autopy.org/documentation/api-reference/",
            "Code": "https://github.com/autopilot-rs/autopy/",
            "Issue Tracker": "https://github.com/autopilot-rs/autopy/issues",
        },
        platforms=["macOS", "Windows", "X11"],
        rust_extensions=[
            RustExtension('autopy.alert', 'Cargo.toml', binding=Binding.PyO3),
            RustExtension('autopy.bitmap', 'Cargo.toml', binding=Binding.PyO3),
            RustExtension('autopy.color', 'Cargo.toml', binding=Binding.PyO3),
            RustExtension('autopy.key', 'Cargo.toml', binding=Binding.PyO3),
            RustExtension('autopy.mouse', 'Cargo.toml', binding=Binding.PyO3),
            RustExtension('autopy.screen', 'Cargo.toml', binding=Binding.PyO3),
        ],
        packages=['autopy'],
        zip_safe=False,  # Rust extensions are not zip safe, like C-extensions.
    )


if __name__ == '__main__':
    main()
