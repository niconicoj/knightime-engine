use std::io;

use knightime::communication::uci::*;
use knightime::{board::Board, constants::SQUARE_NAME};

extern crate knightime;

fn main() {
    // start engine
    println!("id name knightime");
    println!("id name niconicoj");
    let mut board = Board::default();
    println!("uciok");
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let commands: Vec<&str> = buffer.split_whitespace().collect();

        match commands.get(0) {
            Some(&"isready") => {
                println!("readyok");
            }
            Some(&"ucinewgame") => {
                board = parse_uci_position("position startpos").unwrap();
            }
            Some(&"position") => {
                board = parse_uci_position(&buffer).unwrap();
            }
            Some(&"go") => {
                match parse_uci_go(&buffer, &board) {
                    Ok(mv) => {
                        let promotion = match mv.get_promotion() {
                            Some(knightime::defs::Promotion::Queen) => "q",
                            Some(knightime::defs::Promotion::Rook) => "r",
                            Some(knightime::defs::Promotion::Knight) => "k",
                            Some(knightime::defs::Promotion::Bishop) => "b",
                            None => "",
                        };
                        println!(
                            "bestmove {}{}{}",
                            SQUARE_NAME[mv.get_source_square() as usize],
                            SQUARE_NAME[mv.get_target_square() as usize],
                            promotion
                        );
                        board.make_move(mv, false).unwrap();
                    }
                    Err(_) => panic!("error while parsing move"),
                };
            }
            Some(&"quit") => {
                break;
            }
            _ => continue,
        };
    }
}
