bitmap — autopy module for working with bitmaps
===============================================

.. automodule:: autopy.bitmap

Bitmap Object Methods
-----------------------------
.. autoclass:: Bitmap
   :member-order: bysource

   .. automethod:: save(path: str, format: str=None)
   .. automethod:: copy_to_pasteboard()
   .. automethod:: point_in_bounds(x: float, y: float) -> bool
   .. automethod:: rect_in_bounds(rect: Tuple[Tuple[float, float], Tuple[float, float]]) -> bool
   .. automethod:: open(path: str) -> Bitmap
   .. automethod:: get_color(x: float, y: float) -> Tuple[int, int, int]
   .. automethod:: find_color(color: Tuple[int, int, int], tolerance: float=None, rect: Tuple[Tuple[float, float], Tuple[float, float]]=None, start_point: Tuple[float, float]=None) -> Tuple[float, float]
   .. automethod:: find_every_color(color: Tuple[int, int, int], tolerance: float=None, rect: Tuple[Tuple[float, float], Tuple[float, float]]=None, start_point: Tuple[float, float]=None) -> List[Tuple[float, float]]
   .. automethod:: count_of_color(color: Tuple[int, int, int], tolerance: float=None, rect: Tuple[Tuple[float, float], Tuple[float, float]]=None, start_point: Tuple[float, float]=None) -> int
   .. automethod:: find_bitmap(needle: Bitmap, tolerance: float=None, rect: Tuple[Tuple[float, float], Tuple[float, float]]=None, start_point: Tuple[float, float]=None) -> Tuple[float, float]
    .. automethod:: find_every_bitmap(needle: Bitmap, tolerance: float=None, rect: Tuple[Tuple[float, float], Tuple[float, float]]=None, start_point: Tuple[float, float]=None) -> [Tuple[float, float]]
    .. automethod:: count_of_bitmap(needle: Bitmap, tolerance: float=None, rect: Tuple[Tuple[float, float], Tuple[float, float]]=None, start_point: Tuple[float, float]=None) -> int
   .. automethod:: cropped(rect: Tuple[Tuple[float, float], Tuple[float, float]]) -> Bitmap

Functions
-----------------------------
 .. automodule:: autopy.bitmap

   .. autofunction:: capture_screen(rect: Tuple[Tuple[float, float], Tuple[float, float]]) -> autopy.bitmap.Bitmap
