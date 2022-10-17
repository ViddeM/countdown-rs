#![forbid(unsafe_code)]

use std::{collections::BTreeMap, ops::Sub};

use chrono::{TimeZone, Utc};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_main])
        .mount("/api/public", FileServer::from("static/public"))
        .attach(Template::fairing())
}

#[get("/")]
async fn get_main() -> Template {
    let countdown_to = Utc.ymd(2022, 10, 25).and_hms(17, 0, 0);
    let now = Utc::now();
    let remaining = if now > countdown_to {
        String::from("VICKY 3 IS OUT!")
    } else {
        let diff = countdown_to.sub(now);
        format!(
            "Out in {}",
            if diff.num_days() > 0 {
                format!("{} days", diff.num_days())
            } else if diff.num_hours() > 0 {
                format!("{} hours", diff.num_hours())
            } else if diff.num_minutes() > 0 {
                format!("{} minutes", diff.num_minutes())
            } else {
                format!("{} seconds", diff.num_seconds())
            }
        )
    };

    let mut data: BTreeMap<&str, String> = BTreeMap::new();
    data.insert("title", String::from("Vicky 3!"));
    data.insert("time", remaining);
    Template::render("main", &data)
}
