use anyhow::Result;


fn population_after(mut fish: [usize; 9], n: usize) -> usize {
    // Could be O(d^3 log(n)) with d = 9 fast matrix exponentiation or using
    // "An effficient formula for linear recurrences" by C. Fiduccia in
    // O(d log(d) log(n)).
    for day in 0..n {
        fish[(day + 7) % 9] += fish[day % 9];
    }

    fish.into_iter().sum::<usize>()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day06.txt")?;
    let start = std::time::Instant::now();
    let mut fish = [0usize; 9];
    for age in input.split(',') {
        let age: usize = age.trim().parse()?;
        fish[age] += 1;
    }

    let (part1, part2) = (population_after(fish, 80), population_after(fish, 256));
    println!("time: {:?}", start.elapsed());
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}
