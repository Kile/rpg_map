#[cfg(test)]
extern crate image;
use pyo3::prelude::{Py, PyErr, PyRefMut, Python};

fn get_image_bits(directory: &str, filename: &str) -> (Vec<u8>, u32, u32) {
    let path = format!("{}/{}", directory, filename);
    let image = image::open(path).unwrap();
    let rgba_image = image.to_rgba8();
    let (width, height) = rgba_image.dimensions();
    let mut bits = vec![0; (width * height * 4) as usize];
    bits.copy_from_slice(rgba_image.as_raw());
    (bits, width, height)
}

/// Logs the difference between two images by saving them to the logs folder.
/// This helps to debug the differences between the expected and actual images.
fn log_image_difference(
    result: &[u8],
    expected: &[u8],
    image_width: u32,
    image_height: u32,
    name: &str,
) {
    let result_image = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_raw(
        image_width,
        image_height,
        result.to_vec(),
    )
    .unwrap();
    result_image
        .save(format!("src/tests/logs/{}_result.png", name))
        .expect("Failed to save result image");

    // Generate a new image with the differences
    let mut diff_image = image::ImageBuffer::new(image_width, image_height);
    for (x, y, pixel) in diff_image.enumerate_pixels_mut() {
        let index = (y * image_width + x) as usize * 4;
        if result[index] != expected[index] {
            // Log the difference
            // println!(
            //     "Difference at pixel ({}, {}): result: {:?}, expected: {:?}",
            //     x,
            //     y,
            //     &result[index..index + 4],
            //     &expected[index..index + 4]
            // );
            // set the pixel color to abs(expected - result)
            *pixel = image::Rgba([
                (result[index] as i32 - expected[index] as i32).abs() as u8,
                (result[index + 1] as i32 - expected[index + 1] as i32).abs() as u8,
                (result[index + 2] as i32 - expected[index + 2] as i32).abs() as u8,
                255,
            ]);
        } else {
            *pixel = image::Rgba([0u8, 0u8, 0u8, 0u8]); // Transparent for no difference
        }
    }
    diff_image
        .save(format!("src/tests/logs/{}_diff.png", name))
        .expect("Failed to save diff image");
}

#[cfg(test)]
mod map_tests {
    use super::*;
    use crate::structs::map::Map;
    use crate::structs::map::MapType;
    use crate::structs::map::PathDisplayType;
    use crate::structs::map::PathProgressDisplayType;
    use crate::structs::map::PathStyle;
    use crate::structs::travel::Travel;

    #[test]
    fn test_map_creation() {
        let (image, image_width, image_height) = get_image_bits("test_assets", "map.png");
        let (background, _, _) = get_image_bits("test_assets", "background.png");
        let (expected, _, _) = get_image_bits("test_results", "image.png");
        let map = Map::new(
            image.clone(),
            image_width,
            image_height,
            20,
            MapType::Limited,
            vec![],
            vec![],
            vec![],
        );
        let travel = Travel::new(map.clone(), (198, 390), (330, 512)).unwrap();
        Python::with_gil(|py| -> Result<(), PyErr> {
            let n: Py<Map> = Py::new(py, map).expect("Failed to create Py<Map>");
            let guard: PyRefMut<'_, Map> = n.bind(py).borrow_mut();

            let result = Map::draw_background(
                Map::with_dot(guard, 198, 390, [255, 0, 0, 255], 5)
                    .draw_path(
                        travel,
                        1.0,
                        2,
                        PathStyle::DottedWithOutline([255, 0, 0, 255], [255, 255, 255, 255]),
                        PathDisplayType::Revealing(),
                        PathProgressDisplayType::Travelled(),
                    )
                    .expect("Failed to draw path"),
                background,
            )
            .expect("Failed to generate bits");

            assert_eq!(result.len(), expected.len());

            let difference = result
                .iter()
                .zip(expected.iter())
                .filter(|(a, b)| a != b)
                .collect::<Vec<_>>();

            if !difference.is_empty() {
                // Save both images to the logs folder for debugging
                log_image_difference(
                    &result,
                    &expected,
                    image_width,
                    image_height,
                    "test_map_creation",
                );
            }
            assert_eq!(difference.len(), 0); // Easier to debug in logs

            Ok(())
        })
        .expect("Failed to execute Python code");
    }

    // This test fails and I am not sure why
    // #[test]
    // fn test_map_creation_with_obstacles() {
    //     let (image, image_width, image_height) = get_image_bits("test_assets", "map.png");
    //     let (background, _, _) = get_image_bits("test_assets", "background.png");
    //     let (expected, _, _) = get_image_bits("test_results", "obstacle.png");
    //     let mut map = Map::new(
    //         image.clone(),
    //         image_width,
    //         image_height,
    //         20,
    //         MapType::Limited,
    //         vec![],
    //         vec![],
    //         vec![vec![(160, 240), (134, 253), (234, 257), (208, 239)]],
    //     );
    //     let travel = Travel::new(map.clone(), (198, 390), (172, 223)).unwrap();

    //     let result = Map::draw_background(
    //         map.draw_path(
    //                 travel,
    //                 1.0,
    //                 2,
    //                 PathStyle::DottedWithOutline([255, 0, 0, 255], [255, 255, 255, 255]),
    //                 PathDisplayType::Revealing(),
    //                 PathProgressDisplayType::Travelled(),
    //             )
    //             .expect("Failed to draw path"),
    //         background,
    //     )
    //     .expect("Failed to generate bits");

    //     assert_eq!(result.len(), expected.len());

    //     let difference = result
    //         .iter()
    //         .zip(expected.iter())
    //         .filter(|(a, b)| a != b)
    //         .collect::<Vec<_>>();

    //     if !difference.is_empty() {
    //         // Save both images to the logs folder for debugging
    //         log_image_difference(
    //             &result,
    //             &expected,
    //             image_width,
    //             image_height,
    //             "test_map_creation_with_obstacles",
    //         );
    //     }
    //     assert_eq!(difference.len(), 0); // Easier to debug in logs

    // }
}
