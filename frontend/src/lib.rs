use seed::{prelude::*, *};
use shared::ObjectId;

mod component;
mod page;
mod request;
use page::Page;

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    let user = LocalStorage::get("name").ok();
    Model {
        base_url: url.to_base_url(),
        page: Page::init(url, orders),
        user,
    }
}

pub struct Model {
    base_url: Url,
    page: Page,
    user: Option<String>,
}

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    CreateGame(page::create::Msg),
    Home(page::home::Msg),
    Game(page::game::Msg),
    Login { name: String },

    LoginPage(page::login::Msg),
    RegisterPage(page::register::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url, orders),
        Msg::CreateGame(msg) => page::create::update(
            msg,
            model.page.as_create_mut().unwrap(),
            &mut orders.proxy(Msg::CreateGame),
        ),

        Msg::Home(msg) => page::home::update(
            msg,
            model.page.as_home_mut().unwrap(),
            &mut orders.proxy(Msg::Home),
        ),

        Msg::Game(msg) => page::game::update(
            msg,
            model.page.as_game_mut().unwrap(),
            &mut orders.proxy(Msg::Game),
        ),

        Msg::Login { name } => model.user = Some(name),
        Msg::LoginPage(msg) => page::login::update(msg, model.page.as_login_mut().unwrap(), orders),
        Msg::RegisterPage(msg) => {
            page::register::update(msg, model.page.as_register_mut().unwrap(), orders)
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        component::navbar::view(model),
        div![
            C!("main"),
            match &model.page {
                Page::Home(model) => page::home::view(model),
                Page::Login(model) => page::login::view(model),
                Page::Register(model) => page::register::view(model),
                Page::Create(model) => page::create::view(model),
                Page::Game(model) => page::game::view(model),
                Page::NotFound => div!["404"],
            }
        ]
    ]
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn create(self) -> Url {
        self.base_url().add_path_part("create")
    }

    pub fn register(self) -> Url {
        self.base_url().add_path_part("register")
    }
    pub fn login(self) -> Url {
        self.base_url().add_path_part("login")
    }
    pub fn user(self, user: &str) -> Url {
        self.base_url().add_path_part("user").add_path_part(user)
    }
    pub fn game(self, game_id: &ObjectId) -> Url {
        self.base_url().add_path_part(format!("game?q={}", game_id))
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
