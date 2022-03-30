use std::env;
use std::collections::HashSet;
use clap::Parser;
use slice::mesh::Mesh;
use slice::plane::Plane;
use slice::v3::V3;

// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     #[clap(long)]
//     name: String,
// }

fn main() {
    // let args: Args = Args::parse();
    // let name = args.name;
    let args: Vec<_> = env::args().collect();

    let mut result = Vec::new();
    for i in slice::app1(&args[1]) {
        result.push(i);
    }
    for i in slice::intersect_top(&args[2]) {
        result.push(i);
    }

    println!("{:?}", result);
    slice::utils::export(&result, "output.txt");

    // let inter = mesh.intersect_plane_line(&plane);
    // println!("{:?}", inter);
}
