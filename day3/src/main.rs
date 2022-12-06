use std::collections::HashSet;

fn priority(item: u8) -> u8 {
    match item {
        b'a'..=b'z' => item  - b'a' + 1,
        b'A'..=b'Z' => item  - b'A' + 27,
        _ => unreachable!(),
    }
}

fn find_common_item(line: &str) -> usize {
    let len = line.len();
    let left = line.bytes().take(len/2).map(priority).collect::<HashSet<_>>();
    let right = line.bytes().skip(len/2).take(len/2).map(priority).collect::<HashSet<_>>();

    *left.intersection(&right).next().unwrap() as usize
}

fn find_common_in_group(elf1: &str, elf2: &str, elf3: &str) -> usize {
    let set1 = elf1.bytes().map(priority).collect::<HashSet<_>>();
    let set2 = elf2.bytes().map(priority).collect::<HashSet<_>>();
    let set3 = elf3.bytes().map(priority).collect::<HashSet<_>>();

    (*set1.intersection(&set2).copied().collect::<HashSet<_>>().intersection(&set3).next().unwrap()) as usize
}

fn process_input_1(input: &str) -> usize {
    input.lines().map(find_common_item).sum()
}

fn process_input_2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();

    lines.chunks_exact(3).map(|chunk| find_common_in_group(chunk[0], chunk[1], chunk[2])).sum()
}

fn main() {
    let input = include_str!("input.txt");

    let solution_1 = process_input_1(input);
    println!("The solution to part 1 is {solution_1}");

    let solution_2 = process_input_2(input);
    println!("The solution to part 2 is {solution_2}");
}

#[cfg(test)]
mod test {
    use crate::process_input_1;

    #[test]
    fn example_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let actual = process_input_1(input);
        let expected = 157;

        assert_eq!(expected, actual);
    }
}
