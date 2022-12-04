#[derive(Clone, Copy)]
enum Shape {
    Rock, Paper, Scissors,
}

impl Shape {
    fn win(self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn lose(self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn draw(self) -> Self {
        self
    }

    fn from_me(me: char) -> Self {
        match me {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => unreachable!()
        }
    }

    fn from_opponent(opponent: char) -> Self {
        match opponent {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => unreachable!()
        }
    }

    fn from_result(opponent: Self, result: char) -> Self {
        match result {
            'X' => opponent.lose(),
            'Y' => opponent.draw(),
            'Z' => opponent.win(),
            _ => unreachable!()
        }
    }
    fn score(self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn eval(self, other: Self) -> usize {
        use Shape::*;
        match (self, other) {
            (Rock, Rock) => 3,
            (Rock, Paper) => 0,
            (Rock, Scissors) => 6,
            (Paper, Rock) => 6,
            (Paper, Paper) => 3,
            (Paper, Scissors) => 0,
            (Scissors, Rock) => 0,
            (Scissors, Paper) => 6,
            (Scissors, Scissors) => 3
        }
    }

}

fn parse_line_1(line: &str) -> (Shape, Shape) {
    let mut split = line.split(" ");
    let opponent = split.next().unwrap().chars().next().unwrap();
    let opponent = Shape::from_opponent(opponent);
    let me = split.next().unwrap().chars().next().unwrap();
    let me = Shape::from_me(me);

    (me, opponent)
}

fn parse_line_2(line: &str) -> (Shape, Shape) {
    let mut split = line.split(" ");
    let opponent = split.next().unwrap().chars().next().unwrap();
    let opponent = Shape::from_opponent(opponent);
    let result = split.next().unwrap().chars().next().unwrap();
    let me = Shape::from_result(opponent, result);

    (me, opponent)
}


fn evaluate_1(line: &str) -> usize {
    let (me, opponent) = parse_line_1(line);    


    let result_score = me.eval(opponent);
    let shape_score = me.score();

    result_score + shape_score
}

fn evaluate_2(line: &str) -> usize {
    let (me, opponent) = parse_line_2(line);

    let shape_score = me.score();
    let result_score = me.eval(opponent);

    result_score + shape_score
}


fn main() {
    let input = include_str!("input.txt");

    let overall_score_1 = input.lines().map(evaluate_1).sum::<usize>();
    let overall_score_2 = input.lines().map(evaluate_2).sum::<usize>();
    println!("The overall score 1 is {overall_score_1}");
    println!("The overall score 2 is {overall_score_2}");

}
