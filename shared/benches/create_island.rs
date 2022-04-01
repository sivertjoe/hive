use criterion::{criterion_group, criterion_main, Criterion};
use shared::{model::*, r#move::*};


fn criterion_benchmark(c: &mut Criterion)
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

    board.remove((2, 1, -3));

    let from = (-1, -2, 3);
    let to = (-2, -1, 3);

    c.bench_function("create island", |b| b.iter(|| create_island(&mut board, from, to)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
