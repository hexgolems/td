use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2};
use ggez::timer;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::env;
use std::path;

use crate::game_state::GameState;
use crate::enemies::Enemy;

pub struct Wave {
    spawnDelay: usize,
    nextSpawn: usize,
    enemyCount: usize,
}

impl Wave {
    pub fn new(spawnDelay:usize, enemyCount: usize) -> Self {
        return Self {
            spawnDelay,
            nextSpawn: 0,
            enemyCount,
        };
    }

    pub fn resetSpawn(&mut self) {
        self.nextSpawn = self.spawnDelay
    }

    pub fn tick(state: &mut GameState) {
        if state.wave.nextSpawn == 0 {
            if state.wave.enemyCount != 0 {
                state.enemies
                    .spawn(Enemy::new(state.map.tile_pos(0, 3), 10.0, 0.25));
                state.wave.enemyCount -= 1;
            }
            state.wave.resetSpawn()
        } else {
            state.wave.nextSpawn -= 1;
        }
    }

}


