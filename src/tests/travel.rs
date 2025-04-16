use super::utils::{compare_images, get_image_bits};

#[cfg(test)]
mod travel_tests {
    use super::*;
    use crate::structs::map::Map;
    use crate::structs::map::MapType;
    use crate::structs::travel::Travel;

    #[test]
    fn test_dbg_map() {
        let (image, image_width, image_height) = get_image_bits("test_assets", "map.png");
        let (expected, _, _) = get_image_bits("test_results", "debug_with_obstacle.png");
        let map = Map::new(
            image.clone(),
            image_width,
            image_height,
            20,
            MapType::Limited,
            vec![],
            vec![],
            vec![vec![(160, 240), (134, 253), (234, 257), (208, 239)]],
        );
        let result = Travel::dbg_map(map);
        compare_images(&result, &expected, &image, image_width, image_height);
    }

    #[test]
    fn test_unreachable_path() {
        let (image, image_width, image_height) = get_image_bits("test_assets", "map.png");
        let map = Map::new(
            image.clone(),
            image_width,
            image_height,
            20,
            MapType::Limited,
            vec![],
            vec![],
            vec![vec![(160, 240), (134, 253), (234, 257), (208, 239)]],
        );
        // Test going into the obstacle
        match Travel::new(map.clone(), (198, 390), (158, 250)) {
            Ok(_) => panic!("Expected an error, but got a valid travel object"),
            Err(e) => assert_eq!(
                e.to_string(),
                "ValueError: Current location or destination is an obstacle"
            ),
        }

        // Test going into the boarder
        match Travel::new(map.clone(), (198, 390), (100, 425)) {
            Ok(_) => panic!("Expected an error, but got a valid travel object"),
            Err(e) => assert_eq!(
                e.to_string(),
                "ValueError: Current location or destination is an obstacle"
            ),
        }

        // Test going out of bounds
        match Travel::new(map.clone(), (198, 390), (1000, 1000)) {
            Ok(_) => panic!("Expected an error, but got a valid travel object"),
            Err(e) => assert_eq!(
                e.to_string(),
                "ValueError: Current location or destination is out of bounds"
            ),
        }

        // Test going to unreachable island
        match Travel::new(map.clone(), (198, 390), (60, 90)) {
            Ok(_) => panic!("Expected an error, but got a valid travel object"),
            Err(e) => assert_eq!(e.to_string(), "ValueError: No path found"),
        }
    }
}
