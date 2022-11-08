use chess::*;
use std::str::FromStr;
use std::time;
mod engine;

fn main() {
    // let now = time::Instant::now();
    // unsafe {
    //     engine::test(
    //         "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
    //         4,
    //         true,
    //     );
    //     let elipsed_time = now.elapsed();
    //     println!("{} seconds!", elipsed_time.as_secs());
    // }
    let board = Board::from_str("rnbqkbnr/pp2pppp/2p5/1B1p4/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");
    let game = Game::new_with_board(board);
    for i in MoveGen::new_legal(&game.current_position()) {
        println!("{}", i);
    }
}
