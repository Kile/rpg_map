use pyo3::prelude::pyclass;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[pyclass]
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub struct PathPoint {
    pub x: u32,
    pub y: u32,
}

impl PathPoint {
    pub fn from_tuple(t: (u32, u32)) -> Self {
        PathPoint { x: t.0, y: t.1 }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    x: u32,
    y: u32,
    cost: u32,
    heuristic: u32,
    parent: Option<(u32, u32)>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn astar(grid: &[Vec<u8>]) -> Option<Vec<PathPoint>> {
    let rows = grid.len() as u32;
    if rows == 0 {
        return None;
    }
    let cols = grid[0].len() as u32;

    let mut start = None;
    let mut end = None;

    for y in 0..rows {
        for x in 0..cols {
            match grid[y as usize][x as usize] {
                2 => start = Some((x, y)),
                3 => end = Some((x, y)),
                _ => {}
            }
        }
    }

    let (start_x, start_y) = start?;
    let (end_x, end_y) = end?;

    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();

    g_score.insert((start_x, start_y), 0);

    open_set.push(Node {
        x: start_x,
        y: start_y,
        cost: 0,
        heuristic: heuristic(start_x, start_y, end_x, end_y),
        parent: None,
    });

    while let Some(current) = open_set.pop() {
        if current.x == end_x && current.y == end_y {
            return reconstruct_path(came_from, (end_x, end_y));
        }

        let neighbors = get_neighbors(current.x, current.y, grid);

        for (nx, ny) in neighbors {
            let tentative_g_score = g_score.get(&(current.x, current.y)).unwrap_or(&u32::MAX) + 1;

            if tentative_g_score < *g_score.get(&(nx, ny)).unwrap_or(&u32::MAX) {
                came_from.insert((nx, ny), (current.x, current.y));
                g_score.insert((nx, ny), tentative_g_score);
                open_set.push(Node {
                    x: nx,
                    y: ny,
                    cost: tentative_g_score,
                    heuristic: heuristic(nx, ny, end_x, end_y),
                    parent: Some((current.x, current.y)),
                });
            }
        }
    }

    None
}

fn heuristic(x1: u32, y1: u32, x2: u32, y2: u32) -> u32 {
    (x1.abs_diff(x2)) + (y1.abs_diff(y2))
}

fn get_neighbors(x: u32, y: u32, grid: &[Vec<u8>]) -> Vec<(u32, u32)> {
    let rows = grid.len() as u32;
    let cols = grid[0].len() as u32;
    let mut neighbors = Vec::new();

    let possible_moves: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for (dx, dy) in possible_moves {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if nx >= 0 && nx < cols as i32 && ny >= 0 && ny < rows as i32 {
            let nx_u = nx as u32;
            let ny_u = ny as u32;

            if grid[ny_u as usize][nx_u as usize] != 1 {
                neighbors.push((nx_u, ny_u));
            }
        }
    }

    neighbors
}

fn reconstruct_path(
    came_from: HashMap<(u32, u32), (u32, u32)>,
    current: (u32, u32),
) -> Option<Vec<PathPoint>> {
    let mut total_path = vec![PathPoint::from_tuple(current)];
    let mut current = current;

    while let Some(&parent) = came_from.get(&current) {
        total_path.push(PathPoint::from_tuple(parent));
        current = parent;
    }

    total_path.reverse();
    Some(total_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astar_diagonal_path() {
        let grid = vec![vec![2, 0, 0], vec![0, 0, 0], vec![0, 0, 3]];

        let path = astar(&grid).unwrap();
        assert_eq!(
            path,
            vec![
                PathPoint::from_tuple((0, 0)),
                PathPoint::from_tuple((1, 1)),
                PathPoint::from_tuple((2, 2))
            ]
        );
    }

    #[test]
    fn test_astar_example() {
        let grid = vec![
            vec![0, 0, 0, 1, 0],
            vec![1, 1, 0, 1, 0],
            vec![2, 0, 0, 0, 3],
            vec![1, 0, 1, 1, 0],
            vec![0, 0, 0, 0, 0],
        ];

        let path = astar(&grid).unwrap();
        assert_eq!(
            path,
            vec![
                PathPoint::from_tuple((0, 2)),
                PathPoint::from_tuple((1, 2)),
                PathPoint::from_tuple((2, 2)),
                PathPoint::from_tuple((3, 2)),
                PathPoint::from_tuple((4, 2))
            ]
        );
    }
}
