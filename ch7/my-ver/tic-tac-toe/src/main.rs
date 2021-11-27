#![allow(unused_imports)]
use libactionkv::{ActionKV, ByteStr, ByteString};
extern crate libactionkv; 
pub mod game;
pub mod input;
use input::*;
use game::*;
use serde::de::MapAccess;

use std::collections::HashMap;
use std::fs::read_dir;
use std::{io::{self, Write}, process::exit};
use std::fmt;
use std::path::PathBuf;
use std::str;
use rand::{Rng, thread_rng};

const SAVE_DIRECTORY: &str = "db/games/"; 
const INDEX_KEY: &ByteStr = b"+index";
const NEXT_PLAYER_INDEX_KEY: &ByteStr = b"next_player_index";
fn store_index_on_disk(a: &mut ActionKV, index_key: &ByteStr){
  a.index.remove(index_key);
  let index_as_bytes = bincode::serialize(&a.index).unwrap();
  a.index = std::collections::HashMap::new();
  a.insert(index_key, &index_as_bytes).unwrap();
}
///
/// loads the game stored in the ActionKV
fn load_game(store: &mut ActionKV) -> Result<TicTacToeGame, &'static str>{
  let index_as_bytes = store.get(&INDEX_KEY)
                        .unwrap()
                        .unwrap();
  let index_decoded = bincode::deserialize(&index_as_bytes);
  let index: HashMap<ByteString, u64> = index_decoded.unwrap();
  println!("index: {:?}", index);
  let mut game: TicTacToeGame = vec![];
  for i in 0..9 {
    let pos = i + 1;
    match index.get(pos.to_string().as_bytes()) {
      None => eprintln!("{:?} not found", i + 1),
      Some(&i) => {
        let kv = store.get_at(i).unwrap();
        let value = match str::from_utf8(kv.value.as_ref()) {
          Ok(v) => v,
          Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("value loaded: {:?}", &value);
        game.push(value.to_string());
      }
    }
  }
  Ok(game)
}
fn exit_process() {
  println!("exiting tic-tac-toe.");
  std::process::exit(0);
}
fn main() -> io::Result<()>{
  // Start
  // 
  println!("Welcome to Philip's Tic Tac Toe game (Implemented with Rust) :)");
  // init vars
  let mut store:ActionKV;
  let mut game: TicTacToeGame;
  let mut next_player_index: usize = 99;
  let mut new_game = false;
  let mut file_path;
  let mut full_path: String;
  // program loop
  loop {
    // main menu
    loop {
      // clear game. 
      game = vec![];
      prompt_user("new game (n), resume game (r) , quit (q): ");
      let mut input = get_input()?;
      match input.as_str() {
        "q" | "Q" => {
          exit_process();
        },
        "n" | "N" => {
          new_game = true;
        },
        "r" | "R" => {
          // list file names on file.
          let mut games:Vec<String> = vec![];
          let mut path;
          match read_dir(SAVE_DIRECTORY) {
            Ok(r) => {
              let mut iter = r.peekable();
              if iter.peek().is_none()  {
                println!("No saved games on file.");
                continue;
              } 
              for  maybe_fname in iter {
                path = maybe_fname.unwrap().path();
                let fname = path.file_name().unwrap().to_str().unwrap();
                games.push(fname.to_string());
              } 
            },
            Err(e) => eprint!("error: {}", e)
          };
          // load game from file
          let num_games = games.len();
          let mut f_index;
          loop {
            for (i, fname) in games.iter().enumerate() {
              println!("{}: {}", i + 1, fname);
            }
            if num_games > 1 {
              prompt_user(&format!("File(1 - {}): ", num_games.to_string().as_str()));
            } else {
              prompt_user("File(1): ");
            }
            let fname = get_input();
            f_index = match fname {
              Ok(_i) => {
                match _i.parse::<usize>() {
                  Ok(i) => i,
                  Err(e) => {
                    println!("Invalid input: {}", e);
                    continue;
                  }
                }
              },
              Err(e) => {
                eprintln!("Error: {}", e);
                continue;
              }
            };
            if f_index < 1 || f_index > num_games {
              println!("Invalid file choice: {}", f_index);
              continue;
            }
            break;
          }
          full_path = format!("{}{}", SAVE_DIRECTORY, &games[f_index - 1]);
          file_path = std::path::Path::new(&full_path);
          if file_path.exists() {
            // load game..
            store = ActionKV::open(file_path).expect("unable to open file.");
            store.load().expect("Unable to load data");
            game = load_game(&mut store).unwrap();
           // get the next player
            next_player_index = match store.index.get(NEXT_PLAYER_INDEX_KEY) {
              None => panic!("{:?} not found", "next_player_index".to_string()),
              Some(&i) => {
                let kv = store.get_at(i).unwrap();
                let value = match str::from_utf8(kv.value.as_ref()) {
                  Ok(v) => v,
                  Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                value.parse::<usize>().unwrap()
              }
            };
            println!("game loaded: {:?}", game);
            println!("next player index: {:?}", next_player_index);
          }
        },
        _ => {
          println!("Invalid menu option {}", input);
          continue;
        }
      }
      // 
      if new_game {
        // initialize new game variables
        game = vec!["".to_string(); 9];
      }
     let game_result = run_game(&mut game, next_player_index, new_game);
      match game_result {
        Ok(result) => {
          if let GameAction::Quit = result.action {
            exit_process();
          } else if let GameAction::Save = result.action {
            // implement writing game to file
            // get a file name from user
            let mut path;
            let mut path_string;
            loop {
              prompt_user("File Name: ");
              let fname = get_input();
              match fname {
                Ok(f) => {
                  // try to open the filename
                  path_string = format!("{}/{}", SAVE_DIRECTORY, &f);
                  path = std::path::Path::new(&path_string);
                  if path.exists() {
                    // existing game. Can just update the existing value.
                    println!("{} exists. Do you wish to over write?", &f);
                    prompt_user("y/n: ");
                    input = get_input().unwrap();
                    match input.as_str() {
                      "y" => {
                        // open this lol
                        store = ActionKV::open(path).expect("unable to open file");
                        store.load().expect("unable to load data.");
                        // update the key value store.
                        for (i,mve) in game.iter().enumerate() {
                          let key = i + 1;
                          store.update(key.to_string().as_ref(), mve.as_ref()).unwrap();
                        }
                        // save the next_player_index
                        store.update(NEXT_PLAYER_INDEX_KEY, result.next_player_index.to_string().as_bytes()).unwrap();
                        store_index_on_disk(&mut store, INDEX_KEY);
                        break;
                      },
                      "n" => {
                        continue;
                      },
                      _ => println!("Invalid option: {}", input)
                    }
                  } else {
                    // new save going on here.
                    store = ActionKV::open_create(path).expect("unable to open file");
                    store.load().expect("unable to load data.");
                    for (i, mve) in game.iter().enumerate() {
                      let key = i + 1;
                      store.insert(key.to_string().as_ref(), mve.as_ref()).unwrap();
                    }
                    store.insert(NEXT_PLAYER_INDEX_KEY, result.next_player_index.to_string().as_bytes()).unwrap();
                    store_index_on_disk(&mut store, INDEX_KEY);
                    break;
                  }
                },
                _ => eprintln!("File save not found. {}", input),
              }
            }
          } else if let GameAction::Winner = result.action {
            println!("Player {} wins!", result.winner_index);
            break;
          }
        },
        Err(e) => eprintln!("Error: {}", e),
      }
    }
  }
}

