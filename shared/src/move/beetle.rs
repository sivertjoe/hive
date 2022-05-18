use super::*;

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

        let fit = |from, to| if board.empty_square(&to) { can_fit(from, to, board) } else { true };

        neighbors(&sq)
            .into_iter()
            .filter(|to| have_neighbor(to) && fit(sq, *to))
            .collect()
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

    #[test]
    fn test_beetle_cant_fit()
    {
        let mut board = Board::default();
        let beetle_square = (1, -4, 3);
        let beetle = Piece::new(BoardPiece::Beetle, Color::White);

        let enemy_square = [(1, -3, 2), (2, -5, 3)];

        for e in enemy_square
        {
            board.insert(e, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black)));
        }
        board.insert(beetle_square, BoardSquare::new(beetle));

        board.turns = 3; // To avoid queen check

        let legal_moves = legal_moves(&beetle, &board, Some(beetle_square));


        assert_eq!(legal_moves, Vec::new());
    }

    #[test]
    fn test_beetle_all_6_moves()
    {
        let mut board = Board::default();

        let beetle_square = (2, 3, -5);
        let beetle = Piece::new(BoardPiece::Beetle, Color::White);

        let pos =
            [(3, 1, -4), (2, 2, -4), (1, 3, -4), (1, 4, -5), (2, 4, -6), (3, 3, -6), (4, 2, -6)];


        let iter = pos
            .into_iter()
            .map(|pos| (pos, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))))
            .chain(std::iter::once((beetle_square, BoardSquare::new(beetle.clone()))));

        board.from_iter(iter);
        board.turns = 10;

        let legal_moves = beetle_move(&board, beetle_square);

        assert_eq!(legal_moves.len(), 6);
    }
}
