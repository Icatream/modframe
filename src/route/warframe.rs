use super::route::AddUrlPath;
use core::iter::FromIterator;
use core::iter::IntoIterator;
use core::str::FromStr;
use seed::Url;
use std::string::ToString;
use strum_macros;

#[derive(strum_macros::EnumString, strum_macros::ToString)]
#[strum(serialize_all = "kebab_case")]
pub enum Warframe {
    Index,
    Ash,
    AshPrime
}

impl Default for Warframe {
    fn default() -> Self {
        Warframe::Index
    }
}

impl AddUrlPath for Warframe {
    fn add_url_path_part(self, url: Url) -> Url {
        url.add_path_part(self.to_string())
    }
}

impl<S> FromIterator<S> for Warframe
where
    S: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        match iter.next() {
            Some(p2) => Warframe::from_str(p2.as_ref()).unwrap_or_default(),
            None => Warframe::default(),
        }
    }
}
