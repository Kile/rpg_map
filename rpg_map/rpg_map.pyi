from typing import List, Tuple, Optional

# if TYPE_CHECKING:
#     from rpg_map.travel import Travel

class MapType:
    Hidden: int = 0
    Limited: int = 1
    Full: int = 2

class PathPoint:
    x: int
    y: int

class PathDisplayType:
    def Revealing(self) -> "PathDisplayType": ...
    def BelowMask(self) -> "PathDisplayType": ...
    def AboveMask(self) -> "PathDisplayType": ...

class PathProgressDisplayType:
    def Remaining(self) -> "PathProgressDisplayType": ...
    def Travelled(self) -> "PathProgressDisplayType": ...
    def Progress(self) -> "PathProgressDisplayType": ...

class PathStyle:
    def Debug(self) -> "PathStyle": ...
    def Dotted(self, arg0: Tuple[int, int, int, int]) -> "PathStyle": ...
    def Solid(self, arg0: Tuple[int, int, int, int]) -> "PathStyle": ...
    def SolidWithOutline(
        self, arg0: Tuple[int, int, int, int], arg1: Tuple[int, int, int, int]
    ) -> "PathStyle": ...
    def DottedWithOutline(
        self, arg0: Tuple[int, int, int, int], arg1: Tuple[int, int, int, int]
    ) -> "PathStyle": ...

class Map:
    """
    A class representing a map.
    """

    width: int
    height: int
    unlocked: List[Tuple[int, int]]

    @staticmethod
    def calculate_grid_points(
        width: int, height: int, grid_size: int
    ) -> List[Tuple[int, int]]: ...
    def __init__(
        self,
        bytes: List[int],
        width: int,
        height: int,
        grid_size: int,
        map_type: MapType = MapType.Full,
        unlocked: List[Tuple[int, int]] = [],
        special_points: List[Tuple[int, int]] = [],
        obstacles: List[List[List[Tuple[int, int]]]] = [],
        background: Optional[List[int]] = None,
    ) -> None: ...
    """
    Parameters
    ----------
    bytes : List[int]
        The bytes of the image.
    width : int
        The width of the image.
    height : int
        The height of the image.
    grid_size : int
        The size of the grid.
    map_type : MapType
        The type of the map. Can be Hidden, Limited or Full.
    unlocked : List[Tuple[int, int]]
        The points that are unlocked on the map.
    special_points : List[Tuple[int, int]]
        The special points on the map. Used to draw the path.
    obstacles : List[List[List[Tuple[int, int]]]]
        The obstacles on the map. Used to draw the path.
    background : Optional[List[int]]
        The background of the map. Used to draw the path.
    """

    @staticmethod
    def draw_background(
        bytes: List[int], background: Optional[List[int]]
    ) -> List[int]:
        """
        Draw the background on the map.

        Parameters
        ----------
        bytes : List[int]
            The bytes of the image.
        background : Optional[List[int]]
            The bytes of the background of the image.
        """
        ...

    def with_dot(
        self, x: int, y: int, color: Tuple[int, int, int, int], radius: int
    ) -> "Map":
        """
        Will draw a dot when get_bits is called.

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

    def with_grid(self) -> "Map":
        """
        Will draw a grid when get_bits is called.
        """
        ...

    def with_obstacles(self) -> "Map":
        """
        Will draw the obstacles when get_bits is called.
        """
        ...

    def unlock_point_from_coordinates(self, x: int, y: int) -> bool:
        """
        Unlock a point from its coordinates.

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

    def draw_path(
        self,
        travel: "Travel",
        percentage: float,
        line_width: int,
        path_type: PathStyle = PathStyle.DottedWithOutline(
            (255, 0, 0, 255), (255, 255, 255, 255)
        ),
        path_display: PathDisplayType = PathDisplayType.Revealing(),
    ) -> List[int]:
        """
        Draws the path from Travel.computed_path on the image.

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
            The type of path display to use. Can be Revealing, BelowMask or AboveMask.

        Returns
        -------
        List[int]
            The bytes of the image with the path drawn.
        """
        ...

    def full_image(self) -> List[int]:
        """
        Returns the full image. If specified, draws the grid, obstacles, and dots.
        """
        ...

    def masked_image(self) -> List[int]:
        """
        Returns the masked image, using the create_mask method.
        Uses full_image to get the full image.
        """
        ...

    def get_bits(self) -> List[int]:
        """
        Get the bits of the image by calling masked_image or full_image
        depending on the map type.
        """
        ...

class Travel:
    """
    A class representing a travel from one point to another on a map.
    This class contains the shortest path from point A to point B on the map.
    It uses the A* algorithm to find the path.
    """
    
    map: "Map"
    computed_path: List["PathPoint"]

    def __init__(
        self,
        map: "Map",
        current_location: Tuple[int, int],
        destination: Tuple[int, int],
    ) -> None: ...
    """
    Parameters
    ----------
    map : Map
        The map to use.
    current_location : Tuple[int, int]
        The current location of the player.
    destination : Tuple[int, int]
        The destination of the player.
    """

    @staticmethod
    def dbg_map(map: "Map") -> List[int]:
        """
        Displays the map in a black and white view where white are the
        obstacles and black are the free spaces. This is to debug if
        a fault is with the pathfinding algorithm or the map reduction
        algorithm.
        """
        ...