#[derive(Default, Debug, Clone)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn parse(game: &str) -> Game {
    let (game, rest) = game.split_once(": ").unwrap();
    let (_, game) = game.split_once(' ').unwrap();
    let id = game.parse().unwrap();

    let mut game = Game::default();
    game.id = id;
    for reveals in rest.split("; ") {
        for reveal in reveals.split(", ") {
            let (count, colour) = reveal.split_once(' ').unwrap();
            let count = count.parse().unwrap();
            match colour {
                "red" => { game.red = game.red.max(count); },
                "green" => { game.green = game.green.max(count); },
                "blue" => { game.blue = game.blue.max(count); },
                _ => { panic!(); },
            }
        }
    }

    game
}

fn p1(games: &[Game]) -> u32 {
    let st = std::time::Instant::now();

    let res = games.iter()
        .filter(|g| g.red <= 12 && g.green <= 13 && g.blue <= 14)
        .map(|x|x.id).sum();

    eprintln!("p1: {}ns", st.elapsed().as_nanos());
    res
}
fn p2(games: &[Game]) -> u32 {
    let st = std::time::Instant::now();
    let res = games.iter().map(Game::power)
        .sum();
    eprintln!("p2: {}ns", st.elapsed().as_nanos());
    res
}
//const INP: &[u8] = include_bytes!("../inp.txt");

fn main() {
    //let inp = std::fs::read_to_string("inp1.txt").unwrap();
    let inp = std::fs::read_to_string("inp.txt").unwrap();
    // let inp = unsafe {std::str::from_utf8_unchecked(INP)};
    let inp = inp.trim();
    let st = std::time::Instant::now();
    let mut games = vec![];
    for line in inp.lines() {
        games.push(parse(line));
    }
    eprintln!("parse: {}us", st.elapsed().as_micros());
    println!("{}", p1(&games));
    println!("{}", p2(&games));
    eprintln!("total: {}us", st.elapsed().as_micros());
}
