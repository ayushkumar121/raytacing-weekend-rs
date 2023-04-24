use std::rc::Rc;

use raytracing::algebra::{dot, random_unit_vector, Vec3};
use raytracing::camera::Camera;
use raytracing::{
    image::{Color, Image},
    Point, Ray,
};
use raytracing::{random, Hit, Hittable, World};

struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, out: &mut raytracing::Hit) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_sqaured();
        let half_b = dot(ray.direction, oc);
        let c = dot(oc, oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        let sqrtd = f64::sqrt(discriminant);

        if discriminant < 0.0 {
            return false;
        }

        // Nearest root in the range [t_min, t_max]
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
        }

        if root < t_min || root > t_max {
            return false;
        }

        out.t = root;
        out.point = ray.at(root);

        let outward_normal = (out.point - self.center) / self.radius;
        out.front_face = dot(ray.direction, outward_normal) < 0.0;
        out.normal = if out.front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        };

        return true;
    }
}

fn ray_color(ray: &Ray, world: &World, depth: i32) -> Color {
    if depth <= 0 {
        return Color::zero();
    }

    let mut hit = Hit::default();
    if world.hit(ray, 0.0, f64::INFINITY, &mut hit) {
        let target = hit.point + hit.normal + random_unit_vector();
        let origin = hit.point;
        let direction = target - origin;
        return 0.5 * ray_color(&Ray { origin, direction }, world, depth - 1);
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.1 + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Setting up the image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let cam = Camera::new();
    let mut world = World::default();
    world
        .objects
        .push(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world
        .objects
        .push(Rc::new(Sphere::new(Point::new(0.0, -100.4, -1.0), 100.0)));

    // Generating Pixels
    let mut pixel_data = vec![Color::zero(); image_width * image_height];
    for y in (0..image_height).rev() {
        eprintln!("Scanline remaining: {y}");
        for x in 0..image_width {
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + random()) / (image_width as f64 - 1.0);
                let v = (y as f64 + random()) / (image_height as f64 - 1.0);

                let ray = cam.cast_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world, max_depth);
            }

            pixel_data[y * image_width + x] = pixel_color;
        }
    }
    eprintln!("Done!");

    // Writing Pixels
    let mut image = Image::new(image_width, image_height);
    image.set_data(pixel_data);
    image.write_ppm(samples_per_pixel);
}
