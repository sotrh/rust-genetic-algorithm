# Example of a genetic algorithm in Rust and Python

## Monkey typewriter

Finding the phrase 'To be or not to be. That is the question.' Inspired by the example given by [Coding Train](https://www.youtube.com/watch?v=9zfeTw-uFCw). 

Both algorithms start with a randomized array of characters as "genes". The list of individuals is sorted by most fit to least fit, then half of the list is considered "dead". The fit half of the least breeds to bring up the population. This is repeated until the algorithm arrives at the target string. I've listed the `main` function for the rust code below:

```rust
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
```

## How to run

```bash
# For rust
cargo run '<OPTIONAL_TARGET_PHRASE>'
# For python
python3 monkey.py '<OPTIONAL_TARGET_PHRASE>'
# For web example
yarn serve # npm run serve
```

### Rust

```bash
3638 To be or not to be. That is the qulstion. 0.9756098
3639 To be or not to be. That is the qulstion. 0.9756098
3640 To be or not to be. That is the qulstion. 0.9756098
3641 To be or not to be. That is the qulstion. 0.9756098
3642 To be or not to be. That is the qulstion. 0.9756098
3643 To be or not to be. That is the question. 1
Most Fit: To be or not to be. That is the question.
Num Generations: 3644
Elapsed Time: 3.12029417s
```

### Python

```bash
To be or not to be. ThCt is the question.
To be or not to be. ThCt is the question.
To be or not to be. ThCt is the question.
To be or not to be. ThCt is the question.
To be or not to be. That is the question.
Total Generations: 966
Time Taken: 3.8000526428222656
```