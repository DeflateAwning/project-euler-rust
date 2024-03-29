use std::fs::File;
use std::io::prelude::Write;
use std::path::Path;
use reqwest;

const TEMP_FOLDER: &str = "temp102";
const FILE_NAME: &str = "0102_triangles.txt";

fn download_target_file() -> Result<(), Box<dyn std::error::Error>> {
    let file_url = "https://projecteuler.net/resources/documents/0102_triangles.txt";
    let target_path = format!("{}/{}", TEMP_FOLDER, FILE_NAME);

    let response_text: String = reqwest::blocking::get(file_url)?.text()?;
    let mut file = File::create(target_path)?;
    file.write_all(response_text.as_bytes())?;

    Ok(())
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

fn is_point_within_triangle(p: Point, t: Triangle) -> bool {
    let area = 0.5 * ((-t.b.y * t.c.x + t.a.y * (-t.b.x + t.c.x) + t.a.x * (t.b.y - t.c.y) + t.b.x * t.c.y) as f64);
    let s = 1.0 / (2.0 * area) * ((t.a.y * t.c.x - t.a.x * t.c.y + (t.c.y - t.a.y) * p.x + (t.a.x - t.c.x) * p.y) as f64);
    let t = 1.0 / (2.0 * area) * ((t.a.x * t.b.y - t.a.y * t.b.x + (t.a.y - t.b.y) * p.x + (t.b.x - t.a.x) * p.y) as f64);

    s > 0.0 && t > 0.0 && 1.0 - s - t > 0.0
}

fn main() {
    let temp_folder = Path::new(TEMP_FOLDER);
    if !temp_folder.exists() {
        std::fs::create_dir(temp_folder).unwrap();
    }
    let file_path = format!("{}/{}", TEMP_FOLDER, FILE_NAME);
    if !Path::new(&file_path).exists() {
        download_target_file().unwrap();
    }

    let origin_point = Point { x: 0, y: 0 };

    let mut count = 0;

    let file_content = std::fs::read_to_string(file_path).unwrap();
    for line in file_content.lines() {
        let points: Vec<i32> = line.split(',').map(|x| x.parse().unwrap()).collect();
        let triangle = Triangle {
            a: Point { x: points[0], y: points[1] },
            b: Point { x: points[2], y: points[3] },
            c: Point { x: points[4], y: points[5] },
        };

        if is_point_within_triangle(origin_point.clone(), triangle.clone()) {
            count += 1;
        }
    }

    println!("Number of triangles containing the origin: {}", count);

}
