
use rltk::RGB;
use specs_derive::Component;
use specs::prelude::*;


//Position
#[derive(Component)]
pub struct Position
{
    pub x: i32,
    pub y: i32,
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


