#[derive(Default, Debug, Clone)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn ok(&self, other: &Round) -> bool {
        self.red >= other.red &&
            self.green >= other.green &&
            self.blue >= other.blue
    }
    fn max(&self, other: &Round) -> Round {
        Round {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn parse(game: &str) -> (u32, Vec<Round>) {
    let (game, rest) = game.split_once(": ").unwrap();
    let (_, game) = game.split_once(' ').unwrap();
    let game = game.parse().unwrap();

    let mut rounds = vec![];

    for reveals in rest.split("; ") {
        let mut round = Round::default();
        for reveal in reveals.split(", ") {
            let (count, colour) = reveal.split_once(' ').unwrap();
            let count = count.parse().unwrap();
            match colour {
                "red" => { round.red = count; },
                "green" => { round.green = count; },
                "blue" => { round.blue = count; },
                _ => { panic!(); },
            }
        }
        rounds.push(round);
    }

    (game, rounds)
}

fn p1(games: &[(u32, Vec<Round>)]) -> u32 {
    let max = Round {
        red: 12,
        green: 13,
        blue: 14,
    };

    games.iter()
        .filter(|(i, rounds)| rounds.iter().all(|r| max.ok(r)))
        .map(|x|x.0).sum()
}
fn p2(games: &[(u32, Vec<Round>)]) -> u32 {
    games.iter().map(|(_, rounds)|
            rounds.iter().cloned()
                .reduce(|a, b| a.max(&b)).unwrap().power())
        .sum()
}

fn main() {
    //let inp = std::fs::read_to_string("inp1.txt").unwrap();
    let inp = std::fs::read_to_string("inp.txt").unwrap();
    let inp = inp.trim();
    let mut games = vec![];
    for line in inp.lines() {
        games.push(parse(line));
    }
    println!("{}", p1(&games));
    println!("{}", p2(&games));
}
