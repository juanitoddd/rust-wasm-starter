use web_sys::Path2d;
use crate::game::utils::coord::Coord;

#[derive(Debug, Clone)]
pub struct Rectangle {
    path: Path2d,
    coord: Coord,
    x: f64,
    y: f64,
}

impl Rectangle {
    pub fn new(path: Path2d, coord: Coord, x: f64, y: f64) -> Rectangle {
        Rectangle { path, coord, x, y }
    }

    pub fn get_path(&self) -> &Path2d {
        &self.path
    }

    pub fn get_coord(&self) -> &Coord {
        &self.coord
    }

    pub fn get_pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}