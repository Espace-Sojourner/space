use rltk::RGB;
use super::entity_components::Coordinate;
use super::tile::MapTile;

pub struct Map
{
    pub tiles: Vec<Vec<Vec<Option<MapTile>>>>,
    pub map_size: Coordinate
}

impl Map
{
    pub fn new(map_size: Coordinate) -> Map
    {
        let mut tiles: Vec<Vec<Vec<Option<MapTile>>>> = vec![vec![vec![None; map_size.z]; map_size.y]; map_size.x];

        for x in 0..map_size.x
        {
            for y in 0..map_size.y
            {
                for z in 0..map_size.z
                {
                    let new_tile: MapTile;

                    if x == 0 || y == 0 || x == map_size.x - 1 || y == map_size.y - 1 
                    {
                        new_tile = MapTile::new(rltk::to_cp437('#'), 
                            RGB::from_f32(0.5, 0.5, 0.5),
                            RGB::named(rltk::BLACK), 
                            false);    
                    }
                    else
                    {
                        new_tile = MapTile::new(rltk::to_cp437('.'), 
                                RGB::from_f32(0.1, 0.1, 0.1),
                                RGB::named(rltk::BLACK), 
                                true);    
                    }
                    tiles[x][y][z] = Some(new_tile);
                }
            }
        }
        Map
        {
            tiles,
            map_size
        }
    }

    pub fn get(&self, coordinate: Coordinate) -> Option<MapTile>
    {
        if coordinate.x < self.map_size.x && coordinate.y < self.map_size.y && coordinate.z < self.map_size.z
        {
            match &self.tiles[coordinate.x][coordinate.y][coordinate.z]
            {
                Some(target_tile) => return Some(*target_tile),
                None => return None,
            }
        }
        else
        {
            None
        }
    }
}
