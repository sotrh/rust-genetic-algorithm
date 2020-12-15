use std::time::Instant;
use std::env;

use rust_genetic_algorithm::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target: &str = args.get(1)
        .and_then(|arg| Some(arg as &str))
        .unwrap_or("To be or not to be. That is the question.");
    let num_agents = args.get(2)
        .and_then(|arg| Some(arg.parse().unwrap()))
        .unwrap_or(100);
    let mut_chance = args.get(3)
        .and_then(|arg| Some(arg.parse().unwrap()))
        .unwrap_or(0.1);

    let mut p = Population::random(target, num_agents, mut_chance);

    let start_time = Instant::now();
    while p.max_fitness() < 1.0 {
        p.select();
        println!("{} {} {}", p.generation(), p.fittest().unwrap(), p.max_fitness());
        p.breed();
    }
    let elapsed = Instant::now() - start_time;

    println!("Most Fit: {}", p.fittest().unwrap());
    println!("Num Generations: {}", p.generation());
    println!("Elapsed Time: {:?}", elapsed);
}