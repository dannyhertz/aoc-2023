use anyhow::{Result, Ok, Error};
use std::str::FromStr;
use hashbrown::HashMap;

pub fn solve(input: &str) -> i32 {
    let games = parse_game_history(input).expect("could not parse games");

    let current_bag: HashMap<Die, i32> = [(Die::Red, 12), (Die::Blue, 14), (Die::Green, 13)]
        .iter()
        .cloned()
        .collect();

    return games
        .iter()
        .fold(0, |count, game| {
            return count + if game.violates_bag(&current_bag) { 0 } else { game.id }
        });
}

pub fn solve_2(input: &str) -> i32 {
    let games = parse_game_history(input).expect("could not parse games");

    return games
        .iter()
        .fold(0, |count, game| {
            let power: i32 = game.min_die_needed().values().product();
            return count + power;
        });
}

#[derive(Debug)]
struct Batch {
    die: Die,
    count: i32
}

impl FromStr for Batch {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (count_part, die_part) = s.split_once(" ").expect("expected a space");
        return Ok(Batch { die: die_part.parse()?, count: count_part.parse()? });
    }
}

#[derive(Debug)]
struct Draw {
    batches: Vec<Batch>
}

impl FromStr for Draw {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let batches: Result<Vec<Batch>> = s.split(", ")
            .map(|p| p.parse())
            .collect();

        return Ok(Draw { batches: batches? });
    }
}

impl Draw {
    fn violates_bag(&self, bag: &HashMap<Die, i32>) -> bool {
        return self.batches.iter().any(|b| {
            return *bag.get(&b.die).unwrap_or(&0) < b.count;
        });
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Die {
    Red,
    Green,
    Blue
}

impl FromStr for Die {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "red" {
            return Ok(Die::Red)
        } else if s == "blue" {
            return Ok(Die::Blue)
        } else if s == "green" {
            return Ok(Die::Green)
        } else {
            return Err(Error::msg("invalid color"))
        }
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    draws: Vec<Draw>
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (game_part, values_part) = s.split_once(": ").expect("Must contain a colon");
        let id: i32 = parse_game_id(game_part)?;
        let draws: Result<Vec<Draw>> = values_part.split("; ").map(|p| p.parse::<Draw>()).collect();

        return Ok(Game { id: id, draws: draws? });
    }
}

impl Game {
    fn violates_bag(&self, bag: &HashMap<Die, i32>) -> bool {
        return self.draws.iter().any(|d| {
            return d.violates_bag(bag);
        });
    }

    fn min_die_needed(&self) -> HashMap<Die, i32> {
        return self.draws
            .iter()
            .fold(HashMap::<Die, i32>::new(), |mut acc, draw| {
                for batch in &draw.batches {
                    let entry = acc.entry(batch.die).or_insert(0);
                    *entry = (*entry).max(batch.count);
                }
                acc
            });
    }
}

fn parse_game_history(input: &str) -> Result<Vec<Game>> {
    return input
        .lines()
        .map(|l| l.parse::<Game>())
        .collect();
}

fn parse_game_id(input: &str) -> Result<i32> {
    let (_, id_str) = input.split_once(" ").expect("Must contain a whitespace");
    let id: i32 = str::parse(id_str)?;
    return Ok(id);
}

