// Hide console window in Windows release builds. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod my_studio;

use std::{cell::RefCell, path::Path, process::exit, rc::Rc};

use config::Config;
use reqwest::Client;

slint::include_modules!();

use slint::{PhysicalPosition, PhysicalSize};
use slint_generatedAppUi as ui;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let config = Rc::new(RefCell::new(
        config::load(Path::new("config.toml")).unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            exit(1);
        }),
    ));

    let client = Client::new();

    let state = init(&config);

    state.app_ui.run()?;

    Ok(())
}

struct State {
    app_ui: AppUi,
    settings_ui: SettingsUi,
}

fn init(config: &Rc<RefCell<Config>>) -> State {
    let app_ui = AppUi::new().unwrap();
    let settings_ui = SettingsUi::new().unwrap();

    app_ui.on_open_settings({
        let app_ui = app_ui.as_weak();
        let settings_ui = settings_ui.as_weak();
        move || {
            // Only allow one instance of the settings menu at a time
            if settings_ui.unwrap().window().is_visible() {
                return;
            } else if settings_ui.unwrap().window().is_minimized() {
                settings_ui.unwrap().window().set_minimized(false);
                return;
            }

            // Center the settings window above the main app window.
            // Does not work on wayland as shown on slint docs
            // https://docs.slint.dev/latest/docs/rust/slint/struct.Window#method.set_position
            let PhysicalPosition {
                x: app_ui_x,
                y: app_ui_y,
            } = app_ui.unwrap().window().position();
            let PhysicalSize {
                width: app_ui_width,
                height: app_ui_height,
            } = app_ui.unwrap().window().size();
            let PhysicalSize {
                width: settings_ui_width,
                height: settings_ui_height,
            } = settings_ui.unwrap().window().size();
            settings_ui
                .unwrap()
                .window()
                .set_position(PhysicalPosition {
                    x: app_ui_x + (app_ui_width as i32) / 2 - (settings_ui_width as i32) / 2,
                    y: app_ui_y + (app_ui_height as i32) / 2 - (settings_ui_height as i32) / 2,
                });

            settings_ui.unwrap().show().unwrap();
        }
    });

    app_ui
        .global::<Settings>()
        .set_theme(match config.borrow().theme {
            config::Theme::System => ui::Theme::System,
            config::Theme::Dark => ui::Theme::Dark,
            config::Theme::Light => ui::Theme::Light,
        });
    app_ui.invoke_reload_theme();

    settings_ui.on_update_settings({
        let settings_ui = settings_ui.as_weak();
        let config = Rc::clone(config);
        move || {
            config.borrow_mut().theme = match settings_ui.unwrap().global::<Settings>().get_theme()
            {
                Theme::System => config::Theme::System,
                Theme::Dark => config::Theme::Dark,
                Theme::Light => config::Theme::Light,
            }
        }
    });

    State {
        app_ui,
        settings_ui,
    }
}
