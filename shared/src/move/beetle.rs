use crate::{model::*, r#move::neighbors};

pub fn beetle_move(board: &Board, sq: Square) -> Vec<Square>
{
    // TODO: Improve this, look at square_has_neighbors
    let def = || {
        let have_neighbor = |square: &Square| {
            sq != *square
                && (board.contains_key(square)
                    || neighbors(square)
                        .into_iter()
                        .any(|_sq| _sq != sq && !board.empty_square(&_sq)))
        };

        neighbors(&sq).into_iter().filter(have_neighbor).collect()
    };

    match board.get(&sq)
    {
        Some(bs) if bs.pieces.len() > 1 => neighbors(&sq).into_iter().collect(),
        _ => def(),
    }
}
#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_beetle_correct_moves_simple()
    {
        let mut board = Board::default();

        let beetle_square = (1, 0, -1);
        let beetle = Piece::new(BoardPiece::Beetle, Color::White);

        let pos = [
            ((0, 0, 0), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            (beetle_square, BoardSquare::new(beetle.clone())),
        ];


        board.from_iter(pos.into_iter());
        board.turns = board.len();

        let mut legal_moves = beetle_move(&board, beetle_square);
        let mut ans = vec![(1, -1, 0), (0, 0, 0), (0, 1, -1)];

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }

    #[test]
    fn test_beetle_correct_moves_on_top()
    {
        let mut board = Board::default();

        let beetle_square = (0, 0, 0);
        let beetle = Piece::new(BoardPiece::Beetle, Color::White);

        board.place_piece(Piece::new(BoardPiece::Ant, Color::Black), beetle_square, None);
        board.place_piece(beetle, beetle_square, None);


        board.turns = 3;

        let mut legal_moves = beetle_move(&board, beetle_square);
        let mut ans = neighbors(&beetle_square);


        ans.sort();
        legal_moves.sort();


        assert_eq!(legal_moves, ans);
        assert_eq!(legal_moves.len(), ans.len());
    }

    #[test]
    fn test_beetle_correct_moves_surrounded()
    {
        let mut board = Board::default();

        let beetle_square = (0, 0, 0);
        let beetle = Piece::new(BoardPiece::Beetle, Color::White);

        let mut vec = vec![(beetle_square, BoardSquare::new(beetle.clone()))];
        for sq in neighbors(&beetle_square)
        {
            vec.push((sq, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))));
        }


        board.from_iter(vec.into_iter());
        board.turns = 8; // To avoid queen check

        let mut legal_moves = beetle_move(&board, beetle_square);
        let mut ans = neighbors(&beetle_square);

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }
}
