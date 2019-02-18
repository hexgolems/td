use crate::algebra::{Point, Vector};
use crate::assets::Data;
use crate::buffs::BuffType;
use crate::debuffs::Debuff;
use crate::enemy::Enemy;
use crate::playing_state::PlayingState;
use crate::utils::distance;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::collections::HashMap;

pub struct Enemies {
    pub enemies: HashMap<usize, Enemy>,
    id: usize,
}

impl Enemies {
    pub fn new() -> Self {
        let id = 0;
        let enemies = HashMap::new();
        return Self { enemies, id };
    }

    pub fn spawn(&mut self, enemy: Enemy) {
        self.enemies.insert(self.id, enemy);
        self.id += 1;
    }

    pub fn draw(state: &PlayingState, data: &Data, ctx: &mut Context) -> GameResult<()> {
        for e in state.enemies.enemies.values() {
            let mut color = e.color;
            match e.debuffs.get(&BuffType::Freeze) {
                Some(debuffs) => {
                    if debuffs.len() > 0 {
                        color = (0.5, 5.0, 1.0);
                    }
                }
                None => (),
            }
            graphics::draw(
                ctx,
                data.get_i(&e.disp),
                graphics::DrawParam::default()
                    .dest(state.gui.cam().world_pos(e.position))
                    .offset(Point::new(0.5, 0.5))
                    .scale(Vector::new(4.0 * e.size, 4.0 * e.size))
                    .color(graphics::Color::new(color.0, color.1, color.2, 1.0)),
            )?;
        }
        Ok(())
    }

    pub fn in_range(&self, pos: Point, range: f32) -> Vec<usize> {
        self.enemies
            .iter()
            .filter(|(_id, e)| distance(&pos, &e.position) <= range && e.health > 0)
            .map(|(id, _e)| *id)
            .collect()
    }

    pub fn weakest_enemy_in_range(&self, range: f32, pos: Point) -> Option<usize> {
        self.in_range(pos, range)
            .iter()
            .min_by_key(|id| self.enemies.get(id).unwrap().health)
            .cloned()
    }

    pub fn tick(state: &mut PlayingState) {
        for e in state.enemies.enemies.values_mut() {
            e.tick(&state.map)
        }
        state.enemies.enemies.retain(|_id, e| e.health > 0);
        state.player_mut().hp = state.player_mut().hp.saturating_sub(
            state
                .enemies
                .enemies
                .iter()
                .filter(|(_id, e)| e.reached_goal)
                .count(),
        );
        state
            .enemies
            .enemies
            .retain(|_id, e| e.reached_goal == false);
    }

    pub fn debuff(&mut self, id: usize, debuffs: &HashMap<BuffType, Debuff>) {
        if let Some(e) = self.enemies.get_mut(&id) {
            for (bt, d) in debuffs.iter() {
                let debuff_vec = e.debuffs.entry(*bt).or_insert(Vec::new());
                debuff_vec.push(d.clone());
            }
        }
    }

    pub fn damage(&mut self, id: usize, damage: usize) {
        if let Some(e) = self.enemies.get_mut(&id) {
            e.health = e.health.saturating_sub(damage);
        }
    }

    pub fn any_alive(&self) -> bool {
        self.enemies.iter().filter(|(_id, e)| e.health > 0).count() > 0
    }
}
