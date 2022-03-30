use std::fs::File;
use std::io::{Error, Write};
use crate::V3;

pub fn export_obj_line(filename: &str, vs: &[V3]) {
    let mut s = String::new();
    s += "o Line\n";
    for v in vs.iter() {
        s.push_str(&format!("v {} {} {}\n", v.0, v.1, v.2));
    }
    s += "l";
    for i in (1..=vs.len()).chain(1..=1) {
        s.push_str(&format!(" {}", i));
    }

    write_file(filename, &s).unwrap();
}

pub fn write_file(filename: &str, content: &str) -> Result<(), Error> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    // write!(file, "{}", content)?;

    Ok(())
}

pub fn get_center(points: &[V3]) -> V3 {
    let center = points.iter().fold(V3::ORIGIN, |x, y| x + *y) / (points.len() as f64);
    center
}

pub fn export(points: &[V3], filename: &str) {
    let mut s = String::new();
    s.push_str(&format!("LM={}\n", points.len()));

    for p in points.iter() {
        s.push_str(&format!("{} {} {}\n", p.0, p.1, p.2));
    }

    write_file(filename, &s);
}
