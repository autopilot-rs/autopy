## Windows Packaging Instructions

The steps to get packaging up and running on Windows are difficult to keep
consistent so I've documented them here. This may also be helpful for others
installing from the latest source on the GitHub repository.

Note: I am currently using VMware Fusion so a few steps are specific to that
(e.g. shared folders).

#### Initial setup

1. [Download the Windows 10
   ISO](https://www.microsoft.com/en-us/software-download/windows10). Use
   separate VMs for 32 and 64-bit versions.
2. Download and run the [Build Tools for Visual
   Studio](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2017)
   installer. Check the "Visual C++ build tools" workload and click install.
   This can take some time.
3. Install
   [rustup](https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe)
   with the default options<sup id="a1">[1](#f1)</sup>. Note: Due to the issue
   described [here](https://github.com/rust-lang-nursery/rustup.rs/issues/763),
   it can significantly improve installation times to temporarily turn off
   Windows Defender security settings during the installation.
4. Install [Python 3.7](https://www.python.org/downloads/release/python-370/)
   and ensure "Add Python 3.7 to path" is checked.
5. Enable Shared Folders from VMware and share the autopy repo directory.
   
#### Confirm Python & Rust setup

1. Open Command Prompt and run `py -3.7` to confirm Python is setup correctly.
2. Run `rustc --help` to confirm Rust is installed correctly.

#### Install adjacent Python versions

1. Install [Python 3.6](https://www.python.org/downloads/release/python-366/).
2. Install [Python 3.5](https://www.python.org/downloads/release/python-356/).
3. Install [Python 2.7](https://www.python.org/downloads/release/python-2715/).

For these installations, "Add Python to path" does **not** need to be checked.

#### Build the Python wheel

1. Enter `pushd ` into the command prompt, open VMware Shared Folders in the
   file browser, drag the autopy repo directory to the prompt, and hit enter.
2. Run `call scripts/windows.cmd`. This will build and validate a binary wheel
   for each of the above Python versions.
3. Alternatively, if just installing from source locally, run `py -3.7 setup.py
   build install`, where `3.7` is your local Python version.

<span id="f1">[1]</span>: Instructions from
[here](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html#installing-rustup-on-windows).
[↩](#a1)
