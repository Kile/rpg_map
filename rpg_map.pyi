# This file is automatically generated by pyo3_stub_gen
# ruff: noqa: E501, F401

import builtins
import typing
from enum import Enum, auto

class Map:
    r"""
    A class representing a map.
    
    Parameters
    ----------
    bytes : List[int]
        The bytes of the image.
    width : int
        The width of the image.
    height : int
        The height of the image.
    grid_size : int
        The size of a single box in the grid defining how many map revealing points the map has.
        To see the grid visually, use the `with_grid` method.
    map_type : MapType
        The type of the map. Can be Hidden, Limited or Full.
    unlocked : List[Tuple[int, int]]
        The points that are unlocked on the map.
    special_points : List[Tuple[int, int]]
        The special points on the map. Used to draw the path.
    obstacles : List[List[List[Tuple[int, int]]]]
        The obstacles on the map. Used to draw the path.
    background : Optional[List[int]]
    
    Attributes
    ----------
    width : int
        The width of the map.
    height : int
        The height of the map.
    unlocked : List[Tuple[int, int]]
        The points that are unlocked on the map.
    """
    width: builtins.int
    height: builtins.int
    unlocked: builtins.list[tuple[builtins.int, builtins.int]]
    def __new__(cls,bytes:typing.Sequence[builtins.int], width:builtins.int, height:builtins.int, grid_size:builtins.int, map_type:MapType=..., unlocked:typing.Sequence[tuple[builtins.int, builtins.int]]=[], special_points:typing.Sequence[tuple[builtins.int, builtins.int]]=[], obstacles:typing.Sequence[typing.Sequence[tuple[builtins.int, builtins.int]]]=[]): ...
    @staticmethod
    def draw_background(bytes:typing.Sequence[builtins.int], background:typing.Sequence[builtins.int]) -> builtins.list[builtins.int]:
        r"""
        Draws the background image at every transparent pixel
        if the background is set
        
        Parameters
        ----------
        bytes : List[int]
            The bytes of the image.
        background : Optional[List[int]]
            The bytes of the background of the image.
        """
        ...

    def with_dot(self, x:builtins.int, y:builtins.int, color:typing.Sequence[builtins.int], radius:builtins.int) -> Map:
        r"""
        Adds a dot do be drawn on the map when :func:`Map.full_image`, :func:`Map.masked_image` or :func:`Map.get_bits` is called
        
        Parameters
        ----------
        x : int
            The x coordinate of the dot.
        y : int
            The y coordinate of the dot.
        color : Tuple[int, int, int, int]
            The color of the dot.
        radius : int
            The radius of the dot.
        
        Returns
        -------
        Map
            The map with the dot.
        """
        ...

    def with_grid(self) -> Map:
        r"""
        If called, a grid is drawn on the map when :func:`Map.full_image`, :func:`Map.masked_image` or :func:`Map.get_bits` is called
        """
        ...

    def with_obstacles(self) -> Map:
        r"""
        If called, the obstacles are drawn on the map when :func:`Map.full_image`, :func:`Map.masked_image` or :func:`Map.get_bits` is called
        """
        ...

    def unlock_point_from_coordinates(self, x:builtins.int, y:builtins.int) -> builtins.bool:
        r"""
        Takes in a coordinate, if it is close to an "unlocked" grid point it will unlock it and return true, if the point is already unlocked it will return false
        
        Parameters
        ----------
        x : int
            The x coordinate of the point to unlock.
        y : int
            The y coordinate of the point to unlock.
        
        Returns
        -------
        bool
            True if the point was unlocked, False otherwise (already unlocked).
        """
        ...

    def draw_path(self, travel:Travel, percentage:builtins.float, line_width:builtins.int, path_type:PathStyle=..., display_style:PathDisplayType=..., progress_display_type:PathProgressDisplayType=...) -> builtins.list[builtins.int]:
        r"""
        Draws the path from :func:`Travel.computed_path` on the image.
        
        Parameters
        ----------
        travel : Travel
            The travel object containing the path to draw.
        percentage : float
            The percentage of the path to draw. 0.0 to 1.0.
        line_width : int
            The width of the line to draw in pixels. Note that if the line has an outline the width will be this +2px
        path_type : PathStyle
            The type of path to draw. Can be Solid, Dotted, SolidWithOutline or DottedWithOutline.
        path_display : PathDisplayType
            The type of path display to use. Can be BelowMask or AboveMask.
        
        Returns
        -------
        List[int]
            The bytes of the image with the path drawn.
        """
        ...

    def full_image(self) -> builtins.list[builtins.int]:
        r"""
        Returns the full image. If specified, draws the grid, obstacles, and dots.
        
        Returns
        -------
        List[int]
           The bytes of the image with the grid, obstacles, and dots drawn.
        """
        ...

    def masked_image(self) -> builtins.list[builtins.int]:
        r"""
        Returns the masked image. If specified, draws the grid, obstacles, and dots.
        
        Returns
        -------
        List[int]
          The bytes of the image with the grid, obstacles, and dots drawn.
        """
        ...

    def get_bits(self) -> builtins.list[builtins.int]:
        r"""
        The main method to get the image bytes.
        Respects the map type and draws the grid, obstacles, and dots if specified.
        
        Returns
        -------
        List[int]
          The bytes of the image with the grid, obstacles, and dots drawn.
        """
        ...


