use opencv::prelude::*;

fn main() {
    let image = Mat::default(); // Dummy image, not important to demonstrate problem.
    let pattern_size = opencv::core::Size::new(5, 7); // Dummy pattern size, not important to demonstrate problem.
    let mut centers = Mat::default();

    let detector_params = opencv::calib3d::CirclesGridFinderParameters::default().unwrap();
    let blob_detector = opencv::features2d::SimpleBlobDetector::create(detector_params).unwrap();

    opencv::calib3d::find_circles_grid(
        &image,
        pattern_size,
        &mut centers,
        opencv::calib3d::CALIB_CB_ASYMMETRIC_GRID,
        &blob_detector,
        detector_params,
    );
}
