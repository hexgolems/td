use crate::algebra::{Point, Vector};
use crate::assets::{Data, ImgID};
use crate::buffs::{Buff, BuffStats, BuffType};
use crate::map::GameMap;
use crate::playing_state::PlayingState;
use crate::tower::Tower;
use crate::tower_stats::TowerStats;
use crate::utils::buff_to_img;
use crate::utils::load_specs;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Towers {
    pub stats: TowerStats,
    pub buff_stats: HashMap<BuffType, Rc<BuffStats>>,
    built: HashMap<usize, Tower>,
    position_to_towerid: HashMap<(usize, usize), usize>,
    next_tower_id: usize,
}

impl Towers {
    pub fn new() -> Self {
        let stats = load_specs::<TowerStats>("tower")[0].clone();
        let buffs = load_specs::<BuffStats>("buffs");
        let mut buff_stats = HashMap::new();
        for buff in buffs.iter() {
            buff_stats.insert(buff.kind, Rc::new(buff.clone()));
        }
        let built = HashMap::new();
        let position_to_towerid = HashMap::new();
        return Self {
            buff_stats,
            stats,
            built,
            position_to_towerid,
            next_tower_id: 0,
        };
    }

    pub fn spawn(&mut self, mut tower: Tower) {
        tower.id = self.next_tower_id;
        self.next_tower_id += 1;
        self.position_to_towerid
            .insert(tower.map_position.clone(), tower.id);
        self.built.insert(tower.id, tower);
    }

    pub fn buffs_at(&self, x: usize, y: usize) -> Option<Vec<Buff>> {
        let mut buffs = Vec::new();
        if let Some(tower) = self.get_tower(x, y) {
            for (_, buff) in tower.buffs.iter() {
                buffs.push(buff.clone());
            }
            return Some(buffs);
        }
        return None;
    }

    pub fn stats_at(&self, x: usize, y: usize) -> Option<TowerStats> {
        let mut auras = HashMap::new();
        for (_id, t) in self.built.iter() {
            auras = self.cast_aura(auras, t);
        }
        let default: HashMap<BuffType, Buff> = HashMap::new();
        if let Some(tower) = self.get_tower(x, y) {
            return Some(TowerStats::get_buffed_stats(
                &tower,
                auras.get(&tower.id).unwrap_or(&default),
                &self.stats,
            ));
        }
        return None;
    }

    pub fn has_building(&self, x: usize, y: usize) -> bool {
        return self.position_to_towerid.contains_key(&(x, y));
    }

    pub fn remove_tower(&mut self, x: usize, y: usize) {
        if let Some(id) = self.position_to_towerid.get(&(x, y)) {
            self.built.remove(id);
            self.position_to_towerid.remove(&(x, y));
        }
    }

    pub fn add_buff_at_pos(&mut self, x: usize, y: usize, buff_type: BuffType) {
        let stats = self.buff_stats.get(&buff_type).unwrap().clone();
        if let Some(tower) = self.get_tower_mut(x, y) {
            tower.add_buff(stats);
        }
    }

    pub fn get_tower_mut(&mut self, x: usize, y: usize) -> Option<&mut Tower> {
        if let Some(id) = self.position_to_towerid.get(&(x, y)) {
            return self.built.get_mut(&id);
        }
        return None;
    }

    pub fn get_tower(&self, x: usize, y: usize) -> Option<&Tower> {
        if let Some(id) = self.position_to_towerid.get(&(x, y)) {
            return self.built.get(&id);
        }
        return None;
    }

    pub fn draw(state: &PlayingState, data: &Data, ctx: &mut Context) -> GameResult<()> {
        for (_id, t) in state.towers.built.iter() {
            graphics::draw(
                ctx,
                data.get_i(&ImgID::Archer),
                graphics::DrawParam::default()
                    .dest(
                        state
                            .gui
                            .cam()
                            .pos(GameMap::tile_center(t.map_position.0, t.map_position.1)),
                    )
                    .offset(Point::new(0.5, 0.5))
                    .scale(Vector::new(4.0, 4.0)),
            )?;
            for (i, buff) in t.buffs.keys().into_iter().enumerate() {
                let mut offset = Point::new(1.25, -0.75);
                if i == 1 {
                    offset = Point::new(-0.25, -0.75);
                }
                graphics::draw(
                    ctx,
                    data.get_i(&buff_to_img(buff)),
                    graphics::DrawParam::default()
                        .dest(
                            state
                                .gui
                                .cam()
                                .pos(GameMap::tile_center(t.map_position.0, t.map_position.1)),
                        )
                        .offset(offset)
                        .scale(Vector::new(1.0, 1.0)),
                )?;
            }
        }
        Ok(())
    }

    pub fn cast_aura(
        &self,
        mut auras: HashMap<usize, HashMap<BuffType, Buff>>,
        t: &Tower,
    ) -> HashMap<usize, HashMap<BuffType, Buff>> {
        for id in self.affected_by_aura(t).iter() {
            for (buff_type, buff) in t.buffs.iter() {
                if *buff_type != BuffType::Aura {
                    let default: HashMap<BuffType, Buff> = HashMap::new();
                    let buffs = auras.entry(*id).or_insert(default);
                    match buffs.get(buff_type) {
                        None => {
                            buffs.insert(*buff_type, buff.clone());
                        }
                        Some(b) => {
                            if buff.level > b.level {
                                buffs.insert(*buff_type, buff.clone());
                            }
                        }
                    }
                }
            }
        }
        return auras;
    }

    pub fn affected_by_aura(&self, t: &Tower) -> Vec<usize> {
        let mut nn = Vec::new();
        for (x, y) in GameMap::tile_potential_neighbors(
            t.map_position.0 as isize,
            t.map_position.1 as isize,
            t.aura_level(),
        )
        .iter()
        {
            println!("Potential neighbor is: {}, {}", *x, *y);
            if *x >= 0 && *y >= 0 {
                if let Some(id) = self.position_to_towerid.get(&(*x as usize, *y as usize)) {
                    if *id != t.id {
                        println!("Found tower with id: {}", *id);
                        nn.push(*id);
                    }
                };
            }
        }
        return nn;
    }

    pub fn tick(state: &mut PlayingState) {
        let mut auras = HashMap::new();
        for (_id, t) in state.towers.built.iter() {
            auras = state.towers.cast_aura(auras, t);
        }
        let default: HashMap<BuffType, Buff> = HashMap::new();
        for (id, t) in state.towers.built.iter_mut() {
            t.tick(
                &state.enemies,
                &mut state.projectiles,
                &TowerStats::get_buffed_stats(
                    &t,
                    auras.get(id).unwrap_or(&default),
                    &state.towers.stats,
                ),
                &auras.get(id).unwrap_or(&default),
            )
        }
    }
}
