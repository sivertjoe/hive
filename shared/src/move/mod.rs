use crate::model::*;

mod ant;
use ant::*;

mod beetle;
use beetle::*;

mod spider;
use spider::*;

mod queen;
use queen::*;

mod grasshopper;
use grasshopper::*;

pub fn legal_moves(p: &Piece, board: &Board, board_pos: Option<Square>) -> Vec<Square>
{
    if board.is_complete()
    {
        return Vec::new();
    }

    match board.turns
    {
        // These first we _know_ and can be hardcoded
        0 => vec![(0, 0, 0)],
        1 => vec![(1, -1, 0), (-1, 1, 0), (1, 0, -1), (-1, 0, 1), (0, 1, -1), (0, -1, 1)],

        _ =>
        {
            let idx = p.color as usize;

            let queen_turn = board.turns == 6 || board.turns == 7;
            let no_queen_placed = board.queens[idx].is_none();
            let piece_not_queen = p.r#type != BoardPiece::Queen;

            // A queen _has_ to be placed in the first four move of each player
            if queen_turn && no_queen_placed && piece_not_queen
            {
                return Vec::new();
            }

            match board_pos
            {
                Some(pos) => legal_on_board_move(p, board, pos),
                None => legal_new_piece_moves(p, board),
            }
        },
    }
}

pub fn square_has_neighbors(sq: Square, board: &Board, me: Square) -> bool
{
    neighbors(&sq).into_iter().filter(|s| *s != me).any(|s| board.contains_key(&s))
}

fn legal_on_board_move(p: &Piece, board: &Board, sq: Square) -> Vec<Square>
{
    let vec = match p.r#type
    {
        BoardPiece::Ant => ant_move(board, sq),
        BoardPiece::Beetle => beetle_move(board, sq),
        BoardPiece::Grasshopper => grasshopper_move(board, sq),
        BoardPiece::Queen => queen_move(board, sq),
        BoardPiece::Spider => spider_move(board, sq),
    };
    create_island_multiple(board, sq, vec)
}

// Hmm, t-this can be simplified r-right?
fn legal_new_piece_moves(piece: &Piece, board: &Board) -> Vec<Square>
{
    // Good neighbors have only same color neighbors or none
    let good_neighbors = |sq: &Square| {
        neighbors(sq).into_iter().all(|sq| match board.get(&sq)
        {
            None => true,
            Some(s) => s.top().color == piece.color,
        })
    };

    let not_touching_other_color =
        //|sq: Square| board.board.get(&sq).map_or(true, |s| s.piece.color == piece.color);
        |sq: Square| board.get(&sq).is_none() && good_neighbors(&sq);

    board
        .iter()
        .filter_map(|(sq, bp)| {
            (bp.top().color == piece.color).then(|| {
                neighbors(sq)
                    .into_iter()
                    .filter_map(|sq| not_touching_other_color(sq).then(|| sq))
            })
        })
        .flatten()
        .collect()
}

