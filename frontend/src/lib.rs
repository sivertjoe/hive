use seed::{prelude::*, *};

mod component;
mod page;
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
    UserCred(component::user_cred::Msg),
    CreateGame(page::create::Msg),
    Home(page::home::Msg),

    Login { name: String },
}

fn get_user_cred(model: &mut Model) -> &mut component::user_cred::Model {
    match model.page {
        Page::Login(ref mut model) => &mut model.user_cred,
        Page::Register(ref mut model) => &mut model.user_cred,
        _ => unreachable!(),
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url, orders),
        Msg::UserCred(msg) => component::user_cred::update(msg, get_user_cred(model), orders),
        Msg::CreateGame(msg) => page::create::update(
            msg,
            model.page.as_create_mut().unwrap(),
            &mut orders.proxy(Msg::CreateGame),
        ),

        Msg::Home(msg) => page::home::update(msg, model.page.as_home_mut().unwrap()),
        Msg::Login { name } => model.user = Some(name),
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        component::navbar::view(&model),
        div![
            C!("main"),
            match &model.page {
                Page::Home(model) => page::home::view(model),
                Page::Login(model) => page::login::view(model),
                Page::Register(model) => page::register::view(model),
                Page::Create(model) => page::create::view(model),
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
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
