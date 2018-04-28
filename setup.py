# -*- coding: utf-8 -*-
import re
from ast import literal_eval
from setuptools import setup
from setuptools_rust import Binding, RustExtension


def module_attr_re(attr):
    return re.compile(r'__{0}__\s*=\s*(.*)'.format(attr))


def grep_attr(body, attr):
    return literal_eval(module_attr_re(attr).search(body).group(1))


def parse_module_metadata():
    with open("autopy/__init__.py", "r") as f:
        body = f.read()
        return [grep_attr(body, attr) for attr in ("version", "author")]


def main():
    version, author = parse_module_metadata()
    description = "A simple, cross-platform GUI automation library for Python."
    setup(
        name='autopy',
        version=version,
        author=author,
        author_email='michael.sanders@fastmail.com',
        description=description,
        license='Apache-2.0',
        classifiers=[
            'Development Status :: 4 - Beta',
            'Environment :: MacOS X',
            'Environment :: Win32 (MS Windows)',
            'Environment :: X11 Applications',
            'Intended Audience :: Developers',
            'License :: OSI Approved :: MIT License',
            'Natural Language :: English',
            'Operating System :: MacOS :: MacOS X',
            'Operating System :: Microsoft :: Windows',
            'Programming Language :: Rust',
            'Programming Language :: Python :: 2.7',
            'Programming Language :: Python :: 3',
        ],
        keywords=[
            "autopy",
            "autopilot",
            "GUI",
            "automation",
            "cross-platform",
        ],
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
