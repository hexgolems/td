use ggez::graphics;
use ggez::graphics::Point2;
use ggez::{Context, GameResult};
use std::collections::HashMap;

use crate::assets::{ImgID, Imgs};
use crate::enemies::Enemies;
use crate::game_state::GameState;
use crate::map::GameMap;
use crate::projectiles::{Projectile, Projectiles};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum TowerType {
    Cannon,
    Archers,
}

impl TowerType {
    pub fn get_image_id(&self) -> ImgID {
        match self {
            TowerType::Cannon => ImgID::Cannon,
            TowerType::Archers => ImgID::Archers,
        }
    }

    pub fn get_base_stats(&self) -> (usize, f32, usize, f32) {
        match self {
            TowerType::Cannon => (50, 200.0, 30, 5.0),
            TowerType::Archers => (10, 100.0, 120, 2.5),
        }
    }
}

pub struct Tower {
    kind: TowerType,
    map_position: (usize, usize),
    damage: usize,
    range: f32,
    rpm: usize,
    cooldown: usize,
    projectile_speed: f32,
}

impl Tower {
    pub fn new(kind: TowerType, map_position: (usize, usize)) -> Self {
        let (damage, range, rpm, projectile_speed) = kind.get_base_stats();
        return Self {
            kind: kind,
            map_position,
            damage,
            range,
            rpm,
            cooldown: 0,
            projectile_speed,
        };
    }

    pub fn tick(&mut self, enemies: &Enemies, projectiles: &mut Projectiles) {
        self.cooldown = self.cooldown.saturating_sub(1);
        if let Some(id) = enemies.weakest_enemy_in_range(
            self.range,
            GameMap::tile_center(self.map_position.0, self.map_position.1),
        ) {
            if self.cooldown == 0 {
                projectiles.spawn(Projectile::new(
                    GameMap::tile_center(self.map_position.0, self.map_position.1),
                    id,
                    self.damage,
                    self.projectile_speed,
                    self.kind,
                ));
                // 60 sec per minute / rpm * 60 ticks per second
                self.cooldown = 3600 / self.rpm;
            }
        }
    }
}

pub struct Towers {
    towers: HashMap<usize, Tower>,
    position_to_towerid: HashMap<(usize, usize), usize>,
    next_tower_id: usize,
}

impl Towers {
    pub fn new() -> Self {
        let towers = HashMap::new();
        let position_to_towerid = HashMap::new();
        return Self {
            towers,
            position_to_towerid,
            next_tower_id: 0,
        };
    }

    pub fn spawn(&mut self, tower: Tower) {
        self.position_to_towerid
            .insert(tower.map_position.clone(), self.next_tower_id);
        self.towers.insert(self.next_tower_id, tower);
        self.next_tower_id += 1;
    }

    pub fn has_building(&self, x: usize, y: usize) -> bool {
        return self.position_to_towerid.contains_key(&(x, y));
    }

    pub fn remove_tower(&mut self, x: usize, y: usize) {
        if let Some(id) = self.position_to_towerid.get(&(x, y)) {
            self.towers.remove(id);
            self.position_to_towerid.remove(&(x, y));
        }
    }

    pub fn draw(&self, imgs: &Imgs, ctx: &mut Context) -> GameResult<()> {
        for (_id, t) in self.towers.iter() {
            graphics::draw_ex(
                ctx,
                imgs.get(&t.kind.get_image_id()),
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
        for (_id, t) in state.towers.towers.iter_mut() {
            t.tick(&state.enemies, &mut state.projectiles)
        }
    }
}
