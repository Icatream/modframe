use super::{
    archwing::Archwing, companion::Companion, necramech::Necramech, warframe::Warframe,
    weapon::Weapon,
};
use core::iter::FromIterator;
use core::iter::Iterator;
use core::str::FromStr;
use seed::Url;
use std::string::ToString;
use strum_macros;

#[derive(strum_macros::EnumString, strum_macros::ToString)]
#[strum(serialize_all = "kebab_case")]
pub enum Route {
    #[strum(serialize = "")]
    Home,
    #[strum(serialize = "404")]
    NotFound,
    Warframe(Warframe),
    Weapon(Weapon),
    Companion(Companion),
    Archwing(Archwing),
    Necramech(Necramech),
}

pub trait AddUrlPath: Sized {
    fn add_url_path_part(self, url: Url) -> Url;
}

impl From<&Url> for Route {
    fn from(url: &Url) -> Self {
        let mut iter = url.path().into_iter().map(|s| s.to_lowercase());
        match iter.next() {
            Some(p1) => {
                let route = Route::from_str(p1.as_ref()).unwrap_or_default();
                match route {
                    Route::Warframe(ref _default) => Route::Warframe(Warframe::from_iter(iter)),
                    Route::Weapon(ref _default) => Route::Weapon(Weapon::from_iter(iter)),
                    _ => route,
                }
            }
            None => Route::default(),
        }
    }
}

impl Into<Url> for Route {
    fn into(self) -> Url {
        let url = Url::new().add_path_part(self.to_string());
        match self {
            Route::Warframe(warframe) => warframe.add_url_path_part(url),
            Route::Weapon(weapon) => weapon.add_url_path_part(url),
            _ => url,
        }
    }
}

impl Default for Route {
    fn default() -> Self {
        Route::Home
    }
}
