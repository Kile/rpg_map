use crate::structs::travel::Travel;
use geo::{Contains, Coord, LineString, Point, Polygon};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapType {
    None,
    Limited,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathStyle {
    Debug,
    Dotted([u8; 4]),
    Solid([u8; 4]),
    SolidWithOutline([u8; 4], [u8; 4]),
    DottedWithOutline([u8; 4], [u8; 4]),
}

#[derive(Clone)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    bytes: Vec<u8>,
    grid_size: u32,
    unlocked: Vec<(u32, u32)>,
    grid_points: Vec<(u32, u32)>,
    special_points: Vec<(u32, u32)>,
    pub obstacles: Vec<Vec<(u32, u32)>>,
    pub map_type: MapType,
    draw_obstacles: bool,
    dots: Vec<(u32, u32, [u8; 4], u32)>, // x, y, color, radius
    with_grid: bool,
}

impl Map {
    /// Calculates the grid points of the map
    fn calculate_grid_points(width: u32, height: u32, grid_size: u32) -> Vec<(u32, u32)> {
        let mut grid_points = Vec::new();

        // calculate intersection points
        for y in (0..height).step_by(grid_size as usize) {
            for x in (0..width).step_by(grid_size as usize) {
                grid_points.push((x, y));
            }
        }

        // Calculate last intersection points row
        for x in (0..width).step_by(grid_size as usize) {
            grid_points.push((x, height - 1));
        }

        // Calculate last intersection points column
        for y in (0..height).step_by(grid_size as usize) {
            grid_points.push((width - 1, y));
        }

        grid_points
    }

    pub fn new(
        bytes: Vec<u8>,
        width: u32,
        height: u32,
        grid_size: u32,
        unlocked: Vec<(u32, u32)>,
        special_points: Vec<(u32, u32)>,
        obstacles: Vec<Vec<(u32, u32)>>,
        map_type: MapType,
    ) -> Self {
        let grid_points = Self::calculate_grid_points(width, height, grid_size);
        Map {
            width,
            height,
            bytes,
            grid_size,
            unlocked,
            grid_points,
            special_points,
            obstacles,
            map_type,
            draw_obstacles: false,
            dots: Vec::new(),
            with_grid: false,
        }
    }

    /// Checks if an intersection point is a special point
    fn is_special_point(&self, x: &u32, y: &u32) -> Option<&(u32, u32)> {
        self.special_points
            .iter()
            .find(|p| self.closest_to_point(**p) == self.closest_to_point((*x, *y)))
    }

    /// Adds a dot do be drawn on the map when full_image or masked_image is called
    pub fn with_dot(mut self, x: u32, y: u32, color: [u8; 4], radius: u32) -> Self {
        self.dots.push((x, y, color, radius));
        self
    }

    /// Signal you want a grid to be drawn on the map as well
    pub fn with_grid(mut self) -> Self {
        self.with_grid = true;
        self
    }

    /// Signal you want obstacles to be drawn on the map as well
    pub fn with_obstacles(mut self) -> Self {
        self.draw_obstacles = true;
        self
    }

    /// Finds the closest grid point with the given coordinates
    fn closest_to_point(&self, point: (u32, u32)) -> (u32, u32) {
        let mut min_dist = std::u32::MAX;
        let mut closest_point = (0, 0);

        for p in &self.grid_points {
            let dist = (p.0 as i32 - point.0 as i32).abs() as u32
                + (p.1 as i32 - point.1 as i32).abs() as u32;
            if dist < min_dist {
                min_dist = dist;
                closest_point = *p;
            }
        }

        closest_point
    }

    /// Takes in a coordinate, if it is close to an "unlocked" grid point
    /// it will unlock it and return true, if the point is already unlocked
    /// it will return false
    pub fn unlock_point_from_coordinates(&mut self, x: u32, y: u32) -> bool {
        let point = self.closest_to_point((x, y));
        if self.unlocked.contains(&point) {
            return false;
        }
        if self.map_type == MapType::Limited {
            self.unlocked.push(point);
        } else {
            self.unlocked = vec![point]; // Only one point for a limited map
        }
        true
    }