pub fn neighbors(sq: &Square) -> [Square; 6]
{
    const CUBE_DIR_VEC: [(isize, isize, isize); 6] =
        [(1, 0, -1), (1, -1, 0), (0, -1, 1), (-1, 0, 1), (-1, 1, 0), (0, 1, -1)];

    let mut iter = CUBE_DIR_VEC.into_iter().map(|d| (sq.0 + d.0, sq.1 + d.1, sq.2 + d.2));

    [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

fn create_island_multiple(board: &Board, from: Square, mut vec: Vec<Square>) -> Vec<Square>
{
    let mut global = Vec::with_capacity(board.len());
    let mut local = Vec::with_capacity(board.len());

    vec.retain(|to| !create_island(board, from, *to, &mut global, &mut local));
    vec
}


pub fn create_island(
    board: &Board,
    from: Square,
    to: Square,
    global: &mut Vec<Square>,
    local: &mut Vec<Square>,
) -> bool
{
    global.clear();
    let mut board = board.clone();
    board.play_from_to(from, to);

    let mut iter = neighbors(&from)
        .into_iter()
        .filter(|sq| match board.get(sq)
        {
            Some(bs) => !bs.pieces.is_empty(),
            _ => false,
        })
        .chain(std::iter::once(to));
    let first = iter.next().unwrap();

    create_set(&board, first, global);

    iter.any(|s| {
        local.clear();
        check_global(&board, s, &global, local)
    })
}

// @TODO, make this better
fn create_set(board: &Board, fst: Square, set: &mut Vec<Square>)
{
    for sq in neighbors(&fst).into_iter().filter(|sq| match board.get(sq)
    {
        Some(bs) => !bs.pieces.is_empty(),
        _ => false,
    })
    {
        if !set.contains(&sq)
        {
            set.push(sq);
            create_set(board, sq, set);
        }
    }
}

fn check_global(board: &Board, sq: Square, global: &Vec<Square>, local: &mut Vec<Square>) -> bool
{
    if !global.contains(&sq)
    {
        return true;
    }

    for sq in neighbors(&sq).into_iter().filter(|sq| match board.get(sq)
    {
        Some(bs) => !bs.pieces.is_empty(),
        _ => false,
    })
    {
        if !local.contains(&sq)
        {
            local.push(sq);
            if check_global(board, sq, global, local)
            {
                return true;
            }
        }
    }
    false
}


pub fn can_fit(current: Square, next: Square, board: &Board) -> bool
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
    let occupied = |a, b| board.contains_key(&a) && board.contains_key(&b);


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
            board.insert(e, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black)));
        }
        board.insert(ant_square, BoardSquare::new(ant));

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
    fn test_get_correct_neighbors()
    {
        let same = |a: [Square; 6], b: [Square; 6]| a.iter().all(|a| b.contains(a));

        assert!(same(neighbors(&(0, 0, 0)), [
            (0, -1, 1),
            (0, 1, -1),
            (1, 0, -1),
            (-1, 0, 1),
            (1, -1, 0),
            (-1, 1, 0)
        ]));

        assert!(same(neighbors(&(0, -2, 2)), [
            (0, -3, 3),
            (1, -3, 2),
            (1, -2, 1),
            (0, -1, 1),
            (-1, -1, 2),
            (-1, -2, 3)
        ]));
    }

    #[test]
    fn can_detect_create_islands_simple()
    {
        let mut board = Board::default();
        for sq in [(0, -1, 1), (0, 0, 0), (0, 1, -1)]
        {
            board.insert(sq, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black)));
        }

        let from = (0, 1, -1);
        let to = (1, 0, -1);

        assert!(!create_island(&mut board, from, to, &mut Vec::new(), &mut Vec::new()));

        let from = (0, 1, -1);
        let to = (0, 2, -2);

        assert!(create_island(&mut board, from, to, &mut Vec::new(), &mut Vec::new()));
    }

    #[test]
    fn can_detect_create_islands_more_pieces()
    {
        let mut board = Board::default();


        let squares = [
            (0, -1, 1),
            (0, 0, 0),
            (0, 1, -1),
            (0, 2, -2),
            (-2, 0, 2),
            (-2, 1, 1),
            (-1, 0, 1),
            (-1, 1, 0),
            (-1, 3, -2),
            (1, -1, 0),
            (1, 0, -1),
            (1, 1, -2),
            (1, 2, -3),
        ];


        for sq in squares
        {
            board.insert(sq, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black)));
        }

        let from = (1, 0, -1);
        let to = (2, -1, -1);

        assert!(!create_island(&mut board, from, to, &mut Vec::new(), &mut Vec::new()));

        let from = (1, 0, -1);
        let to = (3, -1, -2);

        assert!(create_island(&mut board, from, to, &mut Vec::new(), &mut Vec::new()));
    }

    #[test]
    fn can_detect_create_island_circle()
    {
        let mut board = Board::default();


        let squares = [
            (-1, -2, 3),
            (-1, -1, 2),
            (-1, 0, 1),
            (-1, 1, 0),
            (0, 1, -1),
            (1, 1, -2),
            (2, 1, -3),
            (3, 0, -3),
            (4, -1, -3),
            (4, -2, -2),
            (4, -3, -1),
            (4, -4, 0),
            (3, -4, 1),
            (2, -4, 2),
            (1, -4, 3),
            (0, -3, 3),
            (0, 0, 0),
        ];


        for sq in squares
        {
            board.insert(sq, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black)));
        }

        let from = (0, 0, 0);
        let to = (1, 0, -1);

        assert!(!create_island(&mut board, from, to, &mut Vec::new(), &mut Vec::new()));

        let from = (-1, -2, 3);
        let to = (-2, -1, 3);

        assert!(!create_island(&mut board, from, to, &mut Vec::new(), &mut Vec::new()));

        board.remove((2, 1, -3));
        let from = (-1, -2, 3);
        let to = (-2, -1, 3);

        assert!(create_island(&mut board, from, to, &mut Vec::new(), &mut Vec::new()));
    }
}
