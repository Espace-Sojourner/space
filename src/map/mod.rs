use super::tile::TileType;

pub fn cartesian_to_index(x: i32, y: i32, map_width: usize) -> usize
{
    (y as usize * map_width) + x as usize
}

pub fn new_map(width: usize, height: usize) -> Vec<TileType>
{
    let mut map = vec![TileType::Floor; width*height];

    for x in 0..width
    {
        map[cartesian_to_index(x as i32, 0, width)] = TileType::Wall;
        map[cartesian_to_index(x as i32, (height - 1) as i32, width)] = TileType::Wall;
    }

    for y in 0..height
    {
        map[cartesian_to_index(0, y as i32, width)] = TileType::Wall;
        map[cartesian_to_index((width - 1) as i32, y as i32, width)] = TileType::Wall;
    }

    map
}
