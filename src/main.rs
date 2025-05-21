// Hide console window in Windows release builds. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod database;
mod my_studio;

use config::Config;
use my_studio::HttpClient;
use slint::{CloseRequestResponse, ToSharedString};
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    process::exit,
    rc::Rc,
};

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

    let ui = init_ui(&config);
    ui.run().unwrap();
}

fn init_ui(config: &Rc<RefCell<Config>>) -> App {
    let ui = App::new().unwrap();
    slint::set_xdg_app_id("youkoso").unwrap();
    implement_ui_callbacks(&ui, config);
    load_config_to_ui(&ui, config);

    ui.window().on_close_requested({
        let config = Rc::clone(config);
        move || {
            config.try_borrow().unwrap().save().unwrap();
            CloseRequestResponse::HideWindow
        }
    });

    ui
}

fn implement_ui_callbacks(ui: &App, config: &Rc<RefCell<Config>>) {
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
        let default_value = Config::default().theme;
        move || {
            config.try_borrow_mut().unwrap().theme = default_value.clone();
            ui.upgrade()
                .unwrap()
                .global::<Settings>()
                .set_theme(match default_value {
                    config::Theme::System => ui::Theme::System,
                    config::Theme::Dark => ui::Theme::Dark,
                    config::Theme::Light => ui::Theme::Light,
                });
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
        let default_value = Config::default().my_studio.email;
        move || {
            config.try_borrow_mut().unwrap().my_studio.email = default_value.clone();
            ui.upgrade()
                .unwrap()
                .global::<Settings>()
                .set_my_studio_email(default_value.to_shared_string());
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
        let default_value = Config::default().my_studio.company_id;
        move || {
            config.try_borrow_mut().unwrap().my_studio.company_id = default_value.clone();
            ui.upgrade()
                .unwrap()
                .global::<Settings>()
                .set_my_studio_company_id(default_value.to_shared_string());
        }
    });

    ui.global::<Settings>().on_changed_student_data_filepath({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        move || {
            config.try_borrow_mut().unwrap().student_data.filepath = PathBuf::from(
                ui.upgrade()
                    .unwrap()
                    .global::<Settings>()
                    .get_student_data_filepath()
                    .to_string(),
            );
        }
    });
    ui.global::<Settings>().on_reset_student_data_filepath({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        let default_value = Config::default().student_data.filepath;
        move || {
            config.try_borrow_mut().unwrap().student_data.filepath = default_value.clone();
            ui.upgrade()
                .unwrap()
                .global::<Settings>()
                .set_student_data_filepath(default_value.display().to_shared_string());
        }
    });

    ui.global::<Settings>().on_changed_student_data_sheet_name({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        move || {
            config.try_borrow_mut().unwrap().student_data.sheet_name = ui
                .upgrade()
                .unwrap()
                .global::<Settings>()
                .get_student_data_sheet_name()
                .into();
        }
    });
    ui.global::<Settings>().on_reset_student_data_sheet_name({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        let default_value = Config::default().student_data.sheet_name;
        move || {
            config.try_borrow_mut().unwrap().student_data.sheet_name = default_value.clone();
            ui.upgrade()
                .unwrap()
                .global::<Settings>()
                .set_student_data_sheet_name(default_value.to_shared_string());
        }
    });

    ui.global::<Settings>()
        .on_changed_student_data_name_column({
            let ui = ui.as_weak();
            let config = Rc::clone(config);
            move || {
                config.try_borrow_mut().unwrap().student_data.name_column = ui
                    .upgrade()
                    .unwrap()
                    .global::<Settings>()
                    .get_student_data_name_column()
                    .try_into()
                    .unwrap();
            }
        });
    ui.global::<Settings>().on_reset_student_data_name_column({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        let default_value = Config::default().student_data.name_column;
        move || {
            config.try_borrow_mut().unwrap().student_data.name_column = default_value;
            ui.upgrade()
                .unwrap()
                .global::<Settings>()
                .set_student_data_name_column(default_value.into());
        }
    });

    ui.global::<Settings>().on_changed_student_data_id_column({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        move || {
            config.try_borrow_mut().unwrap().student_data.id_column = ui
                .upgrade()
                .unwrap()
                .global::<Settings>()
                .get_student_data_id_column()
                .try_into()
                .unwrap();
        }
    });
    ui.global::<Settings>().on_reset_student_data_id_column({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        let default_value = Config::default().student_data.id_column;
        move || {
            config.try_borrow_mut().unwrap().student_data.id_column = default_value;
            ui.upgrade()
                .unwrap()
                .global::<Settings>()
                .set_student_data_id_column(default_value.into());
        }
    });

    ui.global::<Settings>()
        .on_changed_student_data_immediate_sign_in_column({
            let ui = ui.as_weak();
            let config = Rc::clone(config);
            move || {
                config
                    .try_borrow_mut()
                    .unwrap()
                    .student_data
                    .immediate_sign_in
                    .column = ui
                    .upgrade()
                    .unwrap()
                    .global::<Settings>()
                    .get_student_data_immediate_sign_in_column()
                    .try_into()
                    .unwrap();
            }
        });
    ui.global::<Settings>()
        .on_reset_student_data_immediate_sign_in_column({
            let ui = ui.as_weak();
            let config = Rc::clone(config);
            let default_value = Config::default().student_data.immediate_sign_in.column;
            move || {
                config
                    .try_borrow_mut()
                    .unwrap()
                    .student_data
                    .immediate_sign_in
                    .column = default_value;
                ui.upgrade()
                    .unwrap()
                    .global::<Settings>()
                    .set_student_data_immediate_sign_in_column(default_value.into());
            }
        });

    ui.global::<Settings>()
        .on_changed_student_data_immediate_sign_in_enabled_symbol({
            let ui = ui.as_weak();
            let config = Rc::clone(config);
            move || {
                config
                    .try_borrow_mut()
                    .unwrap()
                    .student_data
                    .immediate_sign_in
                    .enabled_symbol = ui
                    .upgrade()
                    .unwrap()
                    .global::<Settings>()
                    .get_student_data_immediate_sign_in_enabled_symbol()
                    .into();
            }
        });
    ui.global::<Settings>()
        .on_reset_student_data_immediate_sign_in_enabled_symbol({
            let ui = ui.as_weak();
            let config = Rc::clone(config);
            let default_value = Config::default()
                .student_data
                .immediate_sign_in
                .enabled_symbol;
            move || {
                config
                    .try_borrow_mut()
                    .unwrap()
                    .student_data
                    .immediate_sign_in
                    .enabled_symbol = default_value.clone();
                ui.upgrade()
                    .unwrap()
                    .global::<Settings>()
                    .set_student_data_immediate_sign_in_enabled_symbol(
                        default_value.to_shared_string(),
                    );
            }
        });
}

