use ggez::graphics;
use ggez::graphics::Point2;
use ggez::{Context, GameResult};
use std::collections::HashMap;

use crate::assets::{Data, ImgID};
use crate::buffs::BuffType;
use crate::map::GameMap;
use crate::playing_state::PlayingState;
use crate::tower::Tower;
use crate::tower_stats::TowerStats;
use crate::utils::buff_to_img;
use crate::utils::load_specs;

pub struct Towers {
    pub stats: TowerStats,
    built: HashMap<usize, Tower>,
    position_to_towerid: HashMap<(usize, usize), usize>,
    next_tower_id: usize,
}

impl Towers {
    pub fn new() -> Self {
        let stats = load_specs::<TowerStats>("tower")[0].clone();
        let built = HashMap::new();
        let position_to_towerid = HashMap::new();
        return Self {
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

    pub fn has_building(&self, x: usize, y: usize) -> bool {
        return self.position_to_towerid.contains_key(&(x, y));
    }

    pub fn remove_tower(&mut self, x: usize, y: usize) {
        if let Some(id) = self.position_to_towerid.get(&(x, y)) {
            self.built.remove(id);
            self.position_to_towerid.remove(&(x, y));
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
            graphics::draw_ex(
                ctx,
                data.get_i(&ImgID::Archer),
                graphics::DrawParam {
                    // src: src,
                    dest: state
                        .gui
                        .cam()
                        .pos(GameMap::tile_center(t.map_position.0, t.map_position.1)),
                    //rotation: self.zoomlevel,
                    offset: Point2::new(0.5, 0.5),
                    scale: Point2::new(4.0, 4.0),
                    // shear: shear,
                    ..Default::default()
                },
            )?;
            for (i, buff) in t.buff_to_level.keys().into_iter().enumerate() {
                let mut offset = Point2::new(1.25, -0.75);
                if i == 1 {
                    offset = Point2::new(-0.25, -0.75);
                }
                graphics::draw_ex(
                    ctx,
                    data.get_i(&buff_to_img(buff)),
                    graphics::DrawParam {
                        // src: src,
                        dest: state
                            .gui
                            .cam()
                            .pos(GameMap::tile_center(t.map_position.0, t.map_position.1)),
                        //rotation: self.zoomlevel,
                        offset: offset,
                        scale: Point2::new(1.0, 1.0),
                        // shear: shear,
                        ..Default::default()
                    },
                )?;
            }
        }
        Ok(())
    }

    pub fn cast_aura(
        &self,
        mut auras: HashMap<usize, HashMap<BuffType, usize>>,
        t: &Tower,
    ) -> HashMap<usize, HashMap<BuffType, usize>> {
        for id in self.affected_by_aura(t).iter() {
            for (buff, level) in t.buff_to_level.iter() {
                if *buff != BuffType::Aura {
                    let default: HashMap<BuffType, usize> = HashMap::new();
                    let hash = auras.entry(*id).or_insert(default);
                    let b = hash.entry(*buff).or_insert(0);
                    *b += level;
                }
            }
        }
        return auras;
    }

    pub fn affected_by_aura(&self, t: &Tower) -> Vec<usize> {
        let mut nn = Vec::new();
        let n = t.aura_level();
        for x in
            (t.map_position.0.saturating_sub(n)..t.map_position.0.saturating_add(n)).into_iter()
        {
            for y in
                (t.map_position.1.saturating_sub(n)..t.map_position.1.saturating_add(n)).into_iter()
            {
                if let Some(id) = self.position_to_towerid.get(&(x, y)) {
                    if *id != t.id {
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
        let default: HashMap<BuffType, usize> = HashMap::new();
        for (id, t) in state.towers.built.iter_mut() {
            t.tick(
                &state.enemies,
                &mut state.projectiles,
                &TowerStats::get_buffed_stats(
                    &t,
                    auras.get(id).unwrap_or(&default),
                    &state.towers.stats,
                ),
            )
        }
    }
}
