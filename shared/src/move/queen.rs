use crate::{
    model::*,
    r#move::{neighbors, square_has_neighbors},
};

pub fn queen_move(board: &Board, sq: Square) -> Vec<Square>
{
    let is_vakant = |sq: Square| !board.contains_key(&sq);

    let legal_square =
        |square: &Square| is_vakant(*square) && square_has_neighbors(*square, board, sq);

    neighbors(&sq).into_iter().filter(legal_square).collect()
}


#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_queen_move_simple()
    {
        let mut board = Board::default();

        let queen_square = (1, 0, -1);
        let queen = Piece::new(BoardPiece::Queen, Color::White);

        let pos = [
            ((0, 0, 0), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            (queen_square, BoardSquare::new(queen.clone())),
        ];


        board.from_iter(pos.into_iter());
        board.turns = 3;

        let mut legal_moves = queen_move(&board, queen_square);
        let mut ans = vec![(1, -1, 0), (0, 1, -1)];

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }
}
