use std::fmt::Formatter;
use std::{cmp::{PartialOrd, Ordering}, fmt::Display};

use rand::Rng;
use rand::distributions::Uniform;

pub struct Population<'a> {
    target: &'a str,
    mut_chance: f32,
    agents: Vec<Agent>,
    generation: usize,
    max_fitness: f32,
    has_selected: bool,
}

impl<'a> Population<'a> {
    pub fn random(target: &'a str, num_agents: usize, mut_chance: f32) -> Self {
        let agents = (0..num_agents)
            .map(|_| Agent::random(target.len()))
            .collect();
        Self {
            target,
            mut_chance,
            agents,
            generation: 0,
            max_fitness: 0.0,
            has_selected: false,
        }
    }

    pub fn select(&mut self) {
        let target = self.target;
        self.agents.iter_mut().for_each(|a| a.calc_fitness(target));
        self.agents.sort_by(|a, b| {
            let f_a = a.fitness();
            let f_b = b.fitness();
            f_b.partial_cmp(&f_a).unwrap_or(Ordering::Equal)
        });
        self.max_fitness = self.agents[0].fitness();
        self.has_selected = true;
    }

    pub fn breed(&mut self) {
        let n = self.agents.len();
        let half = n / 2;
        for i in 0..half + n % 2 {
            let a = &self.agents[i];
            let b = &self.agents[i + 1];
            let mut child = a.breed(b);
            if rand::random::<f32>() < self.mut_chance {
                child.mutate();
            }
            self.agents[half + i] = child;
        }
        self.generation += 1;
    }

    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn max_fitness(&self) -> f32 {
        self.max_fitness
    }

    pub fn fittest(&self) -> Option<&Agent> {
        if self.has_selected {
            self.agents.get(0)
        } else {
            None
        }
    }
}

pub struct Agent {
    fitness: f32,
    genes: Vec<char>,
}

impl Agent {
    pub fn random(num_genes: usize) -> Self {
        let genes = (0..num_genes)
            .map(|_| rand_char())
            .collect();
        Self { fitness: 0.0, genes }
    }

    pub fn breed(&self, other: &Self) -> Self {
        let num_genes = self.genes.len();
        let genes = (0..num_genes)
            .map(|i| if i < num_genes / 2 {
                self.genes[i]
            } else {
                other.genes[i]
            })
            .collect();
        Self { genes, fitness: 0.0 }
    }

    pub fn mutate(&mut self) {
        let index = rand::thread_rng().gen_range(0, self.genes.len());
        self.genes[index] = rand_char();
    }
    
    pub fn calc_fitness(&mut self, target: &str) {
        assert_eq!(self.genes.len(), target.len());
        self.fitness = {
            let mut f = 0f32;
            for (i, c) in target.chars().enumerate() {
                if self.genes[i] == c {
                    f += 1.0;
                }
            }
            f / target.len() as f32
        }
    }

    pub fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl Display for Agent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.genes.iter().collect::<String>())
    }
}

fn rand_char() -> char {
    rand::thread_rng()
        .sample(&Uniform::new_inclusive(32u8, 126)) as char
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen(start)]
pub fn main_js() {
    console_error_panic_hook::set_once();
    println!("Please work!");
}