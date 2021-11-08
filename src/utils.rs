use crate::adjectives::ADJECTIVES;
use crate::colors::COLORS;
use crate::whales::WHALES;
use rand::Rng;

fn random_from_array(arr: &[&'static str]) -> &'static str {
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(0..arr.len());
    arr[rand]
}

pub fn random_color() -> &'static str {
    random_from_array(COLORS)
}

pub fn random_adjective() -> &'static str {
    random_from_array(ADJECTIVES)
}

pub fn random_whale() -> &'static str {
    random_from_array(WHALES)
}
