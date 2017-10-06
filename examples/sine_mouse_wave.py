import autopy
import math
import random
import time


TWO_PI = math.pi * 2.0
def sine_mouse_wave():
    """
    Moves mouse in a sine wave from the left edge of the screen to the right.
    """
    width, height = autopy.screen.size()
    height /= 2
    height -= 10 # Stay in the screen bounds.

    for x in xrange(int(width)):
        y = round(height * math.sin((TWO_PI * x) / width) + height)
        autopy.mouse.move(float(x), float(y))
        time.sleep(random.uniform(0.001, 0.003))

sine_mouse_wave()
