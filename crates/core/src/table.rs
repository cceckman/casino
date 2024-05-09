use std::collections::HashSet;

use crate::casino::GameType;
use crate::player::Player;

#[derive(Debug, Eq, PartialEq)]
pub struct Table {
    pub name: String,
    pub players: HashSet<Player>,
    pub game: GameType,
    pub minimum_buy_in_chips_amount: u32,
    pub maximum_players_count: usize,
}

impl Table {
    pub fn new(
        name: String,
        game_type: GameType,
        minimum_buy_in_chips_amount: u32,
        maximum_players_count: usize,
    ) -> Self {
        Self {
            name,
            players: HashSet::new(),
            game: game_type,
            minimum_buy_in_chips_amount,
            maximum_players_count,
        }
    }

    /// Add a player into the game.
    pub fn add_player(&mut self, player: Player) -> Result<(), &'static str> {
        if self.players.len() > self.maximum_players_count {
            return Err("Unable to join the table. It is already at max capacity.");
        }

        if player.chips < self.minimum_buy_in_chips_amount {
            println!("You do not have enough chips to play at this table.");
            println!("Current chips amount: {}", player.chips);
            println!(
                "Required chips amount: {}",
                self.minimum_buy_in_chips_amount
            );
            println!(
                "Additional chips needed: {}",
                self.minimum_buy_in_chips_amount - player.chips
            );
            return Err("You do not have enough chips to play at this table.");
        }

        println!(
            "{} bought in with {} chips. Good luck!",
            &player.name, &player.chips
        );

        self.players.insert(player);
        Ok(())
    }

    /// Remove a player from the game.
    pub fn remove_player(&mut self, player: &mut Player) -> Option<Player> {
        if self.players.len() < 1 {
            eprintln!("Unable to remove player. The table is empty.");
            return None;
        }

        if self.players.get(player).is_none() {
            eprintln!(
                "Unable to remove player. {} is not at the table.",
                player.name
            );
            return None;
        }

        self.players.take(player)
    }
}
