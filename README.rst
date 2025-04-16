rpg_map
=======

.. image:: https://img.shields.io/pypi/v/rpg_map.svg
    :target: https://pypi.org/project/rpg_map/
    :alt: PyPI version

.. image:: https://readthedocs.org/projects/rpg-map/badge/?version=latest
    :target: https://rpg-map.readthedocs.io/en/latest/
    :alt: Documentation Status

A fast, zero-dependency Python library for visualizing exploration and movement across large pixel-based maps. Built in Rust for speed and efficiency, ``rpg_map`` is perfect for turn-based or real-time RPGs, strategy games, and D&D-style map reveals.

Key Features
------------

- Hides unexplored areas of the map
- Reveals areas as they're explored or passed through
- Draws travel paths using A* pathfinding
- Fast and lightweight with zero Python dependencies
- Operates directly on pixel data
- Customizable visual styles and highlight zones
- Includes examples using static image processing and Pygame

Install
-------

Install via pip:

.. code:: bash

    pip install rpg-map

Documentation
-------------

Full documentation and examples available at: https://rpg-map.readthedocs.io/

How It Works
------------

The library uses step-by-step image processing to reveal and annotate the map. Here's an overview of the process:

1. **Draw Obstacles**  
   The ``Map`` class accepts an ``obstacles`` parameter which allows you to define N-sided polygons. These are rendered onto the map as solid barriers.

   .. image:: https://github.com/Kile/rpg_map/blob/master/assets/1.png?raw=true
      :width: 600

2. **Add Padding and Convert to Pathfinding Grid**  
   Obstacles and map edges are padded and the image is converted into a binary map (1 = path, 0 = obstacle) for pathfinding.

   .. image:: https://github.com/Kile/rpg_map/blob/master/assets/2.png?raw=true
      :width: 600

3. **Pathfinding with A\***  
   The library uses the A* algorithm to find the shortest path from point A to point B. The path is drawn on the map using a customizable style.

   .. image:: https://github.com/Kile/rpg_map/blob/master/assets/3.png?raw=true
      :width: 600

4. **Draw Dots**  
   Optional dots can be placed on the map (e.g., for points of interest, the player, markers).

   .. image:: https://github.com/Kile/rpg_map/blob/master/assets/4.png?raw=true
      :width: 600

5. **Divide into Grid Squares**  
   The image is divided into equal squares based on the ``grid_size`` parameter.

   .. image:: https://github.com/Kile/rpg_map/blob/master/assets/5.png?raw=true
      :width: 600

6. **Reveal Explored Areas**  
   A mask overlays the map. Areas near the travel path or manually unlocked via ``Map.unlock_point`` are revealed in circular zones.

   .. image:: https://github.com/Kile/rpg_map/blob/master/assets/6.png?raw=true
      :width: 600

7. **Fill Transparent Areas**  
   Any remaining transparent pixels are filled with a background layer.

   .. image:: https://github.com/Kile/rpg_map/blob/master/assets/7.png?raw=true
      :width: 600

8. **Final Map Output**  
   The completed map shows explored areas, paths, markers, and hidden regions yet to be discovered.

   .. image:: https://github.com/Kile/rpg_map/blob/master/assets/8.png?raw=true
      :width: 600

Advanced Features
-----------------

- You can define **special grid points** where the reveal radius is larger — perfect for cities or key landmarks.
- The library supports **tons of styles** for different themes and usecases.

   .. image:: https://github.com/Kile/rpg_map/blob/master/assets/9.png?raw=true
      :width: 300
   .. image:: https://github.com/Kile/rpg_map/blob/master/assets/10.png?raw=true
      :width: 300
   .. image:: https://github.com/Kile/rpg_map/blob/master/assets/11.png?raw=true
      :width: 300
   .. image:: https://github.com/Kile/rpg_map/blob/master/assets/12.png?raw=true
      :width: 300

Examples
--------

Check out these demos:

- `examples/static_poc.py <https://github.com/Kile/rpg_map/blob/master/examples/static_poc.py>`_ – Generate one image from your code
- `examples/pygame_poc <https://github.com/Kile/rpg_map/blob/master/examples/pygame_poc.py>`_ – Interactively do pathfinding to wherever you click


Contributing & Development
==========================

We welcome contributions and ideas! If you'd like to work on ``rpg_map`` locally, here's how to get started.

Set Up the Development Environment
----------------------------------

1. **Compile the Rust Extension Locally**

   Use ``maturin`` to build and install the Rust extension module in your local Python environment:

   .. code:: bash

      maturin develop --features "extension-module"

2. **Generate Python Typings** ( ``.pyi`` )

   The library includes a binary that auto-generates Python type stubs. Run it with:

   .. code:: bash

      cargo run --bin stub_gen --features stubgen

3. **Build the Documentation**

   The documentation uses Sphinx and can be built locally as follows:

   .. code:: bash

      python3 -venv env && source env/bin/activate
      cd docs
      pip3 install -r requirements.txt
      sphinx-apidoc -o source/ ../ -f
      make html

   The output will be available in the `docs/build/html/` directory.

Additional Notes
----------------

- The Rust project uses ``pyo3`` to create Python bindings — see ``Cargo.toml`` for feature flags and build options.
- Type hints are manually generated via the ``stub_gen`` tool, ensuring compatibility with type checkers and IDEs. Interestingly sphinx uses the docs defined in the Rust code though, the `pyi` file is only for IDE type hinting when using the library.

License
-------

MIT

