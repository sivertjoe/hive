use std::collections::HashMap;

use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub type Square = (isize, isize, isize);
type Name = String;


#[derive(Serialize, Deserialize)]
pub struct Move
{
    pub player_id: ObjectId,
    pub game_id:   ObjectId,
    pub piece:     Piece,
    pub sq:        Square,
    pub old_sq:    Option<Square>,
}

#[derive(Serialize, Deserialize)]
pub struct OnGoingGame
{
    pub game_object_id: ObjectId,
    pub players:        [Name; 2],
}


// Database representation
#[derive(Serialize, Deserialize)]
pub struct Game
{
    players:   [ObjectId; 2],
    pub board: Board,
}

// Remove ObjectId basically
#[derive(Serialize, Deserialize)]
pub struct GameResource
{
    pub _id:     ObjectId,
    pub players: [Name; 2],
    pub board:   Board,
}

impl Game
{
    pub fn new(players: [ObjectId; 2]) -> Self
    {
        Self {
            players,
            board: Board::default(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Color
{
    White,
    Black,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Piece
{
    pub r#type: BoardPiece,
    pub color:  Color,
}

impl Piece
{
    #[allow(dead_code)]
    pub fn new(r#type: BoardPiece, color: Color) -> Self
    {
        Self {
            color,
            r#type,
        }
    }
}

pub fn legal_moves(p: &Piece, board: &Board, board_pos: Option<Square>) -> Vec<Square>
{
    /*let search_stack = |next: &Stack| match next
    {
        Some(bp) => bp.piece.r#type == BoardPiece::Queen && bp.piece.color == p.color,
        None => false,
    };

    let contains_queen = || {
        board
            .board
            .values()
            .position(|bp| {
                bp.piece.r#type == BoardPiece::Queen && bp.piece.color == p.color
                    || search_stack(&bp.next)
            })
            .is_some()
    };*/


    match board.turns
    {
        // These first we _know_ and can be hardcoded
        0 => vec![(0, 0, 0)],
        1 => vec![(1, -1, 0), (-1, 1, 0), (1, 0, -1), (-1, 0, 1), (0, 1, -1), (0, -1, 1)],

        _ =>
        {
            /*if (board.turns == 7 || board.turns == 8) && !contains_queen()
            {
                todo!()
            }*/

            match board_pos
            {
                Some(pos) => legal_on_board_move(p, board, pos),
                None => legal_new_piece_moves(p, board),
            }
        },
    }
}


fn ant_move(board: &Board, sq: Square) -> Vec<Square>
{
    board
        .board
        .iter()
        .filter_map(|(s, _p)| {
            // skip myself
            if *s == sq
            {
                None
            }
            else
            {
                Some(neighbors(s).into_iter().filter(|s| !board.board.contains_key(s)))
            }
        })
        .flatten()
        .collect()
}

fn beetle_move(board: &Board, sq: Square) -> Vec<Square>
{
    // TODO: Improve this, look at square_has_neighbors
    let def = || {
        let have_neighbor = |square: &Square| {
            sq != *square
                && (board.board.contains_key(square)
                    || neighbors(square)
                        .into_iter()
                        .any(|_sq| _sq != sq && board.board.contains_key(&_sq)))
        };

        neighbors(&sq).into_iter().filter(have_neighbor).collect()
    };

    match board.board.get(&sq)
    {
        Some(bs) if bs.pieces.len() > 1 => neighbors(&sq).into_iter().collect(),
        _ => def(),
    }
}

fn grass_hopper_move(board: &Board, sq: Square) -> Vec<Square>
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

fn square_has_neighbors(sq: Square, board: &Board, me: Square) -> bool
{
    neighbors(&sq)
        .into_iter()
        .filter(|s| *s != me)
        .any(|s| board.board.contains_key(&s))
}

fn queen_move(board: &Board, sq: Square) -> Vec<Square>
{
    let is_vakant = |sq: Square| !board.board.contains_key(&sq);

    let legal_square =
        |square: &Square| is_vakant(*square) && square_has_neighbors(*square, board, sq);

    neighbors(&sq)
        .into_iter()
        .inspect(|s| println!("{s:?}"))
        .filter(legal_square)
        .collect()
}


fn legal_on_board_move(p: &Piece, board: &Board, sq: Square) -> Vec<Square>
{
    match p.r#type
    {
        BoardPiece::Ant => ant_move(board, sq),
        BoardPiece::Beetle => beetle_move(board, sq),
        BoardPiece::Grasshopper => grass_hopper_move(board, sq),
        BoardPiece::Queen => queen_move(board, sq),
        _ => vec![(-2, 2, 0)],
    }
}

// Hmm, t-this can be simplified r-right?
fn legal_new_piece_moves(piece: &Piece, board: &Board) -> Vec<Square>
{
    // Good neighbors have only same color neighbors or none
    let good_neighbors = |sq: &Square| {
        neighbors(sq).into_iter().all(|sq| match board.board.get(&sq)
        {
            None => true,
            Some(s) => s.top().color == piece.color,
        })
    };

    let not_touching_other_color =
        //|sq: Square| board.board.get(&sq).map_or(true, |s| s.piece.color == piece.color);
        |sq: Square| board.board.get(&sq).is_none() && good_neighbors(&sq);

    board
        .board
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


#[derive(Debug, Serialize, Deserialize)]
pub struct BoardSquare
{
    pub pieces: Vec<Piece>,
}

impl BoardSquare
{
    fn new(piece: Piece) -> Self
    {
        Self {
            pieces: vec![piece]
        }
    }

    fn place_piece(&mut self, piece: Piece)
    {
        self.pieces.push(piece);
    }

    fn remove_piece(&mut self)
    {
        self.pieces.pop();
    }

    fn top(&self) -> &Piece
    {
        self.pieces.last().unwrap()
    }
}


// Manually derived trait in board_serialize.rs because of `Square` key
#[derive(Debug, Default)]
pub struct Board
{
    pub board: HashMap<Square, BoardSquare>,
    pub turns: usize,
}

impl Board
{
    pub fn play_move(&mut self, r#move: Move)
    {
        // Have logic to check for legal move here
        self.place_piece(r#move.piece, r#move.sq, r#move.old_sq);
    }

    pub fn place_piece(&mut self, piece: Piece, sq: Square, old: Option<Square>)
    {
        self.board
            .entry(sq)
            .and_modify(|bs| bs.place_piece(piece))
            .or_insert_with(|| BoardSquare::new(piece));

        if let Some(sq) = old
        {
            let bs = self.board.get_mut(&sq).unwrap();
            bs.remove_piece();
            if bs.pieces.is_empty()
            {
                self.board.remove(&sq).unwrap();
            }
        }

        self.turns += 1;
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum BoardPiece
{
    Queen,
    Ant,
    Spider,
    Beetle,
    Grasshopper,
}

impl From<String> for BoardPiece
{
    fn from(s: String) -> Self
    {
        use BoardPiece::*;
        match s.as_str()
        {
            "Queen" => Queen,
            "Ant" => Ant,
            "Spider" => Spider,
            "Beetle" => Beetle,
            "Grasshopper" => Grasshopper,
            _ => unreachable!(),
        }
    }
}


fn neighbors(sq: &Square) -> [Square; 6]
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

#[cfg(test)]
mod test
{
    use super::*;

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
    fn test_beetle_correct_moves_simple()
    {
        let mut board = Board::new();

        let beetle_square = (1, 0, -1);
        let beetle = Piece::new(BoardPiece::Beetle, Color::White);

        let pos = [
            ((0, 0, 0), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            (beetle_square, BoardSquare::new(beetle.clone())),
        ];


        board.board = HashMap::from_iter(pos.into_iter());
        board.turns = board.board.len();

        let mut legal_moves = legal_moves(&beetle, &board, Some(beetle_square));
        let mut ans = vec![(1, -1, 0), (0, 0, 0), (0, 1, -1)];

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }

    #[test]
    fn test_beetle_correct_moves_on_top()
    {
        let mut board = Board::new();

        let beetle_square = (0, 0, 0);
        let beetle = Piece::new(BoardPiece::Beetle, Color::White);

        board.place_piece(Piece::new(BoardPiece::Ant, Color::Black), beetle_square, None);
        board.place_piece(beetle, beetle_square, None);


        board.turns = 3;

        let mut legal_moves = legal_moves(&beetle, &board, Some(beetle_square));
        let mut ans = neighbors(&beetle_square);


        ans.sort();
        legal_moves.sort();


        assert_eq!(legal_moves, ans);
        assert_eq!(legal_moves.len(), ans.len());
    }

    #[test]
    fn test_beetle_correct_moves_surrounded()
    {
        let mut board = Board::new();

        let beetle_square = (0, 0, 0);
        let beetle = Piece::new(BoardPiece::Beetle, Color::White);

        let mut vec = vec![(beetle_square, BoardSquare::new(beetle.clone()))];
        for sq in neighbors(&beetle_square)
        {
            vec.push((sq, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))));
        }


        board.board = HashMap::from_iter(vec.into_iter());
        board.turns = board.board.len();

        let mut legal_moves = legal_moves(&beetle, &board, Some(beetle_square));
        let mut ans = neighbors(&beetle_square);

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }


    #[test]
    fn test_grass_hopper_simple()
    {
        let mut board = Board::new();

        let grass_hopper_square = (1, 0, -1);
        let grass_hopper = Piece::new(BoardPiece::Grasshopper, Color::White);

        let pos = [
            ((0, 0, 0), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            (grass_hopper_square, BoardSquare::new(grass_hopper.clone())),
        ];


        board.board = HashMap::from_iter(pos.into_iter());
        board.turns = 3;

        let mut legal_moves = legal_moves(&grass_hopper, &board, Some(grass_hopper_square));
        let mut ans = vec![(-1, 0, 1)];

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }

    #[test]
    fn test_grass_hopper_two_squares()
    {
        let mut board = Board::new();

        let grass_hopper_square = (1, 0, -1);
        let grass_hopper = Piece::new(BoardPiece::Grasshopper, Color::White);

        let pos = [
            ((0, 0, 0), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            ((-1, 0, 1), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            (grass_hopper_square, BoardSquare::new(grass_hopper.clone())),
        ];


        board.board = HashMap::from_iter(pos.into_iter());
        board.turns = board.board.len();

        let mut legal_moves = legal_moves(&grass_hopper, &board, Some(grass_hopper_square));
        let mut ans = vec![(-2, 0, 2)];

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }

    #[test]
    fn test_grass_hopper_surround()
    {
        let mut board = Board::new();

        let grass_hopper_square = (0, 0, 0);
        let grass_hopper = Piece::new(BoardPiece::Grasshopper, Color::White);

        let mut pos = vec![(grass_hopper_square, BoardSquare::new(grass_hopper.clone()))];
        for sq in neighbors(&grass_hopper_square)
        {
            pos.push((sq, BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))));
        }


        board.board = HashMap::from_iter(pos.into_iter());
        board.turns = board.board.len();

        let mut legal_moves = legal_moves(&grass_hopper, &board, Some(grass_hopper_square));
        let mut ans = vec![(-2, 0, 2), (0, -2, 2), (2, -2, 0), (2, 0, -2), (0, 2, -2), (-2, 2, 0)];

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }


    #[test]
    fn test_queen_move_simple()
    {
        let mut board = Board::new();

        let queen_square = (1, 0, -1);
        let queen = Piece::new(BoardPiece::Queen, Color::White);

        let pos = [
            ((0, 0, 0), BoardSquare::new(Piece::new(BoardPiece::Ant, Color::Black))),
            (queen_square, BoardSquare::new(queen.clone())),
        ];


        board.board = HashMap::from_iter(pos.into_iter());
        board.turns = 3;

        println!("{:?}", board.board);
        let mut legal_moves = legal_moves(&queen, &board, Some(queen_square));
        let mut ans = vec![(1, -1, 0), (0, 1, -1)];

        ans.sort();
        legal_moves.sort();

        assert_eq!(legal_moves, ans);
    }
}
