use crate::assets::{ImgID, Imgs};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum CardType {
    BuildCannon,
    BuildArchers,
}

impl CardType {
    pub fn get_image_id(&self) -> ImgID {
        match self {
            CardType::BuildCannon => ImgID::Cannon,
            CardType::BuildArchers => ImgID::Archers,
        }
    }
}
