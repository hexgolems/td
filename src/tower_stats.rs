use crate::buffs::BuffType;
use crate::tower::Tower;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct TowerStats {
    pub damage: usize,
    pub projectile_speed: f32,
    pub range: f32,
    pub rpm: usize,
    pub price: usize,
}

impl TowerStats {
    pub fn get_buffed_stats(
        t: &Tower,
        aura_buffs_to_level: &HashMap<BuffType, usize>,
        base: &TowerStats,
    ) -> Self {
        let mut base = base.clone();
        let buffs = t.get_buffs();
        base.rpm += base.get_buffed_rpm(buffs, aura_buffs_to_level);
        base.damage += base.get_buffed_damage(buffs, aura_buffs_to_level);
        base.range += base.get_buffed_range(buffs, aura_buffs_to_level);
        return base;
    }

    fn get_buffed_range(
        &self,
        buffs: &HashMap<BuffType, usize>,
        auras: &HashMap<BuffType, usize>,
    ) -> f32 {
        return (20
            * (buffs.get(&BuffType::Range).unwrap_or(&0)
                + auras.get(&BuffType::Range).unwrap_or(&0))) as f32;
    }

    fn get_buffed_damage(
        &self,
        buffs: &HashMap<BuffType, usize>,
        auras: &HashMap<BuffType, usize>,
    ) -> usize {
        return 25
            * (buffs.get(&BuffType::Damage).unwrap_or(&0)
                + auras.get(&BuffType::Damage).unwrap_or(&0));
    }

    fn get_buffed_rpm(
        &self,
        buffs: &HashMap<BuffType, usize>,
        auras: &HashMap<BuffType, usize>,
    ) -> usize {
        return 30
            * (buffs.get(&BuffType::RPM).unwrap_or(&0) + auras.get(&BuffType::RPM).unwrap_or(&0));
    }
}
