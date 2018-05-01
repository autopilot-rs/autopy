#!/usr/bin/env bash
set -e -x

mkdir ~/rust-installer
curl -sL https://static.rust-lang.org/rustup.sh -o ~/rust-installer/rustup.sh
sh ~/rust-installer/rustup.sh --prefix=~/rust --spec=nightly -y --disable-sudo --date="2018-04-19"
export PATH="$HOME/rust/bin:$PATH"
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$HOME/rust/lib"

yum install -y libXtst libXtst-devel libXext libXext-devel

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
for PYBIN in /opt/python/cp{27,35,36}*/bin/; do
    "${PYBIN}/pip" install autopy --no-index -f /io/dist/
done
