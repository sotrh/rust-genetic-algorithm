use std::fmt::Formatter;
use std::{cmp::{PartialOrd, Ordering}, fmt::Display};

#[cfg(not(target_arch="wasm32"))]
use rand::Rng;
#[cfg(not(target_arch="wasm32"))]
use rand::distributions::Uniform;
#[cfg(not(target_arch="wasm32"))]
use std::time::Instant;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch="wasm32")]
use web_sys::console;

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

            if rand() < self.mut_chance {
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
        let index = rand_int(self.genes.len());
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

fn rand() -> f32 {
    #[cfg(target_arch="wasm32")] {
        js_sys::Math::random() as f32
    }
    #[cfg(not(target_arch="wasm32"))] {
        rand::random()
    }
}

fn rand_int(max: usize) -> usize {
    #[cfg(target_arch="wasm32")] {
        (js_sys::Math::random() * max as f64).floor() as usize
    }
    #[cfg(not(target_arch="wasm32"))] {
        rand::thread_rng().gen_range(0, max)
    }
}

fn rand_char() -> char {
    #[cfg(target_arch="wasm32")] {
        let c = (js_sys::Math::random() * (126.0 - 32.0) + 32.0).floor() as u8;
        c as char
    }
    #[cfg(not(target_arch="wasm32"))] {
        rand::thread_rng()
            .sample(&Uniform::new_inclusive(32u8, 126)) as char
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn simulation(
    target: &str, 
    num_agents: usize, 
    mut_chance: f32
) {
    #[cfg(target_arch="wasm32")]
    console_error_panic_hook::set_once();

    #[cfg(target_arch="wasm32")]
    console::log_1(&"Creating population".into());
    #[cfg(not(target_arch="wasm32"))]
    println!("Creating population");
    let mut p = Population::random(target, num_agents, mut_chance);
    
    #[cfg(target_arch="wasm32")]
    console::log_1(&"Starting simulation".into());
    
    #[cfg(target_arch="wasm32")]
    console::time_with_label("simulation");
    #[cfg(not(target_arch="wasm32"))]
    let start_time = Instant::now();
    while p.max_fitness() < 1.0 {
        p.select();
        #[cfg(target_arch="wasm32")]
        console::log_1(&format!("{} {} {}", p.generation(), p.fittest().unwrap(), p.max_fitness()).into());
        #[cfg(not(target_arch="wasm32"))]
        println!("{} {} {}", p.generation(), p.fittest().unwrap(), p.max_fitness());
        p.breed();
    }
    #[cfg(target_arch="wasm32")] {
        console::time_end_with_label("simulation");
        console::log_1(&format!("Most Fit: {}", p.fittest().unwrap()).into());
        console::log_1(&format!("Num Generations: {}", p.generation()).into());
    }
    #[cfg(not(target_arch="wasm32"))] {
        println!("Elapsed Time: {:?}", Instant::now() - start_time);
        println!("Most Fit: {}", p.fittest().unwrap());
        println!("Num Generations: {}", p.generation());
    }
}