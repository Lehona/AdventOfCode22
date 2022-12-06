struct Line {
    a: (usize, usize),
    b: (usize, usize),
}

fn parse_line(line: &str) -> Line {
    let (a, b) = line.split_once(",").unwrap();
    
    let (a_start, a_end) = a.split_once("-").unwrap();
    let (a_start, a_end) = (a_start.parse::<usize>().unwrap(), a_end.parse::<usize>().unwrap()); 

    let (b_start, b_end) = b.split_once("-").unwrap();
    let (b_start, b_end) = (b_start.parse::<usize>().unwrap(), b_end.parse::<usize>().unwrap()); 

    Line {
        a: (a_start, a_end),
        b: (b_start, b_end),
    }
}

fn is_contained(line: &Line) -> bool {
    (line.a.0 <= line.b.0 && line.a.1 >= line.b.1) || (line.a.0 >= line.b.0 && line.a.1 <= line.b.1)
}

fn any_overlap(line: &Line) -> bool {
    (line.a.0 <= line.b.0 && line.a.1 >= line.b.0) || (line.b.0 <= line.a.0 && line.b.1 >= line.a.0)
}

fn main() {
    let input = include_str!("input.txt");

    let solution_1 = input.lines().map(parse_line).filter(is_contained).count();
    println!("The solution to part 1 is {solution_1}");

    let solution_2 = input.lines().map(parse_line).filter(any_overlap).count();
    println!("The solution to part 1 is {solution_2}");
}

#[cfg(test)]
mod test {
    use crate::{parse_line, is_contained, any_overlap};

    #[test]
    fn example_1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let actual = input.lines().map(parse_line).filter(is_contained).count();
        let expected = 2;

        assert_eq!(expected, actual);
    }
    
    #[test]
    fn example_2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let actual = input.lines().map(parse_line).filter(any_overlap).count();
        let expected = 4;

        assert_eq!(expected, actual);
    }
}

