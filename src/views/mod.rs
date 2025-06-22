mod about;
mod document;
mod home;
mod nav;
mod projects;
mod snake;
mod stack_card;

use std::fmt::Display;

use strum::EnumIter;

pub use self::{about::*, document::*, home::*, nav::*, projects::*, snake::*, stack_card::*};

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    About,
    Projects,
    Snake,
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Page::Home => write!(f, "Home"),
            Page::About => write!(f, "About"),
            Page::Projects => write!(f, "Projects"),
            Page::Snake => write!(f, "Snake"),
        }
    }
}

impl Page {
    fn get_ref(&self) -> &str {
        match self {
            Page::Home => "/",
            Page::About => "/about",
            Page::Projects => "/projects",
            Page::Snake => "/snake",
        }
    }
}
