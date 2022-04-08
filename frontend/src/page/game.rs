mod hex;
mod menu;
mod selected_piece;
mod util;

use hex::*;
use menu::*;
use selected_piece::*;
use util::*;

use crate::request::game::*;
use crate::request::ws_url;
use seed::{self, prelude::*, *};
use shared::{model::*, r#move::*, ObjectId};
use web_sys::{Event, SvgGraphicsElement};

#[derive(Default)]
pub struct Model {
    pub game: Option<GameResource>,
    pub gridv3: Vec<Hex>,
    pub piece: Option<SelectedPiece>,
    pub svg: ElRef<SvgGraphicsElement>,
    pub color: Option<Color>,
    pub menu: Option<Menu>,
    pub size: String,
    pub label: Option<String>,
    pub socket: Option<WebSocket>,
    pub legal_moves_cache: Option<Vec<Square>>,

    pub radius: usize,

    pub _size: f32,
    pub _modifier: f32,
}

fn gen_size(n: f32) -> String {
    let l = 5. * n * 0.8;
    let h = 9. * n * 0.8;
    let w = 10. * n * 0.8;

    format!("{l}, -{h} -{l}, -{h} -{w}, 0 -{l}, {h} {l}, {h} {w}, 0")
}

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Option<Model> {
    // TODO: Figure this out
    match url.next_path_part() {
        Some(id) => match ObjectId::parse_str(id) {
            Ok(id) => {
                orders.perform_cmd(async move { Msg::FetchGame(get_game(id).await) });
                let size = gen_size(0.5);

                let socket = WebSocket::builder(ws_url(id), orders)
                    .on_message(Msg::MessageReceived)
                    .on_open(|| Msg::Open)
                    .on_close(|_| Msg::Close)
                    .build_and_open()
                    .ok();
                const DEFAULT_RAD: usize = 0;
                const DEFAULT_SIZE: f32 = 0.5;
                const DEFAULT_MOD: f32 = 1.0;


                Some(Model {
                    game: None,
                    gridv3: create_gridv3(DEFAULT_RAD),
                    menu: None,
                    svg: ElRef::default(),
                    color: None,
                    piece: None,
                    size,
                    label: None,
                    socket,
                    legal_moves_cache: None,
                    radius: DEFAULT_RAD,

                    _size: DEFAULT_SIZE,
                    _modifier: DEFAULT_MOD,
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

    Open,
    Close,
    MessageReceived(WebSocketMessage),

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
        Msg::Open => {
            log("OPEN");
        }
        Msg::Close => {
            log("CLOSE");
        }
        Msg::MessageReceived(msg) => {
            if let Ok(r#move) = msg.json::<Move>() {
                // No need to play the move if we just played it.
                if !just_my_move(model, &r#move) {
                    let rad = sq_radius(r#move.sq);
                    play_move(model, r#move);
                    if rad > model.radius {
                        model.radius = rad;
                        model.gridv3 = create_gridv3(rad);
                        grid_from_board(model);
                    }
                } else {
                    log("IGNORE");
                }
            }
        }

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
                    use BoardPiece::*;
                    let items = [Ant, Beetle, Grasshopper, Spider, Queen]
                        .into_iter()
                        .map(|r#type| Piece { color, r#type });

                    let board = get_board(model).unwrap();
                    model.menu = Some(Menu::new(items, board));
                }

                let rad = get_radius(model);
                log(rad);
                if rad > model.radius {
                    model.radius = rad;
                    model.gridv3 = create_gridv3(rad);
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

                    let rad = get_radius(model);
                    if rad > model.radius {
                        model.radius = rad;
                        model.gridv3 = create_gridv3(rad);
                        grid_from_board(model);
                    }

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
                model.legal_moves_cache = None;
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
                    let board = &mut model.game.as_mut().unwrap().board;

                    if model.legal_moves_cache.is_none() {
                        model.legal_moves_cache =
                            Some(legal_moves(piece, board, Some(sel.old_square)));
                    }
                }
            }

            set_highlight(model, true);
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

                model.menu.as_mut().unwrap().reduce_piece(r#type);

                if let Some(ref mut b) = get_board_mut(model) {
                    b.place_piece(piece, sq, None);
                }

                if let Some(r#move) = get_move(model, piece, sq, None) {
                    orders.perform_cmd(async move { Msg::SentMove(send_move(r#move).await) });
                }
                let rad = get_radius(model);
                if rad > model.radius {
                    model.radius = rad;
                    model.gridv3 = create_gridv3(rad);
                    grid_from_board(model);
                }

                if game_complete(model) {
                    let id = model.game.as_ref().unwrap()._id;
                    orders.perform_cmd(async move { Msg::CompleteGame(complete_game(id).await) });
                }
            }

            model.legal_moves_cache = None;
            clear_highlighs(model);
        }

        Msg::Drag(piece) => {
            clear_highlighs(model);
            if legal_turn(model) {
                let board = &mut model.game.as_mut().unwrap().board;
                if model.legal_moves_cache.is_none() {
                    model.legal_moves_cache = Some(legal_moves(&piece, board, None));
                }
                set_highlight(model, true);
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
        IF!(model.menu.is_some() => {
            div![C!("piece-menu"), model.menu.as_ref().unwrap().to_node()]
        }),
        IF!(model.label.is_some() => match model.label {
            Some(ref s) => h2! [C!("error"), s],
            _ => unreachable!()
        })
    ]]
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
        C!("board-container"),
        view_pieces(model),
        div![
            C!("board"),
            view_board(model),
            IF!(model.piece.is_some() => {
                model.piece.as_ref().unwrap().node(&model)
            })
        ],
    ]
}
