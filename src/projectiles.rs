use crate::assets::{Data, ImgID};
use crate::curses::CurseType;
use crate::effects::Effects;
use crate::enemies::Enemies;
use crate::game_state::GameState;
use crate::utils::move_to;
use ggez::graphics;
use ggez::graphics::Point2;
use ggez::{Context, GameResult};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Projectile {
    disp: ImgID,
    enemy_id: usize,
    tower_id: usize,
    position: graphics::Point2,
    damage: usize,
    speed: f32,
    next_walk_target: graphics::Point2,
    reached_goal: bool,
    curses: HashSet<CurseType>,
}

impl Projectile {
    pub fn new(
        position: graphics::Point2,
        tower_id: usize,
        enemy_id: usize,
        damage: usize,
        speed: f32,
    ) -> Self {
        return Self {
            disp: ImgID::Arrow,
            curses: HashSet::new(),
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
            enemies.damage(self.enemy_id, self.damage, &self.curses);
            effects.smoke(self.next_walk_target.x, self.next_walk_target.y);
        }
    }

    pub fn add_curse(&mut self, curse: CurseType) {
        self.curses.insert(curse);
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

    pub fn draw(&self, data: &Data, ctx: &mut Context) -> GameResult<()> {
        for p in self.projectiles.values() {
            let dir = p.next_walk_target - p.position;
            let rot = dir.y.atan2(dir.x);
            graphics::draw_ex(
                ctx,
                data.get_i(&p.disp),
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
            p.tick(&mut state.enemies, &mut state.effects)
        }
        state
            .projectiles
            .projectiles
            .retain(|_id, p| p.reached_goal == false);
    }
}
