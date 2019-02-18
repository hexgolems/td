use crate::buffs::{Buff, BuffType};
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
    pub fn get_buffed_stats(t: &Tower, auras: &HashMap<BuffType, Buff>, base: &TowerStats) -> Self {
        let mut base = base.clone();
        let buffs = t.get_buffs();
        base.rpm += base.get_buffed(buffs, auras, &BuffType::RPM);
        base.damage += base.get_buffed(buffs, auras, &BuffType::Damage);
        base.range += base.get_buffed(buffs, auras, &BuffType::Range) as f32;
        return base;
    }

    fn get_buffed(
        &self,
        buffs: &HashMap<BuffType, Buff>,
        auras: &HashMap<BuffType, Buff>,
        buff_type: &BuffType,
    ) -> usize {
        let own = match buffs.get(buff_type) {
            Some(buff) => buff.effectiveness(),
            None => 0,
        };
        let from_aura = match auras.get(buff_type) {
            Some(buff) => buff.effectiveness(),
            None => 0,
        };
        return own + from_aura;
    }

    pub fn info(&self) -> String {
        return format!(
            "Damage: {}\nRange: {}\nRPM: {}",
            &self.damage.to_string(),
            &self.range.to_string(),
            &self.rpm.to_string(),
        );
    }
}
