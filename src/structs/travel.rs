use crate::structs::path::{astar, PathPoint};
use core::panic;
use pyo3::prelude::*;
use std::vec;

use crate::structs::map::Map;
use geo::{Contains, Coord, LineString, Point, Polygon};

#[pyclass]
#[derive(Clone)]
pub struct Travel {
    pub map: Map,
    pub computed_path: Vec<PathPoint>,
}

/// Give all 1s a X px "buffer" of 1s around them
/// For efficiency reasons it will only do this if a 0
/// is within a 1px radius of the 1
fn buffer_edges(reduced: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let buffer_size = 5;
    let mut new = reduced.clone();
    // We do not want to mutate the original in the loop

    for y in 0..reduced.len() {
        for x in 0..reduced[y].len() {
            if reduced[y][x] == 0 {
                continue;
            }

            let mut buffer = false;
            for i in -1..2 {
                for j in -1..2 {
                    if y as i32 + i < 0
                        || y as i32 + i >= reduced.len() as i32
                        || x as i32 + j < 0
                        || x as i32 + j >= reduced[y].len() as i32
                    {
                        continue;
                    }

                    if reduced[(y as i32 + i) as usize][(x as i32 + j) as usize] == 0 {
                        buffer = true;
                        break;
                    }
                }
            }

            if buffer {
                for i in -buffer_size..(buffer_size + 1) {
                    for j in -buffer_size..(buffer_size + 1) {
                        if y as i32 + i < 0
                            || y as i32 + i >= reduced.len() as i32
                            || x as i32 + j < 0
                            || x as i32 + j >= reduced[y].len() as i32
                        {
                            continue;
                        }

                        new[(y as i32 + i) as usize][(x as i32 + j) as usize] = 1;
                    }
                }
            }
        }
    }

    new
}

/// Converts the image to a grid where 0 is a free space and 1 is an obstacle
pub fn image_to_grid(map: &mut Map) -> Vec<Vec<u8>> {
    let mut grid = vec![vec![0; (map.width) as usize]; (map.height) as usize];
    let binding: Vec<u8> = map.get_bits();
    for (i, byte) in binding.chunks_exact(4).enumerate() {
        let x = i % map.width as usize;
        let y = i / map.width as usize;
        let alpha = byte[3]; // Alpha channel
        if alpha == 0 {
            grid[y][x] = 1; // Transparent pixels -> Obstacle
        }
    }

    // Step 2: Process polygon obstacles
    let mut polygons: Vec<Polygon> = vec![];
    for obstacle in &map.obstacles {
        if obstacle.len() < 3 {
            continue; // Skip invalid polygons
        }
        let exterior = obstacle
            .iter()
            .map(|&coords| Coord {
                x: coords.0 as f64,
                y: coords.1 as f64,
            })
            .collect::<Vec<Coord>>();

        let polygon = Polygon::new(LineString::from(exterior), vec![]);
        polygons.push(polygon);

    }

    for y in 0..map.height {
        for x in 0..map.width {
            for polygon in &polygons {
                if polygon.contains(&Point::new(x as f64, y as f64)) {
                    grid[y as usize][x as usize] = 1; // Mark obstacle
                }
            }
        }
        
    }

    // Step 3: Buffer edges of obstacles
    grid = buffer_edges(grid);

    grid
}

#[pymethods]
impl Travel {
    #[new]
    pub fn new(
        mut map: Map,
        current_location: (u32, u32),
        destination: (u32, u32),
    ) -> PyResult<Travel> {
        // draw obstacles on the map
        let mut grid = image_to_grid(&mut map);

        // put in start and end
        grid[current_location.1 as usize][current_location.0 as usize] = 2;
        grid[destination.1 as usize][destination.0 as usize] = 3;

        match astar(&grid) {
            Some(path) => Ok(Travel {
                map,
                computed_path: path,
            }),
            None => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "No path found",
            )),
        }
    }

    /// Displays the map in a black and white view where white are the
    /// obstacles and black are the free spaces. This is to debug if
    /// a fault is with the pathfinding algorithm or the map reduction
    /// algorithm.
    #[staticmethod]
    pub fn dbg_map(mut map: Map) -> Vec<u8> {
        let grid = image_to_grid(&mut map);
        let mut long_map = vec![0; map.width as usize * map.height as usize * 4];
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let byte = match grid[y][x] {
                    0 => vec![255, 255, 255, 255],
                    1 => vec![0, 0, 0, 255],
                    2 => vec![0, 0, 255, 255],
                    3 => vec![255, 0, 0, 255],
                    _ => panic!("Invalid grid value"),
                };
                if y * map.width as usize + x + 4 >= long_map.len() {
                    println!("{:?}", byte);
                    continue;
                }
                long_map
                    [y * map.width as usize * 4 + x * 4..y * map.width as usize * 4 + x * 4 + 4]
                    .copy_from_slice(&byte);
            }
        }
        long_map
    }
}
