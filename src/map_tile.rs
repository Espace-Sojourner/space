use rltk::RGB;

#[derive(Clone, Copy)]
pub struct MapTile
{
    pub glyph: rltk::FontCharType,
    pub foreground_color: RGB,
    pub background_color: RGB,
    pub passable: bool,
    pub opaque: bool
}

impl MapTile
{
    pub fn new(glyph: rltk::FontCharType, foreground_color: RGB, background_color: RGB, passable: bool, opaque: bool) -> MapTile
    {
        MapTile 
        { 
            glyph, 
            foreground_color, 
            background_color, 
            passable,
            opaque,
        }
    }
}
