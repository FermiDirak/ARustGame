rltk::add_wasm_support!();

#[macro_use]
extern crate specs_derive;

use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use std::cmp::{max, min};

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut enemy_ai = EnemyAI {};
        enemy_ai.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (position, render) in (&positions, &renderables).join() {
            ctx.set(position.x, position.y, render.fg, render.bg, render.glyph);
        }
    }
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct EnemyAI {}

impl<'a> System<'a> for EnemyAI {
    type SystemData = (
        ReadStorage<'a, EnemyAI>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (enemy, mut pos) : Self::SystemData) {
        for (_enemy, pos) in (&enemy, &mut pos).join() {
            pos.x = (pos.x - 1).rem_euclid(80);
        }
    }
}

fn main() {
    let mut rng = rltk::RandomNumberGenerator::seeded(0);

    let context = Rltk::init_simple8x8(80, 50, "My game", "resources");
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<EnemyAI>();

    for _ in 1..10 {
        gs.ecs
            .create_entity()
            .with(Position {
                x: rng.range(1, 80),
                y: rng.range(1, 50),
            })
            .with(EnemyAI {})
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }

    rltk::main_loop(context, gs);
}
