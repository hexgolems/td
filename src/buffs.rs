use crate::curses::CurseType;
use crate::projectiles::Projectile;
use crate::towers::Tower;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum BuffType {
    Freeze,
}

pub fn calc_buff_projectile_effect(tower: &Tower, p: &mut Projectile) {
    if tower.get_buffs().contains_key(&BuffType::Freeze) {
        p.add_curse(CurseType::Freeze);
    }
}
