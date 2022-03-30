use crate::V3;

pub struct Plane {
    pub pos: V3,
    pub normal: V3,
}

impl Plane {
    pub fn intersect_segment(&self, p1: V3, p2: V3) -> Option<V3> {
        let maybe_point = self.intersect_line(p1, p2);
        if maybe_point.is_none() {
            None
        } else {
            let p = maybe_point.unwrap();
            let t = (p1 - p).dot(p2 - p);
            if t > 0.0 {
                None
            } else {
                Some(p)
            }
        }
    }

    pub fn intersect_line(&self, p1: V3, p2: V3) -> Option<V3> {
        let t = self.normal.dot(p2 - p1);

        if t.abs() < 1e-6 {
            None
        } else {
            let k = self.normal.dot(self.pos - p1) / t;
            let p = p1 + (p2 - p1) * k;
            Some(p)
        }
    }
}
