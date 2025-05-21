// Hide console window in Windows release builds. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod my_studio;

use config::Config;
use my_studio::HttpClient;
use slint::CloseRequestResponse;
use std::{cell::RefCell, path::Path, process::exit, rc::Rc};

slint::include_modules!();
use slint_generatedApp as ui;

#[tokio::main]
async fn main() {
    let config = Rc::new(RefCell::new(
        config::load(Path::new("config.toml")).unwrap_or_else(|e| {
            eprintln!("Error when loading config from 'config.toml': {e}");
            exit(1);
        }),
    ));

    let client = HttpClient::new(Rc::clone(&config));

    let ui = init(&config);
    ui.run().unwrap();
}

fn init(config: &Rc<RefCell<Config>>) -> App {
    let ui = App::new().unwrap();
    slint::set_xdg_app_id("youkoso").unwrap();

    ui.global::<Settings>().on_reset_all({
        let ui = ui.as_weak();
        move || {
            ui.upgrade()
                .unwrap()
                .global::<Settings>()
                .invoke_reset_theme();
            ui.upgrade()
                .unwrap()
                .global::<Settings>()
                .invoke_reset_my_studio_email();
            ui.upgrade()
                .unwrap()
                .global::<Settings>()
                .invoke_reset_my_studio_company_id();
        }
    });
    ui.global::<Settings>().on_changed_theme({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        move || {
            config.try_borrow_mut().unwrap().theme =
                match ui.upgrade().unwrap().global::<Settings>().get_theme() {
                    Theme::System => config::Theme::System,
                    Theme::Dark => config::Theme::Dark,
                    Theme::Light => config::Theme::Light,
                };
            ui.upgrade().unwrap().invoke_reload_theme();
        }
    });
    ui.global::<Settings>().on_reset_theme({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        move || {
            config.try_borrow_mut().unwrap().theme = Config::default().theme;
            ui.upgrade().unwrap().global::<Settings>().set_theme(
                match config.try_borrow().unwrap().theme {
                    config::Theme::System => ui::Theme::System,
                    config::Theme::Dark => ui::Theme::Dark,
                    config::Theme::Light => ui::Theme::Light,
                },
            );
            ui.upgrade().unwrap().invoke_reload_theme();
        }
    });
    ui.global::<Settings>().on_changed_my_studio_email({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        move || {
            config.try_borrow_mut().unwrap().my_studio.email = ui
                .upgrade()
                .unwrap()
                .global::<Settings>()
                .get_my_studio_email()
                .into();
        }
    });
    ui.global::<Settings>().on_reset_my_studio_email({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        move || {
            config.try_borrow_mut().unwrap().my_studio.email = Config::default().my_studio.email;
            ui.upgrade()
                .unwrap()
                .global::<Settings>()
                .set_my_studio_email(config.try_borrow().unwrap().my_studio.email.clone().into());
        }
    });
    ui.global::<Settings>().on_changed_my_studio_company_id({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        move || {
            config.try_borrow_mut().unwrap().my_studio.company_id = ui
                .upgrade()
                .unwrap()
                .global::<Settings>()
                .get_my_studio_company_id()
                .into();
        }
    });
    ui.global::<Settings>().on_reset_my_studio_company_id({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        move || {
            config.try_borrow_mut().unwrap().my_studio.company_id =
                Config::default().my_studio.company_id;
            ui.upgrade()
                .unwrap()
                .global::<Settings>()
                .set_my_studio_company_id(
                    config
                        .try_borrow()
                        .unwrap()
                        .my_studio
                        .company_id
                        .clone()
                        .into(),
                );
        }
    });

    ui.global::<Settings>()
        .set_theme(match config.try_borrow().unwrap().theme {
            config::Theme::System => ui::Theme::System,
            config::Theme::Dark => ui::Theme::Dark,
            config::Theme::Light => ui::Theme::Light,
        });
    ui.invoke_reload_theme();
    ui.global::<Settings>()
        .set_my_studio_email(config.try_borrow().unwrap().my_studio.email.clone().into());
    ui.global::<Settings>().set_my_studio_company_id(
        config
            .try_borrow()
            .unwrap()
            .my_studio
            .company_id
            .clone()
            .into(),
    );

    ui.window().on_close_requested({
        let config = Rc::clone(config);
        move || {
            config.try_borrow().unwrap().save().unwrap();
            CloseRequestResponse::HideWindow
        }
    });

    ui
}
