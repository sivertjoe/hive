use crate::{
    model::*,
    r#move::{neighbors, square_has_neighbors},
};

pub fn ant_move(board: &Board, sq: Square) -> Vec<Square>
{
    const LEFT: [Square; 6] =
        [(0, 1, -1), (1, 0, -1), (1, -1, 0), (0, -1, 1), (-1, 0, 1), (-1, 1, 0)];
    let sq_add = |a: Square, b: Square| (a.0 + b.0, a.1 + b.1, a.2 + b.2);
    let common_neighbors = |a: Square, b: Square| {
        let an = neighbors(&a);
        let bn = neighbors(&b);

        an.into_iter().any(|a| bn.contains(&a) && board.board.contains_key(&a))
    };

    let org = sq;
    let mut current = sq;

    let mut res = Vec::with_capacity(board.board.len() * 2);

    loop
    {
        let mut next = None;
        for &dir in &LEFT
        {
            let sq = sq_add(current, dir);

            let not_prev_pos = !res.contains(&sq);
            let empty_square = !board.board.contains_key(&sq);

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
                    return res;
                }
                else
                {
                    res.push(sq);
                }
            },
            None =>
            {
                return res;
            },
        }
    }
}

fn can_fit(current: Square, next: Square, board: &Board) -> bool
{
    fn cmp<F: Fn(isize, isize) -> isize>(a: Square, b: Square, n: usize, f: F) -> isize
    {
        match n
        {
            0 => f(a.0, b.0),
            1 => f(a.1, b.1),
            2 => f(a.2, b.2),
            _ => unreachable!(),
        }
    }
    let max_n = |n: usize| cmp(current, next, n, std::cmp::max);
    let min_n = |n: usize| cmp(current, next, n, std::cmp::min);
    let occupied = |a, b| board.board.contains_key(&a) && board.board.contains_key(&b);


    let (a, b) = match (current, next)
    {
        ((a, _, _), (b, _, _)) if a == b =>
        {
            let a = (a + 1, min_n(1), min_n(2));
            let b = (b - 1, max_n(1), max_n(2));
            (a, b)
        },
        ((_, a, _), (_, b, _)) if a == b =>
        {
            let a = (min_n(0), a + 1, min_n(2));
            let b = (max_n(0), b - 1, max_n(2));
            (a, b)
        },
        ((_, _, a), (_, _, b)) if a == b =>
        {
            let a = (min_n(0), min_n(1), a + 1);
            let b = (max_n(0), max_n(1), b - 1);
            (a, b)
        },
        _ => unreachable!(),
    };

    !occupied(a, b)
}


#[cfg(test)]
mod test
{
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


    #[test]
    fn test_ant_simple()
    {
        let mut board = Board::default();
        let ant_square = (1, 0, -1);
        let ant = Piece::new(BoardPiece::Ant, Color::White);

        let enemy_square = [(0, -1, 1), (0, 0, 0)];

        for e in enemy_square
        {
            board
                .board
                .insert(e, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black)));
        }
        board.board.insert(ant_square, BoardSquare::new(ant));

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

        board.board.insert((-4, 0, 4), BoardSquare::new(p));
        board.board.insert((-2, -1, 3), BoardSquare::new(p));
        assert!(!can_fit((-3, 0, 3), (-3, -1, 4), &board));

        board.board.insert((-2, 2, 0), BoardSquare::new(p));
        board.board.insert((-3, 4, -1), BoardSquare::new(p));
        assert!(!can_fit((-2, 3, -1), (-3, 3, 0), &board));

        board.board.insert((2, -4, 2), BoardSquare::new(p));
        board.board.insert((3, -3, 0), BoardSquare::new(p));
        assert!(!can_fit((2, -3, 1), (3, -4, 1), &board));
    }
}
