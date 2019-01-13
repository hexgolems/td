use crate::assets::{Data, ImgID};
use crate::buffs::BuffType;
use ggez::graphics::{self, Point2, Vector2};
use ggez::{Context, GameResult};

struct ParticleData {
    disp: ImgID,
    position: Point2,
    vel: Vector2,
    size: f32,
    color: (f32, f32, f32),
    alpha: f32,
    rotation: f32,
    ttl: f32,
}

impl ParticleData {
    pub fn new(disp: ImgID, x: f32, y: f32) -> Self {
        return ParticleData {
            disp,
            position: Point2::new(x, y),
            vel: Vector2::new(0.0, 0.0),
            size: 1.0,
            color: (1.0, 1.0, 1.0),
            alpha: 1.0,
            rotation: 0.0,
            ttl: 30.0,
        };
    }
}

trait Effect {
    fn tick(&mut self);

    fn get_particles(&self) -> &Vec<ParticleData>;

    fn draw(&self, data: &Data, ctx: &mut Context) -> GameResult<()> {
        for e in self.get_particles() {
            graphics::draw_ex(
                ctx,
                data.get_i(&e.disp),
                graphics::DrawParam {
                    // src: src,
                    dest: e.position, //+e.offset_in_tile,
                    rotation: e.rotation,
                    offset: Point2::new(0.5, 0.5),
                    scale: Point2::new(e.size, e.size),
                    color: Some(graphics::Color::new(
                        e.color.0, e.color.1, e.color.2, e.alpha,
                    )),
                    // shear: shear,
                    ..Default::default()
                },
            )?;
        }
        return Ok(());
    }

    fn alive(&self) -> bool {
        return self.get_particles().len() > 0;
    }
}

pub struct Effects {
    effects: Vec<Box<Effect>>,
}

impl Effects {
    pub fn new() -> Self {
        return Self { effects: vec![] };
    }

    pub fn tick(&mut self) {
        for e in self.effects.iter_mut() {
            e.tick()
        }
        self.effects.retain(|e| e.alive());
    }

    pub fn draw(&self, data: &Data, ctx: &mut Context) -> GameResult<()> {
        for e in self.effects.iter() {
            e.draw(data, ctx)?;
        }
        return Ok(());
    }

    pub fn smoke(&mut self, x: f32, y: f32) {
        self.effects
            .push(Box::new(SmokeEffect::new(x, y, 4.0, ImgID::Smoke)));
    }

    pub fn buff(&mut self, x: f32, y: f32, buff: &BuffType) {
        match buff {
            BuffType::Aura => self
                .effects
                .push(Box::new(SmokeEffect::new(x, y, 4.0, ImgID::Aura))),
            BuffType::Damage => {
                self.effects
                    .push(Box::new(SmokeEffect::new(x, y, 4.0, ImgID::Damage)))
            }
            BuffType::Freeze => {
                self.effects
                    .push(Box::new(SmokeEffect::new(x, y, 4.0, ImgID::Freeze)))
            }
            BuffType::Range => {
                self.effects
                    .push(Box::new(SmokeEffect::new(x, y, 4.0, ImgID::Range)))
            }
            BuffType::RPM => self
                .effects
                .push(Box::new(SmokeEffect::new(x, y, 4.0, ImgID::RPM))),
        }
    }

    pub fn fire(&mut self, x: f32, y: f32) {
        self.effects
            .push(Box::new(SmokeEffect::new(x, y, 4.0, ImgID::Fire)));
    }
}

struct SmokeEffect {
    particles: Vec<ParticleData>,
}

impl Effect for SmokeEffect {
    fn get_particles(&self) -> &Vec<ParticleData> {
        return &self.particles;
    }

    fn tick(&mut self) {
        for p in self.particles.iter_mut() {
            p.position += p.vel;
            p.size += 0.1;
            p.alpha *= 0.8;
            p.ttl -= 1.0;
        }
        self.particles.retain(|e| e.ttl > 0.0);
    }
}

impl SmokeEffect {
    pub fn new(x: f32, y: f32, size: f32, img: ImgID) -> Self {
        let particles = (0..5)
            .map(|_| SmokeEffect::particle(x, y, size, img.clone()))
            .collect::<Vec<_>>();
        return Self { particles };
    }

    pub fn particle(x: f32, y: f32, size: f32, img: ImgID) -> ParticleData {
        let mut p = ParticleData::new(img, x, y);
        p.size = size;
        return p;
    }
}
