use seed::prelude::*;

pub mod create;
pub mod game;
pub mod home;
pub mod login;
pub mod register;

use enum_as_inner::EnumAsInner;

#[derive(EnumAsInner)]
pub enum Page {
    Home(home::Model),
    Login(login::Model),
    Register(register::Model),
    Create(create::Model),
    Game(Box<game::Model>),

    NotFound,
}

impl Page {
    pub fn init(mut url: Url, orders: &mut impl Orders<crate::Msg>) -> Self {
        match url.next_path_part() {
            None => Self::Home(home::init(&mut orders.proxy(crate::Msg::Home))),
            Some("login") => login::init(url).map_or(Self::NotFound, Self::Login),
            Some("register") => register::init(url).map_or(Self::NotFound, Self::Register),
            Some("create") => Self::Create(create::init()),
            Some("game") => game::init(url, &mut orders.proxy(crate::Msg::Game))
                .map_or(Self::NotFound, |o| Self::Game(Box::new(o))),
            Some(_) => Self::NotFound,
        }
    }
}
