use super::route::AddUrlPath;
use core::iter::FromIterator;
use core::iter::IntoIterator;
use core::str::FromStr;
use seed::Url;
use std::string::ToString;
use strum_macros;

#[derive(strum_macros::EnumString, strum_macros::ToString)]
#[strum(serialize_all = "kebab_case")]
pub enum Weapon {
    Index,
    Rifle,
    Bow,
    Shotgun,
    Pistol,
    Melee(Melee),
    Exalted,
    ArchGun,
    ArchMelee,
}

#[derive(strum_macros::EnumString, strum_macros::ToString)]
#[strum(serialize_all = "kebab_case")]
pub enum Melee {
    Index,
    BladeAndWhip,
    Claws,
    Dagger,
    DualDaggers,
    DualSwords,
    Fist,
    Glaive,
    Gunblade,
    Hammer,
    HeavyBlade,
    Machete,
    Nikana,
    Nunchaku,
    Polearm,
    Rapier,
    Scythe,
    Sparring,
    Staff,
    Sword,
    SwordAndShield,
    TwoHandedNikana,
    Tonfa,
    Warfan,
    Whip,
}

impl Default for Weapon {
    fn default() -> Self {
        Weapon::Index
    }
}

impl Default for Melee {
    fn default() -> Self {
        Melee::Index
    }
}

impl AddUrlPath for Weapon {
    fn add_url_path_part(self, url: Url) -> Url {
        let url = url.add_path_part(self.to_string());
        match self {
            Weapon::Melee(melee) => url.add_path_part(melee.to_string()),
            _ => url,
        }
    }
}

impl<S> FromIterator<S> for Weapon
where
    S: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        match iter.next() {
            Some(p2) => {
                let weapon = Weapon::from_str(p2.as_ref()).unwrap_or_default();
                match weapon {
                    Weapon::Melee(ref _default) => match iter.next() {
                        Some(p3) => {
                            let melee = Melee::from_str(p3.as_ref()).unwrap_or_default();
                            Weapon::Melee(melee)
                        }
                        None => weapon,
                    },
                    _ => weapon,
                }
            }
            None => Weapon::default(),
        }
    }
}
