use std::collections::HashSet;
use crate::mesh::Mesh;
use crate::plane::Plane;
use crate::utils::{get_center, write_file};
use crate::v3::V3;

pub mod mesh;
pub mod v3;
pub mod ray;
pub mod plane;
pub mod utils;


pub fn intersect_top(filename: &str) -> Vec<V3> {
    // let mesh = Mesh::from_file(filename);
    let mut points_indices = HashSet::new();
    let mut points = Vec::new();

    let mesh = Mesh::from_file(filename, false);
    for i in 0..mesh.line_count() {
        let index1 = mesh.mesh.indices[i * 2];
        let index2 = mesh.mesh.indices[i * 2 + 1];

        points_indices.insert(index1);
        points_indices.insert(index2);
    }

    for index in points_indices {
        let point = mesh.get_vertex(index as usize);
        points.push(point);
    }

    let center = get_center(&points);

    let mut result = Vec::new();
    for (index, ray) in center.radiative_ray(1, 32).enumerate() {
        if index == 16 {
            break;
        }
        let plane = Plane {
            pos: center,
            normal: ray.direction
        };

        let inter = mesh.intersect_plane_line(&plane);
        // println!("{:?}", inter);
        for p in inter {
            result.push(p);
        }
    }

    result
}

pub fn app1(filename: &str) -> Vec<V3> {
    let mesh = Mesh::from_file(filename, true);
    let bound = mesh.bound(1);
    let length = bound.1 - bound.0;

    let d0 = bound.0 + length * 0.25;
    let d1 = bound.0 + length * 0.5;
    let d2 = bound.0 + length * 0.75;

    let min_y = mesh.iter_vertex().reduce(|x, y| {
        if x.1 < y.1 { x } else { y }
    }).unwrap();
    let max_y = mesh.iter_vertex().reduce(|x, y| if x.1 < y.1 { y } else { x }).unwrap();
    println!("min y point: {:?}", min_y);
    println!("max y point: {:?}", max_y);

    let ds = vec![d2, d1, d0];

    // let mut s = String::from("LM=97\n");

    let mut result = Vec::new();
    for (index, &d) in ds.iter().enumerate() {
        let plane = Plane {
            pos: V3(0.0, d, 0.0),
            normal: V3(0.0, 1.0, 0.0)
        };

        let points = mesh.intersect_plane(&plane);
        let center = points.iter().fold(V3::ORIGIN, |x, y| x + *y) / (points.len() as f64);

        for r in center.radiative_ray(1, 32) {
            let intersect = mesh.intersect_ray(&r);
            let v = intersect[0].0;
            result.push(v);
            // s.push_str(&format!("{} {} {}\n", v.0, v.1, v.2));
        }
    }

    result.push(min_y);
    result.push(max_y);

    result

    // s.push_str(&format!("{} {} {}", min_y.0, min_y.1, min_y.2));

    // write_file("output.txt", &s);
}