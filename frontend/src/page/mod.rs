use seed::prelude::*;

pub mod create;
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

    NotFound,
}

impl Page {
    pub fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::Home(home::init()),
            Some("login") => login::init(url).map_or(Self::NotFound, Self::Login),
            Some("register") => register::init(url).map_or(Self::NotFound, Self::Register),
            Some("create") => Self::Create(create::init()),
            Some(_) => Self::NotFound,
        }
    }
}
