use crate::buffs::{self, BuffType};
use crate::enemies::Enemies;
use crate::map::GameMap;
use crate::projectiles::{Projectile, Projectiles};
use crate::tower_stats::TowerStats;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Tower {
    pub id: usize,
    cooldown: usize,
    pub map_position: (usize, usize),
    pub buff_to_level: HashMap<BuffType, usize>,
}

impl Tower {
    pub fn new(map_position: (usize, usize)) -> Self {
        let buff_to_level = HashMap::new();
        return Self {
            id: 0,
            map_position,
            cooldown: 0,
            buff_to_level,
        };
    }

    pub fn aura_level(&self) -> usize {
        return *self.buff_to_level.get(&BuffType::Aura).unwrap_or(&0);
    }

    pub fn can_have_aura(&self, buff: &BuffType) -> bool {
        if self.buff_to_level.keys().count() < 2 {
            return true;
        }
        if let Some(_) = self.buff_to_level.get(buff) {
            return true;
        }
        return false;
    }

    pub fn add_buff(&mut self, buff: BuffType) {
        match self.buff_to_level.get(&buff) {
            Some(x) => self.buff_to_level.insert(buff, x + 1),
            None => self.buff_to_level.insert(buff, 1),
        };
    }

    pub fn get_buffs(&self) -> &HashMap<BuffType, usize> {
        return &self.buff_to_level;
    }

    pub fn tick(&mut self, enemies: &Enemies, projectiles: &mut Projectiles, stats: &TowerStats) {
        self.cooldown = self.cooldown.saturating_sub(1);
        if let Some(enemy_id) = enemies.weakest_enemy_in_range(
            stats.range,
            GameMap::tile_center(self.map_position.0, self.map_position.1),
        ) {
            if self.cooldown == 0 {
                let mut projectile = Projectile::new(
                    GameMap::tile_center(self.map_position.0, self.map_position.1),
                    self.id,
                    enemy_id,
                    stats.damage,
                    stats.projectile_speed,
                );
                buffs::calc_buff_projectile_effect(self, &mut projectile);
                projectiles.spawn(projectile);
                // 60 sec per minute / rpm * 60 ticks per second
                self.cooldown = 3600 / stats.rpm;
            }
        }
    }
}
