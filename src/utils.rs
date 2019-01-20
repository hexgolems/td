use crate::algebra::{Point, Vector};
use crate::assets::ImgID;
use crate::buffs::BuffType;
use ron;
use ron::de::from_reader;
use serde;
use std::fs::File;

pub fn distance(p1: &Point, p2: &Point) -> f32 {
    (p1 - p2).norm()
}

pub fn move_to(pos: Point, target: Point, speed: f32) -> (Point, bool) {
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

pub fn buff_to_img(buff: &BuffType) -> ImgID {
    return match buff {
        BuffType::Aura => ImgID::Aura,
        BuffType::Damage => ImgID::Damage,
        BuffType::RPM => ImgID::RPM,
        BuffType::Freeze => ImgID::Freeze,
        BuffType::Range => ImgID::Range,
    };
}
