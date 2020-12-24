from random import randint, random
from time import time

MIN_CHAR = 32
MAX_CHAR = 126

def rand_char():
    return chr(randint(MIN_CHAR, MAX_CHAR))

class Population:
    def __init__(self, target, num, mut_chance):
        self.target = target
        self.num = num
        self.mut_chance = mut_chance
        self.most_fit = None
        num_genes = len(target)
        self.population = [Individual.random(num_genes) for _ in range(num)]
    
    def selection(self):
        self.population.sort(key=lambda i: i.fitness(self.target), reverse=True)
        self.most_fit = self.population[0]

    def reproduce(self):
        # Perfectly balanced as all things should be
        self.population = self.population[:len(self.population)//2]
        n = len(self.population)
        for i in range(0, n + self.num % 2):
            a = self.population[i]
            b = self.population[i+1]
            self.population.append(a.breed(b, self.mut_chance))
    
    def __repr__(self):
        result = ''
        for i in self.population:
            result += f'{i}\n'
        return result

class Individual:
    def __init__(self, genes):
        self.genes = genes

    def random(num_genes):
        genes = [rand_char() for _ in range(num_genes)]
        return Individual(genes)

    def fitness(self, target):
        f = 0
        for i, g in enumerate(target):
            if g == self.genes[i]:
                f += 1
        return f / len(target)

    def breed(self, other, mut_chance):
        half = len(self.genes) // 2
        genes = self.genes[0:half] + other.genes[half:]
        individual = Individual(genes)
        if mut_chance >= random():
            individual.mutate()
        return individual

    def mutate(self):
        index = randint(0, len(self.genes)-1)
        self.genes[index] = rand_char()

    def __str__(self):
        return ''.join(self.genes)
    
    def __repr__(self):
        return str(self)

if __name__ == '__main__':
    import sys

    target = "To be or not to be. That is the question."
    num = 500
    mut_chance = 0.5
    num_args = len(sys.argv)
    if num_args > 1:
        target = str(sys.argv[1])
    if num_args > 2:
        num = int(sys.argv[2])
    if num_args > 3:
        mut_chance = float(sys.argv[3])

    p = Population(target, num, mut_chance)
    generations = 0
    start_time = time()
    last_fitness = None
    fitness = 0.0
    while p.most_fit is None or fitness < 1.0:
        p.selection()
        most_fit = p.most_fit
        fitness = most_fit.fitness(target)
        if last_fitness is None or last_fitness != fitness:
            print(f'{generations}\t{p.most_fit}\t{fitness}')
            last_fitness = fitness
        p.reproduce()
        generations += 1
    end_time = time()

    print(f"Simulation ended:")
    print(f"Most Fit:\t{p.most_fit}")
    print(f"Generations:\t{generations}")
    print(f"Elapsed Time:\t{end_time - start_time}")