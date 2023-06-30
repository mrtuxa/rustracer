use rustracer::math::Vector3;
use rustracer::math::Point3;
use rustracer::math::geometry::ParametricLine;
use rustracer::math::geometry::Plane3;
use rustracer::math::geometry::Intersect;
use rustracer::camera;

fn main() {
    let width = 640;
    let height = 480;

    let plane = Plane3::new(
        Point3::new(0.0f32, 0.0, 0.0),
        Vector3::new(0.0f32, 1.0, 0.0)
    );

    let cam = camera::Perspective::new(
        Point3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        90.0 / 180.0 * 3.1415,
        width,
        height
    );

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let ray = cam.ray_for(x, y);
        let intersects = ray.intersect(plane);
        
        if intersects.len() != 0 && intersects[0] > 0.0 {
            *pixel = image::Rgb([255u8, 0, 0]);
        } else {
            *pixel = image::Rgb([0u8, 0, 0]);
        }
    }

    imgbuf.save("output.png").unwrap();

    println!("Hello, world!");
}
