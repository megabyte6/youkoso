// Hide console window in Windows release builds. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod my_studio;
mod scheduler;
mod spreadsheet;
mod ui;

use std::{cell::RefCell, path::Path, process::exit, rc::Rc, time::Duration};

use slint::ComponentHandle;
use tokio::runtime::Runtime;

use crate::my_studio::HttpClient;
use crate::scheduler::{Config as SchedulerConfig, Scheduler};
use crate::spreadsheet::load_student_info_from_xlsx;

fn main() {
    let runtime = Runtime::new().unwrap();
    let mut _scheduler = Scheduler::new(
        runtime,
        SchedulerConfig {
            max_poll_interval: Duration::from_secs(1),
            ..Default::default()
        },
    );

    let config = Rc::new(RefCell::new(
        config::load(Path::new("config.toml")).unwrap_or_else(|e| {
            eprintln!("Error when loading config from 'config.toml': {e}");
            exit(1);
        }),
    ));

    let _database = load_student_info_from_xlsx(&config.try_borrow().unwrap()).unwrap();

    let _client = HttpClient::new(Rc::clone(&config));

    let ui = ui::init(&config);
    ui.run().unwrap();
}
