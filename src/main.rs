// Hide console window in Windows release builds. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod my_studio;

use std::{cell::RefCell, path::Path, process::exit, rc::Rc};

use config::Config;
use reqwest::Client;

slint::include_modules!();

use slint_generatedApp as ui;

#[tokio::main]
async fn main() {
    let config = Rc::new(RefCell::new(
        config::load(Path::new("config.toml")).unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            exit(1);
        }),
    ));

    let client = Client::new();

    let ui = init(&config);
    ui.run().unwrap();
}

fn init(config: &Rc<RefCell<Config>>) -> App {
    let ui = App::new().unwrap();
    slint::set_xdg_app_id("youkoso").unwrap();

    ui.global::<Settings>().on_update_settings({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        move || {
            config.borrow_mut().theme = match ui.unwrap().global::<Settings>().get_theme() {
                Theme::System => config::Theme::System,
                Theme::Dark => config::Theme::Dark,
                Theme::Light => config::Theme::Light,
            }
        }
    });

    ui.global::<Settings>()
        .set_theme(match config.try_borrow().unwrap().theme {
            config::Theme::System => ui::Theme::System,
            config::Theme::Dark => ui::Theme::Dark,
            config::Theme::Light => ui::Theme::Light,
        });
    ui.invoke_reload_theme();

    ui
}
