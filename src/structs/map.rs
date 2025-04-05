use crate::structs::path::PathPoint;
use crate::structs::travel::Travel;
use geo::{Contains, Coord, LineString, Point, Polygon};
use pyo3::prelude::*;

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapType {
    Hidden,
    Limited,
    Full,
}

#[pyclass(eq)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathStyle {
    Debug(),
    Dotted([u8; 4]),
    Solid([u8; 4]),
    SolidWithOutline([u8; 4], [u8; 4]),
    DottedWithOutline([u8; 4], [u8; 4]),
}

#[pyclass(eq)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathProgressDisplayType {
    Remaining(),
    Travelled(),
    Progress(),
}

#[pyclass(eq)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathDisplayType {
    Revealing(),
    BelowMask(),
    AboveMask(),
}

#[pyclass]
#[derive(Clone)]
pub struct Map {
    #[pyo3(get)]
    pub width: u32,
    #[pyo3(get)]
    pub height: u32,
    bytes: Vec<u8>,
    grid_size: u32,
    #[pyo3(get)]
    unlocked: Vec<(u32, u32)>,
    grid_points: Vec<(u32, u32)>,
    special_points: Vec<(u32, u32)>,
    pub obstacles: Vec<Vec<(u32, u32)>>,
    pub map_type: MapType,
    draw_obstacles: bool,
    dots: Vec<(u32, u32, [u8; 4], u32)>, // x, y, color, radius
    should_draw_with_grid: bool,
}

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

#[pymethods]
impl Map {
    #[new]
    #[pyo3(signature = (
        bytes,
        width,
        height,
        grid_size,
        map_type = MapType::Full,
        unlocked = vec![],
        special_points = vec![],
        obstacles = vec![]
    ))]
    pub fn new(
        bytes: Vec<u8>,
        width: u32,
        height: u32,
        grid_size: u32,
        map_type: MapType,
        unlocked: Vec<(u32, u32)>,
        special_points: Vec<(u32, u32)>,
        obstacles: Vec<Vec<(u32, u32)>>,
    ) -> Self {
        let grid_points = calculate_grid_points(width, height, grid_size);
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
            should_draw_with_grid: false,
        }
    }

    /// Draws the background image at every transparent pixel
    /// if the background is set
    #[staticmethod]
    fn draw_background(bytes: Vec<u8>, background: Vec<u8>) -> PyResult<Vec<u8>> {
        if background.len() != bytes.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Background image must have the same size as the map",
            ));
        }
        let mut bytes_clone = bytes.clone(); // We do not want to mutate the original bytes
        for (i, chunk) in background.chunks_exact(4).enumerate() {
            let index = i * 4;
            if bytes_clone[index + 3] == 0 {
                bytes_clone[index..index + 4].copy_from_slice(chunk);
            }
        }

        Ok(bytes_clone)
    }

    /// Adds a dot do be drawn on the map when full_image or masked_image is called
    pub fn with_dot(
        mut slf: PyRefMut<'_, Self>,
        x: u32,
        y: u32,
        color: [u8; 4],
        radius: u32,
    ) -> PyRefMut<'_, Self> {
        slf.dots.push((x, y, color, radius));
        slf
    }

    /// Signal you want a grid to be drawn on the map as well
    pub fn with_grid(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf.should_draw_with_grid = true;
        slf
    }

    /// Signal you want obstacles to be drawn on the map as well
    pub fn with_obstacles(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf.draw_obstacles = true;
        slf
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

    /// Draws a path from a travel struct onto the map with the specified style and percentage of the path drawn.
    #[pyo3(signature = (
        travel,
        percentage,
        line_width,
        path_type = PathStyle::DottedWithOutline([255, 0, 0, 255], [255, 255, 255, 255]),
        display_style = PathDisplayType::Revealing(),
        progress_display_type = PathProgressDisplayType::Travelled()
    ))]
    pub fn draw_path(
        &mut self,
        travel: Travel,
        percentage: f32,
        line_width: i32,
        path_type: PathStyle,
        display_style: PathDisplayType,
        progress_display_type: PathProgressDisplayType,
    ) -> PyResult<Vec<u8>> {
        self.line_width_checker(line_width, path_type)?;
        let distance = (line_width * 5) as usize;
        let mut image = self.setup_image_for_path(display_style);
        let path = travel.computed_path.clone();
        let critical_index = ((path.len() - 1) as f32 * percentage) as usize;
        let to_be_drawn: Vec<PathPoint> = match progress_display_type {
            PathProgressDisplayType::Remaining() => path[critical_index..].to_vec(),
            PathProgressDisplayType::Travelled() => path[..=critical_index].to_vec(),
            PathProgressDisplayType::Progress() => path,
        };
        // Unlock the points traversed so far
        if display_style == PathDisplayType::Revealing() {
            travel.computed_path[..=critical_index]
                .iter()
                .for_each(|point| {
                    self.unlock_point_from_coordinates(point.x, point.y);
                });
        }

        for (pos, point) in to_be_drawn.iter().enumerate() {
            if match path_type {
                PathStyle::Dotted(_) | PathStyle::DottedWithOutline(..) => {
                    pos / 10 % (distance / 10 + 1) == 0
                }
                _ => false,
            } {
                continue;
            }

            image = self.draw_path_point(
                image,
                *point,
                &path_type,
                travel.computed_path.clone(),
                pos,
                distance,
                line_width,
                progress_display_type,
                critical_index,
            );
        }

        match display_style {
            PathDisplayType::BelowMask() | PathDisplayType::Revealing() => {
                match self.map_type {
                    MapType::Hidden | MapType::Limited => Ok(self.mask_image(image)),
                    MapType::Full => Ok(image),
                }
            }
            PathDisplayType::AboveMask() => Ok(image),
        }
    }

    /// Returns the full map
    fn full_image(&mut self) -> Vec<u8> {
        let mut new_bytes = self.bytes.clone();
        new_bytes = self.draw_obstacles(new_bytes);
        new_bytes = self.draw_dots(new_bytes);
        new_bytes = self.draw_with_grid(new_bytes);
        new_bytes
    }

    /// Returns the full map with a mask applied
    fn masked_image(&mut self) -> Vec<u8> {
        let mask = self.create_mask();
        let mut image = self.bytes.clone();
        image = self.draw_obstacles(image);
        image = self.draw_dots(image);
        image = Self::put_mask_on_image(self, image, mask);
        image = self.draw_with_grid(image);
        image
    }

    pub fn get_bits(&mut self) -> Vec<u8> {
        match self.map_type {
            MapType::Full => self.full_image(),
            MapType::Hidden | MapType::Limited => self.masked_image(),
        }
    }
}

