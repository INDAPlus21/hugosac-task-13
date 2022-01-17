mod vector;
use vector::Vector3;

fn main() {
    let u: Vector3 = Vector3::new(1.0, 0.0, 0.0);
    let v: Vector3 = Vector3::new(0.0, 1.0, 0.0);
    println!("{}", Vector3::dot(u,v));
    u += -u;
    println!("{}", (u).to_string());
}
