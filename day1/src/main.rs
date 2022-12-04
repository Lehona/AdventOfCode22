fn main() {
    let input = include_str!("input.txt");

    let elfs = input.split("\r\n\r\n");

    let mut calories: Vec<usize> = elfs.map(|calories| calories.lines().map(|line| line.parse::<usize>().unwrap()).sum()).collect();

    let max = *calories.iter().max().unwrap();

    println!("the elf with the  maximum amount of  calories has {max}");

    calories.sort_by_key(|&c|-(c as isize));

    let top_three: usize = calories[0..3].iter().sum();

    println!("The top 3 calorie amounts summed is {top_three}");
}