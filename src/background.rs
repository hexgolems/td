use crate::algebra::{Point, Vector};
use crate::assets::{Data, ImgID};
use crate::playing_state::PlayingState;
use crate::utils::load_specs;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::{Context, GameResult};
use rand::prelude::*;
use rand::thread_rng;
use std::collections::HashMap;
use std::ops::Range;

pub struct Wave {
    pos: Point,
    disp: ImgID,
    time: f32,
}

impl Wave {
    pub fn new() -> Self {
        let wave_id = 1 + thread_rng().gen::<usize>() % 4;
        let disp = ImgID::BackgroundWave(wave_id);
        let pos = Point::new(
            thread_rng().gen::<f32>() * 800.0,
            thread_rng().gen::<f32>() * 800.0,
        );
        return Self {
            disp,
            pos,
            time: thread_rng().gen::<f32>() * 16.0,
        };
    }

    pub fn tick(&mut self) {
        self.pos.x += 0.5;
        self.pos.y += 0.05;
        self.time += 0.01;
        if self.pos.x > 800.0 {
            self.pos.x = -10.0;
        }
        if self.pos.y > 600.0 {
            self.pos.y = -10.0;
        }
    }

    pub fn draw(&self, data: &Data, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(
            ctx,
            data.get_i(&self.disp),
            graphics::DrawParam::default()
                .dest(self.pos)
                .scale(Vector::new(4.0, 4.0))
                .color(Color::new(1.0, 1.0, 1.0, (self.time.sin() + 1.0) / 2.0)),
        )?;
        return Ok(());
    }
}

pub struct Background {
    pub offset: Point,
    pub waves: Vec<Wave>,
}

impl Background {
    pub fn new() -> Self {
        let mut waves = vec![];
        for _ in (0..20) {
            waves.push(Wave::new());
        }
        Self {
            waves,
            offset: Point::new(0.0, 0.0),
        }
    }

    pub fn tick(&mut self) {
        for w in self.waves.iter_mut() {
            w.tick();
        }
        self.offset += Vector::new(0.0002, 0.0002);
        if self.offset.x > 1.0 {
            self.offset.x -= 1.0;
        }
        if self.offset.y > 1.0 {
            self.offset.y -= 1.0;
        }
    }

    pub fn draw(state: &PlayingState, data: &Data, ctx: &mut Context) -> GameResult<()> {
        for x in (-1..5) {
            for y in (-1..5) {
                graphics::draw(
                    ctx,
                    data.get_i(&ImgID::BackgroundWater),
                    graphics::DrawParam::default()
                        .dest(
                            state
                                .gui
                                .cam()
                                .pos(Point::new(4.0 * 80.0 * x as f32, 4.0 * 80.0 * y as f32)),
                        )
                        .offset(state.background.offset)
                        .scale(Vector::new(4.0, 4.0)),
                )?;
            }
        }

        for w in state.background.waves.iter() {
            w.draw(data, ctx);
        }
        return Ok(());
    }
}
