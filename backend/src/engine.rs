use chess::*;
use std::collections::HashMap;
use std::str::FromStr;

static mut COUNTER: i32 = 0;

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

pub fn minimax(position: String, depth: u8, maximising_player: bool) -> i32 {
    if depth == 0 {
        unsafe {
            COUNTER += 1;
        }
        return evaluate(position);
    } else {
        let board = Board::from_str(position.as_str()).unwrap();
        let moves = MoveGen::new_legal(&board);
        if maximising_player {
            let mut max_eval = -100000;
            for mov in moves {
                let mut game = Game::new_with_board(board);
                game.make_move(mov);
                let current_eval = minimax(game.current_position().to_string(), depth - 1, false);
                if current_eval > max_eval {
                    max_eval = current_eval;
                }
            }
            return max_eval;
        } else {
            let mut min_eval = 100000;
            for mov in moves {
                let mut game = Game::new_with_board(board);
                game.make_move(mov);
                let current_eval = minimax(game.current_position().to_string(), depth - 1, true);
                if current_eval < min_eval {
                    min_eval = current_eval;
                }
            }
            return min_eval;
        }
    }
}

pub fn test(position: String, depth: u8, maximising_player: bool) {
    println!(
        "best eval after {} moves: {}",
        depth,
        minimax(position, depth, maximising_player)
    );
    unsafe {
        println!("positions evaluated: {}", COUNTER);
    }
}

// pub unsafe fn minimax_genesis(position: String, depth: u8, maximising_player: bool) -> String {
//     let board = Board::from_str(position.as_str()).unwrap();
//     let mut moves = MoveGen::new_legal(&board);
//     let mut best_move: ChessMove = moves.next().unwrap();
//     let mut value = -100000;
//     for mov in moves {
//         let mut game = Game::new_with_board(board);
//         game.make_move(mov);
//         let evaluation = minimax(
//             game.current_position().to_string(),
//             depth,
//             maximising_player,
//             -100000,
//             100000,
//         );
//         if evaluation > value {
//             value = evaluation;
//             best_move = mov;
//         }
//     }
//     println!("number of positions evaluated: {}", COUNTER);
//     return best_move.to_string();
// }
