use crate::assets::{ImgID, Imgs};
use crate::enemies::Enemies;
use crate::game_state::GameState;
use crate::towers::TowerType;
use crate::utils::move_to;
use ggez::graphics;
use ggez::graphics::Point2;
use ggez::{Context, GameResult};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Projectile {
    disp: ImgID,
    enemy_id: usize,
    position: graphics::Point2,
    damage: usize,
    speed: f32,
    next_walk_target: graphics::Point2,
    reached_goal: bool,
    kind: TowerType,
}

impl Projectile {
    pub fn new(
        position: graphics::Point2,
        enemy_id: usize,
        damage: usize,
        speed: f32,
        kind: TowerType,
    ) -> Self {
        return Self {
            disp: match kind {
                TowerType::Cannon => ImgID::CannonBall,
                TowerType::Archers => ImgID::Arrow,
            },
            enemy_id,
            position,
            damage,
            next_walk_target: position,
            speed,
            kind,
            reached_goal: false,
        };
    }

    pub fn tick(&mut self, enemies: &mut Enemies) {
        if let Some(e) = enemies.enemies.get(&self.enemy_id) {
            self.next_walk_target = e.position;
        }
        let (new_pos, finished) = move_to(self.position, self.next_walk_target, self.speed);
        self.position = new_pos;
        self.reached_goal = finished;
        if self.reached_goal == true {
            enemies.damage(self.enemy_id, self.damage);
        }
    }
}

#[derive(Debug)]
pub struct Projectiles {
    pub projectiles: HashMap<usize, Projectile>,
    id: usize,
}

impl Projectiles {
    pub fn new() -> Self {
        let id = 0;
        let projectiles = HashMap::new();
        return Self { projectiles, id };
    }

    pub fn spawn(&mut self, projectile: Projectile) {
        self.projectiles.insert(self.id, projectile);
        self.id = self.id.wrapping_add(1);
    }

    pub fn draw(&self, imgs: &Imgs, ctx: &mut Context) -> GameResult<()> {
        for p in self.projectiles.values() {
            let dir = p.next_walk_target - p.position;
            let rot = dir.y.atan2(dir.x);
            graphics::draw_ex(
                ctx,
                imgs.get(&p.disp),
                graphics::DrawParam {
                    // src: src,
                    dest: p.position, //+p.offset_in_tile,
                    rotation: rot,
                    offset: Point2::new(0.5, 0.5),
                    scale: Point2::new(4.0, 4.0),
                    // shear: shear,
                    ..Default::default()
                },
            )?;
        }
        Ok(())
    }

    pub fn tick(state: &mut GameState) {
        for p in state.projectiles.projectiles.values_mut() {
            p.tick(&mut state.enemies)
        }
        state
            .projectiles
            .projectiles
            .retain(|_id, p| p.reached_goal == false);
    }
}
