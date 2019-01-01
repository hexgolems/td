use crate::enemies::Enemy;
use crate::game_state::GameState;
use crate::map::GameMap;
use ron;
use ron::de::from_reader;
#[macro_use]
use serde;
use serde_derive;
use std::collections::HashMap;
use std::fs::File;

#[derive(Debug, Deserialize, Clone)]
pub struct Wave {
    speed: f32,
    health: usize,
    enemy_count: usize,
    spawn_delay: usize,
}

pub struct Waves {
    pub id: usize,
    pub waves: Vec<Wave>,
    pub status: WaveStatus,
    pub next_spawn: usize,
    pub enemy_count: usize,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum WaveStatus {
    Finished,
    Ongoing,
    Waiting(usize),
    Ready,
}

impl Waves {
    pub fn new() -> Self {
        return Self {
            id: 0,
            waves: Waves::init_waves(),
            status: WaveStatus::Waiting(5 * 60),
            next_spawn: 0,
            enemy_count: 0,
        };
    }

    fn init_waves() -> Vec<Wave> {
        let waves_path = "resources/rons/waves.ron";
        let mut waves = Vec::new();
        let f = File::open(&waves_path).expect("Failed opening waves.ron");
        waves = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load waves: {}", e);
                ::std::process::exit(1);
            }
        };
        return waves;
    }

    fn current_wave(&self) -> Wave {
        println!("in current_wave: \nwaves: {:?}", self.waves);
        println!("id: {:?}", self.id);
        self.waves
            .get(self.id)
            .expect("All your base belongs to you!")
            .clone()
    }

    pub fn tick(state: &mut GameState) {
        let wave = state.waves.current_wave();
        match state.waves.status {
            WaveStatus::Waiting(ref mut a) => {
                if *a > 0 {
                    *a -= 1;
                    return;
                } else {
                    state.waves.status = WaveStatus::Ready;
                    return;
                }
            }
            WaveStatus::Finished | WaveStatus::Ready => {
                return;
            }
            WaveStatus::Ongoing => {}
        }
        println!("alive? {}", state.enemies.any_alive());
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
                state.waves.status = WaveStatus::Finished;
            }
        }
    }
}
