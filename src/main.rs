use parking_lot::{const_mutex, Mutex};
use rocket::{form::Form, get, launch, post, response::Redirect, routes, uri, FromForm};
use rocket_dyn_templates::Template;
use serde_json::json;

static BALANCE: Mutex<u64> = const_mutex(0);

#[get("/")]
fn index() -> Template {
    Template::render("index", json!({}))
}

#[derive(FromForm)]
struct StartingBalance {
    balance: u64,
}

#[post("/", data = "<form>")]
fn get_starting_balance(form: Form<StartingBalance>) -> Redirect {
    *BALANCE.lock() = form.balance;
    Redirect::to(uri!(play(display = false, won = false)))
}

#[get("/play?<display>&<won>")]
fn play(display: bool, won: bool) -> Template {
    let balance = BALANCE.lock();
    if *balance == 0 {
        Template::render("nomoney", json!({}))
    } else {
        Template::render(
            "play",
            json!({
                "balance": *balance,
                "won": won,
                "display": display
            }),
        )
    }
}

#[derive(FromForm)]
struct Bet {
    bet: u64,
}

#[post("/play", data = "<form>")]
fn play_post(form: Form<Bet>) -> Redirect {
    let mut balance = BALANCE.lock();
    if *balance < form.bet {
        Redirect::to(uri!(insufficient()))
    } else {
        let won = rand::random::<bool>();
        if won {
            *balance = balance.checked_add(form.bet).unwrap();
        } else {
            *balance -= form.bet;
        }
        Redirect::to(uri!(play(display = true, won = won)))
    }
}

#[get("/insufficient")]
fn insufficient() -> Template {
    let balance = BALANCE.lock();
    Template::render("insufficient", json!({"balance": *balance}))
}

#[post("/insufficient", data = "<form>")]
fn insufficient_post(form: Form<Bet>) -> Redirect {
    let mut balance = BALANCE.lock();
    if *balance < form.bet {
        Redirect::to(uri!(insufficient()))
    } else {
        let won = rand::random::<bool>();
        if won {
            *balance = balance.checked_add(form.bet).unwrap();
        } else {
            *balance -= form.bet;
        }
        Redirect::to(uri!(play(display = true, won = won)))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                get_starting_balance,
                play,
                play_post,
                insufficient,
                insufficient_post
            ],
        )
        .attach(Template::fairing())
}
