use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchUp {
    pub athlete_one: String,
    pub athlete_two: String,
    pub bets: Vec<Bet>,
    pub winner: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum BetError {
    WrongAthlete,
}

impl MatchUp {
    pub fn new(athlete_one: String, athlete_two: String) -> Self {
        MatchUp {
            athlete_one,
            athlete_two,
            bets: Vec::new(),
            winner: None,
        }
    }

    pub fn add_bet(&mut self, athlete: String, user: String, amount: f32) -> Result<(), BetError> {
        if self.athlete_one != athlete && self.athlete_two != athlete {
            return Err(BetError::WrongAthlete);
        }

        self.bets.push(Bet {
            user,
            amount,
            athlete,
        });
        Ok(())
    }

    pub fn payout(&mut self, athlete: &str) -> Result<Vec<String>, BetError> {
        if !self.athlete_exists(athlete) {
            return Err(BetError::WrongAthlete);
        }
        self.winner = Some(athlete.to_string());
        let mut winning_bets: Vec<&Bet> = Vec::new();
        let mut winning_pot: f32 = 0.0;
        let mut losing_bets: Vec<&Bet> = Vec::new();
        let mut losing_pot: f32 = 0.0;
        let mut output_text: Vec<String> = Vec::new();

        for bet in self.bets.iter() {
            if bet.athlete == athlete {
                winning_pot += bet.amount;
                winning_bets.push(bet);
            } else {
                losing_pot += bet.amount;
                losing_bets.push(bet);
            }
        }
        let winning_ratio: f32 = losing_pot / winning_pot;
        for bet in winning_bets.iter() {
            let winnings: f32 = bet.amount * winning_ratio;
            output_text.push(format!("/add-money {} {}", bet.user, winnings));
        }
        for bet in losing_bets.iter() {
            output_text.push(format!("/remove-money {} {}", bet.user, bet.amount));
        }
        Ok(output_text)
    }

    fn athlete_exists(&self, name: &str) -> bool {
        self.athlete_one == name || self.athlete_two == name
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bet {
    pub user: String,
    pub amount: f32,
    pub athlete: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_match_up() -> MatchUp {
        MatchUp::new(
            "Brzenk".to_string(),
            "Cyplenkov".to_string(),
        )
    }

    #[test]
    fn test_create_match() {
        let match_up = create_match_up();

        assert_eq!("Brzenk".to_string(), match_up.athlete_one);
        assert_eq!("Cyplenkov".to_string(), match_up.athlete_two);
        assert!(match_up.bets.len() == 0);
        assert_eq!(None, match_up.winner);
    }

    #[test]
    fn test_create_bet() {
        let mut match_up = create_match_up();

        let user = "Toop".to_string();
        let athlete = "Brzenk".to_string();
        let amount = 100.0;
        let result = match_up.add_bet(athlete.clone(), user.clone(), amount);

        assert!(result.is_ok());
        assert!(match_up.bets.len() == 1);
        assert!(match_up.bets[0].user == user);
        assert!(match_up.bets[0].athlete == athlete);
        assert!(match_up.bets[0].amount == amount);
    }

    #[test]
    fn test_athlete_name_wrong_error() {
        let mut match_up = create_match_up();

        let result = match_up.add_bet("Pushkar".to_string(), "Toop".to_string(), 100.0);
        assert!(result == Err(BetError::WrongAthlete));
    }

    #[test]
    fn test_payout_match_with_wrong_athlete() {
        let mut match_up = create_match_up();

        match_up.add_bet("Brzenk".to_string(), "Toop".to_string(), 100.0).expect("Foo");
        match_up.add_bet("Cyplenkov".to_string(), "Poot".to_string(), 150.0).expect("Foo");

        let result = match_up.payout("Pushkar");

        assert!(!result.is_ok());
    }

    #[test]
    fn test_payout() {
        let mut match_up = create_match_up();

        match_up.add_bet("Brzenk".to_string(), "Toop".to_string(), 100.0).expect("Foo");
        match_up.add_bet("Brzenk".to_string(), "Toop".to_string(), 200.0).expect("Foo");
        match_up.add_bet("Cyplenkov".to_string(), "Poot".to_string(), 220.0).expect("Foo");
        match_up.add_bet("Cyplenkov".to_string(), "Foo".to_string(), 130.5).expect("Foo");

        let result = match_up.payout("Brzenk").expect("Fail");

        assert_eq!(Vec::from(["/add-money Toop 116.83333", "/add-money Toop 233.66666",  "/remove-money Poot 220", "/remove-money Foo 130.5"]), result);
        assert_eq!(Some("Brzenk".to_string()), match_up.winner);
    }
}


