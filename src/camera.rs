use crate::{
    color::Color,
    hittable::Hittable,
    ray::Ray,
    utils,
    vec3::{Point3, Vec3},
};
use rand::Rng;

pub const MAX_COLOR: u8 = 255;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u16,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub vfov: f64,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub samples_per_pixel: u16,
    pub max_bounces: u16,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    image_height: u16,
    center: Point3,
    pixel_samples_scale: f64,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let image_width = 400;
        let aspect_ratio = 16.0 / 9.0;
        let vfov = 90.0;

        let lookfrom = Point3::new_with(0.0, 0.0, 0.0);
        let lookat = Point3::new_with(0.0, 0.0, -1.0);
        let vup = Vec3::new_with(0.0, 1.0, 0.0);

        let samples_per_pixel = 100;
        let max_bounces = 50;

        let defocus_angle = 0.0;
        let focus_dist = 10.0;

        Camera {
            image_width,
            aspect_ratio,
            max_bounces,
            samples_per_pixel,
            lookfrom,
            lookat,
            vup,
            vfov,
            defocus_angle,
            focus_dist,
            image_height: 0,
            u: Vec3::new(),
            w: Vec3::new(),
            v: Vec3::new(),
            center: Point3::new(),
            pixel_samples_scale: 0.0,
            pixel00_loc: Point3::new(),
            pixel_delta_u: Vec3::new(),
            pixel_delta_v: Vec3::new(),
            defocus_disk_u: Vec3::new(),
            defocus_disk_v: Vec3::new(),
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u16;
        self.image_height = std::cmp::max(self.image_height, 1);

        self.center = self.lookfrom.clone();
        let theta = utils::degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (&self.lookfrom - &self.lookat).unit_vector();
        self.u = self.vup.cross(&self.w);
        self.v = self.w.cross(&self.u);

        let viewport_u = viewport_width * &self.u;
        let viewport_v = viewport_height * &(-&self.v);

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        self.pixel_delta_u = &viewport_u / self.image_width as f64;
        self.pixel_delta_v = &viewport_v / self.image_height as f64;

        let viewport_upper_left = &(&(&self.center - &(self.focus_dist * &self.w))
            - &(&viewport_u / 2.0))
            - &(&viewport_v / 2.0);

        self.pixel00_loc =
            &viewport_upper_left + &(0.5 * &(&self.pixel_delta_u + &self.pixel_delta_v));

        let defocus_radius =
            self.focus_dist * utils::degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = &self.u * defocus_radius;
        self.defocus_disk_v = &self.v * defocus_radius;
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            log(&format!(
                "Iteration {} (out of {}) [{:.2}%]\r",
                j,
                self.image_height,
                (j as f64 / self.image_height as f64) * 100.0
            ));
            for i in 0..self.image_width {
                let mut pixel_color = Color::new();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&ray, self.max_bounces, world);
                }
                let pixel_color = self.pixel_samples_scale * &pixel_color;
                pixel_color.print();
            }
        }
    }

    fn get_ray(&self, i: u16, j: u16) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = &(&self.pixel00_loc + &((i as f64 + offset.x()) * &self.pixel_delta_u))
            + &((j as f64 + offset.y()) * &self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center.clone()
        } else {
            self.defocus_disk_samples()
        };
        let ray_direction = &pixel_sample - &ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        let mut rng = rand::thread_rng();
        let num1 = rng.gen_range(-0.5..0.5);
        let num2 = rng.gen_range(-0.5..0.5);
        Vec3::new_with(num1, num2, 0.0)
    }

    fn defocus_disk_samples(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        &(&self.center + &(p.x() * &self.defocus_disk_u)) + &(p.y() * &self.defocus_disk_v)
    }

    fn ray_color(&self, ray: &Ray, depth: u16, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new();
        }

        match world.hits(ray, 0.001..utils::INFINITY) {
            Some(hit_record) => {
                if let Some((attenuation, scattered)) =
                    hit_record.material.scatter(ray, &hit_record)
                {
                    let color = self.ray_color(&scattered, depth - 1, world);
                    return &color * &attenuation;
                }
                return Color::new();
            }
            None => {
                let unit_direction = ray.direction().unit_vector();
                let a = 0.5 * (unit_direction.y() + 1.0);
                &((1.0 - a) * &Color::new_with(1.0, 1.0, 1.0))
                    + &(a * &Color::new_with(0.5, 0.7, 1.0))
            }
        }
    }
}

fn log(text: &str) {
    eprint!("{}", text);
}
