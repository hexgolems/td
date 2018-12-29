use ggez::graphics;
use ggez::{Context, GameResult};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum ImgID {
    Cursor,

    Zombie,

    Cannon,
    CannonBall,
    Explosion,

    FloorWalkLeft,
    FloorWalkRight,
    FloorWalkDown,
    FloorWalkUp,

    FloorSpawnLeft,
    FloorSpawnRight,
    FloorSpawnDown,
    FloorSpawnUp,

    FloorTarget,
    FloorBuild,
}
use self::ImgID::*;

pub struct Imgs {
    images: HashMap<ImgID, graphics::Image>,
}

impl Imgs {
    pub fn new() -> Self {
        let images = HashMap::new();
        return Self { images };
    }

    fn load_img(&mut self, ctx: &mut Context, map: ImgID, path: &str) -> GameResult<()> {
        let mut img = graphics::Image::new(ctx, path)?;
        img.set_filter(graphics::FilterMode::Nearest);
        self.images.insert(map, img);
        return Ok(());
    }

    pub fn init(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.load_img(ctx, Cursor, "/cursor.png")?;
        self.load_img(ctx, Zombie, "/enemy.png")?;
        self.load_img(ctx, Cannon, "/cannon.png")?;
        self.load_img(ctx, CannonBall, "/cannon_ball.png")?;
        self.load_img(ctx, Explosion, "/explosion.png")?;

        self.load_img(ctx, FloorWalkLeft, "/floor_walk_left.png")?;
        self.load_img(ctx, FloorWalkRight, "/floor_walk_right.png")?;
        self.load_img(ctx, FloorWalkUp, "/floor_walk_up.png")?;
        self.load_img(ctx, FloorWalkDown, "/floor_walk_down.png")?;

        self.load_img(ctx, FloorSpawnLeft, "/floor_spawn_left.png")?;
        self.load_img(ctx, FloorSpawnRight, "/floor_spawn_right.png")?;
        self.load_img(ctx, FloorSpawnUp, "/floor_spawn_up.png")?;
        self.load_img(ctx, FloorSpawnDown, "/floor_spawn_down.png")?;

        self.load_img(ctx, FloorTarget, "/floor_target.png")?;
        self.load_img(ctx, FloorBuild, "/floor_build.png")?;
        return Ok(());
    }

    pub fn get(&self, id: &ImgID) -> &graphics::Image {
        return &self.images[id];
    }
}
