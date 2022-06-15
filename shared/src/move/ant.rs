use super::*;

pub fn ant_move(board: &Board, sq: Square) -> Vec<Square>
{
    let mut vec = Vec::new();

    const RIGHT: [Square; 6] =
        [(0, -1, 1), (1, -1, 0), (1, 0, -1), (0, 1, -1), (-1, 1, 0), (-1, 0, 1)];

    const LEFT: [Square; 6] =
        [(0, 1, -1), (1, 0, -1), (1, -1, 0), (0, -1, 1), (-1, 0, 1), (-1, 1, 0)];

    _ant_move(board, sq, RIGHT, &mut vec);
    _ant_move(board, sq, LEFT, &mut vec);
    vec
}

pub fn _ant_move(board: &Board, sq: Square, dirs: [Square; 6], res: &mut Vec<Square>)
{
    let sq_add = |a: Square, b: Square| (a.0 + b.0, a.1 + b.1, a.2 + b.2);
    let common_neighbors = |a: Square, b: Square| {
        let an = neighbors(&a);
        let bn = neighbors(&b);

        an.into_iter().any(|a| bn.contains(&a) && !board.empty_square(&a))
    };

    let org = sq;
    let mut current = sq;

    loop
    {
        let mut next = None;
        for &dir in &dirs
        {
            let sq = sq_add(current, dir);

            let not_prev_pos = !res.contains(&sq);
            let empty_square = board.empty_square(&sq);

            // we want to make the piece to 'hug the left wall'. These
            // functions (hopefully) avoids jumping between paralell structures
            let follows_path =
                square_has_neighbors(sq, board, org) && common_neighbors(current, sq);

            if not_prev_pos && empty_square && follows_path && can_fit(current, sq, board)
            {
                next = Some(sq);
                current = sq;
                break;
            }
        }

        match next
        {
            Some(sq) =>
            {
                if sq == org
                {
                    return;
                }
                else
                {
                    res.push(sq);
                }
            },
            None =>
            {
                return;
            },
        }
    }
}


#[cfg(test)]
mod test
{
    use super::*;


    #[test]
    fn test_ant_simple()
    {
        let mut board = Board::default();
        let ant_square = (1, 0, -1);
        let ant = Piece::new(BoardPiece::Ant, Color::White);

        let enemy_square = [(0, -1, 1), (0, 0, 0)];

        for e in enemy_square
        {
            board.insert(e, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black)));
        }
        board.insert(ant_square, BoardSquare::new(ant));

        board.turns = 3; // To avoid queen check

        let mut legal_moves = ant_move(&board, ant_square);

        let mut ans = vec![
            (1, -1, 0),
            (1, -2, 1),
            (0, -2, 2),
            (-1, -1, 2),
            (-1, 0, 1),
            (-1, 1, 0),
            (0, 1, -1),
        ];

        legal_moves.sort();
        ans.sort();

        assert_eq!(legal_moves, ans);
    }

    #[test]
    fn test_can_fit()
    {
        let mut board = Board::default();
        let p = Piece::new(BoardPiece::Ant, Color::Black);

        board.insert((-4, 0, 4), BoardSquare::new(p));
        board.insert((-2, -1, 3), BoardSquare::new(p));
        assert!(!can_fit((-3, 0, 3), (-3, -1, 4), &board));

        board.insert((-2, 2, 0), BoardSquare::new(p));
        board.insert((-3, 4, -1), BoardSquare::new(p));
        assert!(!can_fit((-2, 3, -1), (-3, 3, 0), &board));

        board.insert((2, -4, 2), BoardSquare::new(p));
        board.insert((3, -3, 0), BoardSquare::new(p));
        assert!(!can_fit((2, -3, 1), (3, -4, 1), &board));
    }

    #[test]
    fn test_cant_move_around()
    {
        let mut board = Board::default();
        let ant_square = (1, -4, 3);
        let ant = Piece::new(BoardPiece::Ant, Color::White);

        let enemy_square = [(1, -3, 2), (2, -5, 3)];

        for e in enemy_square
        {
            board.insert(e, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black)));
        }
        board.insert(ant_square, BoardSquare::new(ant));

        board.turns = 3; // To avoid queen check

        use super::legal_moves;

        let legal_moves = legal_moves(&ant, &board, Some(ant_square));

        assert_eq!(legal_moves.len(), 0);
    }


    #[test]
    fn test_ant_move_can_only_move_once_err()
    {
        let mut board = Board::default();

        let ant_square = (0, 2, -2);
        let ant = Piece::new(BoardPiece::Ant, Color::White);

        let pos = [
            (0, -1, 1),
            (0, 0, 0),
            (0, 1, -1),
            (1, 1, -2),
            (1, 2, -3),
            (2, 2, -4),
            (3, 1, -4),
            (4, 2, -6),
            (4, 3, -7),
            (3, 3, -6),
            (2, 3, -5),
            (1, 4, -5),
            (1, 5, -6),
            (1, 6, -7),
            (0, 5, -5),
            (-1, 6, -5),
            (0, 4, -4),
            (-1, 4, -3),
        ];


        let iter = pos
            .into_iter()
            .map(|pos| (pos, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))))
            .chain(std::iter::once((ant_square, BoardSquare::new(ant.clone()))));

        board.from_iter(iter);
        board.turns = 10;

        let legal_moves = legal_moves(&ant, &board, Some(ant_square));

        assert_ne!(legal_moves.len(), 1);
    }
}
