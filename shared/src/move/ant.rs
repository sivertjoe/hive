use super::*;
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

        an.into_iter().any(|a| bn.contains(&a) && !board.empty_square(&a))
    };

    let org = sq;
    let mut current = sq;

    let mut res = Vec::with_capacity(board.len() * 2);

    loop
    {
        let mut next = None;
        for &dir in &LEFT
        {
            let sq = sq_add(current, dir);

            let not_prev_pos = !res.contains(&sq);
            let empty_square = board.empty_square(&sq);

            // we want to make the piece to 'hug the left wall'. These
            // functions (hopefully) avoids jumping between paralell structures
            let follows_path =
                square_has_neighbors(sq, board, org) && common_neighbors(current, sq);

            if not_prev_pos
                && empty_square
                && follows_path
                && can_fit(current, sq, board)
                && !create_island(board, org, sq)
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

        let legal_moves = ant_move(&board, ant_square);


        assert_eq!(legal_moves.len(), 0);
    }
}
