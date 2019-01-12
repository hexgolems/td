use crate::buffs::{self, BuffType};
use crate::towers::Tower;

pub struct Auras {
    pub id_to_buffs: HashMap<id, HashMap<BuffType, usize>>,
}

impl Auras {
    fn get_aura_levels(
        id: usize,
        id_aura_buff_to_level: &HashMap<(usize, BuffType), usize>,
    ) -> HashMap<BuffType, usize> {
        let mut auras = HashMap::new();
        for buff in vec![BuffType::Damage].iter() {
            if let Some(x) = id_aura_buff_to_level.get(&(id, *buff)) {
                auras.insert(*buff, *x);
            };
        }
        return auras;
    }
}
