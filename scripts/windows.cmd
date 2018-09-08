rustup default nightly

py -3.7 setup.py clean
py -3.7 -m pip install -U pip wheel
py -3.7 -m pip install -r requirements.txt
py -3.7 setup.py bdist_wheel
py -3.7 -m pip uninstall -y autopy
py -3.7 -m pip install autopy --no-index -f dist
pushd %Temp%
py -3.7 -c "import autopy"
popd

py -3.6 setup.py clean
py -3.6 -m pip install -U pip wheel
py -3.6 -m pip install -r requirements.txt
py -3.6 setup.py bdist_wheel
py -3.6 -m pip uninstall -y autopy
py -3.6 -m pip install autopy --no-index -f dist
pushd %Temp%
py -3.6 -c "import autopy"
popd

py -2.7 setup.py clean
py -2.7 -m pip install -U pip wheel
py -2.7 -m pip install -r requirements.txt
set PYTHON_SYS_EXECUTABLE="C:\Python27\python.exe"
py -2.7 setup.py bdist_wheel
py -2.7 -m pip uninstall -y autopy
py -2.7 -m pip install autopy --no-index -f dist
pushd %Temp%
py -2.7 -c "import autopy"
popd
