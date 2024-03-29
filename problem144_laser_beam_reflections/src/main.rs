use roots::{self, Roots};
use core::f64::consts::PI;
use vecmath::{Vector2};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

fn get_ellipse_slope_at_point(p: &Point) -> f64 {
    -4.0 * p.x / p.y
}

fn feq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

fn is_point_on_ellipse(p: &Point) -> bool {
    feq(
        (4.0 * p.x.powi(2)) + p.y.powi(2),
        100.0,
    )
}

fn angle_from_one_point_to_another(p1: &Point, p2: &Point) -> f64 {
    (p2.y - p1.y).atan2(p2.x - p1.x)
}

fn get_next_point_after_reflection(prev_point: &Point, reflec_point: &Point) -> Point {
    // reflection occurs at "reflec_point"

    let incoming_vector = points_to_vector(*prev_point, *reflec_point);

    let tan_slope_at_reflec_point = get_ellipse_slope_at_point(reflec_point);
    let norm_slope_at_reflec_point = -1.0 / tan_slope_at_reflec_point;

    let outgoing_slope = get_slope_of_outgoing_bounce(
        vector_to_slope(incoming_vector),
        norm_slope_at_reflec_point,
    );

    // Recall, atan always returns an angle in Q1 or Q4.
    // If the reflection point is in Q2 or Q3 (i.e., neg Y point), that's good! The angle will be going up.
    // If the reflection point is in Q1 or Q4 (i.e., pos Y point), we need to ensure the angle is pointed downwards by adding PI.
    let outgoing_angle_radians =
        if reflec_point.y <= 0.0 {
            outgoing_slope.atan()
        } else {
            outgoing_slope.atan() + PI
    };

    let const_a = (-outgoing_slope * reflec_point.x) + reflec_point.y; // some const that made sense to factor out
    println!("const_a: {}", const_a);

    let next_point_x_roots = roots::find_roots_quadratic(
        4.0 + outgoing_slope.powi(2),
        2.0 * outgoing_slope * const_a,
        const_a.powi(2) - 100.0,
    );

    // TODO: figure out how to unpack the roots tuple directly
    let next_point: Point = match next_point_x_roots {
        Roots::No(_) => panic!("No roots found"),
        Roots::One(next_x) => Point { x: next_x[0], y: (outgoing_slope * next_x[0]) + const_a },
        Roots::Two(next_point_x_roots) => {
            let next_point_x_try1 = next_point_x_roots[0];
            let next_point_x_try2 = next_point_x_roots[1];

            // figure out which root is the correct one
            let next_point_try1 = Point { x: next_point_x_try1, y: (outgoing_slope * next_point_x_try1) + const_a };
            let next_point_try2 = Point { x: next_point_x_try2, y: (outgoing_slope * next_point_x_try2) + const_a };

            let is_try1_good = feq(angle_from_one_point_to_another(&reflec_point, &next_point_try1), outgoing_angle_radians);
            let is_try2_good = feq(angle_from_one_point_to_another(&reflec_point, &next_point_try2), outgoing_angle_radians);

            println!("Point, is_good - try1: {} ({}), try2: {} ({})",
                next_point_try1.to_string(), is_try1_good, next_point_try2.to_string(), is_try2_good);

            // valid points should be on the ellipse
            if !is_point_on_ellipse(&next_point_try1) && !is_point_on_ellipse(&next_point_try2) {
                panic!("Neither next_point try is on the ellipse.");
            }
            else if !is_point_on_ellipse(&next_point_try1) {
                panic!("next_point_try1 is not on the ellipse.");
            }
            else if !is_point_on_ellipse(&next_point_try2) {
                panic!("next_point_try2 is not on the ellipse.");
            }

            // decide on a final point
            if is_try1_good {
                next_point_try1
            } else if is_try2_good {
                next_point_try2
            }
            else {
                panic!("Neither point matches the angle.");
            }
        }
        _ => panic!("Unexpected number of roots found."),
    };

    next_point
}

fn points_to_vector(p1: Point, p2: Point) -> Vector2<f64> {
    Vector2::from([p2.x - p1.x, p2.y - p1.y])
}

fn vector_to_slope(vector2: Vector2<f64>) -> f64 {
    vector2[1] / vector2[0]
}

fn get_slope_of_outgoing_bounce(incoming_slope: f64, normal_slope: f64) -> f64 {
    // Source: https://math.stackexchange.com/a/2239245
    // Solution (uses sympy in Python):
        // from sympy import symbols, Eq, solve
        // k1, k2, k3 = symbols('incoming_slope normal_slope outgoing_slope')
        // equation = Eq((k1 - k2) / (1 + (k1*k2)), (k2 - k3) / (1 + (k2*k3)))
        // solution = solve(equation, k3)
        // print(solution)    

    (incoming_slope*normal_slope*normal_slope - incoming_slope + 2.0*normal_slope) / 
        (2.0*incoming_slope*normal_slope - normal_slope*normal_slope + 1.0)
}

fn main() {
    println!("Starting...");

    let initial_point_p0 = Point { x: 0.0, y: 10.1 };
    let initial_bounce_point_p1 = Point { x: 1.4, y: -9.6 };

    let mut point_last = initial_point_p0;
    let mut point_now = initial_bounce_point_p1;

    let mut bounce_num: u32 = 1;

    loop {
        println!("=== Bounce {}: {}", bounce_num, point_now.to_string());

        let next_point = get_next_point_after_reflection(&point_last, &point_now);

        if feq(next_point.x, initial_point_p0.x) && feq(next_point.y, initial_point_p0.y) {
            println!("Found the point: {}", next_point.to_string());
            break;
        }
        
        point_last = point_now;
        point_now = next_point;
        bounce_num += 1;

        if bounce_num > 3 {
            break;
            // panic!("Too many bounces");
        }
    }

    println!("End.");
}
