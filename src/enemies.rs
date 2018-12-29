use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2};
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;
use std::collections::HashMap;

#[derive(Eq,PartialEq,Hash,Copy,Clone)]
enum Display{
    Zombie,
}
use self::Display::*;

pub struct Enemy{
    disp: Display,
    position: graphics::Point2,
    health: f32,
}

impl Enemy {
    pub fn new(position: graphics::Point2, health: f32) -> Self {
        return Self{disp: Zombie, position, health}
    }
}

pub struct Enemies{
    enemies: Vec<Enemy>,
    images: HashMap<Display, graphics::Image>,
}

impl Enemies{

    fn load_img(&mut self, ctx: &mut Context, disp: Display, path: &str) -> GameResult<()>{
        let mut img = graphics::Image::new(ctx, path)?;
        img.set_filter(graphics::FilterMode::Nearest);
        self.images.insert(disp, img);
        return Ok(());
    }

    pub fn new() -> Self{
        let enemies = vec!();
        let images  = HashMap::new();
        return Self{enemies, images}
    }

    pub fn init(&mut self, ctx: &mut Context) -> GameResult<()>{
        self.load_img(ctx, Zombie, "/enemy.png")?;
        return Ok(());
    }

    pub fn spawn(&mut self, enemy: Enemy){
        self.enemies.push(enemy);
    }


    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for e in self.enemies.iter() {
            graphics::draw_ex(
                ctx,
                &self.images[&e.disp],
                graphics::DrawParam {
                    // src: src,
                    dest: e.position,
                    //rotation: self.zoomlevel,
                    // offset: Point2::new(-16.0, 0.0),
                    scale: Point2::new(4.0,4.0),
                    // shear: shear,
                    ..Default::default()
                },
            )?;
        }
        Ok(())
    }

    pub fn tick(&mut self){
    }

}