// These methods are not exposed to the Python library
impl Map {
    /// Checks if an intersection point is a special point
    fn is_special_point(&self, x: u32, y: u32) -> Option<&(u32, u32)> {
        self.special_points
            .iter()
            .find(|p| self.closest_to_point(**p) == self.closest_to_point((x, y)))
    }

    /// Does some checks if the line width is too small
    fn line_width_checker(&self, line_width: i32, style: PathStyle) -> PyResult<()> {
        if line_width < 1 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Line width must be at least 1",
            ));
        }
        if line_width > self.grid_size as i32 {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Line width must be less than the grid size {}",
                self.grid_size
            )));
        }
        if let PathStyle::SolidWithOutline(_, _) | PathStyle::DottedWithOutline(_, _) = style {
            if line_width < 2 {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "Line width must be at least 2 for outline",
                ));
            }
        }
        Ok(())
    }

    /// Checks if a point is a background obstracle based on either if it is
    /// transparent or if the coordinate is the same as the background at that point
    // fn is_boarder_point(
    //     &self,
    //     x: u32,
    //     y: u32,
    // ) -> bool {
    //     let index = (y * self.width + x) as usize * 4;
    //     if let Some(bg) = &self.background {
    //         return self.bytes[index] == bg[index]; // Background pixel
    //     } else {
    //         return self.bytes[index + 3] == 0; // Transparent pixel
    //     }
    // }

    /// Finds the closest grid point with the given coordinates
    fn closest_to_point(&self, point: (u32, u32)) -> (u32, u32) {
        let mut min_dist = u32::MAX;
        let mut closest_point = (0, 0);

        for p in &self.grid_points {
            let dist = (p.0 as i32 - point.0 as i32).unsigned_abs()
                + (p.1 as i32 - point.1 as i32).unsigned_abs();
            if dist < min_dist {
                min_dist = dist;
                closest_point = *p;
            }
        }

        closest_point
    }

    /// Turns every pixel of the image black where the mask is not transparent
    fn put_mask_on_image(&self, mut image: Vec<u8>, mask: Vec<u8>) -> Vec<u8> {
        for (i, chunk) in mask.chunks_exact(4).enumerate() {
            let a = chunk[3];
            if a != 0 {
                let index = i * 4;
                image[index..index + 4].copy_from_slice(&[0, 0, 0, 255]);
            }
        }
        image
    }

    /// Helper function to check if four points form a square
    fn is_square(&mut self, points: Vec<(u32, u32)>) -> bool {
        let mut sorted = points.clone();
        sorted.sort(); // Sort by x, then y

        let (x1, y1) = sorted[0];
        let (x4, y4) = sorted[3];

        let side1 = (x4 as i32 - x1 as i32).abs();
        let side2 = (y4 as i32 - y1 as i32).abs();

        side1 == side2 && side1 > 0 // Check if square with non-zero side length
    }

    /// Helper function to make everything inside a square transparent
    fn make_square_transparent(&mut self, mut mask: Vec<u8>, points: Vec<(u32, u32)>) -> Vec<u8> {
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

        mask
    }

    /// Creates a mask for the map, taking into account the unlocked points
    /// and transparent background
    fn create_mask(&mut self) -> Vec<u8> {
        let mut mask = self.bytes.clone();

        for (mut cx, mut cy) in &self.unlocked {
            let radius: i32;
            if let Some((x, y)) = self.is_special_point(cx, cy) {
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

        // If the radius is larger than diagonal length of a square,
        // we can stop and return here since the field would already be revealed
        let smallest_radius = ((self.grid_size as f32) / 0.8) as i32;
        if smallest_radius > (2.0_f32.sqrt() * self.grid_size as f32) as i32 {
            return mask;
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
                        if self.is_square(points.clone()) {
                            mask = self.make_square_transparent(mask, points);
                        }
                    }
                }
            }
        }

        mask
    }

    /// Draws all dots defined in the `dots` vector on the image
    fn draw_dots(&mut self, mut bytes: Vec<u8>) -> Vec<u8> {
        for (x, y, color, radius) in &self.dots {
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
        bytes
    }

    /// Draws a grid on the image
    fn draw_with_grid(&mut self, mut image: Vec<u8>) -> Vec<u8> {
        if !self.should_draw_with_grid {
            return image;
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

        image
    }

    /// Draws all defined obstacles on the map. Useful for debugging.
    fn draw_obstacles(&mut self, mut bytes: Vec<u8>) -> Vec<u8> {
        if !self.draw_obstacles {
            return bytes;
        }
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

        bytes
    }

    /// Sets up the image for a path to be drawn on it
    fn setup_image_for_path(&mut self, display_style: PathDisplayType) -> Vec<u8> {
        match self.map_type {
            MapType::Hidden | MapType::Limited => {
                if display_style == PathDisplayType::AboveMask() {
                    self.masked_image()
                } else {
                    self.full_image()
                }
            }
            MapType::Full => self.full_image(),
        }
    }

    /// Draws a normal box outline around a point
    fn outline_helper(
        &mut self,
        mut image: Vec<u8>,
        point: PathPoint,
        thickness: i32,
        color: [u8; 4],
        outline: [u8; 4],
    ) -> Vec<u8> {
        for dy in -thickness..=thickness {
            for dx in -thickness..=thickness {
                let x = point.x as i32 + dx;
                let y = point.y as i32 + dy;
                if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                    let index = (y as u32 * self.width + x as u32) as usize * 4;
                    if dx == -thickness || dx == thickness || dy == -thickness || dy == thickness {
                        // do not fill with outline if the color is the same as the color value
                        if image[index..index + 4] == color {
                            continue;
                        }
                        image[index..index + 4].copy_from_slice(&outline);
                    } else {
                        image[index..index + 4].copy_from_slice(&color);
                    }
                }
            }
        }
        image
    }

    /// Draws an endpoint of a path with a circular outline
    fn endpoint_helper(
        &mut self,
        mut image: Vec<u8>,
        point: PathPoint,
        thickness: i32,
        color: [u8; 4],
        outline: [u8; 4],
    ) -> Vec<u8> {
        // Draw outline in circular shape
        let radius = thickness;
        let radius_sq = radius * radius;
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let x = point.x as i32 + dx;
                let y = point.y as i32 + dy;

                // Check if the point is within the circle radius
                if dx * dx + dy * dy <= radius_sq {
                    // Ensure the pixel is within bounds
                    if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                        let index = (y as u32 * self.width + x as u32) as usize * 4;
                        if image[index..index + 4] == color {
                            continue;
                        }
                        image[index..index + 4].copy_from_slice(&outline);
                    }
                }
            }
        }
        image
    }

    /// Draws a simple point of a path with the specified style
    fn simple_point_helper(
        &mut self,
        mut image: Vec<u8>,
        point: PathPoint,
        thickness: i32,
        color: [u8; 4],
    ) -> Vec<u8> {
        for dy in -thickness..=thickness {
            for dx in -thickness..=thickness {
                let x = point.x as i32 + dx;
                let y = point.y as i32 + dy;
                if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                    let index = (y as u32 * self.width + x as u32) as usize * 4;
                    image[index..index + 4].copy_from_slice(&color);
                }
            }
        }
        image
    }

    /// Checks if two points are diagonal to each other
    fn is_diagonal_to(&mut self, point1: PathPoint, point2: PathPoint) -> bool {
        let dx = (point1.x as i32 - point2.x as i32).abs();
        let dy = (point1.y as i32 - point2.y as i32).abs();
        dx == dy
    }

    /// Converts an RGBA color to grayscale using the luminance method
    /// Thank you to https://stackoverflow.com/a/596243
    fn rgba_to_grayscale(&self, rgba: &[u8; 4]) -> [u8; 4] {
        let r = rgba[0] as f32;
        let g = rgba[1] as f32;
        let b = rgba[2] as f32;

        // Common grayscale conversion formula (luminance)
        let grayscale = (0.299 * r + 0.587 * g + 0.114 * b).round() as u8;

        [grayscale, grayscale, grayscale, rgba[3]]
    }

    /// Helper to determine the color of a point based on the progress display type
    fn color_helper(
        &mut self,
        color: [u8; 4],
        progress_display_type: PathProgressDisplayType,
        index: usize,
        critical_index: usize,
    ) -> [u8; 4] {
        match progress_display_type {
            PathProgressDisplayType::Progress() => {
                // If it is before the critical index, make it greyscale
                if index < critical_index {
                    return self.rgba_to_grayscale(&color);
                } else {
                    return color;
                }
            }
            _ => color,
        }
    }

    /// Draws a point of a path with the specified style
    fn draw_path_point(
        &mut self,
        mut image: Vec<u8>,
        point: PathPoint,
        path_type: &PathStyle,
        path: Vec<PathPoint>,
        pos: usize,
        distance: usize,
        line_width: i32,
        progress_display_type: PathProgressDisplayType,
        critical_index: usize,
    ) -> Vec<u8> {
        let x = point.x as usize;
        let y = point.y as usize;
        let i = y * self.width as usize + x;
        match path_type {
            PathStyle::Debug() => {
                let chunk = &mut image[i * 4..(i + 1) * 4];
                chunk.copy_from_slice(&[255, 0, 0, 255]);
            }
            PathStyle::Solid(color) => {
                let color = self.color_helper(*color, progress_display_type, pos, critical_index);
                if (pos == 0 && !self.is_diagonal_to(point, path[pos + 1]))
                    || (pos == path.len() - 1 && !self.is_diagonal_to(point, path[pos - 1]))
                {
                    image = self.endpoint_helper(image, point, line_width, color, color);
                } else {
                    image = self.simple_point_helper(image, point, line_width, color);
                }
            }
            PathStyle::Dotted(color) => {
                let color = self.color_helper(*color, progress_display_type, pos, critical_index);
                if ((pos == path.len() - 1 || (pos - 1) / 10 % (distance / 10 + 1) == 0)
                    && !self.is_diagonal_to(point, path[pos - 1]))
                    || ((pos == 0 || (pos + 1) / 10 % (distance / 10 + 1) == 0)
                        && !self.is_diagonal_to(point, path[pos + 1]))
                {
                    image = self.endpoint_helper(image, point, line_width, color, color);
                } else {
                    image = self.simple_point_helper(image, point, line_width, color);
                }
            }
            PathStyle::SolidWithOutline(color, outline) => {
                let color = self.color_helper(*color, progress_display_type, pos, critical_index);
                let outline =
                    self.color_helper(*outline, progress_display_type, pos, critical_index);
                if (pos == 0 && !self.is_diagonal_to(point, path[pos + 1]))
                    || (pos == path.len() - 1 && !self.is_diagonal_to(point, path[pos - 1]))
                {
                    image = self.endpoint_helper(image, point, line_width, color, outline);
                } else {
                    image = self.outline_helper(image, point, line_width, color, outline);
                }
            }
            PathStyle::DottedWithOutline(color, outline) => {
                let color = self.color_helper(*color, progress_display_type, pos, critical_index);
                let outline =
                    self.color_helper(*outline, progress_display_type, pos, critical_index);
                if ((pos == path.len() - 1 || (pos - 1) / 10 % (distance / 10 + 1) == 0)
                    && !self.is_diagonal_to(point, path[pos - 1]))
                    || ((pos == 0 || (pos + 1) / 10 % (distance / 10 + 1) == 0)
                        && !self.is_diagonal_to(point, path[pos + 1]))
                {
                    image = self.endpoint_helper(image, point, line_width, color, outline);
                } else {
                    image = self.outline_helper(image, point, line_width, color, outline);
                }
            }
        }
        image
    }

    /// Applies the mask to the image
    /// and draws the grid again in case it was overwritten
    fn mask_image(&mut self, image: Vec<u8>) -> Vec<u8> {
        let mask = self.create_mask();
        let mut new_image = image.clone();
        new_image = Self::put_mask_on_image(self, new_image, mask);
        new_image = self.draw_with_grid(new_image); // Draw grid again in case it was overwritten
        new_image
    }
}
