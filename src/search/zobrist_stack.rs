use crate::move_generation::board_rep::Board;

use super::zobrist::ZobristHash;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ZobristStack {
    zobrist_vec: Vec<ZobristHash>,
}

impl ZobristStack {
    pub fn new(board: &Board) -> Self {
        Self {
            zobrist_vec: vec![ZobristHash::complete(board)],
        }
    }

    // pub fn add_hash(&mut self, hash_base: ZobristHash) {
    //     let new_hash = self.current_hash().combine(hash_base);
    //     self.zobrist_vec.push(new_hash);
    // }

    pub fn push(&mut self, new: ZobristHash) {
        self.zobrist_vec.push(new);
    }

    pub fn pop(&mut self) {
        self.zobrist_vec.pop();
    }

    pub fn current_hash(&self) -> ZobristHash {
        let len = self.zobrist_vec.len();
        self.zobrist_vec[len - 1]
    }

    pub fn twofold_repetition(&self, halfmoves: u16) -> bool {
        if self.zobrist_vec.len() < 4 {
            return false;
        }

        let current_hash = self.current_hash();
        for &hash in self
            .zobrist_vec
            .iter()
            .rev()
            .take((halfmoves + 1) as usize)
            .skip(2)
            .step_by(2)
        {
            if hash == current_hash {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::move_generation::{
        board_rep::{Square, START_FEN},
        chess_move::{Flag, Move},
    };

    #[test]
    fn twofold_repetition_works() {
        use super::*;

        let mut board = Board::from_fen(START_FEN);
        let mut zobrist_stack = ZobristStack::new(&board);

        let w_knight_out = Move::new(Square::F3, Square::G1, Flag::NONE);
        let b_knight_out = Move::new(Square::F6, Square::G8, Flag::NONE);
        let w_knight_back = Move::new(Square::G1, Square::F3, Flag::NONE);
        let b_knight_back = Move::new(Square::G8, Square::F6, Flag::NONE);

        board.try_play_move(w_knight_out, &mut zobrist_stack);
        board.try_play_move(b_knight_out, &mut zobrist_stack);
        board.try_play_move(w_knight_back, &mut zobrist_stack);

        assert!(!zobrist_stack.twofold_repetition(board.halfmoves));

        board.try_play_move(b_knight_back, &mut zobrist_stack);

        assert!(zobrist_stack.twofold_repetition(board.halfmoves));
    }
}
