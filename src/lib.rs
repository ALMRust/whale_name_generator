mod adjectives;
mod colors;
mod utils;
mod whales;

use crate::utils::{random_adjective, random_color, random_whale};

use inflector::Inflector;

pub fn generate_whale_name() -> String {
    let adjective = random_adjective().to_title_case();
    let color = random_color().to_title_case();
    let whale = random_whale().to_title_case();

    return format!("{} {} {}", adjective, color, whale);
}
