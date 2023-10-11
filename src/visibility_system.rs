use rltk::{field_of_view, Point};

use super::map::*;
use specs::prelude::*;

use super::{entity_components::*, coordinate::Coordinate};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem
{
    type SystemData = (WriteExpect<'a, Map>,
                       Entities<'a>,
                       WriteStorage<'a, Viewshed>,
                       WriteStorage<'a, Coordinate>,
                       ReadStorage<'a, Player>);

    fn run(&mut self, data : Self::SystemData)
    {
        let (mut map, entities, mut viewshed, coordinate, player) = data;

        for (entity, viewshed, coordinate) in (&entities, &mut viewshed, &coordinate).join()
        {
            if viewshed.dirty
            {
                map.reset_visibility(coordinate.z);
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(Point::new(coordinate.x, coordinate.y), viewshed.range, &*map);
                viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.map_size.x as i32 && p.y >= 0 && p.y < map.map_size.y as i32);
            
                let player_entity : Option<&Player> = player.get(entity);

                if let Some(_player_entity) = player_entity
                {
                    for visible_tile in viewshed.visible_tiles.iter()
                    {
                        map.set_tile_visibility(Coordinate { x:visible_tile.x as usize, y: visible_tile.y as usize, z: coordinate.z }, true);
                        map.set_tile_revealed(Coordinate { x:visible_tile.x as usize, y: visible_tile.y as usize, z: coordinate.z }, true);
                    }
                }
                viewshed.dirty = false;
            }
        }
    }
}