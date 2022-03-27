use crate::model::*;

pub fn grasshopper_move(board: &Board, sq: Square) -> Vec<Square>
{
    const CUBE_DIR_VEC: [(isize, isize, isize); 6] =
        [(1, 0, -1), (1, -1, 0), (0, -1, 1), (-1, 0, 1), (-1, 1, 0), (0, 1, -1)];

    let sq_add = |a: Square, b: Square| (a.0 + b.0, a.1 + b.1, a.2 + b.2);


    let explore_dir = |add: Square| {
        let mut start = sq_add(sq, add);
        let st = start;

        while board.board.contains_key(&start)
        {
            start = sq_add(start, add);
        }
        (st != start).then(|| start)
    };



    CUBE_DIR_VEC.into_iter().filter_map(explore_dir).collect()
}

#[cfg(test)]
mod test
{
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_grasshopper_simple()
    {
        let mut board = Board::default();

        let grasshopper_square = (1, 0, -1);
        let grasshopper = Piece::new(BoardPiece::Grasshopper, Color::White);

        let pos = [
            ((0, 0, 0), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            (grasshopper_square, BoardSquare::new(grasshopper.clone())),
        ];


        board.board = HashMap::from_iter(pos.into_iter());
        board.turns = 3;

        let mut legal_moves = grasshopper_move(&board, grasshopper_square);
        let mut ans = vec![(-1, 0, 1)];

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }

    #[test]
    fn test_grasshopper_two_squares()
    {
        let mut board = Board::default();

        let grasshopper_square = (1, 0, -1);
        let grasshopper = Piece::new(BoardPiece::Grasshopper, Color::White);

        let pos = [
            ((0, 0, 0), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            ((-1, 0, 1), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            (grasshopper_square, BoardSquare::new(grasshopper.clone())),
        ];


        board.board = HashMap::from_iter(pos.into_iter());
        board.turns = board.board.len();

        let mut legal_moves = grasshopper_move(&board, grasshopper_square);
        let mut ans = vec![(-2, 0, 2)];

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }

    #[test]
    fn test_grasshopper_surround()
    {
        let mut board = Board::default();

        let grasshopper_square = (0, 0, 0);
        let grasshopper = Piece::new(BoardPiece::Grasshopper, Color::White);

        let mut pos = vec![(grasshopper_square, BoardSquare::new(grasshopper.clone()))];
        for sq in crate::r#move::neighbors(&grasshopper_square)
        {
            pos.push((sq, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))));
        }


        board.board = HashMap::from_iter(pos.into_iter());
        board.turns = 8; // To avoid queen check

        let mut legal_moves = grasshopper_move(&board, grasshopper_square);
        let mut ans = vec![(-2, 0, 2), (0, -2, 2), (2, -2, 0), (2, 0, -2), (0, 2, -2), (-2, 2, 0)];

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }
}
