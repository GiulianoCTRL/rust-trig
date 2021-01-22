#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn dst_to(&self, other: &Point) -> f32{
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
    }

    pub fn angle_to(&self, other: &Point) -> f32 {
        90f32 - ((self.x - other.x).abs() / self.dst_to(other)).asin() * 180.0 / std::f32::consts::PI
    }
}

/// All points in triangle follow common geometry naming schemes.
/// A, B, C are the three points of the triangle, a is the distance of the
/// vector/stretch opposite point A (BC, respectively CB), b opposite B (CA/AC)
/// and c describes the stretch opposite C (AB/BA).
/// alpha is the angle at point A, beta at point B, and gamma at point C. 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub A: Point,
    pub B: Point,
    pub C: Point,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
}


fn angle_from_cosines(adj1: f32, adj2: f32, opp: f32) -> f32 {
    ((adj1.powf(2.0) + adj2.powf(2.0) - opp.powf(2.0)) / (2.0 * adj1 * adj2)).acos() * 180.0 / std::f32::consts::PI
}

impl Triangle {
    fn from_three_points(A: Point, B: Point, C: Point) -> Triangle {
        let a = B.dst_to(&C);
        let b = C.dst_to(&A);
        let c = A.dst_to(&B);
        let alpha = angle_from_cosines(b, c, a);
        let beta = angle_from_cosines(c, a, b);
        let gamma = angle_from_cosines(a, b, c);
        Triangle { A,B, C, a, b, c, alpha, beta, gamma }
    }
}


mod tests {
    use super::*;
    use math::round;

    #[test]
    fn test_point_length() {
        let a = Point { x: 1.0, y: 3.0};
        let b = Point { x: 3.0, y: 1.0};
        let expected = 8f32.sqrt();
        let result = a.dst_to(&b);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_point_angle() {
        let a = Point { x: 1.0, y: 3.0};
        let b = Point { x: 3.0, y: 1.0};
        let expected = 45f64;
        let result = round::half_away_from_zero(a.angle_to(&b).into(), 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_triangle_from_three_points() {
        let A = Point {x: 4.0, y: 7.0};
        let B = Point {x: 12.0, y: 9.0};
        let C = Point {x: 8.0, y: 12.0};
        let expected_alpha = 37.3;
        let expected_beta = 50.9;
        let expected_gamma = 91.8;
        let result = Triangle::from_three_points(A, B, C);
        assert_eq!(expected_alpha, round::half_away_from_zero(result.alpha.into(), 1));
        assert_eq!(expected_beta, round::half_away_from_zero(result.beta.into(), 1));
        assert_eq!(expected_gamma, round::half_away_from_zero(result.gamma.into(), 1));
    }
}