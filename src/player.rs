use rltk::{VirtualKeyCode, Rltk};
use std::cmp::{max, min};
use specs::prelude::*;
use super::components::{Position, Player};
use super::map::{TileType, xy_index};
use super::{State};

pub fn attempt_move_player(x: i32, y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_index = xy_index(pos.x + x, pos.y + y);
        if map[destination_index] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + x));
            pos.y = min(49, max(0, pos.y + y));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => attempt_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Numpad4 => attempt_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::H => attempt_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => attempt_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Numpad6 => attempt_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::L => attempt_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => attempt_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad8 => attempt_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::K => attempt_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => attempt_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad2 => attempt_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::J => attempt_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}