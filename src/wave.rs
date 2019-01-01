use crate::enemies::Enemy;
use crate::game_state::GameState;
use crate::map::GameMap;
use crate::utils::load_specs;

#[derive(Debug, Deserialize, Clone)]
pub struct WaveSpec {
    speed: f32,
    health: usize,
    enemy_count: usize,
    spawn_delay: usize,
}

pub struct Waves {
    pub id: usize,
    pub waves: Vec<WaveSpec>,
    pub next_spawn: usize,
    pub enemy_count: usize,
}

impl Waves {
    pub fn new() -> Self {
        return Self {
            id: 0,
            waves: load_specs("waves"),
            next_spawn: 0,
            enemy_count: 0,
        };
    }

    fn current_wave(&self) -> WaveSpec {
        self.waves
            .get(self.id)
            .expect("All your base belongs to you!")
            .clone()
    }

    pub fn tick(state: &mut GameState) {
        let wave = state.waves.current_wave();
        if state.waves.enemy_count < wave.enemy_count {
            if state.waves.next_spawn == 0 {
                if state.waves.enemy_count < wave.enemy_count {
                    let (x, y) = state
                        .map
                        .get_spawn_points()
                        .pop()
                        .expect("I need to spawn zombies");
                    state.enemies.spawn(Enemy::new(
                        GameMap::tile_center(x, y),
                        wave.health,
                        wave.speed,
                    ));
                    state.waves.enemy_count += 1;
                    state.waves.next_spawn = wave.spawn_delay;
                }
            } else {
                state.waves.next_spawn -= 1;
            }
        } else {
            if !state.enemies.any_alive() {
                state.waves.id += 1;
                state.waves.next_spawn = 0;
                state.waves.enemy_count = 0;
            }
        }
    }
}
