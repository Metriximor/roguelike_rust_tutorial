use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        input(self, ctx);
        
        self.run_systems();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

#[derive(Component)]
struct Renderable {
    glyph: u16,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Debug)]
struct Player {}

fn attempt_move_player(x: i32, y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + x));
        pos.y = min(49, max(0, pos.y + y));
    }
}

fn input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {},
        Some(key) => match key {
            VirtualKeyCode::Left => attempt_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => attempt_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => attempt_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => attempt_move_player(0, 1, &mut gs.ecs),
            _ => {},
        }
    }
}

fn main() {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()
        .expect("Couldn't build rltk");
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();
    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 25 })
            .with(Renderable {
                glyph: 1,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }
    rltk::main_loop(context, gs).expect("Main loop failed");
}