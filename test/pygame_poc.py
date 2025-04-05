import pygame
from PIL import Image
from rpg_map import Map, MapType, Travel, PathStyle, PathDisplayType, PathProgressDisplayType

# Constants
TILE_SIZE = 1
LOCAL_DIR = "test_assets/map.png"  # Replace with the path to your image
BACKGROUND_DIR = "test_assets/background.png"  # Replace with the path to your background image

# Load image and create map
image = Image.open(LOCAL_DIR).convert("RGBA")
image_bytes = list(image.tobytes())
background = Image.open(BACKGROUND_DIR).convert("RGBA")
background_bytes = list(background.tobytes())
map = Map(
    image_bytes, 
    image.size[0], 
    image.size[1], 
    20, 
    MapType.Full, 
    obstacles=[
        [
            (160, 240),
            (134, 253),
            (234, 257),
            (208, 239)
        ]
    ]
)

SCREEN_HEIGHT = image.size[1]
SCREEN_WIDTH = image.size[0]

# Initialize PyGame
pygame.init()
screen = pygame.display.set_mode((SCREEN_WIDTH, SCREEN_HEIGHT))
pygame.display.set_caption("Simple PyGame Map")

# Player position
player_x = SCREEN_WIDTH // 2
player_y = SCREEN_HEIGHT // 2

def update_map_with_bits(bits):
    for y in range(SCREEN_HEIGHT):
        for x in range(SCREEN_WIDTH):
            index = (y * SCREEN_WIDTH + x) * 4
            r, g, b, a = bits[index : index + 4]
            if a == 0:
                r, g, b = 255, 255, 255
            pygame.draw.rect(
                screen, (r, g, b), (x * TILE_SIZE, y * TILE_SIZE, TILE_SIZE, TILE_SIZE)
            )

    # Update the display
    pygame.display.flip()

map.unlock_point_from_coordinates(player_x, player_y)
update_map_with_bits(Map.draw_background(map.with_dot(player_x, player_y, (255, 0, 0, 255), 5).with_obstacles().get_bits(), background_bytes))
# Game loop
running = True
while running:
    map_x = None
    map_y = None
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

        # Handle mouse click
        if event.type == pygame.MOUSEBUTTONDOWN:
            # Get mouse position
            mouse_x, mouse_y = pygame.mouse.get_pos()
            # Convert mouse position to map coordinates
            map_x = mouse_x // TILE_SIZE
            map_y = mouse_y // TILE_SIZE
            print(f"Clicked at map coordinates: ({map_x}, {map_y})")

    if map_x is not None and map_y is not None:  # If the player clicked on the map
        # Clear the screen
        screen.fill((0, 0, 0))
        try:
            travel = Travel(map, (player_x, player_y), (map_x, map_y))
            map_bits = Map.draw_background(
                    map.with_dot(player_x, player_y, (255, 0, 0, 255), 5).draw_path(
                        travel,
                        0.5,
                        2,
                        PathStyle.DottedWithOutline((255, 0, 0, 255), (255, 255, 255, 255)),
                        PathDisplayType.Revealing(),
                        PathProgressDisplayType.Progress(),
                    ), 
                    background_bytes
                )
            update_map_with_bits(map_bits)
        except ValueError:
            print("No path found")

    # Cap the frame rate
    pygame.time.Clock().tick(30)

# Quit PyGame
pygame.quit()
