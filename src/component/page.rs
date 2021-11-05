use core::{convert::TryFrom, fmt};
use seed::prelude::*;
use seed::Url;

#[derive(Clone, Debug)]
pub enum Page {
    Home,
    Warframe { name: String },
    Weapon,
    NotFound,
}

impl Page {
    pub fn path(&self) -> Vec<&str> {
        use Page::*;
        match self {
            Home => vec![],
            Warframe { name } => vec!["warframe", name],
            Weapon => vec!["weapon"],
            NotFound => vec!["notfound"],
        }
    }
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}", self.path().join("/"))
    }
}

impl From<Url> for Page {
    fn from(url: Url) -> Self {
        let mut path = url.path().into_iter();

        match path.next().map(String::as_str) {
            Some("") | None => Page::Home,
            Some("warframe") => match path.next() {
                Some(s) => Page::Warframe {
                    name: String::from(s),
                },
                None => Page::NotFound,
            },
            Some("weapon") => Page::Weapon,
            _ => Page::NotFound,
        }
    }
}
