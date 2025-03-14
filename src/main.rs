use minifb;
use png;
use rand::Rng;
use std::fs::File;

mod structs;

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
const GRID_SIZE: u32 = 20;

fn image_dir_to_bytes(image_dir: &str) -> (Vec<u8>, u32, u32) {
    let mut bytes = Vec::new();

    let decoder = png::Decoder::new(File::open(image_dir).unwrap());
    let mut reader = decoder.read_info().unwrap();

    let mut buf = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    bytes.extend_from_slice(&buf);

    (bytes, reader.info().width, reader.info().height)
}

fn display_image_from_bytes(bytes: Vec<u8>, width: usize, height: usize) {
    let mut window = minifb::Window::new(
        "Test - ESC to exit",
        width,
        height,
        minifb::WindowOptions::default(),
    )
    .unwrap();

    let mut u32_buf = vec![0u32; width * height];

    // Convert raw RGBA bytes to u32 pixel format for minifb
    for (i, chunk) in bytes.chunks_exact(4).enumerate() {
        let (r, g, b, a) = (chunk[0], chunk[1], chunk[2], chunk[3]);

        // If the alpha is 0, replace the pixel with a black background (or any color you want)
        let (r, g, b) = if a == 0 { (255, 255, 255) } else { (r, g, b) };

        u32_buf[i] = u32::from_le_bytes([b, g, r, 255]); // BGRA order with full opacity
    }

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&u32_buf, width, height).unwrap();
    }
}

fn draw_point(image: &mut Vec<u8>, width: u32, height: u32, point: (u32, u32), color: [u8; 4]) {
    for y in -2..=2 {
        for x in -2..=2 {
            let x = point.0 as i32 + x;
            let y = point.1 as i32 + y;
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                let index = (y as u32 * width + x as u32) as usize * 4;
                image[index..index + 4].copy_from_slice(&color);
            }
        }
    }
}

fn main() {
    let image_dir = "map.png";
    let (bytes, width, height) = image_dir_to_bytes(image_dir);
    // dbg!(&bytes);
    // let all_grid_points = draw_grid_on_image(&mut bytes, width, height);
    // // let mid_grid_point = all_grid_points[all_grid_points.len() / 2];
    // // println!("Mid grid point: {:?}", mid_grid_point);
    // let midpoint = (height / 2, width / 2);
    // let mid_grid_point = (
    //     midpoint.1 - midpoint.1 % GRID_SIZE,
    //     midpoint.0 - midpoint.0 % GRID_SIZE,
    // );
    // let next_to_midpoint = (mid_grid_point.0 + GRID_SIZE, mid_grid_point.1);
    // let above1 = (mid_grid_point.0, mid_grid_point.1 - GRID_SIZE);
    // let above2 = (next_to_midpoint.0, next_to_midpoint.1 - GRID_SIZE);
    // let additional_point = (mid_grid_point.0, mid_grid_point.1 + GRID_SIZE);
    // let additional_point2 = (additional_point.0, additional_point.1 + GRID_SIZE);
    // let unlocked_grid_points = vec![
    //     mid_grid_point,
    //     next_to_midpoint,
    //     above1,
    //     above2,
    //     additional_point,
    //     additional_point2,
    // ];
    // let mask = create_mask(bytes.clone(), width, height, unlocked_grid_points);
    // put_mask_on_image(&mut bytes, mask);

    // let random_coordinate = (rand::rng().random_range(0..width), rand::rng().random_range(0..height));
    // let closest_point = closest_to_point(&all_grid_points, random_coordinate);
    // println!("Closes point to random coordiunate {:?} is {:?}", random_coordinate, closest_point);
    // draw_point(&mut bytes, width, height, random_coordinate, [0, 255, 0, 255]);
    // draw_point(&mut bytes, width, height, closest_point, [0, 255, 0, 255]);
    // display_image_from_bytes(bytes, width as usize, height as usize);
    let obstacle1 = vec![(400, 400), (400, 700), (450, 700), (450, 400)];
    let mut map = structs::map::Map::new(
        bytes,
        width,
        height,
        GRID_SIZE,
        Vec::new(),
        vec![(99 * 2, 195 * 2)],
        vec![],
        structs::map::MapType::Limited,
    );
    map.unlock_point_from_coordinates(99 * 2, 195 * 2);
    // display_image_from_bytes(map.with_grid().with_dot(133 * 2, 313 * 2, [255, 0, 255, 0], 5).full_image(), width as usize, height as usize);
    let current_position: (u32, u32) = (99 * 2, 193 * 2);
    let goal_position: (u32, u32) = (165 * 2, 256 * 2);
    let mut travel = structs::travel::Travel::new(map.clone(), current_position, goal_position);
    let bytes2 = map.draw_path(
        travel,
        1.0,
        structs::map::PathStyle::DottedWithOutline([255, 0, 0, 255], [255, 255, 255, 255]),
    );
    display_image_from_bytes(bytes2, width as usize, height as usize);
    // let mut travel = structs::travel::Travel::new(map, current_position, goal_position);
    // map.unlock_point_from_coordinates(400, 400);
    // map.unlock_point_from_coordinates(450, 230);
    // map.unlock_point_from_coordinates(500, 400);
    // map.unlock_point_from_coordinates(current_position.0, current_position.1);
    // let bytes = map
    //     .with_grid()
    //     .with_dot(current_position.0, current_position.1, [0, 255, 0, 255], 5)
    //     .masked_image();
    // let bytes = travel.show_path(1.0);
    // display_image_from_bytes(bytes, width as usize, height as usize);
    // display_image_from_bytes(travel.map.with_grid().masked_image(), width as usize, height as usize);
}
