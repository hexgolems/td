use crate::buffs::{Buff, BuffStats, BuffType};
use crate::debuffs::Debuff;
use crate::enemies::Enemies;
use crate::map::GameMap;
use crate::projectiles::{Projectile, Projectiles};
use crate::tower_stats::TowerStats;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Tower {
    pub id: usize,
    cooldown: usize,
    pub map_position: (usize, usize),
    pub buffs: HashMap<BuffType, Buff>,
}

impl Tower {
    pub fn new(map_position: (usize, usize)) -> Self {
        let buffs = HashMap::new();
        return Self {
            id: 0,
            map_position,
            cooldown: 0,
            buffs,
        };
    }

    pub fn aura_level(&self) -> usize {
        if let Some(aura) = self.buffs.get(&BuffType::Aura) {
            return aura.level;
        }
        return 0;
    }

    pub fn can_have_buff(&self, buff: &BuffType) -> bool {
        if self.buffs.len() < 2 {
            return true;
        }
        if let Some(_) = self.buffs.get(buff) {
            return true;
        }
        return false;
    }

    pub fn add_buff(&mut self, stats: &Rc<BuffStats>) {
        match self.buffs.get_mut(&stats.kind) {
            Some(buff) => buff.upgrade(),
            None => {
                self.buffs.insert(stats.kind, Buff::new(stats));
            }
        }
    }

    pub fn get_buffs(&self) -> &HashMap<BuffType, Buff> {
        return &self.buffs;
    }

    pub fn tick(
        &mut self,
        enemies: &Enemies,
        projectiles: &mut Projectiles,
        stats: &TowerStats,
        aura_buffs: &HashMap<BuffType, Buff>,
    ) {
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
                self.add_projectile_buffs(&mut projectile, aura_buffs);
                projectiles.spawn(projectile);
                // 60 sec per minute / rpm * 60 ticks per second
                self.cooldown = 3600 / stats.rpm;
            }
        }
    }

    pub fn add_projectile_buffs(&self, p: &mut Projectile, aura_buffs: &HashMap<BuffType, Buff>) {
        if let Some(buff) = self.get_buffs().get(&BuffType::Freeze) {
            p.add_debuff(Debuff::new(buff));
        }
        for (_, buff) in aura_buffs {
            p.add_debuff(Debuff::new(buff));
        }
    }
}
