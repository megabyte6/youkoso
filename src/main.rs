// Hide console window in Windows release builds. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod my_studio;

use std::{cell::RefCell, path::Path, process::exit, rc::Rc};

use config::Config;
use reqwest::Client;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let config = Rc::new(RefCell::new(
        config::load(Path::new("config.toml")).unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            exit(1);
        }),
    ));

    let client = Client::new();

    let app_window = Rc::new(AppWindow::new()?);
    app_window.impl_update_settings(&config);
    app_window.load_theme_from_config(&config.borrow());
    app_window.run()?;

    Ok(())
}

impl AppWindow {
    fn load_theme_from_config(&self, config: &Config) {
        self.set_theme(match config.theme {
            config::Theme::System => slint_generatedAppWindow::Theme::System,
            config::Theme::Dark => slint_generatedAppWindow::Theme::Dark,
            config::Theme::Light => slint_generatedAppWindow::Theme::Light,
        });
        self.invoke_reload_theme();
    }
}

trait AppWindowCallback {
    fn impl_update_settings(&self, config: &Rc<RefCell<Config>>);
}

impl AppWindowCallback for Rc<AppWindow> {
    fn impl_update_settings(&self, config: &Rc<RefCell<Config>>) {
        let app_window = Rc::clone(self);
        let config = Rc::clone(config);

        self.on_update_settings(move || {
            config.borrow_mut().theme = match app_window.get_theme() {
                Theme::System => config::Theme::System,
                Theme::Dark => config::Theme::Dark,
                Theme::Light => config::Theme::Light,
            }
        });
    }
}
