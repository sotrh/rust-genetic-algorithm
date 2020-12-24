use futures::executor::block_on;
use std::env;

use rust_genetic_algorithm::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target = args
        .get(1)
        .and_then(|arg| Some(arg as &str))
        .unwrap_or("To be or not to be. That is the question.");
    let num_agents = args
        .get(2)
        .and_then(|arg| Some(arg.parse().unwrap()))
        .unwrap_or(100);
    let mut_chance = args
        .get(3)
        .and_then(|arg| Some(arg.parse().unwrap()))
        .unwrap_or(0.5);

    block_on(simulation(target.to_string(), num_agents, mut_chance));
}
