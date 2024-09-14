use chess::{Action, ChessMove, Game, GameResult, MoveGen, Piece};
use std::str::FromStr;
use std::{collections::HashMap, fs::File, io::Read, time::Instant};

pub fn to_binary_string(value: usize, length: usize) -> String {
    format!("{:0length$b}", value, length = length)
}

pub fn encode(file_path: &str) -> Vec<String> {
    let start_time = Instant::now();

    let mut file = File::open(file_path).expect("Unable to open file");
    let mut file_bytes = Vec::new();
    file.read_to_end(&mut file_bytes)
        .expect("Unable to read file");

    let file_bits_count = file_bytes.len() * 8; // Total bits in file

    let mut file_bit_index = 0;
    let mut output_pgns = Vec::new();
    let mut chess_board = Game::new();

    while file_bit_index < file_bits_count {
        let legal_moves: Vec<ChessMove> =
            MoveGen::new_legal(&chess_board.current_position()).collect();
        let move_count = legal_moves.len();

        let max_binary_length = (move_count as f64).log2().floor() as usize;

        let mut move_bits = HashMap::new();
        for (index, legal_move) in legal_moves.iter().enumerate() {
            let move_binary = to_binary_string(index, max_binary_length);
            move_bits.insert(legal_move.to_string(), move_binary);
        }

        let available_bits = file_bits_count - file_bit_index;
        let chunk_size = std::cmp::min(available_bits, 16);

        let mut file_chunk = String::new();
        for byte in &file_bytes[(file_bit_index / 8)..((file_bit_index + chunk_size) / 8)] {
            file_chunk.push_str(&to_binary_string(*byte as usize, 8));
        }

        let bit_offset = file_bit_index % 8;
        let bits_remaining_in_chunk = file_chunk.len() - bit_offset;
        let effective_max_length = std::cmp::min(bits_remaining_in_chunk, max_binary_length);

        let next_file_chunk = &file_chunk[bit_offset..bit_offset + effective_max_length];

        let mut found_move = false;
        for (move_uci, move_binary) in &move_bits {
            if move_binary == next_file_chunk {
                chess_board.make_move(ChessMove::from_str(&move_uci).unwrap());
                found_move = true;
                break;
            }
        }

        if !found_move {
            let game_actions = chess_board
                .actions()
                .iter()
                .filter(|act| matches!(act, Action::MakeMove(_)))
                .collect::<Vec<_>>();

            output_pgns.push(generate_pgn(game_actions));

            chess_board = Game::new();
            file_bit_index += effective_max_length;
            continue;
        }

        file_bit_index += effective_max_length;

        let result = chess_board.result();
        if result == Some(GameResult::Stalemate)
            || result == Some(GameResult::WhiteCheckmates)
            || result == Some(GameResult::BlackCheckmates)
            || file_bit_index >= file_bits_count
        {
            let game_actions = chess_board
                .actions()
                .iter()
                .filter(|act| matches!(act, Action::MakeMove(_)))
                .collect::<Vec<_>>();

            output_pgns.push(generate_pgn(game_actions));

            chess_board = Game::new();
        }
    }

    println!(
        "Encoded file in {:.3} seconds.",
        start_time.elapsed().as_secs_f64()
    );

    output_pgns
}

pub fn to_pgn(mve: &ChessMove) -> String {
    let mut move_str = format!("{}{}", mve.get_source(), mve.get_dest());

    if let Some(piece) = &mve.get_promotion() {
        let promo_str = match piece {
            Piece::Knight => "n",
            Piece::Bishop => "b",
            Piece::Rook => "r",
            Piece::Queen => "q",
            _ => "",
        };
        move_str.push_str(promo_str);
    }

    move_str
}

fn generate_pgn(actions: Vec<&Action>) -> String {
    let mut pgn = String::new();
    let mut move_number = 1;

    for (i, action) in actions.iter().enumerate() {
        if let Action::MakeMove(chess_move) = action {
            if i % 2 == 0 {
                if !pgn.is_empty() {
                    pgn.push_str(" ");
                }
                pgn.push_str(&format!("{}. ", move_number));
                move_number += 1;
            } else {
                pgn.push_str(" ");
            }
            pgn.push_str(&to_pgn(chess_move));
        }
    }

    pgn
}
