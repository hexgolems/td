use crate::assets::ImgID;
use crate::enemy::Enemy;
use crate::map::GameMap;
use crate::playing_state::PlayingState;
use crate::utils::load_specs;

#[derive(Debug, Deserialize, Clone)]
pub struct WaveSpec {
    pub speed: f32,
    pub health: usize,
    pub enemy_count: usize,
    pub spawn_delay: usize,
    pub size: f32,
    pub color: (f32, f32, f32),
    pub img: ImgID,
}

pub struct Waves {
    pub id: usize,
    pub waves: Vec<WaveSpec>,
    pub status: WaveStatus,
    pub next_spawn: usize,
    pub enemy_count: usize,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum WaveStatus {
    WaveFinished,
    Ongoing,
    Waiting(usize),
    Ready,
    LevelFinished,
}
impl Waves {
    pub fn new() -> Self {
        return Self {
            id: 0,
            waves: load_specs("waves"),
            status: WaveStatus::Waiting(5 * 60),
            next_spawn: 0,
            enemy_count: 0,
        };
    }

    fn current_wave(&self) -> WaveSpec {
        self.waves.get(self.id).unwrap().clone()
    }

    fn is_finished(&self) -> bool {
        self.waves.get(self.id).is_none()
    }

    pub fn tick(state: &mut PlayingState) {
        if state.waves.is_finished() {
            state.waves.status = WaveStatus::LevelFinished;
            return;
        }

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
            WaveStatus::WaveFinished | WaveStatus::LevelFinished | WaveStatus::Ready => {
                return;
            }
            WaveStatus::Ongoing => {}
        }
        if state.waves.enemy_count < wave.enemy_count {
            if state.waves.next_spawn == 0 {
                if state.waves.enemy_count < wave.enemy_count {
                    let (x, y) = state
                        .map
                        .get_spawn_points()
                        .pop()
                        .expect("I need to spawn zombies");
                    let pos = GameMap::tile_center(x, y);
                    state.enemies.spawn(Enemy::new(pos, &wave));
                    state.effects.fire(pos.x, pos.y);
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
                state.waves.status = WaveStatus::WaveFinished;
            }
        }
    }
}
