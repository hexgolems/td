use crate::algebra::{Point, Vector};
use crate::assets::{Data, ImgID};
use crate::buffs::BuffType;
use crate::debuffs::Debuff;
use crate::effects::Effects;
use crate::enemies::Enemies;
use crate::playing_state::PlayingState;
use crate::utils::move_to;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Projectile {
    disp: ImgID,
    enemy_id: usize,
    tower_id: usize,
    position: Point,
    damage: usize,
    speed: f32,
    next_walk_target: Point,
    reached_goal: bool,
    debuffs: HashMap<BuffType, Debuff>,
}

impl Projectile {
    pub fn new(
        position: Point,
        tower_id: usize,
        enemy_id: usize,
        damage: usize,
        speed: f32,
    ) -> Self {
        return Self {
            disp: ImgID::Arrow,
            debuffs: HashMap::new(),
            tower_id,
            enemy_id,
            position,
            damage,
            next_walk_target: position,
            speed,
            reached_goal: false,
        };
    }

    pub fn tick(&mut self, enemies: &mut Enemies, effects: &mut Effects) {
        if let Some(e) = enemies.enemies.get(&self.enemy_id) {
            self.next_walk_target = e.position;
        }
        let (new_pos, finished) = move_to(self.position, self.next_walk_target, self.speed);
        self.position = new_pos;
        self.reached_goal = finished;
        if self.reached_goal == true {
            enemies.damage(self.enemy_id, self.damage);
            enemies.debuff(self.enemy_id, &self.debuffs);
            effects.smoke(self.next_walk_target.x, self.next_walk_target.y);
        }
    }

    pub fn add_debuff(&mut self, debuff: Debuff) {
        match self.debuffs.get(&debuff.kind) {
            Some(own) => {
                if own.effectiveness < debuff.effectiveness {
                    self.debuffs.insert(debuff.kind, debuff);
                }
            }
            None => {
                self.debuffs.insert(debuff.kind, debuff);
            }
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

    pub fn draw(state: &PlayingState, data: &Data, ctx: &mut Context) -> GameResult<()> {
        for p in state.projectiles.projectiles.values() {
            let dir = p.next_walk_target - p.position;
            let rot = dir.y.atan2(dir.x);
            graphics::draw(
                ctx,
                data.get_i(&p.disp),
                graphics::DrawParam::default()
                    .dest(state.gui.cam().world_pos(p.position))
                    .rotation(rot)
                    .offset(Point::new(0.5, 0.5))
                    .scale(Vector::new(4.0, 4.0)),
            )?;
        }
        Ok(())
    }

    pub fn tick(state: &mut PlayingState) {
        for p in state.projectiles.projectiles.values_mut() {
            p.tick(&mut state.enemies, &mut state.effects)
        }
        state
            .projectiles
            .projectiles
            .retain(|_id, p| p.reached_goal == false);
    }
}
