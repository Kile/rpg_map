// // #[cfg(test)]
// // extern crate image;
// // use pyo3::prelude::{Py, Python, PyRefMut};

// fn get_image_bits(directory: &str, filename: &str) -> (Vec<u8>, u32, u32) {
//     let path = format!("{}/{}", directory, filename);
//     let image = image::open(path).unwrap();
//     let rgba_image = image.to_rgba8();
//     let (width, height) = rgba_image.dimensions();
//     let mut bits = vec![0; (width * height * 4) as usize];
//     bits.copy_from_slice(rgba_image.as_raw());
//     (bits, width, height)
// }

// #[cfg(test)]
// mod map_tests {
//     use super::*;
//     use crate::structs::map::Map;
//     use crate::structs::map::MapType;
//     use crate::structs::map::PathStyle;
//     use crate::structs::map::PathDisplayType;
//     use crate::structs::map::PathProgressDisplayType;
//     use crate::structs::travel::Travel;
//     use image::{DynamicImage, GenericImageView};
//     use std::fmt::Result;
//     use std::fs;

//     #[test]
//     fn test_map_creation() {
//         let (image, image_width, image_height) = get_image_bits("test_assets", "map.png");
//         let (background, _, _) = get_image_bits("test_assets", "background.png");
//         let (expected, _, _) = get_image_bits("test_results", "image.png");
//         let mut map = Map::new(
//             image.clone(),
//             image_width,
//             image_height,
//             20,
//             MapType::Limited,
//             vec![],
//             vec![],
//             vec![],
//         );
//         let travel = Travel::new(
//             map.clone(),
//             (198, 390), 
//             (330,  512)
//         ).unwrap();
//         // Python::with_gil(|py| -> Result {
//         //     let n: Py<Map> = Py::new(py, map ).expect("Failed to create Py<Map>");
        
//         // //     We borrow the guard and then dereference
//         // //     it to get a mutable reference to Number
//         // //     let guard: PyRefMut<'_, Map> = n.bind(py).borrow_mut();
        
//         //     let result = Map::draw_background(
//         //         Map::with_dot(guard,198, 390, [255, 0, 0, 255], 5).draw_path(
//         //             travel, 
//         //             1.0, 
//         //             2, 
//         //             PathStyle::DottedWithOutline([255, 0, 0, 255], [255, 255, 255, 255]), 
//         //             PathDisplayType::Revealing(), 
//         //             PathProgressDisplayType::Progress()
//         //         ).expect("Failed to draw path"), 
//         //         background
//         //     ).expect("Failed to generate bits");
        
//         // //     To avoid panics we must dispose of the
//         // //     `PyRefMut` before borrowing again.
//         // //     drop(guard);
        
//         //     let n_immutable: &Map = &n.bind(py).borrow();
//         //     assert_eq!(result, expected);
        
//         //     Ok(())
//         // }).expect("Failed to execute Python code");

//         let result = Map::draw_background(
//             map.draw_path(
//                 travel, 
//                 1.0, 
//                 2, 
//                 PathStyle::DottedWithOutline([255, 0, 0, 255], [255, 255, 255, 255]), 
//                 PathDisplayType::Revealing(), 
//                 PathProgressDisplayType::Progress()
//             ).expect("Failed to draw path"), 
//             background
//         ).expect("Failed to generate bits");
//         assert_eq!(result, expected);
//     }

//     // #[test]
//     // fn test_map_from_image() {
//     //     let map = Map::from_image("tests", "test_image.png");
//     //     assert_eq!(map.width, 5);
//     //     assert_eq!(map.height, 5);
//     //     assert_eq!(map.bits.len(), (5 * 5 * 4) as usize);
//     // }
// }