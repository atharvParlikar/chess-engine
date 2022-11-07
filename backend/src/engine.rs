use chess::*;
use std::collections::HashMap;
use std::str::FromStr;

pub fn evaluate(fen: String) -> i32 {
    let mut eval: i32 = 0;
    let fen_pure = fen.split(" ").next().unwrap().to_string();
    let values = HashMap::from([
        ('p', 1),
        ('q', 9),
        ('r', 5),
        ('b', 3),
        ('n', 3),
        ('k', 1000),
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

pub fn minimax(
    position: String,
    depth: u8,
    maximisingPlayer: bool,
    mut alpha: i32,
    mut beta: i32,
) -> i32 {
    if depth == 0 {
        return evaluate(position);
    } else {
        if maximisingPlayer {
            let mut value = -100000;
            let moves = MoveGen::new_legal(&Board::from_str(position.as_str()).unwrap());
            for mov in moves {
                let game = Game::from_str(position.as_str());
                match game {
                    Ok(mut g) => {
                        g.make_move(mov);
                        let evaluation = minimax(
                            g.current_position().to_string(),
                            depth - 1,
                            false,
                            alpha,
                            beta,
                        );
                        if evaluation > value {
                            value = evaluation;
                        }
                    }
                    Err(e) => println!(""),
                }
                if alpha > value {
                    alpha = value;
                }
                if alpha >= beta {
                    return value;
                }
            }
            return value;
        } else {
            let mut value = 100000;
            let moves = MoveGen::new_legal(&Board::from_str(position.as_str()).unwrap());
            for mov in moves {
                let game = Game::from_str(position.as_str());
                match game {
                    Ok(mut g) => {
                        g.make_move(mov);
                        let evaluation = minimax(
                            g.current_position().to_string(),
                            depth - 1,
                            true,
                            alpha,
                            beta,
                        );
                        if evaluation < value {
                            value = evaluation;
                        }
                    }
                    Err(e) => println!(""),
                };
                if beta <= value {
                    beta = value;
                }
                if alpha >= beta {
                    return value;
                }
            }
            return value;
        }
    }
}

pub fn minimax_genesis(position: String, depth: u8, maximisingPlayer: bool) -> String {
    let mut board = Board::from_str(position.as_str()).unwrap();
    let mut moves = MoveGen::new_legal(&board);
    let mut best_move: ChessMove = moves.next().unwrap();
    let mut value = -100000;
    for mov in moves {
        let mut game = Game::new_with_board(board);
        game.make_move(mov);
        let evaluation = minimax(
            game.current_position().to_string(),
            depth,
            false,
            -100000,
            100000,
        );
        if evaluation > value {
            value = evaluation;
            best_move = mov;
        }
    }
    return best_move.to_string();
}
