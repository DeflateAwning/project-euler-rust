use roots::{self, Roots};
use core::f64::consts::PI;

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

fn get_slope_at_point(p: &Point) -> f64 {
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

fn get_next_point_after_reflection(p1: &Point, p2: &Point) -> Point {
    // reflection occurs at p2

    let tangent_slope_at_p2 = get_slope_at_point(p2);
    let tangent_angle_at_p2_radians = tangent_slope_at_p2.atan();

    let prev_laser_angle_radians = angle_from_one_point_to_another(p1, p2);
    println!("prev_laser_angle_degrees: {}", prev_laser_angle_radians.to_degrees());
    println!("tangent_angle_at_p2_degrees: {}", tangent_angle_at_p2_radians.to_degrees());

    // now, never use p1 again; p2 is the source, and we're trying to find the next one after

    let prev_laser_angle_other_direction_radians = prev_laser_angle_radians + PI;
    println!("prev_laser_angle_other_direction_degrees: {}", prev_laser_angle_other_direction_radians.to_degrees());

    let angle_from_tangent_radians = prev_laser_angle_other_direction_radians - tangent_angle_at_p2_radians; // e.g., 65 degrees
    println!("angle_from_tangent_degrees: {}", angle_from_tangent_radians.to_degrees());

    let angle_from_normal_radians = (PI - 2.0 * (angle_from_tangent_radians)) / 2.0;
    println!("angle_from_normal_degrees: {}", angle_from_normal_radians.to_degrees());

    // TODO: in two quadrants, I think the next line needs to have the opposite sign

    let new_laser_angle_radians = prev_laser_angle_other_direction_radians + (2.0 * angle_from_normal_radians);
    println!("new_laser_angle_degrees: {}", new_laser_angle_radians.to_degrees());

    let new_laser_slope = new_laser_angle_radians.tan(); // TODO: check if this is correct
    println!("new_laser_slope: {}", new_laser_slope);

    // using the new slope (assuming it's right), find the new point
    let const_a = (-new_laser_slope * p2.x) + p2.y; // some const that made sense to factor out
    println!("const_a: {}", const_a);

    let new_x_roots = roots::find_roots_quadratic(
        4.0 + new_laser_slope.powi(2),
        2.0 * new_laser_slope * const_a,
        const_a.powi(2) - 100.0,
    );

    // TODO: figure out how to unpack the roots tuple directly
    let new_point: Point = match new_x_roots {
        Roots::No(_) => panic!("No roots found"),
        Roots::One(_new_x) => Point { x: _new_x[0], y: (new_laser_slope * _new_x[0]) + const_a },
        Roots::Two(new_x_roots) => {
            let new_x_try1 = new_x_roots[0];
            let new_x_try2 = new_x_roots[1];

            // figure out which root is the correct one
            let new_point_try1 = Point { x: new_x_try1, y: (new_laser_slope * new_x_try1) + const_a };
            let new_point_try2 = Point { x: new_x_try2, y: (new_laser_slope * new_x_try2) + const_a };

            let is_try1_good = feq(angle_from_one_point_to_another(&p2, &new_point_try1), new_laser_angle_radians);
            let is_try2_good = feq(angle_from_one_point_to_another(&p2, &new_point_try2), new_laser_angle_radians);

            println!("Point, is_good - try1: {} ({}), try2: {} ({})",
                new_point_try1.to_string(), is_try1_good, new_point_try2.to_string(), is_try2_good);

            if !(is_point_on_ellipse(&new_point_try1) && is_point_on_ellipse(&new_point_try2)) {
                panic!("A new_point try is not on the ellipse.");
            }

            if is_try1_good {
                new_point_try1
            } else if is_try2_good {
                new_point_try2
            }
            else {
                panic!("Neither point matches the angle.");
            }
        }
        _ => panic!("Unexpected number of roots found."),
    };

    new_point
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
