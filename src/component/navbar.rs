use seed::{prelude::*, *};

#[derive(Default)]
pub struct Model {
    burger_active: bool,
    //actived_page: ,
}

#[derive(Copy, Clone)]
pub enum Msg {
    BurgerActive,
    //PageActive()
}

pub fn update<Ms: 'static>(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Ms>) {
    match msg {
        Msg::BurgerActive => {
            model.burger_active = !model.burger_active;
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    nav![
        C!["navbar"],
        attrs! {
            At::from("role") => "navigation",
            At::AriaLabel => "main navigation"
        },
        div![
            C!["navbar-brand"],
            div![
                C!["navbar-item", "has-background-primary", "has-text-white"],
                ev(Ev::Click, |_| { Url::new().go_and_push() }),
                span![
                    C!["icon", "is-small"],
                    i![
                        C!["fas", "fa-home"],
                        attrs! {
                            At::AriaHidden => true
                        }
                    ]
                ],
                span!("Modframe")
            ],
            a![
                C![
                    "navbar-burger",
                    "burger",
                    IF!(model.burger_active => "is-active")
                ],
                attrs! {
                    At::from("role") => "button",
                    At::AriaLabel => "menu",
                    At::AriaExpanded => "false",
                    At::from("data-target") => "navbar0"
                },
                span![attrs! {
                    At::AriaHidden => "true"
                }],
                span![attrs! {
                    At::AriaHidden => "true"
                }],
                span![attrs! {
                    At::AriaHidden => "true"
                }],
                ev(Ev::Click, |_| { Msg::BurgerActive })
            ]
        ],
        div![
            id!("navbar0"),
            C!["navbar-menu", IF!(model.burger_active => "is-active")],
            div![
                C!["navbar-start"],
                a![
                    C!["navbar-item"],
                    "Warframe",
                    ev(Ev::Click, |_| { Url::new().go_and_push() })
                ],
                div![
                    C!["navbar-item", "has-dropdown", "is-hoverable"],
                    a![
                        C!["navbar-link"],
                        "Weapon",
                        ev(Ev::Click, |_| { Url::go_and_load_with_str("/hello") })
                    ],
                    div![
                        C!["navbar-dropdown"],
                        a![
                            C!["navbar-item"],
                            "Rifle",
                            ev(Ev::Click, |_| { Url::new().go_and_push() })
                        ],
                        a![
                            C!["navbar-item"],
                            "Bow",
                            ev(Ev::Click, |_| { Url::new().go_and_push() })
                        ],
                        a![
                            C!["navbar-item"],
                            "Shotgun",
                            ev(Ev::Click, |_| { Url::new().go_and_push() })
                        ],
                        a![
                            C!["navbar-item"],
                            "Pistol",
                            ev(Ev::Click, |_| { Url::new().go_and_push() })
                        ],
                        a![
                            C!["navbar-item"],
                            "Melee",
                            ev(Ev::Click, |_| { Url::new().go_and_push() })
                        ],
                        a![
                            C!["navbar-item"],
                            "Exalted-weapon",
                            ev(Ev::Click, |_| { Url::new().go_and_push() })
                        ],
                        a![
                            C!["navbar-item"],
                            "Arch-gun",
                            ev(Ev::Click, |_| { Url::new().go_and_push() })
                        ],
                        a![
                            C!["navbar-item"],
                            "Arch-melee",
                            ev(Ev::Click, |_| { Url::new().go_and_push() })
                        ]
                    ]
                ],
                a![
                    C!["navbar-item"],
                    "Companion",
                    ev(Ev::Click, |_| { Url::go_back(1) })
                ],
                a![
                    C!["navbar-item"],
                    "Archwing",
                    ev(Ev::Click, |_| { Url::go_forward(1) })
                ]
            ],
            div![
                C!["navbar-end"],
                div![
                    C!["navbar-item"],
                    div![
                        C!["buttons"],
                        a![
                            C!["button", "is-primary"],
                            strong!("Prev"),
                            ev(Ev::Click, |_| { Url::go_back(1) })
                        ],
                        a![
                            C!["button"],
                            "Next",
                            ev(Ev::Click, |_| { Url::go_forward(1) })
                        ]
                    ]
                ]
            ]
        ]
    ]
}
