extern crate lib;
use lib::{entity_components::*, visibility_system::VisibilitySystem, coordinate::Coordinate};

use lib::map::Map;

use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};


struct State 
{
    entity_system: World,
    map_size: Coordinate, 
}

impl State
{
    fn run_systems(&mut self)
    {
        let mut visibility_system = VisibilitySystem{};

        visibility_system.run_now(&self.entity_system);
        self.entity_system.maintain();
    }
}

impl GameState for State 
{
    fn tick(&mut self, context : &mut Rltk) 
    {
        context.cls();
        self.run_systems();

        player_input(self, context);

        let coordinates = self.entity_system.write_storage::<Coordinate>();
        let renderables = self.entity_system.read_storage::<Renderable>(); 
    
        let players = self.entity_system.write_storage::<Player>();
        let mut camera_z: usize = 0;

        for (_player, coordinate) in (&players, &coordinates).join() 
        {
            camera_z = coordinate.z;
        }
        drop(players);

        let map = self.entity_system.fetch::<Map>();
        map.draw(context, camera_z);

        for (coordinate, renderable) in (&coordinates, &renderables).join()
        {
            context.set(coordinate.x, coordinate.y, renderable.foreground_color, renderable.background_color, renderable.glyph);
        }
    }
}

fn main() -> rltk::BError 
{
    use rltk::RltkBuilder;

    const MAP_WIDTH: usize = 150;
    const MAP_HEIGHT: usize = 100;
    const MAP_DEPTH: usize = 2;

    let context = RltkBuilder::simple(MAP_WIDTH, MAP_HEIGHT)
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

    let (map, rooms) = Map::rooms_and_corridors_map(20,5, 10, game_state.map_size);
    let player_start_coordinate = rooms[0].center();

    game_state.entity_system.insert(map);
    //Test player
    game_state.entity_system.create_entity()
                            .with(Player{})
                            .with(player_start_coordinate)
                            .with(Renderable { glyph: rltk::to_cp437('@'), 
                                                foreground_color: RGB::named(rltk::YELLOW), 
                                                background_color: RGB::named(rltk::BLACK)})
                            .with(Viewshed { visible_tiles: Vec::new(), range: 18, dirty: true})
                            .build();

                        
    rltk::main_loop(context, game_state)
}

fn try_move_player(delta_x: i32, delta_y: i32, game_state: &mut State)
{
    
    let mut coordinates = game_state.entity_system.write_storage::<Coordinate>();
    let mut players = game_state.entity_system.write_storage::<Player>();
    let mut viewsheds = game_state.entity_system.write_storage::<Viewshed>();
    let map = game_state.entity_system.fetch::<Map>();
    
    for (_player, coordinate, viewshed) in (&mut players, &mut coordinates, &mut viewsheds).join() 
    {
        let target_coordinate = Coordinate { x: (coordinate.x as i32 + delta_x) as usize, y: (coordinate.y as i32 + delta_y) as usize, z : coordinate.z};
        match map.get(target_coordinate)
        {
            Some(tile) => if tile.passable
            {
                coordinate.x = min(game_state.map_size.x - 1 , max(0, target_coordinate.x));
                coordinate.y = min(game_state.map_size.y - 1, max(0, target_coordinate.y));
                viewshed.dirty = true;
                
            }
            None => 
            {
                coordinate.x = min(game_state.map_size.x - 1 , max(0, target_coordinate.x));
                coordinate.y = min(game_state.map_size.y - 1, max(0, target_coordinate.y));
                viewshed.dirty = true;
            }
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
            VirtualKeyCode::Home => try_move_player(-1, -1, game_state),
            VirtualKeyCode::PageUp => try_move_player(1, -1, game_state),
            VirtualKeyCode::PageDown => try_move_player(1, 1, game_state),
            VirtualKeyCode::End => try_move_player(-1, 1, game_state),
            _ => {}
        },
    }
}

fn register_components(game_state: &mut State)
{
    game_state.entity_system.register::<Coordinate>();
    game_state.entity_system.register::<Renderable>();
    game_state.entity_system.register::<Player>();
    game_state.entity_system.register::<Viewshed>();
}
