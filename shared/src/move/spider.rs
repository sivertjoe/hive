use super::can_fit;
use crate::{
    model::*,
    r#move::{neighbors, square_has_neighbors},
};

fn _spider_move(
    fin: &mut Vec<Square>,
    dirs: [Square; 6],
    board: &Board,
    sq: Square,
    level: u8,
    origin: Square,
    mut visit: Vec<Square>,
)
{
    let sq_add = |a: Square, b: Square| (a.0 + b.0, a.1 + b.1, a.2 + b.2);

    let common_neighbors = |a: Square, b: Square| {
        let an = neighbors(&a);
        let bn = neighbors(&b);

        an.into_iter().any(|a| bn.contains(&a) && !board.empty_square(&a))
    };

    for &dir in &dirs
    {
        let dt = sq_add(sq, dir);
        if !visit.contains(&dt)
            && board.empty_square(&dt)
            && square_has_neighbors(dt, board, origin)
            && common_neighbors(sq, dt)
            && can_fit(sq, dt, board)
        {
            if level == 2 && !fin.contains(&dt)
            {
                fin.push(dt);
            }
            else
            {
                visit.push(sq);
                _spider_move(fin, dirs, board, dt, level + 1, origin, visit.clone());
            }
        }
    }
}

pub fn spider_move(board: &Board, sq: Square) -> Vec<Square>
{
    const RIGHT: [Square; 6] =
        [(0, -1, 1), (1, -1, 0), (1, 0, -1), (0, 1, -1), (-1, 1, 0), (-1, 0, 1)];

    const LEFT: [Square; 6] =
        [(0, 1, -1), (1, 0, -1), (1, -1, 0), (0, -1, 1), (-1, 0, 1), (-1, 1, 0)];


    let mut board = board.clone();
    let _ = board.remove(sq);

    let mut fin = Vec::new();

    _spider_move(&mut fin, RIGHT, &board, sq, 0, sq, Vec::new());
    _spider_move(&mut fin, LEFT, &board, sq, 0, sq, Vec::new());

    fin
}
#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_spider_move_simple()
    {
        let mut board = Board::default();

        //let spider_square = (-1, 1, 0);
        let spider_square = (1, -1, 0);
        let spider = Piece::new(BoardPiece::Spider, Color::White);

        let pos = [
            ((0, 0, 0), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            (spider_square, BoardSquare::new(spider.clone())),
        ];


        board.from_iter(pos.into_iter());
        board.turns = 3;

        let mut legal_moves = spider_move(&board, spider_square);
        // let mut ans = vec![(1, -1, 0)];
        let mut ans = vec![(-1, 1, 0)];

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }

    #[test]
    fn test_spider_move_two_squares()
    {
        let mut board = Board::default();

        //let spider_square = (-1, 1, 0);
        let spider_square = (-1, 1, 0);
        let spider = Piece::new(BoardPiece::Spider, Color::White);

        let pos = [
            ((0, 0, 0), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            ((1, 0, -1), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            (spider_square, BoardSquare::new(spider.clone())),
        ];


        board.from_iter(pos.into_iter());
        board.turns = 3;

        let mut legal_moves = spider_move(&board, spider_square);
        // let mut ans = vec![(1, -1, 0)];
        let mut ans = vec![(1, -1, 0), (2, 0, -2)];

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }

    #[test]
    fn test_spider_move_split()
    {
        let mut board = Board::default();

        //let spider_square = (-1, 1, 0);
        let spider_square = (0, 1, -1);
        let spider = Piece::new(BoardPiece::Spider, Color::White);

        let pos = [
            ((1, 0, -1), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            ((1, 1, -2), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            ((-1, 0, 1), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            ((-2, 1, 1), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            (spider_square, BoardSquare::new(spider.clone())),
        ];


        board.from_iter(pos.into_iter());
        board.turns = 3;

        let mut legal_moves = spider_move(&board, spider_square);
        // let mut ans = vec![(1, -1, 0)];
        let mut ans = vec![(2, 1, -3), (2, -1, -1), (-1, -1, 2), (-2, 2, 0)];
        //let mut ans = vec![(2, -1, -1), (2, 1, -3)];

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }

    #[test]
    fn test_spider_move_cant_fit()
    {
        let mut board = Board::default();

        //let spider_square = (-1, 1, 0);
        let spider_square = (1, 2, -3);
        let spider = Piece::new(BoardPiece::Spider, Color::White);



        let pos = [
            (2, -1, -1),
            (1, 0, -1),
            (0, 1, -1),
            (0, 2, -2),
            (2, 1, -3),
            (3, 0, -3),
            (3, -2, -1),
            (4, -1, -3),
        ];


        let iter = pos
            .into_iter()
            .map(|pos| (pos, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))))
            .chain(std::iter::once((spider_square, BoardSquare::new(spider.clone()))));



        board.from_iter(iter);
        board.turns = 10;

        let legal_moves = spider_move(&board, spider_square);

        assert!(!legal_moves.contains(&(3, -1, -2)));
    }
}