    /// Turns every pixel of the image black where the mask is not transparent
    fn put_mask_on_image(image: &mut Vec<u8>, mask: Vec<u8>) {
        for (i, chunk) in mask.chunks_exact(4).enumerate() {
            let a = chunk[3];
            if a != 0 {
                let index = i * 4;
                image[index..index + 4].copy_from_slice(&[0, 0, 0, 255]);
            }
        }
    }

    /// Helper function to check if four points form a square
    fn is_square(&mut self, points: &Vec<(u32, u32)>) -> bool {
        let mut sorted = points.clone();
        sorted.sort(); // Sort by x, then y

        let (x1, y1) = sorted[0];
        let (x4, y4) = sorted[3];

        let side1 = (x4 as i32 - x1 as i32).abs();
        let side2 = (y4 as i32 - y1 as i32).abs();

        side1 == side2 && side1 > 0 // Check if square with non-zero side length
    }

    /// Helper function to make everything inside a square transparent
    fn make_square_transparent(&mut self, mask: &mut Vec<u8>, points: Vec<(u32, u32)>) {
        let mut sorted = points.clone();
        sorted.sort(); // Sort by x, then y

        let (x_min, y_min) = sorted[0];
        let (x_max, y_max) = sorted[3];

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if x < self.width && y < self.height {
                    let index = (y * self.width + x) as usize * 4;
                    mask[index + 3] = 0; // Transparent
                }
            }
        }
    }

    /// Creates a mask for the map, taking into account the unlocked points
    /// and transparent background
    fn create_mask(&mut self) -> Vec<u8> {
        let mut mask = self.bytes.clone();

        for (mut cx, mut cy) in &self.unlocked {
            let radius: i32;
            if let Some((x, y)) = self.is_special_point(&cx, &cy) {
                cx = *x;
                cy = *y;
                radius = ((self.grid_size as f32) / 0.3) as i32;
            } else {
                radius = ((self.grid_size as f32) / 0.8) as i32;
            }
            let cx = cx as i32;
            let cy = cy as i32;
            let radius_sq = radius * radius;
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let x = cx + dx;
                    let y = cy + dy;

                    // Check if the point is within the circle radius
                    if dx * dx + dy * dy <= radius_sq {
                        // Ensure the pixel is within bounds
                        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                            let index = (y * self.width as i32 + x) as usize * 4;
                            mask[index + 3] = 0; // Make it transparent
                        }
                    }
                }
            }
        }

        let len = self.unlocked.len();
        for i in 0..len {
            let (x1, y1) = self.unlocked[i];

            for j in i + 1..len {
                let (x2, y2) = self.unlocked[j];

                if (x1 as i32 - x2 as i32).abs() > self.grid_size as i32
                    || (y1 as i32 - y2 as i32).abs() > self.grid_size as i32
                {
                    continue; // Skip if too far apart
                }

                for k in j + 1..len {
                    let (x3, y3) = self.unlocked[k];

                    if (x1 as i32 - x3 as i32).abs() > self.grid_size as i32
                        || (y1 as i32 - y3 as i32).abs() > self.grid_size as i32
                    {
                        continue;
                    }

                    for l in k + 1..len {
                        let (x4, y4) = self.unlocked[l];

                        if (x1 as i32 - x4 as i32).abs() > self.grid_size as i32
                            || (y1 as i32 - y4 as i32).abs() > self.grid_size as i32
                        {
                            continue;
                        }

                        // Check if these four points form a square
                        let points = vec![(x1, y1), (x2, y2), (x3, y3), (x4, y4)];
                        if self.is_square(&points) {
                            self.make_square_transparent(&mut mask, points);
                        }
                    }
                }
            }
        }

        mask
    }

    fn draw_dots(&mut self, bytes: &mut Vec<u8>) {
        for (x, y, color, radius) in &self.dots {
            println!("Drawing dot at ({}, {})", x, y);
            let radius_sq = (*radius as i32) * (*radius as i32);

            for dy in -(*radius as i32)..=*radius as i32 {
                for dx in -(*radius as i32)..=*radius as i32 {
                    let x = *x as i32 + dx;
                    let y = *y as i32 + dy;

                    // Check if the point is within the circle radius
                    if dx * dx + dy * dy <= radius_sq {
                        // Ensure the pixel is within bounds
                        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                            let index = (y * self.width as i32 + x) as usize * 4;
                            bytes[index..index + 4].copy_from_slice(color);
                        }
                    }
                }
            }
        }
    }

    /// Draws a grid on the image
    fn draw_with_grid(&mut self, image: &mut Vec<u8>) {
        if self.with_grid == false {
            return;
        }

        let grid_color = [255, 255, 255, 255];

        for y in (0..self.height).step_by(self.grid_size as usize) {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize * 4;
                image[index..index + 4].copy_from_slice(&grid_color);
            }
        }

        for x in (0..self.width).step_by(self.grid_size as usize) {
            for y in 0..self.height {
                let index = (y * self.width + x) as usize * 4;
                image[index..index + 4].copy_from_slice(&grid_color);
            }
        }

        // Draw the last line
        for x in 0..self.width {
            let index = ((self.height - 1) * self.width + x) as usize * 4;
            image[index..index + 4].copy_from_slice(&grid_color);
        }

        // Draw the last column
        for y in 0..self.height {
            let index = (y * self.width + (self.width - 1)) as usize * 4;
            image[index..index + 4].copy_from_slice(&grid_color);
        }
        // Draw the last intersection points in last row
        for x in (0..self.width).step_by(self.grid_size as usize) {
            let index = ((self.height - 1) * self.width + x) as usize * 4;
            image[index..index + 4].copy_from_slice(&[255, 0, 0, 255]);
        }

        // Draw the last intersection points in last column
        for y in (0..self.height).step_by(self.grid_size as usize) {
            let index = (y * self.width + (self.width - 1)) as usize * 4;
            image[index..index + 4].copy_from_slice(&[255, 0, 0, 255]);
        }
    }

    /// Draws all defined obstacles on the map. Useful for debugging.
    fn draw_obstacles(&mut self, bytes: &mut Vec<u8>) {
        for obstacle in &self.obstacles {
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

                let x = i % self.width as usize;
                let y = i / self.width as usize;
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

    /// Sets up the image for a path to be drawn on it
    fn setup_image_for_path(&mut self, ref path: &Vec<(u32, u32)>, percentage: f32) -> Vec<u8> {
        let last_coordinate = path[((path.len() - 1) as f32 * percentage) as usize];
        match self.map_type {
            MapType::None => {
                self.unlock_point_from_coordinates(last_coordinate.0, last_coordinate.1);
                self.masked_image()
            }
            MapType::Limited => {
                for (pos, point) in path
                    .iter()
                    .take((path.len() as f32 * percentage) as usize)
                    .enumerate()
                {
                    if (pos as f32) / path.len() as f32 > percentage {
                        break;
                    }
                    self.unlock_point_from_coordinates(point.0, point.1);
                }
                self.masked_image()
            }
            MapType::Full => self.full_image(),
        }
    }

    /// Draws a normal box outline around a point
    fn outline_helper(
        &mut self,
        image: &mut Vec<u8>,
        point: (u32, u32),
        thickness: i32,
        color: &[u8; 4],
        outline: &[u8; 4],
    ) {
        for dy in -thickness..=thickness {
            for dx in -thickness..=thickness {
                let x = point.0 as i32 + dx;
                let y = point.1 as i32 + dy;
                if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                    let index = (y as u32 * self.width + x as u32) as usize * 4;
                    if dx == -thickness || dx == thickness || dy == -thickness || dy == thickness {
                        // do not fill with outline if the color is the same as the color value
                        if image[index..index + 4] == *color {
                            continue;
                        }
                        image[index..index + 4].copy_from_slice(outline);
                    } else {
                        image[index..index + 4].copy_from_slice(color);
                    }
                }
            }
        }
    }

    fn is_diagonal_to(&mut self, point1: (u32, u32), point2: (u32, u32)) -> bool {
        let dx = (point1.0 as i32 - point2.0 as i32).abs();
        let dy = (point1.1 as i32 - point2.1 as i32).abs();
        dx == dy
    }

    /// Draws a point of a path with the specified style
    fn draw_path_point(
        &mut self,
        image: &mut Vec<u8>,
        point: (u32, u32),
        path_type: &PathStyle,
        ref path: &Vec<(u32, u32)>,
        pos: usize,
        distance: usize,
    ) {
        let x = point.0 as usize;
        let y = point.1 as usize;
        let i = y * self.width as usize + x;
        let thickness = 2;
        match path_type {
            PathStyle::Debug => {
                let chunk = &mut image[i * 4..(i + 1) * 4];
                chunk.copy_from_slice(&[255, 0, 0, 255]);
            }
            PathStyle::Dotted(color) | PathStyle::Solid(color) => {
                // Add dot and some buffer around it
                for dy in -thickness..=thickness {
                    for dx in -thickness..=thickness {
                        let x = point.0 as i32 + dx;
                        let y = point.1 as i32 + dy;
                        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                            let index = (y as u32 * self.width + x as u32) as usize * 4;
                            image[index..index + 4].copy_from_slice(color);
                        }
                    }
                }
            }
            PathStyle::SolidWithOutline(color, outline) => {
                self.outline_helper(image, point, thickness, color, outline);
            }
            PathStyle::DottedWithOutline(color, outline) => {
                if ((pos == 0 || (pos + 1) / 10 % (distance / 10 + 1) == 0)
                    && !self.is_diagonal_to(point, path[pos + 1]))
                    || ((pos == path.len() - 1 || (pos - 1) / 10 % (distance / 10 + 1) == 0)
                        && !self.is_diagonal_to(point, path[pos - 1]))
                {
                    // Draw outline in circular shape
                    let radius = thickness;
                    let radius_sq = radius * radius;
                    for dy in -radius..=radius {
                        for dx in -radius..=radius {
                            let x = point.0 as i32 + dx;
                            let y = point.1 as i32 + dy;

                            // Check if the point is within the circle radius
                            if dx * dx + dy * dy <= radius_sq {
                                // Ensure the pixel is within bounds
                                if x >= 0
                                    && x < self.width as i32
                                    && y >= 0
                                    && y < self.height as i32
                                {
                                    let index = (y as u32 * self.width + x as u32) as usize * 4;
                                    if image[index..index + 4] == *color {
                                        continue;
                                    }
                                    image[index..index + 4].copy_from_slice(outline);
                                }
                            }
                        }
                    }
                } else {
                    self.outline_helper(image, point, thickness, color, outline);
                }
            }
        }
    }

    /// Draws a path from a travel struct onto the map with the specified style and percentage of the path drawn.
    pub fn draw_path(&mut self, travel: Travel, percentage: f32, path_type: PathStyle) -> Vec<u8> {
        let distance = 10;
        let mut image = self.setup_image_for_path(&travel.computed_path, percentage);

        for (pos, point) in travel
            .computed_path
            .iter()
            .take((travel.computed_path.len() as f32 * percentage) as usize)
            .enumerate()
        {
            if (pos as f32) / travel.computed_path.len() as f32 > percentage {
                break;
            }
            if match path_type {
                PathStyle::Dotted(_) | PathStyle::DottedWithOutline(..) => {
                    if pos / 10 % (distance / 10 + 1) == 0 {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            } {
                continue;
            }
            self.draw_path_point(
                &mut image,
                *point,
                &path_type,
                &travel.computed_path,
                pos,
                distance,
            );
        }

        if self.draw_obstacles {
            self.draw_obstacles(&mut image);
        }

        image
    }

    /// Returns the full map
    pub fn full_image(&mut self) -> Vec<u8> {
        let mut new_bytes = self.bytes.clone();
        self.draw_dots(&mut new_bytes);
        self.draw_with_grid(&mut new_bytes);
        new_bytes
    }

    /// Returns the full map with a mask applied
    pub fn masked_image(&mut self) -> Vec<u8> {
        let mask = self.create_mask();
        let mut image = self.full_image();
        Self::put_mask_on_image(&mut image, mask);
        self.draw_with_grid(&mut image); // draw again to go above mask
        image
    }
}
