use colored::Colorize;

mod view;
mod data;
mod repository;
use crate::{
    data::MatchUp,
    repository::MatchUpRepository,
};

pub fn run() {
    let mut match_up_controller = MatchUpController::new(MatchUpRepository::new());

    loop {
        println!("    ");
        println!("{}", "* ----------------------- *".red().bold());
        println!("    ");
        println!("{}", "Options".yellow().underline());
        println!("1. Display matches");
        println!("2. See bets");
        println!("3. Add match");
        println!("4. Add bet");
        println!("5. Delete match");
        println!("6. Delete bet");
        println!("7. Generate results");
        println!("8. Exit\n");

        let option = match view::get_usize_input("Enter an option") {
            Ok(option) => option,
            Err(error) => {
                view::display_error(&error);
                continue;
            }
        };

        print!("{}[2J", 27 as char);
        match option {
            1 => match_up_controller.show_match_ups(),
            2 => match_up_controller.show_bets(),
            3 => match_up_controller.add_match_up(),
            4 => match_up_controller.add_bet(),
            5 => match_up_controller.delete_match(),
            6 => match_up_controller.delete_bet(),
            7 => match_up_controller.calculate_winnings(),
            8 => exit(),
            _ => println!("That's not right, try again"),
        }
    }
}

fn exit() {
    println!("Bye!");
    std::process::exit(420);
}

struct MatchUpController {
    match_up_repository: MatchUpRepository,
}

impl MatchUpController {
    fn new(match_up_repository: MatchUpRepository) -> MatchUpController {
        MatchUpController {
            match_up_repository,
        }
    }

    fn add_match_up(&mut self) {
        let athlete_one = match view::get_string_input("Enter the first athlete's name") {
            Ok(athlete_one) => athlete_one,
            Err(error) => {
                view::display_error(&error);
                return;
            }
        };
        let athlete_two = match view::get_string_input("Enter the second athlete's name") {
            Ok(athlete_two) => athlete_two,
            Err(error) => {
                view::display_error(&error);
                return;
            }
        };
        self.match_up_repository.add_match_up(MatchUp::new(athlete_one, athlete_two)).unwrap_or_else(|_error| {
            view::display_danger("Failed to save match");
        });
    }

    fn add_bet(&mut self) {
        let match_up = match self.get_match_up() {
            Ok(match_up) => match_up,
            Err(error) => {
                view::display_error(&error);
                return;
            }
        };

        println!("Select an athlete");
        println!("1. {}", &match_up.athlete_one);
        println!("2. {}", &match_up.athlete_two);

        let athlete_number = match view::get_usize_input("Enter the athlete number") {
            Ok(athlete_number) => athlete_number,
            Err(error) => {
                view::display_error(&error);
                return;
            }
        };

        let athlete = match athlete_number {
            1 => &match_up.athlete_one,
            2 => &match_up.athlete_two,
            _ => {
                view::display_danger("Failed to add bet: Invalid athlete number");
                return;
            }
        };

        let user = match view::get_string_input("Enter user's name") {
            Ok(user) => user,
            Err(error) => {
                view::display_error(&error);
                return;
            }
        };

        let amount = match view::get_positive_f32_input("Enter amount") {
            Ok(amount) => amount,
            Err(error) => {
                view::display_error(&error);
                return;
            }
        };

        match match_up.add_bet(athlete.clone(), user, amount) {
            Ok(_) => (),
            Err(error) => {
                view::display_error(&error);
                return;
            }
        };

        self.match_up_repository.save();

        view::display_sucess("Successfully added bet");
    }

    fn show_match_ups(&mut self) {
        view::display_title("Matches");
        for (i, match_up) in self.match_up_repository.get_match_ups().iter().enumerate() {
            println!("{}. {} vs {}", i + 1, match_up.athlete_one, match_up.athlete_two);
        }
    }

    fn show_bets(&mut self) {
        let match_up = match self.get_match_up() {
            Ok(match_up) => match_up,
            Err(error) => {
                view::display_error(&error);
                return;
            }
        };

        view::display_title("Bets");
        for (i, bet) in match_up.bets.iter().enumerate() {
            println!("{}. {}: {} - {}", i + 1, bet.user, bet.athlete, bet.amount);
        }
        println!("");
    }

    fn delete_bet(&mut self) {
        let match_up = match self.get_match_up() {
            Ok(match_up) => match_up,
            Err(error) => {
                view::display_error(&error);
                return;
            }
        };
        view::display_title("Bets");
        for (i, bet) in match_up.bets.iter().enumerate() {
            println!("{}. {}: {} - {}", i + 1, bet.user, bet.athlete, bet.amount);
        }
        println!("");
        let index = match view::get_usize_input("Enter the bet number") {
            Ok(index) => index - 1,
            Err(error) => {
                view::display_error(&error);
                return;
            }
        };
        if index >= match_up.bets.len() {
            view::display_error("Failed to delete bet: Invalid bet number");
            return;
        }

        match_up.bets.remove(index);
        self.match_up_repository.save();

        view::display_sucess("Successfully deleted bet");
    }

    fn get_match_up(&mut self) -> Result<&mut MatchUp, String> {
        self.show_match_ups();
        let index = match view::get_usize_input("Enter the match number") {
            Ok(index) => index,
            Err(error) => {
                return Err(error);
            }
        };
        let match_up = match self.match_up_repository.get_match_ups().get_mut(index - 1) {
            Some(match_up) => match_up,
            None => {
                return Err("Failed to add bet: Invalid match number".to_string());
            }
        };
        Ok(match_up)
    }

    fn calculate_winnings(&mut self) {
        let match_up = match self.get_match_up() {
            Ok(match_up) => match_up,
            Err(error) => {
                view::display_error(&error);
                return;
            }
        };

        println!("Select an athlete");
        println!("1. {}", &match_up.athlete_one);
        println!("2. {}", &match_up.athlete_two);

        let athlete_number = match view::get_usize_input("Enter the athlete number") {
            Ok(athlete_number) => athlete_number,
            Err(error) => {
                view::display_error(&error);
                return;
            }
        };

        let athlete = match athlete_number {
            1 => &match_up.athlete_one,
            2 => &match_up.athlete_two,
            _ => {
                view::display_danger("Wrong athlete number");
                return;
            }
        };

        let payouts = match match_up.payout(&athlete) {
            Ok(payouts) => payouts,
            Err(error) => {
                view::display_error(&error);
                return;
            }
        };

        for payout in payouts {
            println!("{payout}");
        }
    }

    fn delete_match(&mut self) {
        self.show_match_ups();
        let index = match view::get_usize_input("Enter the match number") {
            Ok(index) => index,
            Err(error) => {
                view::display_error(&error);
                return;
            }
        };
        let _match_up = match self.match_up_repository.get_match_ups().get(index - 1) {
            Some(match_up) => match_up,
            None => {
                view::display_danger("No match found for this number");
                return;
            }
        };
        self.match_up_repository.delete(index - 1);
        view::display_sucess("Successfully deleted match");
    }
}

