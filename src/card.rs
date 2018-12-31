use crate::assets::ImgID;
use crate::game_state::GameState;
use crate::gui::CursorMode;
use crate::towers::TowerType;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum CardType {
    Empty,
    BuildCannon,
    BuildArchers,
}

impl CardType {
    pub fn get_image_id(&self) -> ImgID {
        match self {
            CardType::Empty => ImgID::EmptySlot,
            CardType::BuildCannon => ImgID::Cannon,
            CardType::BuildArchers => ImgID::Archers,
        }
    }

    pub fn activate(&self, state: &mut GameState) {
        match self {
            CardType::Empty => {}
            CardType::BuildCannon => {
                state.gui.set_cursor(
                    CursorMode::Build {
                        x: 0,
                        y: 0,
                        t: TowerType::Cannon,
                        valid: false,
                    }
                    .update(state),
                );
            }
            CardType::BuildArchers => {
                state.gui.set_cursor(
                    CursorMode::Build {
                        x: 0,
                        y: 0,
                        t: TowerType::Archers,
                        valid: false,
                    }
                    .update(state),
                );
            }
        }
    }
}
