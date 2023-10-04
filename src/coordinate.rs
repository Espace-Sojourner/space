use specs_derive::Component;
use specs::prelude::*;


//Position
#[derive(Component, Clone, Copy, Debug)]
pub struct Coordinate
{
    pub x: usize,
    pub y: usize,
    pub z: usize
}

impl Coordinate
{
    pub fn new(x: usize, y: usize, z: usize) -> Coordinate
    {
        Coordinate { x, y, z }
    }
}