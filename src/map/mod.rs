use std::cmp::{min, max};

use rand::Rng;
use rltk::RGB;
use crate::coordinate;

use super::coordinate::Coordinate;
use super::rectangle::Rectangle;
use super::tile::MapTile;


pub struct Map
{
    pub map_size: Coordinate,
    pub tiles: Vec<Vec<Vec<Option<MapTile>>>>,
}

impl Map
{
    /// Makes a map with solid boundries
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

    /// Makes a map consisting of random rooms and corridors connecting them
    pub fn rooms_and_corridors_map(number_of_rooms: usize, min_room_size: usize, max_room_size: usize,  map_size: Coordinate) -> Map
    {
        const MAX_ATTEMPTS: usize = 100;
        let mut tiles: Vec<Vec<Vec<Option<MapTile>>>> = vec![vec![vec![None; map_size.z]; map_size.y]; map_size.x];
        let mut rooms: Vec<Rectangle> = Vec::new();
        let mut previous_room_center = Coordinate::new(0, 0, 0);

        for _ in 0..number_of_rooms
        {
            let mut current_attempts = 0;
            let mut room_placed = false;

            while !room_placed && current_attempts < MAX_ATTEMPTS
            {
                let mut room_valid = true;

                let room_width = rand::thread_rng().gen_range(min_room_size..max_room_size + 1);
                let room_height = rand::thread_rng().gen_range(min_room_size..max_room_size + 1);

                let room_coordinate = Coordinate::new(rand::thread_rng().gen_range(1..map_size.x - (room_width + 1)),
                                                                rand::thread_rng().gen_range(1..map_size.y - (room_height + 1)),
                                                                0);

                let room = Rectangle::new(room_coordinate, Coordinate::new(room_coordinate.x + room_width, 
                                                    room_coordinate.y + room_height, 
                                                    room_coordinate.z));

                for other_room in rooms.iter()
                {
                    if room.intersect(other_room)
                    {
                        room_valid = false;
                    }
                }

                if room_valid
                {
                    add_room_to_map(&room, &mut tiles);

                    if !rooms.is_empty()
                    {
                        if rand::thread_rng().gen_range(0..2) == 1
                        {
                            add_horizontal_corridor(room.center(), previous_room_center, &mut tiles);
                            add_vertical_corridor(room.center(), previous_room_center, &mut tiles);
                        }
                        else
                        {
                            add_horizontal_corridor(room.center(), previous_room_center, &mut tiles);
                            add_vertical_corridor(room.center(), previous_room_center, &mut tiles);
                        }
                    }
                    previous_room_center = room.center();

                    rooms.push(room);
                    room_placed = true;
                    
                }
                else 
                {
                    current_attempts += 1;
                }
            }
            
        }
        
        let mut map = Map { tiles, map_size };

        add_walls(&mut map);

        map
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

pub fn get_tile_neighbors(target_tile: Coordinate, map: &Map) -> (Vec<Option<MapTile>>, Vec<Coordinate>)
{
    let mut neighbors: Vec<Option<MapTile>> = Vec::new();
    let mut coordinates: Vec<Coordinate> = Vec::new();

    if target_tile.x > 0 && target_tile.y > 0
    {
        coordinates.push(Coordinate::new(target_tile.x - 1, target_tile.y - 1, target_tile.z));
        neighbors.push(map.get(coordinates[coordinates.len() - 1]));
    }

    if target_tile.x > 0
    {
        coordinates.push(Coordinate::new(target_tile.x - 1, target_tile.y + 1, target_tile.z));
        neighbors.push(map.get(coordinates[coordinates.len() - 1]));
    }

    if target_tile.y > 0
    {
        coordinates.push(Coordinate::new(target_tile.x, target_tile.y - 1, target_tile.z));
        neighbors.push(map.get(coordinates[coordinates.len() - 1]));

        coordinates.push(Coordinate::new(target_tile.x + 1, target_tile.y - 1, target_tile.z));
        neighbors.push(map.get(coordinates[coordinates.len() - 1]));

        coordinates.push(Coordinate::new(target_tile.x, target_tile.y - 1, target_tile.z));
        neighbors.push(map.get(coordinates[coordinates.len() - 1]));
    }

    coordinates.push(Coordinate::new(target_tile.x, target_tile.y + 1, target_tile.z));
    neighbors.push(map.get(coordinates[coordinates.len() - 1]));

    coordinates.push(Coordinate::new(target_tile.x + 1, target_tile.y + 1, target_tile.z));
    neighbors.push(map.get(coordinates[coordinates.len() - 1]));

    coordinates.push(Coordinate::new(target_tile.x, target_tile.y + 1, target_tile.z));
    neighbors.push(map.get(coordinates[coordinates.len() - 1]));

    


    (neighbors, coordinates)
}

fn add_room_to_map(room: &Rectangle, tiles: &mut Vec<Vec<Vec<Option<MapTile>>>>)
{
    for x in room.corner_one.x + 1 ..= room.corner_two.x
    {
        for y in room.corner_one.y + 1 ..= room.corner_two.y
        {
            tiles[x][y][room.corner_one.z] = Some(MapTile::new(rltk::to_cp437('.'), 
                                RGB::from_f32(0.3, 0.3, 0.3),
                                RGB::named(rltk::BLACK), 
                                true));    
        }
    }
}

fn add_horizontal_corridor(origin: Coordinate, target: Coordinate, tiles: &mut Vec<Vec<Vec<Option<MapTile>>>>)
{
    for x in min(origin.x, target.x) ..= max(origin.x, target.x)
    {
        tiles[x][origin.y][origin.z] = Some(MapTile::new(rltk::to_cp437('.'), 
                            RGB::from_f32(0.3, 0.3, 0.3),
                            RGB::named(rltk::BLACK), 
                            true)); 
    }
}

fn add_vertical_corridor(origin: Coordinate, target: Coordinate, tiles: &mut Vec<Vec<Vec<Option<MapTile>>>>)
{
    for y in min(origin.y, target.y) ..= max(origin.y, target.y)
    {
        tiles[target.x][y][origin.z] = Some(MapTile::new(rltk::to_cp437('.'), 
                            RGB::from_f32(0.3, 0.3, 0.3),
                            RGB::named(rltk::BLACK), 
                            true)); 
    }
}

pub fn add_walls(map: &mut Map)
{
    for x in 0..map.tiles.len() - 1
    {
        for y in 0..map.tiles[x].len() - 1
        {
            for z in 0..map.tiles[x][y].len() - 1
            {
                let tile = map.tiles[x][y][z];

                match tile
                {
                    Some(tile) if tile.passable =>
                    {
                        let (neighbors, coordinates) = get_tile_neighbors(Coordinate::new(x, y, z), map);

                        for i in 0..neighbors.len()
                        {
                            let neighbor = neighbors[i];
                            let coordinate = coordinates[i];

                            match neighbor
                            {
                                None =>
                                {
                                    map.tiles[coordinate.x][coordinate.y][coordinate.z] = Some(MapTile::new(rltk::to_cp437('#'), 
                                    RGB::from_f32(0.5, 0.5, 0.5),
                                    RGB::named(rltk::BLACK), 
                                    false)); 
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                } 
            }
        }
    }
}