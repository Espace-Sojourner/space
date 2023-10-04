extern crate lib;
use lib::entity_components::*;

use lib::map::Map;

use lib::tile::MapTile;
use rltk::{GameState, Rltk, RGB, VirtualKeyCode, Tile};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;


struct State 
{
    entity_system: World,
    map_size: Coordinate, 
}

impl State
{
    fn run_systems(&mut self)
    {

    }
}

impl GameState for State 
{
    fn tick(&mut self, context : &mut Rltk) 
    {
        context.cls();
        self.run_systems();

        player_input(self, context);

        let coordinates = self.entity_system.read_storage::<Coordinate>();
        let renderables = self.entity_system.read_storage::<Renderable>(); 

        let map = self.entity_system.fetch::<Map>();

        let players = self.entity_system.write_storage::<Player>();

        let mut player_coordinate = &Coordinate { x:0, y: 0, z: 0 };
    
        for (_player, coordinate) in (&players, &coordinates).join() 
        {
            player_coordinate = coordinate;
        }

        draw_map(&map, self.map_size, context, player_coordinate.z);

        for (coordinate, renderable) in (&coordinates, &renderables).join()
        {
            context.set(coordinate.x, coordinate.y, renderable.foreground_color, renderable.background_color, renderable.glyph);
        }
    }
}

fn main() -> rltk::BError 
{
    use rltk::RltkBuilder;

    const MAP_WIDTH: usize = 50;
    const MAP_HEIGHT: usize = 50;
    const MAP_DEPTH: usize = 2;

    let mut context = RltkBuilder::simple(MAP_WIDTH, MAP_HEIGHT)
    .unwrap()
    .with_title("Roguelike Tutorial")
    .with_font("vga8x16.png", 8, 16)
    .with_sparse_console(MAP_WIDTH, MAP_HEIGHT, "vga8x16.png")
    .with_vsync(false)
    .build()?;

    let mut game_state = State
    { 
        entity_system: World::new(),
        map_size: Coordinate{ x: MAP_WIDTH, y: MAP_HEIGHT, z: MAP_DEPTH },
    };
    
    register_components(&mut game_state);

    game_state.entity_system.insert(Map::new(game_state.map_size));

    //Test player
    game_state.entity_system.create_entity()
                            .with(Player{})
                            .with(Coordinate { x: 40, y: 25, z: 0 })
                            .with(Renderable { glyph: rltk::to_cp437('@'), 
                                                 foreground_color: RGB::named(rltk::YELLOW), 
                                                 background_color: RGB::named(rltk::BLACK)})
                              .build();

                        
    rltk::main_loop(context, game_state)
}

fn try_move_player(delta_x: i32, delta_y: i32, game_state: &mut State)
{
    
    let mut coordinates = game_state.entity_system.write_storage::<Coordinate>();
    let mut players = game_state.entity_system.write_storage::<Player>();
    let map = game_state.entity_system.fetch::<Map>();
    
    for (_player, coordinate) in (&mut players, &mut coordinates).join() 
    {
        let target_coordinate = Coordinate { x: (coordinate.x as i32 + delta_x) as usize, y: (coordinate.y as i32 + delta_y) as usize, z : coordinate.z};
        match map.get(target_coordinate)
        {
            Some(tile) => if tile.passable
            {
                coordinate.x = min(game_state.map_size.x - 1 , max(0, target_coordinate.x));
                coordinate.y = min(game_state.map_size.y - 1, max(0, target_coordinate.y));
            }
            None => (),
        }
    }
}

fn player_input(game_state: &mut State, context: &mut Rltk) 
{
    // Player movement
    match context.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, game_state),
            VirtualKeyCode::Right => try_move_player(1, 0, game_state),
            VirtualKeyCode::Up => try_move_player(0, -1, game_state),
            VirtualKeyCode::Down => try_move_player(0, 1, game_state),
            _ => {}
        },
    }
}

fn draw_map(map: &Map, map_size: Coordinate, context: &mut Rltk, camera_z: usize)
{
    let mut x = 0;
    let mut y = 0;

    for x in 0..map_size.x
    {
        for y in 0..map_size.y
        {

            let tile = map.tiles[x][y][camera_z];

            match tile
            {
                Some(tile) => context.set(x, y, tile.foreground_color, tile.background_color, tile.glyph),
                None => (),
            }
        }
    }
    
}

fn register_components(game_state: &mut State)
{
    game_state.entity_system.register::<Coordinate>();
    game_state.entity_system.register::<Renderable>();
    game_state.entity_system.register::<Player>();
}
