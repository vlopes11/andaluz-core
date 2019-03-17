use crate::heuristic::Heuristic;

pub mod attacksum;
pub mod attacksuminverse;
pub mod bruteforce;
pub mod horse;
pub mod prioritizecenter;

pub trait HeuristicImplementation {
    fn new(weigth: f64) -> Self;
    fn to_heuristic<'a>(self) -> Heuristic<'a>;
}
