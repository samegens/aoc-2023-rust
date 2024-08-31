use common::Point;

#[derive(Debug, PartialEq)]
pub struct PartNr {
    nr: u32,
    position: Point,
    len: usize
}

impl PartNr {
    pub fn new(nr: u32, position: Point) -> Self {
        let len: usize = nr.to_string().len();
        PartNr { nr, position, len }
    }

    pub fn nr(&self) -> u32 {
        self.nr
    }

    pub fn position(&self) -> &Point {
        &self.position
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn contains_point(&self, p: Point) -> bool {
        p.y == self.position.y && p.y >= self.position.x && p.x < self.position.x + self.len as i64
    }
}
