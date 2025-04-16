use super::utils::{compare_images, get_image_bits};
use pyo3::prelude::{Py, PyErr, PyRefMut, Python};

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
    fn test_full_map_creation() {
        let (image, image_width, image_height) = get_image_bits("test_assets", "map.png");
        let (background, _, _) = get_image_bits("test_assets", "background.png");
        let (expected, _, _) = get_image_bits("test_results", "full.png");
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
                        PathDisplayType::BelowMask,
                        PathProgressDisplayType::Travelled,
                    )
                    .expect("Failed to draw path"),
                background,
            )
            .expect("Failed to generate bits");

            compare_images(&result, &expected, &image, image_width, image_height);

            Ok(())
        })
        .expect("Failed to execute Python code");
    }

    #[test]
    fn test_map_creation_with_obstacles() {
        let (image, image_width, image_height) = get_image_bits("test_assets", "map.png");
        let (expected, _, _) = get_image_bits("test_results", "obstacle.png");
        let mut map = Map::new(
            image.clone(),
            image_width,
            image_height,
            20,
            MapType::Limited,
            vec![],
            vec![],
            vec![vec![(160, 240), (134, 253), (234, 257), (208, 239)]],
        );
        let travel = Travel::new(map.clone(), (198, 390), (172, 223)).unwrap();

        let result = map
            .draw_path(
                travel,
                1.0,
                2,
                PathStyle::DottedWithOutline([255, 0, 0, 255], [255, 255, 255, 255]),
                PathDisplayType::BelowMask,
                PathProgressDisplayType::Travelled,
            )
            .expect("Failed to draw path");

        compare_images(&result, &expected, &image, image_width, image_height);
    }

    #[test]
    fn test_map_creation_hidden_and_progress() {
        let (image, image_width, image_height) = get_image_bits("test_assets", "map.png");
        let (expected, _, _) = get_image_bits("test_results", "progress_and_hidden.png");
        let mut map = Map::new(
            image.clone(),
            image_width,
            image_height,
            20,
            MapType::Hidden,
            vec![],
            vec![],
            vec![],
        );
        let travel = Travel::new(map.clone(), (198, 390), (330, 512)).unwrap();

        let result = map
            .draw_path(
                travel,
                0.5,
                2,
                PathStyle::DottedWithOutline([255, 0, 0, 255], [255, 255, 255, 255]),
                PathDisplayType::AboveMask,
                PathProgressDisplayType::Progress,
            )
            .expect("Failed to draw path");

        compare_images(&result, &expected, &image, image_width, image_height);
    }

    #[test]
    fn test_wrong_background() {
        let (image, _, _) = get_image_bits("test_assets", "map.png");
        let (background, _, _) = get_image_bits("test_assets", "cat.png");
        match Map::draw_background(image, background) {
            Ok(_) => panic!("Expected an error, but got a valid image"),
            Err(e) => assert_eq!(
                e.to_string(),
                "ValueError: Background image must have the same size as the map"
            ),
        }
    }
}
