pub fn solution() {
    let top_three = include_str!("inputs/01.txt")
        .split("\n\n")
        .map(|set| {
            set.lines()
                .map(str::parse::<u64>)
                .map(Result::unwrap)
                .sum::<u64>()
        })
        .fold([0, 0, 0], |mut acc, n| {
            for value in &mut acc {
                if n > *value {
                    *value = n;
                    break;
                }
            }
            acc
        });      
        
    let max = top_three[0];
    let sum = top_three.iter().sum::<u64>();
    
    println!("The highest number of calories carried by any individual elf is {max}.");
    println!("The total number of calories carried by the top three load-bearing elves is {sum}.");
}