use rltk::RGB;
use crate::coordinate::Coordinate;

#[derive(Clone, Copy)]
pub struct MapTile
{
    pub glyph: rltk::FontCharType,
    pub foreground_color: RGB,
    pub background_color: RGB,
    pub passable: bool
}

impl MapTile
{
    pub fn new(glyph: rltk::FontCharType, foreground_color: RGB, background_color: RGB, passable: bool) -> MapTile
    {
        MapTile 
        { 
            glyph, 
            foreground_color, 
            background_color, 
            passable,
        }
    }

    pub fn get_neighbors(target_tile: Coordinate) -> Vec<Coordinate>
    {
        let mut neighbors: Vec<Coordinate> = Vec::new();

        neighbors.push(Coordinate::new(target_tile.x - 1, target_tile.y - 1, target_tile.z));
        neighbors.push(Coordinate::new(target_tile.x, target_tile.y - 1, target_tile.z));
        neighbors.push(Coordinate::new(target_tile.x + 1, target_tile.y - 1, target_tile.z));
        neighbors.push(Coordinate::new(target_tile.x, target_tile.y - 1, target_tile.z));
        neighbors.push(Coordinate::new(target_tile.x, target_tile.y + 1, target_tile.z));
        neighbors.push(Coordinate::new(target_tile.x + 1, target_tile.y + 1, target_tile.z));
        neighbors.push(Coordinate::new(target_tile.x, target_tile.y + 1, target_tile.z));
        neighbors.push(Coordinate::new(target_tile.x - 1, target_tile.y + 1, target_tile.z));

        neighbors
    }
}
