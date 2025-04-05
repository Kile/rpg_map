# from .map import Map, MapType, PathStyle, PathPoint
# from .travel import Travel
# import os
# import importlib.util

# # Get the directory of the current file.
# dir_path = os.path.dirname(os.path.realpath(__file__))

# # Construct the path to the compiled module.
# module_path = os.path.join(dir_path, "rpg_map.cpython-313-darwin.so")
# # Create a module spec.
# spec = importlib.util.spec_from_file_location("rpg_map", module_path)

# # Create a module from the spec.
# module = importlib.util.module_from_spec(spec)

# # Execute the module.
# spec.loader.exec_module(module)

# # Import the Travel class from the module.
# Travel = module.Travel
# Map = module.Map
# MapType = module.MapType
from .rpg_map import Map, MapType, PathStyle, Travel, PathDisplayType, PathProgressDisplayType