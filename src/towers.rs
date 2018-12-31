use crate::enemies::Enemies;
use crate::game_state::GameState;
use ggez::graphics;
use ggez::graphics::Point2;
use ggez::{Context, GameResult};
use std::collections::HashSet;

use crate::assets::{ImgID, Imgs};
use crate::map::GameMap;

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
}

pub struct Tower {
    kind: TowerType,
    map_position: (usize, usize),
    damage: usize,
    range: f32,
    rpm: usize,
    cooldown: usize,
}

impl Tower {
    pub fn new(
        kind: TowerType,
        map_position: (usize, usize),
        damage: usize,
        range: f32,
        rpm: usize,
    ) -> Self {
        return Self {
            kind: kind,
            map_position,
            damage,
            range,
            rpm,
            cooldown: 0,
        };
    }

    pub fn tick(&mut self, enemies: &mut Enemies) {
        self.cooldown = self.cooldown.saturating_sub(1);
        if let Some(id) = enemies.weakest_enemy_in_range(
            self.range,
            GameMap::tile_center(self.map_position.0, self.map_position.1),
        ) {
            if self.cooldown == 0 {
                enemies.damage(id, self.damage);
                // 60 sec per minute / rpm * 60 ticks per second
                self.cooldown = 3600 / self.rpm;
            }
        }
    }
}

pub struct Towers {
    towers: Vec<Tower>,
    blocked_positions: HashSet<(usize, usize)>,
}

impl Towers {
    pub fn new() -> Self {
        let towers = vec![];
        let blocked_positions = HashSet::new();
        return Self {
            towers,
            blocked_positions,
        };
    }

    pub fn spawn(&mut self, tower: Tower) {
        self.blocked_positions.insert(tower.map_position.clone());
        self.towers.push(tower);
    }

    pub fn is_buildable(&self, x: usize, y: usize) -> bool {
        return !self.blocked_positions.contains(&(x, y));
    }

    pub fn draw(&self, imgs: &Imgs, ctx: &mut Context) -> GameResult<()> {
        for e in self.towers.iter() {
            graphics::draw_ex(
                ctx,
                imgs.get(&e.kind.get_image_id()),
                graphics::DrawParam {
                    // src: src,
                    dest: GameMap::tile_center(e.map_position.0, e.map_position.1),
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
        for t in state.towers.towers.iter_mut() {
            t.tick(&mut state.enemies)
        }
    }
}
