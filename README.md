[![PyPI version](https://pypip.in/version/autopy/badge.svg)](https://pypi.python.org/pypi/autopy/)

AutoPy Introduction and Tutorial
=================================

## Introduction

AutoPy is a simple, cross-platform GUI automation library for Python. It
includes functions for controlling the keyboard and mouse, finding colors and
bitmaps on-screen, and displaying alerts.

Currently supported on macOS, Windows, and X11 with the XTest extension.

## Getting Started

### Requirements

* Python 2.7, or Python 3.5 and up.
* Rust 1.23.0-nightly 2018-05-01 or later (unless using a binary wheel
  distribution).
* macOS 10.6 and up.
* Windows 7 and up.
* X11 with the XTest extension.

### Installation

First, see if a binary wheel is available for your machine by running:

    $ pip install -U autopy

If that fails, install [rustup](https://rustup.rs) and then run:

    $ rustup default nightly
    $ pip install -U setuptools-rust
    $ pip install -U autopy

Another option is to compile from the latest source on the GitHub repository:

    $ git clone git://github.com/autopilot-rs/autopy-rs.git
    $ cd autopy
    $ rustup default nightly
    $ pip install -r requirements.txt
    $ python setup.py build
    # python setup.py install

Additional instructions for installing from source on Windows are available [here](https://github.com/autopilot-rs/autopy/blob/master/scripts/windows-setup.md).

### Hello World

The following is the source for a "hello world" script in autopy. Running this
code will cause an alert dialog to appear on every major platform:

```python
import autopy
def hello_world():
    autopy.alert.alert("Hello, world")
hello_world()
```

![Cross platform alerts](https://github.com/msanders/autopy/raw/gh-pages/alerts.png)

## Tutorials

### Controlling the Mouse

AutoPy includes a number of functions for controlling the mouse. For a full
list, consult the [API
Reference](https://www.autopy.org/documentation/api-reference/mouse.html). E.g.,
to immediately "teleport" the mouse to the top left corner of the screen:

	>>> import autopy
	>>> autopy.mouse.move(1, 1)

To move the mouse a bit more realistically, we could use:

	>>> import autopy
	>>> autopy.mouse.smooth_move(1, 1)

Even better, we could write our own function to move the mouse across the screen
as a sine wave:

```python
import autopy
import math
import time
import random
import sys

if sys.version_info.major == 3:
    xrange = range

TWO_PI = math.pi * 2.0


def sine_mouse_wave():
    """
    Moves the mouse in a sine wave from the left edge of
    the screen to the right.
    """
    width, height = autopy.screen.size()
    height /= 2
    height -= 10  # Stay in the screen bounds.

    for x in xrange(int(width)):
        y = int(height * math.sin((TWO_PI * x) / width) + height)
        autopy.mouse.move(x, y)
        time.sleep(random.uniform(0.001, 0.003))


sine_mouse_wave()
```

<a href="https://www.autopy.org/documentation/sine-wave"><img src="https://github.com/msanders/autopy/raw/gh-pages/sine-move-mouse-thumbnail.jpg" alt="Demonstration video"/></a>

### Working with Bitmaps

All of autopy's bitmap routines can be found in the module `autopy.bitmap`. A
useful way to explore autopy is to use Python's built-in `help()` function, for
example in `help(autopy.bitmap.Bitmap)`. AutoPy's functions are documented with
descriptive docstrings, so this should show a nice overview.

	>>> import autopy
	>>> autopy.bitmap.capture_screen()
	<Bitmap object at 0x12278>

This takes a screenshot of the main screen, copies it to a bitmap, displays its
memory address, and then immediately destroys it. Let's do something more
useful, like look at its pixel data:

	>>> import autopy
	>>> autopy.bitmap.capture_screen().get_color(1, 1)
	15921906

AutoPy uses a coordinate system with its origin starting at the top-left, so
this should return the color of pixel at the top-left corner of the screen. The
number shown looks a bit unrecognizable, but we can format it with Python's
built-in `hex` function:

	>>> import autopy
	>>> hex(autopy.bitmap.capture_screen().get_color(1, 1))
	'0xF2F2F2'

Alternatively, we can use:


	>>> import autopy
	>>> autopy.color.hex_to_rgb(autopy.screen.get_color(1, 1))
	(242, 242, 242)

which converts that hex value to a tuple of `(r, g, b)` values. (Note that
`autopy.screen.get_color()`, used here, is merely a more convenient and
efficient version of `autopy.bitmap.capture_screen().get_color()`.)

To save the screen capture to a file, we can use:

	>>> import autopy
	>>> autopy.bitmap.capture_screen().save('screengrab.png')

The filetype is either parsed automatically from the filename, or given as an
optional parameter. Currently only jpeg and png files are supported.

	>>> import autopy
	>>> autopy.bitmap.Bitmap.open('needle.png')
	<Bitmap object at 0x1001d5378>

Aside from analyzing a bitmap's pixel data, the main use for loading a bitmap is
finding it on the screen or inside another bitmap. For example, the following
prints the coordinates of the first image found in a bitmap (scanned from left
to right, top to bottom):

```python
import autopy


def find_image_example():
    needle = autopy.bitmap.Bitmap.open('needle.png')
    haystack = autopy.bitmap.Bitmap.open('haystack.png')

    pos = haystack.find_bitmap(needle)
    if pos:
        print("Found needle at: %s" % str(pos))

find_image_example()
```

## Projects using AutoPy

- [AutoPyDriverServer](https://github.com/daluu/autopydriverserver), AutoPy
  through WebDriver or a webdriver-compatible server.
- [guibot](https://github.com/intra2net/guibot), A tool for GUI automation using
  a variety of computer vision and desktop control backends.
- [spynner](https://github.com/makinacorpus/spynner), Programmatic web browsing
  module with AJAX support for Python.
- [SUMO](https://github.com/eclipse/sumo), An open source, highly portable,
  microscopic and continuous road traffic simulation package designed to handle
  large road networks.

## API Reference

Hope you enjoy using autopy! For a more in depth overview, see the [API
Reference](https://www.autopy.org/documentation/api-reference/).

