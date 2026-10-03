/// straight with the form y=m*x+t
///
/// only meant to be used in [`StraightEquation`]
#[derive(Debug, Clone, PartialEq, Copy)]
struct Straight {
    m: f32,
    t: f32,
}

/// accumulates Straight equations to also allow for Y= and X=
#[derive(Debug, Clone, PartialEq, Copy)]
enum StraightEquation {
    Regular(Straight),
    X(f32),
    Y(f32),
}

/// straight with the form (x-p)*n=0
#[derive(Debug, Clone, PartialEq, Copy)]
struct NormalEquation {
    p: [f32; 2],
    n: [f32; 2],
}

impl NormalEquation {
    fn new(p: [f32; 2], n: [f32; 2]) -> Self {
        NormalEquation { p, n }
    }

    /// computes the y(x)
    pub fn y(&self, point: &Point) -> f32 {
        ((point.x - self.p[0]) * self.n[0]) + ((point.y - self.p[1]) * self.n[1])
    }

    /// converts the length of the normal vector to a unit-length vector
    pub fn normalize(&mut self) {
        let length = (self.p[0].powi(2) + self.p[1].powi(2)).powf(0.5);
        self.p = self.p.map(|x| x * (1.0 / length));
    }
}

/// point in a 2-dimensional coordinate system
#[derive(Debug, Clone, PartialEq, Copy)]
struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

/// a square in a 2-dimensional coordinate system, with a height and width of 1
#[derive(Debug, Clone, PartialEq, Copy)]
struct Square {
    center: Point,
    upper_edge: NormalEquation,
    lower_edge: NormalEquation,
    left_edge: NormalEquation,
    right_edge: NormalEquation,
}

impl Square {
    fn new(centre_coordinates: [f32; 2]) -> Self {
        let center = Point {
            x: centre_coordinates[0],
            y: centre_coordinates[1],
        };
        Self {
            center,
            upper_edge: NormalEquation::new([0.0, 1.0], [center.x + 0.5, center.y]),
            lower_edge: NormalEquation::new([0.0, -1.0], [center.x - 0.5, center.y]),
            left_edge: NormalEquation::new([-1.0, 0.0], [center.x, center.y - 0.5]),
            right_edge: NormalEquation::new([1.0, 0.0], [center.x, center.y + 0.5]),
        }
    }

    fn is_point_inside(&self, point: &Point) -> bool {
        self.upper_edge.y(point) <= 0.0
            && self.lower_edge.y(point) <= 0.0
            && self.left_edge.y(point) <= 0.0
            && self.right_edge.y(point) <= 0.0
    }
}

#[test]
fn normal_equation_compute() {
    let equation1 = NormalEquation::new([3.0, 3.0], [2.0, 1.0]);
    let null1 = Point::new(2.5, 4.0);
    let null2 = Point::new(2.0, 5.0);
    let larger_than_zero = Point::new(4.0, 4.0);
    let lower_than_zero = Point::new(0.0, 0.0);
    assert_eq!(equation1.y(&null1), 0.0);
    assert_eq!(equation1.y(&null2), 0.0);
    assert!((equation1.y(&larger_than_zero)) > 0.0);
    assert!((equation1.y(&lower_than_zero)) < 0.0);
}

#[test]
fn normal_equation_normalized() {
    let mut normal_equation = NormalEquation {
        p: [3.0, 4.0],
        n: [3.0, 4.0],
    };
    normal_equation.normalize();
    let normalized_equation = NormalEquation {
        p: [3.0 / 5.0, 4.0 / 5.0],
        n: [3.0, 4.0],
    };
    assert_eq!(normal_equation, normalized_equation);
}

#[test]
fn square_is_point_inside() {
    let origin_square = Square::new([0.0, 0.0]);
    // is inside
    assert!(origin_square.is_point_inside(&Point { x: 0.0, y: 0.0 }));
    // is outside
    assert!(!origin_square.is_point_inside(&Point { x: 0.6, y: 0.0 }));
    assert!(!origin_square.is_point_inside(&Point { x: 0.0, y: -0.6 }));
    assert!(!origin_square.is_point_inside(&Point { x: -0.6, y: 0.0 }));
    assert!(!origin_square.is_point_inside(&Point { x: 0.0, y: 0.6 }));
}
