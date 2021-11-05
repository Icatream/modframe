// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
//#![allow(clippy::wildcard_imports)]
mod component;
mod page;
mod route;

use seed::{prelude::*, *};
use route::route::Route;
// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let route = Route::from(&url);
    orders.subscribe(|url_changed: subs::UrlChanged| Msg::UrlChanged(url_changed.0))
        .notify(subs::UrlChanged(url));
    Model {
        route: route,
        navbar: navbar::Model::default()
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
#[derive(Default)]
struct Model {
    route: Route,
    navbar: navbar::Model,
}
// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
//#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Navbar(navbar::Msg),
    UrlChanged(Url)
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Navbar(msg) => navbar::update(msg, &mut model.navbar, orders),
        Msg::UrlChanged(url) => log!("Url Changed", url)
    }
}

// ------ ------
//     View
// ------ ------
use component::navbar;

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![navbar::view(&model.navbar).map_msg(Msg::Navbar)]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[warn(dead_code)]
#[warn(unused_imports)]
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
