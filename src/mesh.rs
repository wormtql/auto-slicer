use std::cmp::Ordering;
use crate::plane::Plane;
use super::v3::V3;
use crate::ray::Ray;

pub struct Mesh {
    pub mesh: tobj::Mesh,
}

pub fn intersect_face(p1: V3, p2: V3, p3: V3, ray: &Ray) -> Option<(V3, f64)> {
    let t1 = p1 - p2;
    let t2 = p2 - p3;
    let normal = t1.cross(t2);

    let normal_x_d = normal.dot(ray.direction);
    if normal_x_d.abs() < 1e-6 {
        return None;
    }

    let d = -normal.dot(p1);
    let t = -(normal.dot(ray.start) + d) / (normal.dot(ray.direction));
    if t < 1e-6 {
        return None;
    }

    let new_point = ray.start + ray.direction * t;

    let flag = (p2 - p1).cross(new_point - p2).dot(normal) > 0.0
        && (p3 - p2).cross(new_point - p3).dot(normal) > 0.0
        && (p1 - p3).cross(new_point - p1).dot(normal) > 0.0;
    if !flag {
        return None;
    }

    Some((new_point, t))
}

impl Mesh {
    pub fn from_file(filename: &str, triangulate: bool) -> Mesh {
        let (mut models, materials) = tobj::load_obj(
            &filename,
            &tobj::LoadOptions {
                triangulate,
                ..Default::default()
            }
        ).expect("Failed to OBJ load file");

        let model = models.remove(0);

        Mesh {
            mesh: model.mesh
        }
    }

    pub fn face_count(&self) -> usize {
        self.mesh.indices.len() / 3
    }

    pub fn vertex_count(&self) -> usize {
        self.mesh.positions.len() / 3
    }

    pub fn line_count(&self) -> usize {
        self.mesh.indices.len() / 2
    }

    pub fn get_vertex(&self, index: usize) -> V3 {
        V3(
            self.mesh.positions[index * 3] as f64,
            self.mesh.positions[index * 3 + 1] as f64,
            self.mesh.positions[index * 3 + 2] as f64,
        )
    }

    pub fn intersect_ray(&self, ray: &Ray) -> Vec<(V3, f64)> {
        let face_count = self.face_count();
        let mut results = Vec::new();
        for i in 0..face_count {
            let index1 = self.mesh.indices[i * 3] as usize;
            let index2 = self.mesh.indices[i * 3 + 1] as usize;
            let index3 = self.mesh.indices[i * 3 + 2] as usize;

            let p1 = self.get_vertex(index1);
            let p2 = self.get_vertex(index2);
            let p3 = self.get_vertex(index3);

            if let Some(x) = intersect_face(p1, p2, p3, ray) {
                results.push(x)
            }
        }

        results.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());

        results
    }

    pub fn intersect_plane_line(&self, plane: &Plane) -> Vec<V3> {
        let mut result = Vec::new();
        for i in 0..self.line_count() {
            let index1 = self.mesh.indices[i * 2];
            let index2 = self.mesh.indices[i * 2 + 1];
            let p1 = self.get_vertex(index1 as usize);
            let p2 = self.get_vertex(index2 as usize);

            if let Some(x) = plane.intersect_segment(p1, p2) {
                result.push(x);
            }
        }

        result
    }

    pub fn intersect_plane(&self, plane: &Plane) -> Vec<V3> {
        let face_count = self.face_count();
        let mut results = Vec::new();

        for i in 0..face_count {
            let index1 = self.mesh.indices[i * 3] as usize;
            let index2 = self.mesh.indices[i * 3 + 1] as usize;
            let index3 = self.mesh.indices[i * 3 + 2] as usize;

            let p1 = self.get_vertex(index1);
            let p2 = self.get_vertex(index2);
            let p3 = self.get_vertex(index3);

            if let Some(x) = plane.intersect_segment(p1, p2) {
                results.push(x);
            }
            if let Some(x) = plane.intersect_segment(p1, p3) {
                results.push(x);
            }
            if let Some(x) = plane.intersect_segment(p2, p3) {
                results.push(x);
            }
        }

        results
    }

    pub fn bound(&self, axis: usize) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = -f64::INFINITY;

        let vertex_count = self.vertex_count();

        for i in 0..vertex_count {
            let v = self.get_vertex(i);
            if axis == 0 {
                min = min.min(v.0);
                max = max.max(v.0);
            } else if axis == 1 {
                min = min.min(v.1);
                max = max.max(v.1);
            } else {
                min = min.min(v.2);
                max = max.max(v.2);
            }
        }

        (min, max)
    }

    pub fn iter_vertex(&self) -> PointIter {
        PointIter {
            mesh: &self.mesh,
            next: 0
        }
    }
}

pub struct PointIter<'a> {
    pub mesh: &'a tobj::Mesh,
    pub next: usize,
}

impl<'a> Iterator for PointIter<'a> {
    type Item = V3;
    fn next(&mut self) -> Option<Self::Item> {
        let vertex_count = self.mesh.positions.len() / 3;
        if self.next == vertex_count {
            None
        } else {
            let result = Some(V3(
                self.mesh.positions[self.next * 3] as f64,
                self.mesh.positions[self.next * 3 + 1] as f64,
                self.mesh.positions[self.next * 3 + 2] as f64,
            ));

            self.next += 1;
            result
        }
    }
}
