use std::{
    io,
    fs,
    error::Error,
};
use colored::Colorize;
use serde::{Serialize, Deserialize};

mod data;
use crate::data::MatchUp;

fn exit() {
    println!("Bye!");
    std::process::exit(420);
}

fn get_input(prompt: &str) -> Result<String, String> {
    println!("{}", prompt);

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read input".to_string()),
    };

    Ok(input.trim().to_string())
}

#[derive(Serialize, Deserialize)]
struct MatchUpContainer {
    match_ups: Vec<MatchUp>,
}

struct MatchUpRepository {
    match_up_container: MatchUpContainer,
}

impl MatchUpRepository {
    pub fn new() -> Self {
        let serialized = fs::read_to_string("data.json").unwrap();
        let match_up_container: MatchUpContainer = serde_json::from_str(&serialized).unwrap();

        MatchUpRepository {
            match_up_container,
        }
    }

    pub fn add_match_up(&mut self, match_up: MatchUp) -> Result<(), Box<dyn Error>> {
        self.match_up_container.match_ups.push(match_up);

        self.save();

        Ok(())
    }

    pub fn get_match_ups(&mut self) -> &mut Vec<MatchUp> {
        &mut self.match_up_container.match_ups
    }

    pub fn save(&self) {
        let serialized = serde_json::to_string(&self.match_up_container).unwrap();

        fs::File::create("data.json").unwrap();

        fs::write("data.json", serialized).unwrap();
    }
}

struct MatchUpController {
    match_up_repository: MatchUpRepository,
}

impl MatchUpController {
    pub fn new(match_up_repository: MatchUpRepository) -> MatchUpController {
        MatchUpController {
            match_up_repository,
        }
    }

    pub fn add_match_up(&mut self) {
        let athlete_one = match get_input("Enter the first athlete's name") {
            Ok(athlete_one) => athlete_one,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };
        let athlete_two = match get_input("Enter the second athlete's name") {
            Ok(athlete_two) => athlete_two,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };
        self.match_up_repository.add_match_up(MatchUp::new(athlete_one, athlete_two)).unwrap_or_else(|error| {
            println!("Error: {}", error);
            println!("Failed to add match up");
        });
    }

    pub fn add_bet(&mut self) {
        self.show_match_ups();
        let match_up = match get_input("Enter the match up number") {
            Ok(match_up) => match_up,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };

        let index: usize = match match_up.trim().parse() {
            Ok(index) => index,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };

        let match_up = match self.match_up_repository.get_match_ups().get_mut(index - 1) {
            Some(match_up) => match_up,
            None => {
                println!("Error: Invalid match up");
                return;
            }
        };

        println!("Select an athlete");
        println!("1. {}", match_up.athlete_one);
        println!("2. {}", match_up.athlete_two);

        let athlete_number = match get_input("Enter the athlete number") {
            Ok(athlete_number) => athlete_number,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };

        let athlete_number: usize = match athlete_number.trim().parse() {
            Ok(athlete_number) => athlete_number,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };

        let athlete = match athlete_number {
            1 => match_up.athlete_one.clone(),
            2 => match_up.athlete_two.clone(),
            _ => {
                println!("Error: Invalid athlete number");
                return;
            }
        };

        let user = match get_input("Enter user's name") {
            Ok(user) => user,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };

        let amount = match get_input("Enter amount") {
            Ok(amount) => amount,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };

        let amount: f32 = match amount.trim().parse() {
            Ok(amount) => amount,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };

        match match_up.add_bet(athlete, user, amount) {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to find athlete");
                return;
            }
        };

        self.match_up_repository.save();

        println!("Bet added successfully");
    }

    pub fn show_match_ups(&mut self) {
        println!("* --- Match Ups --- *");
        for (i, match_up) in self.match_up_repository.get_match_ups().iter().enumerate() {
            println!("{}. {} vs {}", i + 1, match_up.athlete_one, match_up.athlete_two);
        }
    }

    pub fn show_bets(&mut self) {
        self.show_match_ups();
        let match_up = match get_input("Enter the match up number") {
            Ok(match_up) => match_up,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };

        let index: usize = match match_up.trim().parse() {
            Ok(index) => index,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };

        let match_up = match self.match_up_repository.get_match_ups().get_mut(index - 1) {
            Some(match_up) => match_up,
            None => {
                println!("Error: Invalid match up");
                return;
            }
        };

        println!("* --- Bets --- *");
        for bet in match_up.bets.iter() {
            println!("{}: {} - {}", bet.user, bet.athlete, bet.amount);
        }
    }

    pub fn calculate_winnings(&mut self) {
        self.show_match_ups();
        let match_up = match get_input("Enter the match up number") {
            Ok(match_up) => match_up,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };

        let index: usize = match match_up.trim().parse() {
            Ok(index) => index,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };

        let match_up = match self.match_up_repository.get_match_ups().get_mut(index - 1) {
            Some(match_up) => match_up,
            None => {
                println!("Error: Invalid match up");
                return;
            }
        };

        println!("Select an athlete");
        println!("1. {}", match_up.athlete_one);
        println!("2. {}", match_up.athlete_two);

        let athlete_number = match get_input("Enter the athlete number") {
            Ok(athlete_number) => athlete_number,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };

        let athlete_number: usize = match athlete_number.trim().parse() {
            Ok(athlete_number) => athlete_number,
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        };

        let athlete = match athlete_number {
            1 => match_up.athlete_one.clone(),
            2 => match_up.athlete_two.clone(),
            _ => {
                println!("Error: Invalid athlete number");
                return;
            }
        };

        let payouts = match match_up.payout(&athlete) {
            Ok(payouts) => payouts,
            Err(_) => {
                println!("Failed to calculate payout.");
                return;
            }
        };

        for payout in payouts {
            println!("{payout}");
        }
    }
}

pub fn run() {
    let mut match_up_controller = MatchUpController::new(MatchUpRepository::new());

    loop {
        println!("    ");
        println!("{}", "* ----------------------- *".red().bold());
        println!("    ");
        println!("{}", "Select an option".yellow().underline());
        println!("1. Display matches");
        println!("2. See bets");
        println!("3. Add match");
        println!("4. Add bet");
        println!("5. Generate results");
        println!("6. Exit");
        println!("    ");
        println!("{}", "* ----------------------- *".red().bold());
        println!("    ");
        let mut option = String::new();

        io::stdin().read_line(&mut option).expect("Failed to read input");

        let option: u8 = match option.trim().parse() {
            Ok(option) => option,
            Err(_) => {
                println!("Failed to parse input. Try again.");
                continue;
            }
        };
        print!("{}[2J", 27 as char);
        match option {
            1 => match_up_controller.show_match_ups(),
            2 => match_up_controller.show_bets(),
            3 => match_up_controller.add_match_up(),
            4 => match_up_controller.add_bet(),
            5 => match_up_controller.calculate_winnings(),
            6 => exit(),
            _ => println!("That's not right, try again"),
        }
    }
}

