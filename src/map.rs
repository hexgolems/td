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
enum WalkDir{
    Up,
    Down,
    Left,
    Right,
}

use self::WalkDir::*;
#[derive(Eq,PartialEq,Hash,Copy,Clone)]
enum MapTile{
    Walk(WalkDir),
    Build,
    Spawn(WalkDir),
    Target
}
use self::MapTile::*;

pub struct GameMap{
    data: Vec<Vec<MapTile>>,
    images: HashMap<MapTile, graphics::Image>,
}

impl GameMap{

    fn load_img(&mut self, ctx: &mut Context, map: MapTile, path: &str) -> GameResult<()>{
        let mut img = graphics::Image::new(ctx, path)?;
        img.set_filter(graphics::FilterMode::Nearest);
        self.images.insert(map, img);
        return Ok(());
    }

    pub fn new() -> Self{
        let data = vec!(
                vec!(Target,        Build,      Build,      Build),
                vec!(Walk(Up),      Walk(Left), Walk(Left), Walk(Left)),
                vec!(Build,         Build,      Build,      Walk(Up)),
                vec!(Spawn(Right),  Walk(Right),Walk(Right),Walk(Up)),
            );
        let mut images = HashMap::new();
        return Self{data, images};
    }

    pub fn init(&mut self, ctx: &mut Context) -> GameResult<()>{
        self.load_img(ctx, Walk(Left),   "/floor_walk_left.png")?;
        self.load_img(ctx, Walk(Right),  "/floor_walk_right.png")?;
        self.load_img(ctx, Walk(Up),     "/floor_walk_up.png")?;
        self.load_img(ctx, Walk(Down),   "/floor_walk_down.png")?;
        self.load_img(ctx, Build,        "/floor_build.png")?;
        self.load_img(ctx, Target,       "/floor_target.png")?;
        self.load_img(ctx, Spawn(Left),  "/floor_spawn_left.png")?;
        self.load_img(ctx, Spawn(Right), "/floor_spawn_right.png")?;
        self.load_img(ctx, Spawn(Up),    "/floor_spawn_up.png")?;
        self.load_img(ctx, Spawn(Down),  "/floor_spawn_down.png")?;
        return Ok(());
    }

    pub fn tile_pos(&self, x: usize, y: usize) -> graphics::Point2 {
        return graphics::Point2::new(4.0*20.0*x as f32, 4.0*20.0*y as f32);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for x in (0..4) {
            for y in (0..4) {
                graphics::draw_ex(
                    ctx,
                    &self.images[&self.data[y][x]],
                    graphics::DrawParam {
                        // src: src,
                        dest: self.tile_pos(x,y),
                        //rotation: self.zoomlevel,
                        // offset: Point2::new(-16.0, 0.0),
                        scale: Point2::new(4.0,4.0),
                        // shear: shear,
                        ..Default::default()
                    },
                )?;
            }
        }
        Ok(())
    }
}

