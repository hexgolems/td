use crate::enemies::Enemy;
use crate::game_state::GameState;
use crate::map::GameMap;

pub struct Wave {
    spawn_delay: usize,
    next_spawn: usize,
    enemy_count: usize,
}

impl Wave {
    pub fn new(spawn_delay: usize, enemy_count: usize) -> Self {
        return Self {
            spawn_delay,
            next_spawn: 0,
            enemy_count,
        };
    }

    pub fn reset_spawn(&mut self) {
        self.next_spawn = self.spawn_delay
    }

    pub fn tick(state: &mut GameState) {
        if state.wave.next_spawn == 0 {
            if state.wave.enemy_count != 0 {
                for x in state.map.xrange() {
                    for y in state.map.yrange() {
                        if state.map.is_spawn(x, y) {
                            state
                                .enemies
                                .spawn(Enemy::new(GameMap::tile_center(x, y), 10.0, 0.25));
                        }
                    }
                }
                state.wave.enemy_count -= 1;
            }
            state.wave.reset_spawn()
        } else {
            state.wave.next_spawn -= 1;
        }
    }
}
