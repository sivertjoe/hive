use seed::{prelude::*, *};

mod navbar;
mod page;
use navbar::navbar;

fn init(url: Url, _: &mut impl Orders<Msg>) -> Model
{
    Model {
        base_url: url.to_base_url(), page: Page::init(url)
    }
}

pub struct Model
{
    base_url: Url,
    page:     Page,
}


enum Page
{
    Home,
    Login,
    Register(page::register::Model),
    NotFound,
}

impl Page
{
    fn init(mut url: Url) -> Self
    {
        match url.next_path_part()
        {
            None => Self::Home,
            Some("login") => page::login::init(url).map_or(Self::NotFound, |_| Self::Login),
            Some("register") => page::register::init(url).map_or(Self::NotFound, Self::Register),
            Some(_) => Self::NotFound,
        }
    }
}

struct_urls!();
impl<'a> Urls<'a>
{
    pub fn home(self) -> Url
    {
        self.base_url()
    }

    pub fn login(self) -> Url
    {
        self.base_url().add_path_part("login")
    }
}


pub enum Msg
{
    UrlChanged(subs::UrlChanged),
    Register(page::register::Msg),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>)
{
    match msg
    {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url),
        Msg::Register(msg) =>
        {
            if let Page::Register(ref mut reg) = model.page
            {
                page::register::update(msg, reg);
            }
        },
    }
}

fn view(model: &Model) -> Node<Msg>
{
    // div![navbar(&model)]
    match model.page
    {
        Page::Home => page::home::view(),
        Page::Login => page::login::view(),
        Page::Register(ref model) => page::register::view(model),
        Page::NotFound => div!["404"],
    }
}

#[wasm_bindgen(start)]
pub fn start()
{
    App::start("app", init, update, view);
}
