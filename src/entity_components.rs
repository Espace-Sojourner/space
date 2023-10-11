
use rltk::{RGB, Point};
use specs_derive::Component;
use specs::prelude::*;


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

#[derive(Component)]
pub struct Viewshed
{
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool
}


