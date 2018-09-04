## Windows Packaging Instructions

The steps to get packaging up and running on Windows are difficult to keep
consistent so I've documented them here. This may also be helpful for others
installing from the latest source on the GitHub repository.

Note: I am currently using VMWare Fusion so a few steps are specific to that
(e.g. shared folders).

#### Initial setup

1. [Download the Windows 10
   ISO](https://www.microsoft.com/en-us/software-download/windows10). Use
   separate VMs for 32-bit and 64-bit versions.
2. Install [Python 3.7](https://www.python.org/downloads/release/python-370/)
   and ensure "Add Python 3.7 to path" is checked.
3. Install
   [rustup](https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe)
   with the default options <sup id="a1">[1](#f1)</sup>. This can take some
   time.
4. Download and run the [Build Tools for Visual
   Studio](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2017)
   installer. Check the "Visual C++ build tools" workload and click install.
   This may also take some time.
4. Enable Shared Folders from VMware and share the autopy repo directory.
   
#### Update and confirm everything's working

1. Open Command Prompt and run `py -3.7` to confirm Python is setup correctly.
3. Run `rustup install nightly`.
4. Run `rustup default nightly`.

#### Install adjacent Python versions

1. Install [Python 3.6](https://www.python.org/downloads/release/python-366/).
2. Install [Python 3.5](https://www.python.org/downloads/release/python-356/).
3. Install [Python 2.7](https://www.python.org/downloads/release/python-2715/).

For these installations, "Add Python to path" does **not** need to be checked.

After each installation, run:

```
py -3.7 -m pip install -U pip wheel
```

(where `3.7` is the corresponding version number.)

#### Build the Python wheel

1. Enter `pushd ` into command prompt, open VMWare Shared Folders in file
   browser, drag the autopy repo directory to the prompt, and hit enter.
2. Run `py -3.7 -m pip install -r requirements.txt`.
3. Run `py -3.7 setup.py build` to confirm the module builds correctly.
4. Then run `py -3.7 setup.py bdist_wheel` to build the binary wheel.
5. Then run the following to ensure integrity of the binary wheel:

```
py -3.7 -m pip uninstall -y autopy
py -3.7 -m pip install autopy --no-index -f dist
pushd %Temp%
py -3.7 -c "import autopy"
popd
```

Repeat this with each Python version (currently 2.7, 3.6, and 3.7).

<span id="f1">[1]</span>: Instructions from
[here](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html#installing-rustup-on-windows).
[↩](#a1)
