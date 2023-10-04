use rltk::{field_of_view, Point};
use super::map::Map;
use specs::prelude::*;

use super::{entity_components::Viewshed, coordinate::Coordinate};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem
{
    type SystemData = (ReadExpect<'a, Map>,
                       WriteStorage<'a, Viewshed>,
                       WriteStorage<'a, Coordinate>);

    fn run(&mut self, data : Self::SystemData)
    {
        let (map, mut viewshed, coordinate) = data;

        for (viewshed, coordinate) in (&mut viewshed, &coordinate).join()
        {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(coordinate.x, coordinate.y), viewshed.range, &*map);
            viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.map_size.x as i32 && p.y >= 0 && p.y < map.map_size.y as i32);
        }
    }
}