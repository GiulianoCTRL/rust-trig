#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// Describe vector AB. As not all values of a vector are always needed vectors
/// are initialized by default. Values will be initialized when called or when
/// calling the vectors init method.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    point_a: Point,
    point_b: Point,
    length: Option<f32>,
    alpha: Option<f32>,
    beta: Option<f32>,
}

impl Vector {
    /// Return a uninitialized vector, containing the two known passed coordinates
    pub fn new(point_a: Point, point_b: Point) -> Vector {
        Vector {
            point_a,
            point_b,
            length: None,
            alpha: None,
            beta: None,
        }
    }

    /// Return a new initialized vector
    pub fn new_initialized(point_a: Point, point_b: Point) -> Vector {
        let mut v = Vector {
            point_a,
            point_b,
            length: None,
            alpha: None,
            beta: None,
        };
        v.init();
        v
    }

    /// Initialize the vector by setting length, alpha and beta
    pub fn init(&mut self) -> () {
        self.length();
        self.set_alpha_beta();
    }

    /// If length is not None length will be returned, else length will be calculated.
    /// length will be initialized after this function has been called.
    pub fn length(&mut self) -> f32 {
        match self.length {
            Some(f) => f,
            None => {
                let opposite = self.point_a.x - self.point_b.x;
                let adjacent = self.point_a.y - self.point_b.y;
                let hypotenuse = (opposite.powf(2.0) + adjacent.powf(2.0)).sqrt();
                self.length = Some(hypotenuse);
                hypotenuse
            }
        }
    }

    /// Intitialize angle alpha and beta by creating a right angled triangle
    /// and calculating the remaining angles.
    fn set_alpha_beta(&mut self) -> () {
        let opposite = self.point_a.x - self.point_b.x;
        let adjacent = self.point_a.y - self.point_b.y;
        let beta =
            (opposite.powf(2f32) / adjacent.powf(2f32)).atan() * 180f32 / std::f32::consts::PI;
        self.alpha = Some(90f32 - beta);
        self.beta = Some(beta);
    }

    /// If alpha is None, initialize angles and return alpha, else return alpha
    pub fn alpha(&mut self) -> f32 {
        match self.alpha {
            Some(f) => f,
            None => {
                self.set_alpha_beta();
                self.alpha.unwrap()
            }
        }
    }

    /// If beta is None, initialize angles and return beta, else return alpha
    pub fn beta(&mut self) -> f32 {
        match self.beta {
            Some(f) => f,
            None => {
                self.set_alpha_beta();
                self.beta.unwrap()
            }
        }
    }
}

/// All points in triangle follow common geometry naming schemes.
/// point_a, point_b, point_c are the three points of the triangle, a is the distance of the
/// vector/stretch opposite point point_a (BC, respectively CB), b opposite point_b (CA/AC)
/// and c describes the stretch opposite point_c (AB/BA).
/// alpha is the angle at point point_a, beta at point point_b, and gamma at point point_c.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Triangle {
    pub point_a: Point,
    pub point_b: Point,
    pub point_c: Point,
    ab: Option<f32>,
    bc: Option<f32>,
    ca: Option<f32>,
    alpha: Option<f32>,
    beta: Option<f32>,
    gamma: Option<f32>,
}


impl Triangle {
    pub fn new(point_a: Point, point_b: Point, point_c: Point) -> Triangle {
        Triangle {
            point_a,
            point_b,
            point_c,
            ab: None,
            bc: None,
            ca: None,
            alpha: None,
            beta: None,
            gamma: None,
        }
    }

    pub fn new_initialized(point_a: Point, point_b: Point, point_c: Point) -> Triangle {
        let mut t = Triangle {
            point_a,
            point_b,
            point_c,
            ab: None,
            bc: None,
            ca: None,
            alpha: None,
            beta: None,
            gamma: None,
        };
        t.init();
        t
    }

    /// All length values are initialized together, as it is likely to request more than just on length
    /// when using triangle  calculations.
    fn init_lengths(&mut self) -> () {
        self.ab = Some(Vector::new(self.point_a, self.point_b).length());
        self.bc = Some(Vector::new(self.point_b, self.point_c).length());
        self.ca = Some(Vector::new(self.point_c, self.point_a).length());
    }

    /// Applied law of cosines -> This function might move outside this struct in the future!
    fn get_angle(adj1: f32, adj2: f32, opp: f32) -> f32 {
        ((adj1.powf(2.0) + adj2.powf(2.0) - opp.powf(2.0)) / (2.0 * adj1 * adj2)).acos() * 180.0
            / std::f32::consts::PI
    }

    fn init(&mut self) -> () {
        self.init_lengths();
        self.init_angles();
    }

    fn init_angles(&mut self) -> () {
        self.alpha = Some(Triangle::get_angle(self.ab(), self.ca(), self.bc()));
        self.beta = Some(Triangle::get_angle(self.bc(), self.ab(), self.ca()));
        self.gamma = Some(Triangle::get_angle(self.ca(), self.bc(), self.ab()));
    }

    pub fn ab(&mut self) -> f32 {
        match self.ab {
            Some(f) => f,
            None => {
                self.init_lengths();
                self.ab.unwrap()
            }
        }
    }

    pub fn bc(&mut self) -> f32 {
        match self.bc {
            Some(f) => f,
            None => {
                self.init_lengths();
                self.bc.unwrap()
            }
        }
    }

    pub fn ca(&mut self) -> f32 {
        match self.ca {
            Some(f) => f,
            None => {
                self.init_lengths();
                self.ca.unwrap()
            }
        }
    }

    pub fn alpha(&mut self) -> f32 {
        match self.alpha {
            Some(f) => f,
            None => {
                self.init_angles();
                self.alpha.unwrap()
            }
        }
    }

    pub fn beta(&mut self) -> f32 {
        match self.beta {
            Some(f) => f,
            None => {
                self.init_angles();
                self.beta.unwrap()
            }
        }
    }

    pub fn gamma(&mut self) -> f32 {
        match self.gamma {
            Some(f) => f,
            None => {
                self.init_angles();
                self.gamma.unwrap()
            }
        }
    }
}

mod tests {
    use super::*;
    use math::round;

    #[test]
    fn test_vector_length() {
        let a = Point { x: 1.0, y: 3.0 };
        let b = Point { x: 3.0, y: 1.0 };
        let mut v = Vector::new(a, b);
        let expected = 8f32.sqrt();
        let result = v.length();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_point_angle() {
        let a = Point { x: 1.0, y: 3.0 };
        let b = Point { x: 3.0, y: 1.0 };
        let mut v = Vector::new(a, b);
        let expected = 45f64;
        let result = round::half_away_from_zero(v.alpha().into(), 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_triangle_from_three_points() {
        let point_a = Point {x: 4.0, y: 7.0};
        let point_b = Point {x: 12.0, y: 9.0};
        let point_c = Point {x: 8.0, y: 12.0};
        let expected_alpha = 37.3;
        let expected_beta = 50.9;
        let expected_gamma = 91.8;
        let mut result = Triangle::new_initialized(point_a, point_b, point_c);
        println!("{:?}", result);
        assert_eq!(expected_alpha, round::half_away_from_zero(result.alpha().into(), 1));
        assert_eq!(expected_beta, round::half_away_from_zero(result.beta().into(), 1));
        assert_eq!(expected_gamma, round::half_away_from_zero(result.gamma().into(), 1));
    }
}
