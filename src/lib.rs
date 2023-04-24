use algebra::Vec3;
use std::rc::Rc;

pub mod algebra;
pub mod camera;
pub mod image;

pub type Point = Vec3;

// Ray can be thought of as equation P(t) = A + t*b
// which gives a point on a 3D line
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}

#[derive(Default, Clone, Copy)]
pub struct Hit {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, out: &mut Hit) -> bool;
}

#[derive(Default)]
pub struct World {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, out: &mut Hit) -> bool {
        let mut found_hit = false;
        let mut closest_hit = t_max;
        let hit = &mut Hit::default();

        for obj in &self.objects {
            if obj.hit(ray, t_min, closest_hit, hit) {
                found_hit = true;
                closest_hit = hit.t;
                *out = *hit;
            }
        }

        return found_hit;
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random() -> f64 {
    fastrand::f64()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}