fn load_config_to_ui(ui: &App, config: &Rc<RefCell<Config>>) {
    ui.global::<Settings>()
        .set_theme(match config.try_borrow().unwrap().theme {
            config::Theme::System => ui::Theme::System,
            config::Theme::Dark => ui::Theme::Dark,
            config::Theme::Light => ui::Theme::Light,
        });
    ui.invoke_reload_theme();

    ui.global::<Settings>().set_my_studio_email(
        config
            .try_borrow()
            .unwrap()
            .my_studio
            .email
            .to_shared_string(),
    );

    ui.global::<Settings>().set_my_studio_company_id(
        config
            .try_borrow()
            .unwrap()
            .my_studio
            .company_id
            .to_shared_string(),
    );

    ui.global::<Settings>().set_student_data_filepath(
        config
            .try_borrow()
            .unwrap()
            .student_data
            .filepath
            .display()
            .to_shared_string(),
    );

    ui.global::<Settings>().set_student_data_sheet_name(
        config
            .try_borrow()
            .unwrap()
            .student_data
            .sheet_name
            .to_shared_string(),
    );

    ui.global::<Settings>()
        .set_student_data_name_column(config.try_borrow().unwrap().student_data.name_column.into());

    ui.global::<Settings>()
        .set_student_data_id_column(config.try_borrow().unwrap().student_data.id_column.into());

    ui.global::<Settings>()
        .set_student_data_immediate_sign_in_column(
            config
                .try_borrow()
                .unwrap()
                .student_data
                .immediate_sign_in
                .column
                .into(),
        );

    ui.global::<Settings>()
        .set_student_data_immediate_sign_in_enabled_symbol(
            config
                .try_borrow()
                .unwrap()
                .student_data
                .immediate_sign_in
                .enabled_symbol
                .to_shared_string(),
        );
}
