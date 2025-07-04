from rpg_map import Travel, Map, MapType, PathStyle, PathProgressDisplayType, PathDisplayType
from PIL import Image

LOCAL_DIR = "../test_assets/map.png"
BACKGROUND_DIR = "../test_assets/background.png"
GRID_SIZE = 20
START, END = (198, 390), (172, 223)
START_X, START_Y = START

image = Image.open(LOCAL_DIR).convert("RGBA")
# get image bytes
image_bytes = list(image.tobytes())
background = Image.open(BACKGROUND_DIR).convert("RGBA")
# get background bytes
background_bytes = list(background.tobytes())
map = Map(
    image_bytes,
    image.size[0],
    image.size[1],
    GRID_SIZE,
    MapType.Full,
    obstacles=[[(160, 240), (134, 253), (234, 257), (208, 239)]],
)

def save_image_from_bits(bits, width, height, filename):
    """
    Save the given bits as an image.
    """
    image = Image.frombytes("RGBA", (width, height), bits)
    image.save("../assets/" + filename)

# First visually draw the obstacles
first_step = map.with_obstacles().get_bits()
save_image_from_bits(first_step, image.width, image.height, "1.png")

# Get the debug map for pathfinding
travel = Travel(map, START, END)
second_step = Travel.dbg_map(map)
save_image_from_bits(second_step, image.width, image.height, "2.png")

# Draw the path on the map
third_step = map.with_obstacles().draw_path(
    travel,
    1.0,
    2,
    PathStyle.DottedWithOutline((255, 0, 0, 255), (255, 255, 255, 255)),
)
save_image_from_bits(third_step, image.width, image.height, "3.png")

# Draw the dot at the start position and the path
fourth_step = map.with_obstacles().with_dot(
    START_X, START_Y, (255, 0, 0, 255), 4
).draw_path(
    travel,
    1.0,
    2,
    PathStyle.DottedWithOutline((255, 0, 0, 255), (255, 255, 255, 255)),
)
save_image_from_bits(fourth_step, image.width, image.height, "4.png")

# Draw the map with path, obstacles, dot and grid
fifth_step = map.with_obstacles().with_grid().with_dot(
    START_X, START_Y, (255, 0, 0, 255), 4
).draw_path(
    travel,
    1.0,
    2,
    PathStyle.DottedWithOutline((255, 0, 0, 255), (255, 255, 255, 255)),
)
save_image_from_bits(fifth_step, image.width, image.height, "5.png")

# Now overlay the mask over the map
map = Map(
    image_bytes,
    image.size[0],
    image.size[1],
    GRID_SIZE,
    MapType.Limited,
    obstacles=[[(160, 240), (134, 253), (234, 257), (208, 239)]],
)

sixth_step = map.with_obstacles().with_grid().with_dot(
    START_X, START_Y, (255, 0, 0, 255), 4
).draw_path(
    travel,
    1.0,
    2,
    PathStyle.DottedWithOutline((255, 0, 0, 255), (255, 255, 255, 255)),
)
save_image_from_bits(sixth_step, image.width, image.height, "6.png")

# Now include the background
seventh_step = Map.draw_background(
    map.with_obstacles().with_grid().with_dot(
        START_X, START_Y, (255, 0, 0, 255), 4
    ).draw_path(
        travel,
        1.0,
        2,
        PathStyle.DottedWithOutline((255, 0, 0, 255), (255, 255, 255, 255)),
    ),
    background_bytes
)
save_image_from_bits(seventh_step, image.width, image.height, "7.png")

# Finally, remove all debugging visuals (the grid and obstacles)
eighth_step = Map.draw_background(
    map.clear_extras().with_dot(START_X, START_Y, (255, 0, 0, 255), 4).draw_path(
        travel,
        1.0,
        2,
        PathStyle.DottedWithOutline((255, 0, 0, 255), (255, 255, 255, 255)),
    ),
    background_bytes
)
save_image_from_bits(eighth_step, image.width, image.height, "8.png")


# FURTHER EXAMPLES
GRID_SIZE = 20
START, END = (198, 390), (330,  512)
START_X, START_Y = START

# Below are 4 more examples of how to use the Map class with different configurations.

## Example 1: Using a hidden map with a travel path
example_1_map = Map(
    image_bytes,
    image.size[0],
    image.size[1],
    GRID_SIZE,
    MapType.Hidden,
)

travel = Travel(map, START, END)

path_bits = Map.draw_background(
    example_1_map.with_dot(START_X, START_Y, (255, 0, 0, 255), 4).draw_path(
        travel,
        0.0,
        2,
        PathStyle.Dotted((255, 0, 0, 255)),
        PathDisplayType.AboveMask,
        PathProgressDisplayType.Remaining,
    ),
    background_bytes
)

save_image_from_bits(path_bits, image.width, image.height, "9.png")

## Example 2: Using a revealing map with a different path style
example_2_map = Map(
    image_bytes,
    image.size[0],
    image.size[1],
    GRID_SIZE,
    MapType.Limited,
)
travel = Travel(map, START, END)
path_bits = Map.draw_background(
    example_2_map.with_dot(START_X, START_Y, (255, 0, 0, 255), 4).draw_path(
        travel,
        1.0,
        2,
        PathStyle.SolidWithOutline((255, 0, 0, 255), (255, 255, 255, 255)),
        PathDisplayType.BelowMask,
        PathProgressDisplayType.Travelled,
    ),
    background_bytes
)
save_image_from_bits(path_bits, image.width, image.height, "10.png")

## Example 3: Using a remaining style and a dot to indicate the current position half way through the path
example_3_map = Map(
    image_bytes,
    image.size[0],
    image.size[1],
    GRID_SIZE,
    MapType.Limited,
)
travel = Travel(map, START, END)
PROGRESS = 0.5  # Halfway through the path
current_coordinate = travel.computed_path[int((len(travel.computed_path)-1) * PROGRESS)]

path_bits = Map.draw_background(
    example_3_map.with_dot(current_coordinate.x, current_coordinate.y, (255, 0, 0, 255), 4).draw_path(
        travel,
        PROGRESS,
        2,
        PathStyle.DottedWithOutline((255, 0, 0, 255), (255, 255, 255, 255)),
        PathDisplayType.AboveMask,
        PathProgressDisplayType.Progress,
    ),
    background_bytes
)
save_image_from_bits(path_bits, image.width, image.height, "11.png")

## Example 4: not show traveled path, hide remaining path below map
example_4_map = Map(
    image_bytes,
    image.size[0],
    image.size[1],
    GRID_SIZE,
    MapType.Limited,
)
travel = Travel(map, START, END)
PROGRESS = 0.75  # Halfway through the path
current_coordinate = travel.computed_path[int((len(travel.computed_path)-1) * PROGRESS)]

path_bits = Map.draw_background(
    example_4_map.with_dot(current_coordinate.x, current_coordinate.y, (255, 0, 0, 255), 4).draw_path(
        travel,
        PROGRESS,
        2,
        PathStyle.DottedWithOutline((255, 0, 0, 255), (255, 255, 255, 255)),
        PathDisplayType.BelowMask,
        PathProgressDisplayType.Remaining,
    ),
    background_bytes
)
save_image_from_bits(path_bits, image.width, image.height, "12.png")