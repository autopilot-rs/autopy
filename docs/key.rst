key — autopy module for working with the keyboard
=================================================

.. currentmodule:: autopy.key
.. automodule:: autopy.key

Functions
-----------------------------
.. automodule:: autopy.key

   .. autofunction:: toggle(key: Any, down: bool, modifiers: List[Modifier])
   .. autofunction:: tap(key: Any, modifiers: List[Modifier])
   .. autofunction:: type_string(string: str, wpm: float=None)


Constants
---------

.. py:class:: Code

   :code: F1
   :code: F2
   :code: F3
   :code: F4
   :code: F5
   :code: F6
   :code: F7
   :code: F8
   :code: F9
   :code: F10
   :code: F11
   :code: F12
   :code: LEFT_ARROW
   :code: CONTROL
   :code: RIGHT_ARROW
   :code: DOWN_ARROW
   :code: END
   :code: UP_ARROW
   :code: PAGE_UP
   :code: ALT
   :code: RETURN
   :code: PAGE_DOWN
   :code: DELETE
   :code: HOME
   :code: ESCAPE
   :code: BACKSPACE

.. class:: Modifier

   :modifier: META
   :modifier: ALT
   :modifier: CONTROL
   :modifier: SHIFT