class Travel:
    r"""
    A class representing a travel from one point to another on a map.
    This class contains the shortest path from point A to point B on the map.
    It uses the A* algorithm to find the path.
    
    Parameters
    ----------
    map : Map
       The map to travel on.
    current_location : tuple[int, int]
       The current location of the traveler. Given as a tuple of (x, y) coordinates.
    destination : tuple[int, int]
       The destination of the traveler. Given as a tuple of (x, y) coordinates.
        
    Attributes
    ---------
    map : Map
       The map to travel on.
    computed_path : list[PathPoint]
       The computed path from the current location to the destination.
    """
    def __new__(cls,map:Map, current_location:tuple[builtins.int, builtins.int], destination:tuple[builtins.int, builtins.int]): ...
    @staticmethod
    def dbg_map(map:Map) -> builtins.list[builtins.int]:
        r"""
        Displays the map in a black and white view where white are the
        obstacles and black are the free spaces. This is to debug if
        a fault is with the pathfinding algorithm or the map reduction
        algorithm.
        
        Parameters
        ---------
        map : Map
          The map to display the black and white view of.
        
        Returns
        -------
        list[int]
          A list of bytes representing the black and white view of the map.
        """
        ...


class MapType(Enum):
    r"""
    The reveal type of the map.
    
    Attributes
    ---------
    Hidden
       The map reveals only the last entry in the unlocked points.
    Limited
       The map reveals all the unlocked points.
    Full
       The map reveals all the points.
    """
    Hidden = auto()
    Limited = auto()
    Full = auto()

class PathDisplayType(Enum):
    r"""
    The way of how to display the path.
    
    Attributes
    ---------
    BelowMask
      The path is always drawn below the mask.
    AboveMask
      The path is always drawn above the mask.
    """
    BelowMask = auto()
    AboveMask = auto()

class PathProgressDisplayType(Enum):
    r"""
    The type of how to display path progress.
    
    Attributes
    ---------
    Remaining
      The path is drawn from the current position to the destination.
    Travelled
      The path is drawn from the start to the current position.
    Progress
      The path is drawn from the start to the destination. The path already travelled is converted to greyscale.
    """
    Remaining = auto()
    Travelled = auto()
    Progress = auto()

class PathStyle(Enum):
    r"""
    The style of the path.
    
    Attributes
    ---------
    Debug
       The path is drawn in debug mode, only a 1px line is drawn.
    Solid
       The path is drawn as a solid line.
    Dotted
       The path is drawn as a dotted line.
    SolidWithOutline
       The path is drawn as a solid line with an outline.
    DottedWithOutline
       The path is drawn as a dotted line with an outline.
    """
    Debug = auto()
    Solid = auto()
    Dotted = auto()
    SolidWithOutline = auto()
    DottedWithOutline = auto()

