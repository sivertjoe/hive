use seed::prelude::*;

pub mod home;
pub mod login;
pub mod register;

use enum_as_inner::EnumAsInner;

#[derive(EnumAsInner)]
pub enum Page
{
    Home,
    Login,
    Register(register::Model),
    NotFound,
}

impl Page
{
    pub fn init(mut url: Url) -> Self
    {
        match url.next_path_part()
        {
            None => Self::Home,
            Some("login") => login::init(url).map_or(Self::NotFound, |_| Self::Login),
            Some("register") => register::init(url).map_or(Self::NotFound, Self::Register),
            Some(_) => Self::NotFound,
        }
    }
}
