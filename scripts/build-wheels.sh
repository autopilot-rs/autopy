#!/usr/bin/env bash
# Modified from
# https://github.com/PyO3/setuptools-rust/blob/5adb24/example/build-wheels.sh
# https://pyo3.github.io/pyo3/guide/distribution.html#binary-wheel-distribution
set -e -x

yum install -y gpg libXtst libXtst-devel libXext libXext-devel

mkdir ~/rust-installer
curl -sL https://static.rust-lang.org/rustup.sh -o ~/rust-installer/rustup.sh
sh ~/rust-installer/rustup.sh --prefix=~/rust --channel=nightly -y --disable-sudo --date="2018-04-19"
export PATH="$HOME/rust/bin:$PATH"
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$HOME/rust/lib"

# Compile wheels
for PYBIN in /opt/python/cp{27,35,36}*/bin; do
    export PYTHON_SYS_EXECUTABLE="$PYBIN/python"
    export PYTHON_LIB=$(${PYBIN}/python -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))")
    export LIBRARY_PATH="$LIBRARY_PATH:$PYTHON_LIB"
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$PYTHON_LIB"
    "${PYBIN}/pip" install -U setuptools setuptools-rust wheel
    "${PYBIN}/pip" wheel /io/ -w /io/dist/
done

# Bundle external shared libraries into the wheels
for whl in /io/dist/*.whl; do
    auditwheel repair "$whl" -w /io/dist/
done

# Install packages and test
for PYBIN in /opt/python/cp{27,35,36,37}*/bin/; do
    "${PYBIN}/pip" install autopy --no-index -f /io/dist/
    "${PYBIN}/python" -c 'import autopy'
done
