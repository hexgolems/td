#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Dir {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}
use self::Dir::*;

// DO NOT CHANGE THE ORDER :)
pub const DIRECTIONS: [Dir; 6] = [East, NorthEast, NorthWest, West, SouthWest, SouthEast];
