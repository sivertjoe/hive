use seed::{prelude::*, *};

mod component;
mod page;
use page::Page;

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

pub enum Msg
{
    UrlChanged(subs::UrlChanged),
    Register(page::register::Msg),
    Login(page::login::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>)
{
    match msg
    {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url),
        Msg::Register(msg) => page::register::update(
            msg,
            model.page.as_register_mut().unwrap(),
            &mut orders.proxy(Msg::Register),
        ),
        Msg::Login(msg) => page::login::update(
            msg,
            model.page.as_login_mut().unwrap(),
            &mut orders.proxy(Msg::Login),
        ),
    }
}

fn view(model: &Model) -> Node<Msg>
{
    div![component::navbar::view(&model), div![C!("main"), match model.page
    {
        Page::Home => page::home::view(),
        Page::Login(ref model) => page::login::view(model),
        Page::Register(ref model) => page::register::view(model),
        Page::NotFound => div!["404"],
    }]]
}

#[wasm_bindgen(start)]
pub fn start()
{
    App::start("app", init, update, view);
}
