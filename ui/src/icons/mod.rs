use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    #[props(default = 24)]
    pub size: u32,
    #[props(default = "currentColor", into)]
    pub color: String,
    #[props(default = false)]
    pub solid: bool,
}

impl IconProps {
    pub fn fill(&self) -> String {
        if self.solid {
            self.color.clone()
        } else {
            "none".to_string()
        }
    }

    pub fn stroke(&self) -> String {
        if self.solid {
            "none".to_string()
        } else {
            self.color.clone()
        }
    }
}

mod apple;
mod archive_box;
mod arrow_top_right_on_square;
mod bars3;
mod beaker;
mod book_open;
mod bookmark;
mod checkbox;
mod cog;
mod envelope;
mod envelope_open;
mod google;
mod information;
mod lock;
mod newspaper;
mod plus;
mod queue;
mod search;
mod share;
mod star;
mod text_settings;
mod user;

pub use apple::AppleIcon;
pub use archive_box::ArchiveBoxIcon;
pub use arrow_top_right_on_square::ArrowTopRightOnSquareIcon;
pub use bars3::Bars3Icon;
pub use beaker::BeakerIcon;
pub use book_open::BookOpenIcon;
pub use bookmark::BookmarkIcon;
pub use checkbox::CheckboxIcon;
pub use cog::Cog6Tooth;
pub use envelope::EnvelopeIcon;
pub use envelope_open::EnvelopeOpenIcon;
pub use google::GoogleIcon;
pub use information::InformationIcon;
pub use lock::LockIcon;
pub use newspaper::NewspaperIcon;
pub use plus::PlusIcon;
pub use queue::QueueIcon;
pub use search::SearchIcon;
pub use share::ShareIcon;
pub use star::StarIcon;
pub use text_settings::TextSettingsIcon;
pub use user::UserIcon;
