use ggez::graphics::Point2;
use ron;
use ron::de::from_reader;
use serde;
use std::fs::File;

pub fn distance(p1: &Point2, p2: &Point2) -> f32 {
    (p1 - p2).norm()
}

pub fn move_to(pos: Point2, target: Point2, speed: f32) -> (Point2, bool) {
    assert!(speed > 0.0);
    let dir = target - pos;
    let len = dir.norm();
    if speed > len {
        return (target, true);
    }
    let norm = dir / len;
    let update = norm * speed;
    return (pos + update, false);
}

pub fn add_mod(value: usize, op: isize, modulus: usize) -> usize {
    if op < 0 {
        return (value + (modulus as isize + (op % modulus as isize)) as usize) % modulus;
    }
    return (value + op as usize) % modulus;
}

pub fn load_specs<T>(name: &str) -> Vec<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    let spec_path = format!("resources/rons/{}.ron", name);
    let f = File::open(&spec_path).expect(&format!("Failed opening {}.ron", name));
    let specs = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load {}.ron: {}", name, e);
            ::std::process::exit(1);
        }
    };
    return specs;
}
