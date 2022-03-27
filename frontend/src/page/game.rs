mod hex;
mod menu;
mod selected_piece;
mod util;

use hex::*;
use menu::*;
use selected_piece::*;
use util::*;

use crate::request::game::*;
use seed::{self, prelude::*, *};
use shared::{model::*, r#move::*, ObjectId};
use web_sys::{Event, SvgGraphicsElement};

#[derive(Default)]
pub struct Model {
    pub game: Option<GameResource>,
    pub gridv3: Vec<Hex>,
    pub piece: Option<SelectedPiece>,
    pub menu: Option<Vec<MenuItem>>,
    pub svg: ElRef<SvgGraphicsElement>,
    pub color: Option<Color>,

    pub size: String,
    pub label: Option<String>,
}

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Option<Model> {
    let gen_size = |n: f32| {
        let l = 5. * n;
        let h = 9. * n;
        let w = 10. * n;

        format!("{l}, -{h} -{l}, -{h} -{w}, 0 -{l}, {h} {l}, {h} {w}, 0")
    };
    match url.next_path_part() {
        Some(id) => match ObjectId::parse_str(id) {
            Ok(id) => {
                orders.perform_cmd(async move { Msg::FetchGame(get_game(id).await) });
                let size = gen_size(0.5);

                Some(Model {
                    game: None,
                    gridv3: create_gridv3(5),
                    menu: None,
                    svg: ElRef::default(),
                    color: None,
                    piece: None,
                    size,
                    label: None,
                })
            }
            _ => None,
        },
        _ => None,
    }
}

pub enum Msg {
    FetchGame(fetch::Result<String>),
    SentMove(fetch::Result<String>),
    CompleteGame(fetch::Result<String>),

    Move(Event),
    Click(Event),
    Release(Event),
    Place((String, Event)),
    Drag(Piece),
    MouseUp(Event),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let parse_resp = |resp: fetch::Result<String>| -> Result<ResponseBody, String> {
        resp.map_err(|e| format!("{e:?}"))
            .and_then(|text| {
                serde_json::from_str::<ResponseBody>(&text).map_err(|e| format!("{e:?}"))
            })
            .and_then(|resp| {
                if resp.status == 200 {
                    Ok(resp)
                } else {
                    Err("wrong statuscode".into())
                }
            })
    };
    match msg {
        Msg::SentMove(resp) => {
            if let Err(e) = parse_resp(resp) {
                model.label = Some(format!("{e:?}"));
            }
        }
        Msg::CompleteGame(resp) => {
            if let Err(e) = parse_resp(resp) {
                model.label = Some(format!("{e:?}"));
            }
        }

        Msg::FetchGame(res) => match parse_resp(res) {
            Ok(resp) => {
                let game: GameResource = resp.get_body();
                model.color = get_color(&game);
                model.game = Some(game);
                if let Some(color) = model.color {
                    model.menu = Some(create_menu(color));
                }

                grid_from_board(model);
            }
            Err(e) => {
                model.label = Some(format!("expected 200 got {e}"));
            }
        },

        Msg::Click(event) => {
            let mm = to_mouse_event(&event);
            if mm.button() != 0 {
                return;
            }
            let (x, y) = get_mouse_pos(model, mm);
            let sq = pixel_to_hex(x as isize, y as isize);

            log(sq);

            if let Some(hex) = get_piece_from_square_mut(model, sq) {
                let cl = hex.clone();
                hex.remove_top();

                let mut sel: SelectedPiece = cl.into();
                sel.x = x;
                sel.y = y;
                model.piece = Some(sel);
            }
            // clear_red(model);
        }

        Msg::Release(event) => {
            let mm = to_mouse_event(&event);
            let (x, y) = get_mouse_pos(model, mm);
            let sq = pixel_to_hex(x as isize, y as isize);

            if mm.button() != 0 {
                return;
            }

            if let Some(selected_piece) = model.piece.take() {
                if legal_move(model, sq) {
                    // Place the piece
                    get_hex_from_square(model, sq)
                        .unwrap()
                        .place_piece(selected_piece.piece);

                    let board = get_board_mut(model).unwrap();
                    board.place_piece(selected_piece.piece, sq, Some(selected_piece.old_square));

                    if let Some(r#move) = get_move(
                        model,
                        selected_piece.piece,
                        sq,
                        Some(selected_piece.old_square),
                    ) {
                        orders.perform_cmd(async move { Msg::SentMove(send_move(r#move).await) });
                    }

                    if game_complete(model) {
                        let id = model.game.as_ref().unwrap()._id;
                        orders
                            .perform_cmd(async move { Msg::CompleteGame(complete_game(id).await) });
                    }
                } else {
                    place_piece_back(model, selected_piece);
                }

                clear_highlighs(model);
                //clear_red(model);
            }
        }

        Msg::Move(event) => {
            let mm = to_mouse_event(&event);
            let (x, y) = get_mouse_pos(model, mm);

            let my_turn = legal_turn(model);
            let correct_piece = legal_piece(model);

            if let Some(sel) = model.piece.as_mut() {
                sel.x = x;
                sel.y = y;

                if my_turn && correct_piece {
                    let piece = &sel.piece;
                    let board = &model.game.as_ref().unwrap().board;

                    let legal_moves = legal_moves(piece, board, Some(sel.old_square));
                    set_highlight(model, legal_moves, true);
                }
            }
        }

        Msg::Place((id, event)) => {
            let mm = to_mouse_event(&event);
            let (x, y) = get_mouse_pos(model, mm);
            let sq = pixel_to_hex(x as isize, y as isize);


            if legal_move(model, sq) {
                let r#type: BoardPiece = id.into();
                let color = model.color.unwrap();
                let piece = Piece { r#type, color };
                place_piece(model, piece, sq);
                if let Some(ref mut b) = get_board_mut(model) {
                    b.place_piece(piece, sq, None);
                }
                if let Some(r#move) = get_move(model, piece, sq, None) {
                    orders.perform_cmd(async move { Msg::SentMove(send_move(r#move).await) });
                }

                if game_complete(model) {
                    let id = model.game.as_ref().unwrap()._id;
                    orders.perform_cmd(async move { Msg::CompleteGame(complete_game(id).await) });
                }
            }

            clear_highlighs(model);
        }

        Msg::Drag(piece) => {
            clear_highlighs(model);
            if legal_turn(model) {
                let board = &model.game.as_ref().unwrap().board;
                let legal_moves = legal_moves(&piece, board, None);
                set_highlight(model, legal_moves, true);
            }
        }

        Msg::MouseUp(event) => {
            let mm = to_mouse_event(&event);
            // Secondary button, i.e, right-click
            if mm.button() == 2 {
                let (x, y) = get_mouse_pos(model, mm);
                let sq = pixel_to_hex(x as isize, y as isize);

                if let Some(hex) = get_hex_from_square(model, sq) {
                    hex.red = !hex.red;
                }
            }
            // main button, i.e, left-click
            // should this not be 1?
            // https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons
            else if mm.button() == 0 {
                //clear_red(model);
            }
        }
    }
}

pub fn view(model: &Model) -> Node<crate::Msg> {
    div![div![
        C!("container"),
        grid(model),
        IF!(model.color.is_some() =>
            div![C!("piece-menu"), piece_menu(model)]
        ),
        IF!(model.label.is_some() => match model.label {
            Some(ref s) => h2! [C!("error"), s],
            _ => unreachable!()
        })
    ]]
}

fn piece_menu(model: &Model) -> Node<crate::Msg> {
    if model.menu.is_none() {
        div![style! {
            St::Display => "flex",
        }]
    } else {
        div![
            style! {
                St::Display => "flex",
            },
            model
                .menu
                .as_ref()
                .unwrap()
                .iter()
                .map(MenuItem::to_menu_node)
        ]
    }
}

pub fn grid(model: &Model) -> Node<crate::Msg> {
    div![
        ev(Ev::Drop, |event| {
            let ev = to_drag_event(&event);
            let id = ev.data_transfer().unwrap().get_data("text/plain").unwrap();
            crate::Msg::Game(Msg::Place((id, event)))
        }),
        ev(Ev::DragOver, |event| {
            //log("DragOver");
            event.prevent_default();
        }),
        ev(Ev::ContextMenu, |event| {
            event.prevent_default();
        }),
        ev(Ev::MouseUp, |event| {
            crate::Msg::Game(Msg::MouseUp(event))
        }),
        svg![
            el_ref(&model.svg),
            ev(Ev::MouseMove, |event| {
                crate::Msg::Game(Msg::Move(event))
            }),
            ev(Ev::MouseUp, |event| {
                event.prevent_default();
                crate::Msg::Game(Msg::Release(event))
            }),
            ev(Ev::MouseDown, |event| {
                crate::Msg::Game(Msg::Click(event))
            }),
            attrs! {
                At::ViewBox => "0 0 100 100",
                At::Draggable => "true",
            },
            defs![g![
                attrs! { At::Id => "pod" },
                polygon![attrs! {
                    //At::Stroke => "gold",
                    At::StrokeWidth => ".5",
                    At::Points => &model.size,
                },]
            ]],
            model.gridv3.iter().map(Hex::node),
            IF!(model.piece.is_some() => {
                model.piece.as_ref().unwrap().node()
            })
        ]
    ]
}
