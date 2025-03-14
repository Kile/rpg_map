use crate::structs::map::MapType;
use crate::structs::path::astar;
use core::panic;
use std::vec;
// use std::fs::File;
// use std::io::Write;

use crate::structs::map::Map;
use geo::{Contains, Coord, LineString, Point, Polygon};

pub struct Travel {
    pub map: Map,
    current_location: (u32, u32),
    destination: (u32, u32),
    draw_obstacles: bool,
    pub computed_path: Vec<(u32, u32)>,
}

impl Travel {
    pub fn new(mut map: Map, current_location: (u32, u32), destination: (u32, u32)) -> Travel {
        let mut grid = Self::image_to_grid(&mut map);

        // put in start and end
        grid[current_location.1 as usize][current_location.0 as usize] = 2;
        grid[destination.1 as usize][destination.0 as usize] = 3;

        let path = astar(&grid).unwrap();
        Travel {
            map,
            current_location,
            destination,
            draw_obstacles: false,
            computed_path: path,
        }
    }

    /// Displays the map in a black and white view where white are the
    /// obstacles and black are the free spaces. This is to debug if
    /// a fault is with the pathfinding algorithm or the map reduction
    /// algorithm.
    pub fn dbg_map(map: &mut Map) -> Vec<u8> {
        let grid = Self::image_to_grid(map);
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
        // Self::draw_obstacles(&mut long_map, map);
        long_map
    }

    /// Give all 1s a X px "buffer" of 1s around them
    /// For efficiency reasons it will only do this if a 0
    /// is within a 1px radius of the 1
    fn buffer_edges(reduced: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let buffer_size = 5;
        let mut new = reduced.clone(); // We do not want to mutate the original

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
        let binding: Vec<u8> = map.full_image();
        for (i, byte) in binding.chunks_exact(4).enumerate() {
            let x = i % map.width as usize;
            let y = i / map.width as usize;
            let alpha = byte[3]; // Alpha channel
            if alpha == 0 {
                grid[y][x] = 1; // Transparent pixels -> Obstacle
            }
        }

        // Step 2: Process polygon obstacles
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

            for y in 0..map.height {
                for x in 0..map.width {
                    let point = Point::new(x as f64, y as f64);
                    if polygon.contains(&point) {
                        grid[y as usize][x as usize] = 1; // Mark obstacle
                    }
                }
            }
        }

        // Step 3: Buffer edges of obstacles
        grid = Self::buffer_edges(grid);

        grid
    }

    fn draw_obstacles(bytes: &mut Vec<u8>, map: &Map) {
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

            for (i, ref mut chunk) in bytes.chunks_exact_mut(4).enumerate() {
                let mut pixel = [0; 4];
                pixel.copy_from_slice(chunk);
                let alpha = pixel[3];
                if alpha == 0 {
                    continue;
                }

                let x = i % map.width as usize;
                let y = i / map.width as usize;
                let point = Point::new(x as f64, y as f64);
                if polygon.contains(&point) {
                    chunk[0] = 255;
                    chunk[1] = 255;
                    chunk[2] = 255;
                    chunk[3] = 255;
                }
            }
        }
    }
}
