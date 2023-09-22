extern crate lib;
use lib::entity_components::*;

use lib::map::{cartesian_to_index, new_map};

use lib::tile::TileType;
use rltk::{GameState, Rltk, RGB, VirtualKeyCode, Tile};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;


struct State 
{
    entity_system: World,
    map_width: usize,
    map_height: usize,
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

        let positions = self.entity_system.read_storage::<Position>();
        let renderables = self.entity_system.read_storage::<Renderable>(); 

        let map = self.entity_system.fetch::<Vec<TileType>>();
        let map_width = self.map_width;

        draw_map(&map, &map_width, context);

        for (position, renderable) in (&positions, &renderables).join()
        {
            context.set(position.x, position.y, renderable.foreground_color, renderable.background_color, renderable.glyph);
        }
    }
}

fn main() -> rltk::BError 
{
    use rltk::RltkBuilder;

    const MAP_WIDTH: usize = 100;
    const MAP_HEIGHT: usize = 80;

    let mut context = RltkBuilder::simple(MAP_WIDTH, MAP_HEIGHT)
    .unwrap()
    .with_title("Roguelike Tutorial")
    .with_font("vga8x16.png", 8, 16)
    .with_sparse_console(80, 30, "vga8x16.png")
    .with_vsync(false)
    .build()?;

    let mut game_state = State
    { 
        entity_system: World::new(),
        map_width: MAP_WIDTH,
        map_height: MAP_HEIGHT,
    };
    
    register_components(&mut game_state);

    game_state.entity_system.insert(new_map(game_state.map_width, game_state.map_height));

    //Test player
    game_state.entity_system.create_entity()
                            .with(Player{})
                            .with(Position {x: 40, y: 25})
                            .with(Renderable { glyph: rltk::to_cp437('@'), 
                                                 foreground_color: RGB::named(rltk::YELLOW), 
                                                 background_color: RGB::named(rltk::BLACK)})
                              .build();

                        
    rltk::main_loop(context, game_state)
}

fn try_move_player(delta_x: i32, delta_y: i32, game_state: &mut State)
{
    
    let mut positions = game_state.entity_system.write_storage::<Position>();
    let mut players = game_state.entity_system.write_storage::<Player>();
    let map = game_state.entity_system.fetch::<Vec<TileType>>();

    for (_player, position) in (&mut players, &mut positions).join() 
    {
        let target_index = cartesian_to_index(position.x + delta_x, position.y + delta_y, game_state.map_width);

        if map[target_index] != TileType::Wall
        {
            position.x = min(game_state.map_width as i32 - 1 , max(0, position.x + delta_x));
            position.y = min(game_state.map_height as i32 - 1, max(0, position.y + delta_y));
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

fn draw_map(map: &[TileType], map_width: &usize, context: &mut Rltk)
{
    let mut x = 0;
    let mut y = 0;

    for tile in map.iter()
    {
        match tile 
        {
            TileType::Floor => 
            {
                context.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            }   
            TileType::Wall => 
            {
                context.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }

        x += 1;

        if x > map_width - 1
        {
            x = 0;
            y += 1;
        }
    }
}

fn register_components(game_state: &mut State)
{
    game_state.entity_system.register::<Position>();
    game_state.entity_system.register::<Renderable>();
    game_state.entity_system.register::<Player>();
}
