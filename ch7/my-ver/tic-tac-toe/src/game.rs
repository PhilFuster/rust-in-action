use std::{io::{self, Write}, os::unix::prelude::OsStrExt, process::exit};
use std::fmt;
use bincode;
use rand::{Rng, thread_rng};
use crate::input::*;
use serde::{ Serialize, Deserialize };

pub enum GameAction {
  Save,
  Quit,
  Winner
}
pub type TicTacToeGame = Vec<String>;
pub type Players = Vec<Player>;

const MARKERS: &[&str; 2] = &["X", "O"];
// each entry contains a winning combination in tic-tac-toe
const WINNING_PATTERNS: &[&[usize];8] = &[&[0, 1, 2], &[0, 3, 6], &[0, 4, 8], &[1, 4, 7], &[2, 5, 8], &[2, 4, 6], &[6, 7, 8], &[3, 4, 5]];
fn display_game(game: &TicTacToeGame, show_moves: bool) -> () {
    // 
    println!("Board:");
    let mut display_board: Vec<String> = vec![];
    let mut move_counter = 1;
    for value in game {
      match value.as_str() {
        "X" | "O" => display_board.push(value.to_string()),
        _ => {
          if show_moves {
            display_board.push(move_counter.to_string());
            move_counter += 1;
          } else {
            display_board.push("_".to_string());
          }
        }
      }
    }
    println!("_{}_|_{}_|_{}_", &display_board[0], &display_board[1], &display_board[2]);
    println!("_{}_|_{}_|_{}_", &display_board[3], &display_board[4], &display_board[5]);
    println!("_{}_|_{}_|_{}_", &display_board[6], &display_board[7], &display_board[8]);    
}
pub struct Player {
  wins: u8,
  name: String,
}
pub struct GameResult {
  // if action is Winner then program will check winner_index
  pub winner_index: usize,
  // no winner, going to have a next_player_index
  pub next_player_index: usize,
  // when winnerIndex is not set, checking GameAction for either save or quit
  pub action: GameAction,
}
struct MoveLocation {
  // move index in relation to other valid moves this turn
  move_index: usize,
  // move index in relation to the position in the board.
  game_index: usize,
}

fn check_winner(game: &TicTacToeGame) -> bool {
  for winning_pattern in WINNING_PATTERNS {
    // make sure no empty strings.
    if game[winning_pattern[0]].trim().is_empty() ||
       game[winning_pattern[1]].trim().is_empty() ||
       game[winning_pattern[2]].trim().is_empty() {
      continue;
    }
    // check markers
    if game[winning_pattern[0]] == game[winning_pattern[1]] && game[winning_pattern[0]] == game[winning_pattern[2]] {
      return true;
    }
  }
  return false
}

pub fn run_game(game: &mut TicTacToeGame, mut next_player_index: usize, new_game: bool) -> Result<GameResult, &str> {
  // local variables
  if new_game {
    next_player_index = thread_rng().gen_range(0..2) as usize;
    println!("player {} goes first.", next_player_index + 1);
  }
  // game loop
  let mut input;
  let mut valid_moves: Vec<MoveLocation> ;
  let mut move_index: usize;
  loop {
    valid_moves = vec![];
    move_index = 1;
    display_game(&game, true);
    // determine valid moves.
    for (i, el) in game.iter().enumerate() {
      if el.trim().is_empty() {
        // move location represents - 
        // position of move in relation to other available moves.
        // position of move in relation to the overall board status.
        valid_moves.push(MoveLocation {
          move_index,
          game_index: i,
        });
        move_index += 1;
      }
    }
    println!("Player {}'s turn.", next_player_index + 1);
    println!("Marker: {}", MARKERS[next_player_index]);
    prompt_user("Enter a move location (1-9), exit(q), or save(s): ");
    input = get_input().unwrap();
    match input.as_str() {
      // quit
      "q" => {
        prompt_user("Are you sure you want to quit without saving?");
        input = get_input().unwrap();
        match input.as_str() {
          "y" | "Y" => {
            return Ok(GameResult{
              winner_index: 999,
              next_player_index,
              action: GameAction::Quit,
            })
          },
          "n" | "N" => {
            println!("exit aborted.");
          },
          _ => {
            println!("Invalid input {}", input);
          }
        }
        continue;
      },
      // save
      "s" => {
        return Ok(GameResult{
          winner_index: 999,
          next_player_index,
          action: GameAction::Save,
        });
      },
      // validate move
      "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
        // validate move in available moves 
        for move_ in &valid_moves {
          if input.parse::<usize>().unwrap() == move_.move_index {
            // valid move. 
            game[move_.game_index] = MARKERS[next_player_index].to_string();
            // check for winner
            if check_winner(&game) {
              return Ok(GameResult{
                winner_index: next_player_index,
                next_player_index,
                action: GameAction::Winner,
              })
            }
            // no winner. increment next_player_index
            if next_player_index == 1 {
              next_player_index = 0
            } else {
              next_player_index += 1;
            }
            continue;
          }
        }
        println!("Invalid move location {}", input.as_str());
        continue;
      },
      _ => {
        println!("Invalid move {}", input);
        continue;
      }
    }
  }
}