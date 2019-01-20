use crate::buffs::Buff;
use crate::buffs::BuffType;

#[derive(Debug, Deserialize, Clone)]
pub struct Debuff {
    pub kind: BuffType,
    pub cooldown: usize,
    pub effectiveness: usize,
}

impl Debuff {
    pub fn new(buff: &Buff) -> Self {
        return Self {
            kind: buff.stats.kind,
            effectiveness: buff.effectiveness(),
            cooldown: buff.effectiveness(),
        };
    }
}
