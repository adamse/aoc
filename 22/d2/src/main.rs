use std::fs::File;
use std::io::{self, Read};
use std::iter::Iterator;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Outcome {
    Me,
    Draw,
    Opponent,
}

impl Shape {
    fn parse(str: &str) -> Option<Self> {
        match str {
            "A" | "X" => Some(Shape::Rock),
            "B" | "Y" => Some(Shape::Paper),
            "C" | "Z" => Some(Shape::Scissor),
            _ => None,
        }
    }

    fn from_outcome(self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Draw => self,
            Outcome::Opponent => match self {
                Shape::Rock => Shape::Scissor,
                Shape::Scissor => Shape::Paper,
                Shape::Paper => Shape::Rock,
            },
            Outcome::Me => match self {
                Shape::Rock => Shape::Paper,
                Shape::Scissor => Shape::Rock,
                Shape::Paper => Shape::Scissor,
            },
        }
    }

    fn score(self) -> u64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }

    fn winner(self, other: Shape) -> Outcome {
        use Shape::*;

        match (self, other) {
            (Rock, Scissor) => Outcome::Me,
            (Scissor, Paper) => Outcome::Me,
            (Paper, Rock) => Outcome::Me,
            (a, b) if a == b
                => Outcome::Draw,
            (_, _) => Outcome::Opponent
        }
    }
}

impl Outcome {
    fn parse(str: &str) -> Option<Self> {
        match str {
            "X" => Some(Outcome::Opponent),
            "Y" => Some(Outcome::Draw),
            "Z" => Some(Outcome::Me),
            _ => None,
        }
    }

    fn score(self) -> u64 {
        match self {
            Outcome::Me => 6,
            Outcome::Draw => 3,
            Outcome::Opponent => 0,
        }
    }
}

struct Pairs<'a, I: 'a>
    where
        I: Iterator,
        I::Item: 'a, {
    parent: &'a mut I,
}

impl<'a, I: Iterator> Pairs<'a, I> {
    fn new(parent: &'a mut I) -> Self {
        Pairs { parent }
    }
}

impl<'a, I: Iterator> Iterator for Pairs<'a, I> {
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.parent.next()?, self.parent.next()?))
    }
}


fn main() -> io::Result<()> {
    {
        let mut file = File::open("big")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut shapes = contents.split_whitespace().map(|x|Shape::parse(x).expect("failed to parse share"));

        let mut score = 0u64;

        for (opponent, me) in Pairs::new(&mut shapes) {
            score += me.score();
            score += me.winner(opponent).score();
        }

        println!("{score}");
    }

    {
        let mut file = File::open("big")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut score = 0u64;

        for (opponent, outcome) in Pairs::new(&mut contents.split_whitespace()) {
            let opponent = Shape::parse(opponent).expect("failed to parse Shape");
            let outcome = Outcome::parse(outcome).expect("failed to parse Outcome");
            let me = opponent.from_outcome(outcome);

            // println!("{opponent:?} {outcome:?} {me:?}");

            score += me.score();
            score += me.winner(opponent).score();
        }

        println!("{score}");
    }

    Ok(())
}
