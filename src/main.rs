#![allow(warnings, unused)]
use chess::*;
use std::collections::HashMap;
use std::str::FromStr;

fn evaluate(fen: String) -> i32 {
    let mut eval: i32 = 0;
    let fen_pure = fen.split(" ").next().unwrap().to_string();
    let values = HashMap::from([
        ('p', 1),
        ('q', 9),
        ('r', 5),
        ('b', 3),
        ('n', 3),
        ('k', 1000)
    ]);
    for i in fen_pure.chars() {
        if i.is_ascii_uppercase() {
            let lowercase = i.to_lowercase().next().unwrap();
            eval += values[&lowercase];
        } else if i.is_lowercase() {
            eval -= values[&i];
        }
    }
    return eval;
}

fn minimax(position: String, depth: u8, maximisingPlayer: bool) -> i32 {
    if depth == 0 {
        return evaluate(position);
    } else {
        if maximisingPlayer {
            let mut value = -10000;
            let moves = MoveGen::new_legal(&Board::from_str(position.as_str()).unwrap());
            for mov in moves {
                let game = Game::from_str(position.as_str());
                match game {
                    Ok(mut g) => {
                        g.make_move(mov);
                        let evaluation = minimax(g.current_position().to_string(), depth - 1, false);
                        if evaluation > value {
                            value = evaluation;
                        }
                    },
                    Err(e) => println!(""),
                }
            }
            return value;
        }
        else {
            let mut value = 10000;
            let moves = MoveGen::new_legal(&Board::from_str(position.as_str()).unwrap());
            for mov in moves {
                let game = Game::from_str(position.as_str());
                match game {
                    Ok(mut g) => {
                        g.make_move(mov);
                        let evaluation = minimax(g.current_position().to_string(), depth - 1, true);
                        if evaluation < value {
                            value = evaluation;
                        }
                    },
                    Err(e) => println!("")
                };
            }
            return value;
        }
    }
}

fn main() {
    let position = "rnbqkbnr/ppppp2p/5p2/6p1/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq g6 0 1".to_string();

    let board = Board::from_str(position.as_str()).unwrap();
    let mut moves = MoveGen::new_legal(&board);
    let mut best_move:ChessMove = moves.next().unwrap();
    let mut value = -100000;
    for mov in moves {
        let mut game = Game::new_with_board(board);
        game.make_move(mov);
        let evaluation = minimax(game.current_position().to_string(), 2, false);
        if evaluation > value {
            value = evaluation;
            best_move = mov;
        }
    }
    println!("evaluation: {}", value);
    println!("best move: {}", best_move);
    println!("best minimax eval: {}", minimax(position, 3, true));
}
