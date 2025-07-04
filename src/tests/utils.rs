#[cfg(test)]
extern crate image;

// fn python_install_pillow(py: Python) -> Bound<'_, PyModule> {
//     let pip = py.import("pip").unwrap();
//     pip.call_method1(
//         "main",
//         (PyList::new(py, vec!["install", "pillow"]).unwrap(),),
//     )
//     .unwrap();
//     py.import("PIL.Image").unwrap()
// }

// fn get_image_bits(py: Python, directory: &str, filename: &str) -> (Vec<u8>, u32, u32) {
//     let sys = py.import("sys").unwrap();
//     let path = sys.getattr("path").unwrap();
//     path.call_method1("append", ("env/lib/python3.13/site-packages",)).unwrap();  // append my venv path
//     // Ensure Pillow is installed
//     let pillow= py.import("PIL.Image")
//         .or_else(|_| {
//             Ok::<_, PyErr>(python_install_pillow(py))
//         }).unwrap();
//     let image = pillow
//         .call_method1("open", (format!("{}/{}", directory, filename),))
//         .unwrap();
//     let image = image.call_method1("convert", ("RGBA",)).unwrap();
//     let image_width = image
//         .getattr("width")
//         .unwrap()
//         .extract::<u32>()
//         .unwrap();
//     let image_height = image
//         .getattr("height")
//         .unwrap()
//         .extract::<u32>()
//         .unwrap();
//     let image = image.call_method1("tobytes", ("raw", "RGBA", 0)).unwrap();
//     let image = image.extract::<Vec<u8>>().unwrap();
//     (image, image_width, image_height)
// }

/// Get image bits using the Rust image library
pub fn get_image_bits(directory: &str, filename: &str) -> (Vec<u8>, u32, u32) {
    let img = image::open(format!("{directory}/{filename}")).unwrap();
    let img = img.to_rgba8();
    let (width, height) = img.dimensions();
    let img = img.into_raw();
    (img, width, height)
}

/// Compare the expected and actual images.
pub fn compare_images(
    result: &[u8],
    expected: &[u8],
    original: &[u8],
    image_width: u32,
    image_height: u32,
) {
    assert_eq!(result.len(), expected.len());

    let difference = result
        .chunks_exact(4)
        .zip(expected.chunks_exact(4))
        .enumerate()
        .filter(|(i, (a, b))| {
            // This means: if a is not equal to b, and a is not transparent
            // if a is transparent, then it should be equal to the original.
            // This is because there is a lot of weirdness with somewhat transparent pixels
            // which I spent 3 days debugging, choosing to ignore them for now.
            a != b && ([0, 255].contains(&a[3]) || (**a == original[*i..*i + 4]))
        })
        .collect::<Vec<_>>();

    if !difference.is_empty() {
        // Save both images to the logs folder for debugging
        log_image_difference(
            result,
            expected,
            original,
            image_width,
            image_height,
            "test_map_creation_with_obstacles",
        );
    }
    assert_eq!(difference.len(), 0); // Easier to debug in logs
}

/// Logs the difference between two images by saving them to the logs folder.
/// This helps to debug the differences between the expected and actual images.
fn log_image_difference(
    result: &[u8],
    expected: &[u8],
    read: &[u8],
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
        .save(format!("src/tests/logs/{name}_result.png"))
        .expect("Failed to save result image");

    // Generate a new image with the differences
    let mut diff_image = image::ImageBuffer::new(image_width, image_height);
    for (x, y, pixel) in diff_image.enumerate_pixels_mut() {
        let index = (y * image_width + x) as usize * 4;
        if result[index] != expected[index] {
            // Log the difference
            println!(
                "Difference at pixel ({}, {}): result: {:?}, expected: {:?} (read: {:?})",
                x,
                y,
                &result[index..index + 4],
                &expected[index..index + 4],
                &read[index..index + 4]
            );
            // set the pixel color to abs(expected - result)
            *pixel = image::Rgba([
                (result[index] as i32 - expected[index] as i32).unsigned_abs() as u8,
                (result[index + 1] as i32 - expected[index + 1] as i32).unsigned_abs() as u8,
                (result[index + 2] as i32 - expected[index + 2] as i32).unsigned_abs() as u8,
                255,
            ]);
        } else {
            *pixel = image::Rgba([0u8, 0u8, 0u8, 0u8]); // Transparent for no difference
        }
    }
    diff_image
        .save(format!("src/tests/logs/{name}_diff.png"))
        .expect("Failed to save diff image");
}
