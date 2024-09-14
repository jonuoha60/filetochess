use chess::{Action, ChessMove, Game, MoveGen};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

use crate::utils::{to_binary_string, to_pgn};

pub fn decode(pgns: &Vec<String>, output_file_path: &str) {
    let mut collected_bits = String::new();
    let mut file_bits = Vec::new();

    for pgn in pgns {
        let mut chess_board = Game::new(); // Reinitialize the board for each PGN
        let actions = parse_pgn(&pgn);

        for action in actions {
            if let Action::MakeMove(chess_move) = action {
                let move_str = to_pgn(&chess_move);
                
                let legal_moves: Vec<ChessMove> = MoveGen::new_legal(&chess_board.current_position()).collect();
                let move_count = legal_moves.len();
                let max_binary_length = (move_count as f64).log2().floor() as usize;
                let mut move_bits = HashMap::new();
                
                for (index, legal_move) in legal_moves.iter().enumerate() {
                    let move_binary = to_binary_string(index, max_binary_length);
                    move_bits.insert(legal_move.to_string(), move_binary);
                }
                
                if let Some(move_binary) = move_bits.get(&move_str) {
                    collected_bits.push_str(move_binary);
                }

                chess_board.make_move(chess_move);
            }
        }
    }

    let mut bit_index = 0;
    while bit_index + 8 <= collected_bits.len() {
        let byte_bits = &collected_bits[bit_index..bit_index + 8];
        let byte = u8::from_str_radix(byte_bits, 2).expect("Invalid bit string");
        file_bits.push(byte);
        bit_index += 8;
    }

    let mut output_file = File::create(output_file_path).expect("Unable to create file");
    output_file.write_all(&file_bits).expect("Unable to write data to file");
}

fn parse_pgn(pgn: &str) -> Vec<Action> {
    let mut actions = Vec::new();
    let moves: Vec<&str> = pgn.split_whitespace().collect();

    for mv in moves {
        if let Ok(chess_move) = ChessMove::from_str(mv) {
            actions.push(Action::MakeMove(chess_move));
        }
    }
    actions
}
