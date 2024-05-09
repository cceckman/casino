use std::collections::{HashMap, HashSet};

use poker::texas_hold_em::TexasHoldEm;

use crate::player::Player;
use crate::table::Table;

type Tables = HashMap<String, Table>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GameType {
    TexasHoldEm(TexasHoldEm),
}

impl GameType {
    pub fn get_string(&self) -> String {
        match self {
            GameType::TexasHoldEm(_) => String::from("Texas Hold 'Em"),
        }
    }
}

#[derive(Debug)]
pub struct Casino {
    games: HashMap<String, Tables>,
    players: HashSet<Player>,
}

impl Casino {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
            players: HashSet::new(),
        }
    }

    // pub fn add_game(&mut self, game_type: GameType) -> Result<(), &'static str> {
    //     let game_type_string = game_type.get_string();
    //
    //     if self.games.get(game_type_string).is_some() {
    //         return Err("Unable to add game. The game already exists.");
    //     }
    //
    //     let game_tables: HashMap<String, Table> = HashMap::new();
    //     self.games.insert(game_type_string, game_tables);
    //     Ok(())
    // }
    //
    // pub fn remove_game(&mut self, game_type: GameType) -> Result<(), &'static str> {
    //     if self.games.get(&game_type).is_none() {
    //         return Err("Unable to remove game. The game does not exist.");
    //     }
    //
    //     self.games.remove(&game_type);
    //     Ok(())
    // }

    pub fn get_games(&self) -> &HashMap<String, Tables> {
        &self.games
    }

    pub fn new_table(
        &mut self,
        name: &str,
        game_type: GameType,
        minimum_buy_in_chips_amount: u32,
        maximum_players_count: usize,
    ) -> Result<Table, &'static str> {
        let table = Table::new(
            name.to_string(),
            game_type,
            minimum_buy_in_chips_amount,
            maximum_players_count,
        );

        return Ok(table);
    }

    pub fn add_table(&mut self, table: Table) -> Result<(), &'static str> {
        // Add game type to the games map if it doesn't exist.
        let game_tables = self
            .games
            .entry(table.game.get_string().clone())
            .or_insert(HashMap::new());

        // Add table to the tables map for the game type entry if it doesn't exist.
        game_tables.entry(table.name.clone()).or_insert(table);

        Ok(())
    }

    pub fn remove_table(
        &mut self,
        table_name: &str,
        game_type: GameType,
    ) -> Result<(), &'static str> {
        if let Some(game_tables) = self.games.get_mut(&game_type.get_string()) {
            // Remove table from the tables map for the game type.
            game_tables.remove(table_name);

            // Remove the game type from the games map if no tables exist for it anymore.
            if game_tables.len() == 0 {
                self.games.remove(&game_type.get_string());
            }

            return Ok(());
        } else {
            return Err("Unable to remove table. The table does not exist.");
        }
    }

    pub fn get_table(&self, table_name: &str, game_type: GameType) -> Result<&Table, &'static str> {
        if let Some(game_tables) = self.games.get(&game_type.get_string()) {
            if let Some(table) = game_tables.get(&table_name.to_string()) {
                return Ok(table);
            }
        }

        Err("Unable to get table. No table with that name and GameType found.")
    }

    pub fn add_player(&mut self, player_name: &str) -> Result<&Player, &'static str> {
        let player = Player::new(player_name);
        self.players.insert(player.clone());

        Ok(self.players.get(&player).unwrap())
    }

    pub fn add_player_with_chips(
        &mut self,
        player_name: &str,
        chips: u32,
    ) -> Result<Player, &'static str> {
        let player = Player::new_with_chips(player_name, chips);
        self.players.insert(player.clone());
        Ok(player)
    }

    pub fn remove_player(&mut self, player: &Player) -> Result<Player, &'static str> {
        let removed_player = self.players.take(player);

        if removed_player.is_none() {
            return Err("Unable to remove player. The player does not exist.");
        }

        return Ok(removed_player.unwrap());
    }
}
