use crate::{model::*, r#move::neighbors};

// Instead of doing _this_ ant scheme, what about doing the spiderwalk?
// and checking if I can fit? How do I know when Im done though?
pub fn ant_move(board: &Board, sq: Square) -> Vec<Square>
{
    board
        .board
        .iter()
        .filter_map(|(s, _p)| {
            // skip myself
            if *s == sq
            {
                None
            }
            else
            {
                Some(neighbors(s).into_iter().filter(|s| !board.board.contains_key(s)))
            }
        })
        .flatten()
        .collect()
}


#[cfg(test)]
mod test
{
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_ant_cant_fit()
    {
        let mut board = Board::default();
        let ant_square = (2, -1, -1);
        let ant = Piece::new(BoardPiece::Ant, Color::White);

        let enemy_square =
            [(2, -2, 0), (1, -2, 1), (1, -3, 2), (2, -4, 2), (3, -4, 1), (4, -4, 0), (4, -3, -1)];

        for e in enemy_square
        {
            board
                .board
                .insert(e, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black)));
        }
        board.board.insert(ant_square, BoardSquare::new(ant));

        board.turns = 8; // To avoid queen check

        let mut legal_moves = ant_move(&board, ant_square);

        let mut ans = vec![
            (1, -1, 0),
            (0, -1, 1),
            (0, -2, 2),
            (0, -3, 3),
            (1, -4, 3),
            (2, -5, 3),
            (3, -5, 2),
            (4, -5, 1),
            (5, -5, 0),
            (5, -4, -1),
            (5, -3, -2),
            (4, -2, -2),
            (3, -2, -1),
        ];

        legal_moves.sort();
        ans.sort();

        assert_eq!(legal_moves, ans);
    }
}
