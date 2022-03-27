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
    neighbors(&sq)
        .into_iter()
        .filter(|s| *s != me)
        .any(|s| board.board.contains_key(&s))
}


fn legal_on_board_move(p: &Piece, board: &Board, sq: Square) -> Vec<Square>
{
    match p.r#type
    {
        BoardPiece::Ant => ant_move(board, sq),
        BoardPiece::Beetle => beetle_move(board, sq),
        BoardPiece::Grasshopper => grasshopper_move(board, sq),
        BoardPiece::Queen => queen_move(board, sq),
        BoardPiece::Spider => spider_move(board, sq),
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
}
