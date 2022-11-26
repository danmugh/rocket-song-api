#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

use std::collections::{LinkedList};
use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

static SONG_QUEUE: Mutex<LinkedList<String>> = Mutex::new(LinkedList::new());

fn acquire_queue<'a>() -> MutexGuard<'a, LinkedList<String>> {
    SONG_QUEUE
        .lock()
        .expect("Unable to acquire lock on song queue because the Mutex was poisoned")
}

#[post("/add/<song_name>")]
fn add_song(song_name: String) -> String {
    let mut lock = acquire_queue();

    if lock.is_empty() {
        thread::spawn(remove_song_timer);
    }

    lock.push_back(song_name);

    format!("Song added. This song is in position {}.", lock.len())
}

#[get("/view")]
fn view() -> String {
    format!("{:?}", acquire_queue())
}

fn remove_song_timer() {
    while !acquire_queue().is_empty() {
        thread::sleep(Duration::from_secs(60));
        acquire_queue().pop_front();
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    Rocket::build()
        .mount("/", routes![add_song, view])
}