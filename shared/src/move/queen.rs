use super::*;


pub fn queen_move(board: &Board, sq: Square) -> Vec<Square>
{
    let is_vakant = |sq: Square| board.empty_square(&sq);

    let legal_square = |square: &Square| {
        is_vakant(*square)
            && square_has_neighbors(*square, board, sq)
            && can_fit(sq, *square, board)
    };

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

    #[test]
    fn test_queen_move_can_fit()
    {
        let mut board = Board::default();
        let queen_square = (1, -4, 3);
        let queen = Piece::new(BoardPiece::Queen, Color::White);

        let enemy_square = [(1, -3, 2), (2, -5, 3)];

        for e in enemy_square
        {
            board.insert(e, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black)));
        }
        board.insert(queen_square, BoardSquare::new(queen));

        board.turns = 3; // To avoid queen check

        let legal_moves = legal_moves(&queen, &board, Some(queen_square));

        assert_eq!(legal_moves, Vec::new());
    }
}
