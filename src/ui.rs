use std::{cell::RefCell, rc::Rc};

use slint::{CloseRequestResponse, SharedString, ToSharedString};
use slint_generatedApp as SlintGenerated;

use crate::config::{self, Config};

slint::include_modules!();

pub fn init(config: &Rc<RefCell<Config>>) -> App {
    let ui = App::new().unwrap();
    slint::set_xdg_app_id("youkoso").unwrap();
    impl_home_page_callbacks(&ui, config);
    impl_settings_page_callbacks(&ui, config);
    load_config(&ui, &config.try_borrow().unwrap());

    ui.window().on_close_requested({
        let config = Rc::clone(config);
        move || {
            config.try_borrow().unwrap().save().unwrap();
            CloseRequestResponse::HideWindow
        }
    });

    ui
}

fn impl_home_page_callbacks(ui: &App, config: &Rc<RefCell<Config>>) {}

fn impl_settings_page_callbacks(ui: &App, config: &Rc<RefCell<Config>>) {
    let settings = ui.global::<Settings>();

    settings.on_save({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        move |id: SharedString| {
            let mut config = config.try_borrow_mut().unwrap();
            // a strong reference to the ui
            let strong_ui = ui.upgrade().unwrap();
            let settings = strong_ui.global::<Settings>();
            match id.as_str() {
                "theme" => {
                    config.theme = match settings.get_theme() {
                        Theme::System => config::Theme::System,
                        Theme::Dark => config::Theme::Dark,
                        Theme::Light => config::Theme::Light,
                    };
                    strong_ui.invoke_reload_theme();
                }
                "my-studio-email" => config.my_studio.email = settings.get_my_studio_email().into(),
                "my-studio-company-id" => {
                    config.my_studio.company_id = settings.get_my_studio_company_id().into()
                }
                "student-data-filepath" => {
                    config.student_data.filepath =
                        settings.get_student_data_filepath().to_string().into()
                }
                "student-data-sheet-name" => {
                    config.student_data.sheet_name = settings.get_student_data_sheet_name().into()
                }
                "student-data-name-column" => {
                    config.student_data.name_column =
                        settings.get_student_data_name_column().try_into().unwrap()
                }
                "student-data-id-column" => {
                    config.student_data.id_column =
                        settings.get_student_data_id_column().try_into().unwrap()
                }
                "student-data-immediate-sign-in-column" => {
                    config.student_data.immediate_sign_in.column = settings
                        .get_student_data_immediate_sign_in_column()
                        .try_into()
                        .unwrap()
                }
                "student-data-immediate-sign-in-enabled-symbol" => {
                    config.student_data.immediate_sign_in.enabled_symbol = settings
                        .get_student_data_immediate_sign_in_enabled_symbol()
                        .into()
                }
                _ => eprintln!("unknown settings identifier"),
            }
        }
    });

    settings.on_reset({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        move |id: SharedString| {
            let mut config = config.try_borrow_mut().unwrap();
            // a strong reference to the ui
            let strong_ui = ui.upgrade().unwrap();
            let settings = strong_ui.global::<Settings>();
            match id.as_str() {
                "theme" => {
                    config.theme = Config::default().theme.clone();
                    settings.set_theme(match config.theme {
                        config::Theme::System => SlintGenerated::Theme::System,
                        config::Theme::Dark => SlintGenerated::Theme::Dark,
                        config::Theme::Light => SlintGenerated::Theme::Light,
                    });
                    strong_ui.invoke_reload_theme();
                }
                "my-studio-email" => {
                    config.my_studio.email = Config::default().my_studio.email;
                    settings.set_my_studio_email(config.my_studio.email.to_shared_string());
                }
                "my-studio-company-id" => {
                    config.my_studio.company_id = Config::default().my_studio.company_id;
                    settings
                        .set_my_studio_company_id(config.my_studio.company_id.to_shared_string());
                }
                "student-data-filepath" => {
                    config.student_data.filepath = Config::default().student_data.filepath;
                    settings.set_student_data_filepath(
                        config.student_data.filepath.display().to_shared_string(),
                    );
                }
                "student-data-sheet-name" => {
                    config.student_data.sheet_name = Config::default().student_data.sheet_name;
                    settings.set_student_data_sheet_name(
                        config.student_data.sheet_name.to_shared_string(),
                    );
                }
                "student-data-name-column" => {
                    config.student_data.name_column = Config::default().student_data.name_column;
                    settings.set_student_data_name_column(config.student_data.name_column.into());
                }
                "student-data-id-column" => {
                    config.student_data.id_column = Config::default().student_data.id_column;
                    settings.set_student_data_id_column(config.student_data.id_column.into());
                }
                "student-data-immediate-sign-in-column" => {
                    config.student_data.immediate_sign_in.column =
                        Config::default().student_data.immediate_sign_in.column;
                    settings.set_student_data_immediate_sign_in_column(
                        config.student_data.immediate_sign_in.column.into(),
                    );
                }
                "student-data-immediate-sign-in-enabled-symbol" => {
                    config.student_data.immediate_sign_in.enabled_symbol = Config::default()
                        .student_data
                        .immediate_sign_in
                        .enabled_symbol;
                    settings.set_student_data_immediate_sign_in_enabled_symbol(
                        config
                            .student_data
                            .immediate_sign_in
                            .enabled_symbol
                            .to_shared_string(),
                    );
                }
                _ => eprintln!("unknown settings identifier"),
            }
        }
    });

    settings.on_reset_all({
        let ui = ui.as_weak();
        move || {
            // a strong reference to the ui
            let strong_ui = ui.upgrade().unwrap();
            let settings = strong_ui.global::<Settings>();
            settings.invoke_reset("theme".to_shared_string());
            settings.invoke_reset("my-studio-email".to_shared_string());
            settings.invoke_reset("my-studio-company-id".to_shared_string());
            settings.invoke_reset("student-data-filepath".to_shared_string());
            settings.invoke_reset("student-data-sheet-name".to_shared_string());
            settings.invoke_reset("student-data-name-column".to_shared_string());
            settings.invoke_reset("student-data-id-column".to_shared_string());
            settings.invoke_reset("student-data-immediate-sign-in-column".to_shared_string());
            settings
                .invoke_reset("student-data-immediate-sign-in-enabled-symbol".to_shared_string());
        }
    });
}

fn load_config(ui: &App, config: &Config) {
    let settings = ui.global::<Settings>();

    settings.set_theme(match config.theme {
        config::Theme::System => SlintGenerated::Theme::System,
        config::Theme::Dark => SlintGenerated::Theme::Dark,
        config::Theme::Light => SlintGenerated::Theme::Light,
    });
    ui.invoke_reload_theme();

    settings.set_my_studio_email(config.my_studio.email.to_shared_string());
    settings.set_my_studio_company_id(config.my_studio.company_id.to_shared_string());

    settings.set_student_data_filepath(config.student_data.filepath.display().to_shared_string());
    settings.set_student_data_sheet_name(config.student_data.sheet_name.to_shared_string());
    settings.set_student_data_name_column(config.student_data.name_column.into());
    settings.set_student_data_id_column(config.student_data.id_column.into());
    settings.set_student_data_immediate_sign_in_column(
        config.student_data.immediate_sign_in.column.into(),
    );
    settings.set_student_data_immediate_sign_in_enabled_symbol(
        config
            .student_data
            .immediate_sign_in
            .enabled_symbol
            .to_shared_string(),
    );
}
