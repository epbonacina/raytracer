use rand::Rng;

pub type Point3 = Vec3;

#[derive(Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 { e: [0.0; 3] }
    }

    pub fn new_with(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..1.0);
        let y = rng.gen_range(0.0..1.0);
        let z = rng.gen_range(0.0..1.0);
        Vec3::new_with(x, y, z)
    }

    pub fn random_with(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(min..max);
        let y = rng.gen_range(min..max);
        let z = rng.gen_range(min..max);
        Vec3::new_with(x, y, z)
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_with(-1.0, 1.0);
            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(&normal) > 0.0 {
            return on_unit_sphere;
        }
        return -&on_unit_sphere;
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn is_near_zero(&self) -> bool {
        let threshold = 1e-8;
        self.e.iter().all(|elem| elem < &threshold)
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        self - &(2.0 * &(self.dot(normal) * normal))
    }

    pub fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-self).dot(&normal).min(1.0);
        let r_out_perp = etai_over_etat * &(self + &(cos_theta * normal));
        let r_out_para = -&(1.0 - &r_out_perp.len_squared()).abs().sqrt() * normal;
        &r_out_para + &r_out_perp
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.len()
    }

    pub fn print_coords(&self) {
        println!("{} {} {}", self.x(), self.y(), self.z());
    }
}

impl std::ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl std::ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.x(), -self.y(), -self.z()],
        }
    }
}

impl std::ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl std::ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.x() * other.x(),
                self.y() * other.y(),
                self.z() * other.z(),
            ],
        }
    }
}

impl std::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, value: f64) -> Vec3 {
        Vec3 {
            e: [self.x() * value, self.y() * value, self.z() * value],
        }
    }
}

impl std::ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vector: &Vec3) -> Vec3 {
        Vec3 {
            e: [self * vector.x(), self * vector.y(), self * vector.z()],
        }
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, value: f64) {
        self.e[0] *= value;
        self.e[1] *= value;
        self.e[2] *= value;
    }
}

impl std::ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, value: f64) -> Vec3 {
        Vec3 {
            e: [self.x() / value, self.y() / value, self.z() / value],
        }
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, value: f64) {
        self.e[0] /= value;
        self.e[1] /= value;
        self.e[2] /= value;
    }
}

impl std::ops::Index<usize> for &Vec3 {
    type Output = f64;

    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        &self.e[i]
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}
