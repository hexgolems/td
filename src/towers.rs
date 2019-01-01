use ggez::graphics;
use ggez::graphics::Point2;
use ggez::{Context, GameResult};
use std::collections::HashMap;

use crate::assets::{Data, ImgID};
use crate::enemies::Enemies;
use crate::game_state::GameState;
use crate::map::GameMap;
use crate::projectiles::{Projectile, Projectiles};
use crate::utils::load_specs;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Deserialize)]
pub enum TowerType {
    Cannon,
    Archer,
}

impl TowerType {
    pub fn get_image_id(&self) -> ImgID {
        match self {
            TowerType::Cannon => ImgID::Cannon,
            TowerType::Archer => ImgID::Archer,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct TowerSpec {
    damage: usize,
    kind: TowerType,
    projectile_speed: f32,
    range: f32,
    rpm: usize,
    price: usize,
}

pub struct Tower {
    cooldown: usize,
    map_position: (usize, usize),
    kind: TowerType,
}

impl Tower {
    pub fn new(kind: TowerType, map_position: (usize, usize)) -> Self {
        return Self {
            map_position,
            kind,
            cooldown: 0,
        };
    }

    pub fn tick(&mut self, enemies: &Enemies, projectiles: &mut Projectiles, spec: &TowerSpec) {
        self.cooldown = self.cooldown.saturating_sub(1);
        if let Some(id) = enemies.weakest_enemy_in_range(
            spec.range,
            GameMap::tile_center(self.map_position.0, self.map_position.1),
        ) {
            if self.cooldown == 0 {
                projectiles.spawn(Projectile::new(
                    GameMap::tile_center(self.map_position.0, self.map_position.1),
                    id,
                    spec.damage,
                    spec.projectile_speed,
                    spec.kind,
                ));
                // 60 sec per minute / rpm * 60 ticks per second
                self.cooldown = 3600 / spec.rpm;
            }
        }
    }
}

pub struct Towers {
    specs: HashMap<TowerType, TowerSpec>,
    built: HashMap<usize, Tower>,
    position_to_towerid: HashMap<(usize, usize), usize>,
    next_tower_id: usize,
}

impl Towers {
    pub fn new() -> Self {
        let specs = load_specs::<TowerSpec>("tower")
            .into_iter()
            .map(|t| (t.kind, t))
            .collect();
        let built = HashMap::new();
        let position_to_towerid = HashMap::new();
        return Self {
            specs,
            built,
            position_to_towerid,
            next_tower_id: 0,
        };
    }

    pub fn spawn(&mut self, tower: Tower) {
        self.position_to_towerid
            .insert(tower.map_position.clone(), self.next_tower_id);
        self.built.insert(self.next_tower_id, tower);
        self.next_tower_id += 1;
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

    pub fn draw(&self, data: &Data, ctx: &mut Context) -> GameResult<()> {
        for (_id, t) in self.built.iter() {
            graphics::draw_ex(
                ctx,
                data.get_i(&t.kind.get_image_id()),
                graphics::DrawParam {
                    // src: src,
                    dest: GameMap::tile_center(t.map_position.0, t.map_position.1),
                    //rotation: self.zoomlevel,
                    offset: Point2::new(0.5, 0.5),
                    scale: Point2::new(4.0, 4.0),
                    // shear: shear,
                    ..Default::default()
                },
            )?;
        }
        Ok(())
    }

    pub fn tick(state: &mut GameState) {
        for (_id, t) in state.towers.built.iter_mut() {
            t.tick(
                &state.enemies,
                &mut state.projectiles,
                state.towers.specs.get(&t.kind).unwrap(),
            )
        }
    }
}
