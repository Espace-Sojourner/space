
use rltk::RGB;
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

//Renderable
#[derive(Component)]
pub struct Renderable
{
    pub glyph: rltk::FontCharType,
    pub foreground_color: RGB,
    pub background_color: RGB,
}

//Player
#[derive(Component)]
pub struct Player {}


