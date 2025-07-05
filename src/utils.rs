use chess::{ChessMove, Piece};

pub fn to_binary_string(value: usize, length: usize) -> String {
    format("{:0length$b}", value, length = length)
}

// This PGN parsing isn't really good
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
