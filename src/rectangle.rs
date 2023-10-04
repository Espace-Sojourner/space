use super::coordinate::Coordinate;

#[derive(Clone, Copy)]
pub struct Rectangle
{
    pub corner_one: Coordinate,
    pub corner_two: Coordinate,
}

impl Rectangle
{
    pub fn new(corner_one: Coordinate, corner_two: Coordinate) -> Rectangle
    {
        Rectangle { corner_one, corner_two }
    }

    pub fn intersect(&self, other: &Rectangle) -> bool
    {
        self.corner_one.x <= other.corner_two.x && 
        self.corner_two.x >= other.corner_one.x &&
        self.corner_one.y <= other.corner_two.y && 
        self.corner_two.y >= other.corner_one.y
    }

    pub fn center(&self) -> Coordinate
    {
        Coordinate { x: (self.corner_one.x + self.corner_two.x)/2, y: (self.corner_one.y + self.corner_two.y)/2, z: self.corner_one.z }
    }
}