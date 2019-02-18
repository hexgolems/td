use std::collections::HashMap;
use std::rc::Rc;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Deserialize)]
pub enum BuffType {
    Freeze,
    Damage,
    RPM,
    Range,
    Aura,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BuffStats {
    pub kind: BuffType,
    pub level_to_effectiveness: HashMap<usize, usize>,
    pub level_to_cooldown: HashMap<usize, usize>,
    pub level_to_price: HashMap<usize, usize>,
}

#[derive(Clone, Debug)]
pub struct Buff {
    pub stats: Rc<BuffStats>,
    pub level: usize,
}

impl Buff {
    pub fn new(stats: Rc<BuffStats>) -> Self {
        let level = 1;
        return Self { stats, level };
    }

    pub fn effectiveness(&self) -> usize {
        return *self
            .stats
            .level_to_effectiveness
            .get(&self.level)
            .unwrap_or(&0);
    }

    pub fn cooldown(&self) -> usize {
        return *self.stats.level_to_cooldown.get(&self.level).unwrap_or(&0);
    }

    pub fn upgrade(&mut self) {
        if self.upgradeable() {
            self.level += 1;
        }
    }

    pub fn upgradeable(&self) -> bool {
        if self.level < 5 {
            return true;
        }
        return false;
    }

    pub fn info(&self) -> String {
        let info = match self.stats.kind {
            BuffType::Freeze => "Freeze",
            BuffType::Damage => "Damage",
            BuffType::RPM => "RPM",
            BuffType::Range => "Range",
            BuffType::Aura => "Aura",
        }
        .to_string();
        return info + ": " + &self.level.to_string();
    }
}
