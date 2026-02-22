use std::{cell::RefCell, rc::Rc, time::Duration};

use slint::{CloseRequestResponse, SharedString, Timer, ToSharedString, Weak};
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
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        move || {
            let strong_ui = ui.upgrade().unwrap();
            let settings = strong_ui.global::<Settings>();
            if settings.get_syncing() {
                save_to_config(&settings, &mut config.try_borrow_mut().unwrap());
                settings.set_syncing(false);
            }
            config.try_borrow().unwrap().save().unwrap();

            CloseRequestResponse::HideWindow
        }
    });

    ui
}

fn impl_home_page_callbacks(ui: &App, config: &Rc<RefCell<Config>>) {}

fn impl_settings_page_callbacks(ui: &App, config: &Rc<RefCell<Config>>) {
    let settings = ui.global::<Settings>();

    settings.on_sync_settings({
        let ui = ui.as_weak();
        let config = Rc::clone(config);
        move || {
            // check if there is a sync queued up
            let strong_ui = ui.upgrade().unwrap();
            let settings = strong_ui.global::<Settings>();
            if settings.get_syncing() {
                return;
            }

            // queue up a sync
            settings.set_syncing(true);
            let ui = Weak::clone(&ui);
            let config = Rc::clone(&config);
            // prevent each keystroke causing full saves by only saving every 5 seconds if there
            // was a change
            Timer::single_shot(Duration::from_secs(5), move || {
                let mut config = config.try_borrow_mut().unwrap();
                // a strong reference to the ui
                let strong_ui = ui.upgrade().unwrap();
                let settings = strong_ui.global::<Settings>();
                save_to_config(&settings, &mut config);

                settings.set_syncing(false);
            });
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
                "my-studio.email" => {
                    let mut updated = settings.get_my_studio();
                    updated.email = Config::default().my_studio.email.into();
                    settings.set_my_studio(updated);
                }
                "my-studio.company-id" => {
                    let mut updated = settings.get_my_studio();
                    updated.company_id = Config::default().my_studio.company_id.into();
                    settings.set_my_studio(updated);
                }
                "student-data.filepath" => {
                    let mut updated = settings.get_student_data();
                    updated.filepath = Config::default()
                        .student_data
                        .filepath
                        .display()
                        .to_shared_string();
                    settings.set_student_data(updated);
                }
                "student-data.sheet-name" => {
                    let mut updated = settings.get_student_data();
                    updated.sheet_name = Config::default().student_data.sheet_name.into();
                    settings.set_student_data(updated);
                }
                "student-data.name-column" => {
                    let mut updated = settings.get_student_data();
                    updated.name_column = Config::default().student_data.name_column.into();
                    settings.set_student_data(updated);
                }
                "student-data.id-column" => {
                    let mut updated = settings.get_student_data();
                    updated.id_column = Config::default().student_data.id_column.into();
                    settings.set_student_data(updated);
                }
                "student-data.immediate-sign-in-column" => {
                    let mut updated = settings.get_student_data();
                    updated.immediate_sign_in_column = Config::default()
                        .student_data
                        .immediate_sign_in
                        .column
                        .into();
                    settings.set_student_data(updated);
                }
                "student-data.immediate-sign-in-enabled-symbol" => {
                    let mut updated = settings.get_student_data();
                    updated.immediate_sign_in_enabled_symbol = Config::default()
                        .student_data
                        .immediate_sign_in
                        .enabled_symbol
                        .into();
                    settings.set_student_data(updated);
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
            settings.invoke_reset("my-studio.email".to_shared_string());
            settings.invoke_reset("my-studio.company-id".to_shared_string());
            settings.invoke_reset("student-data.filepath".to_shared_string());
            settings.invoke_reset("student-data.sheet-name".to_shared_string());
            settings.invoke_reset("student-data.name-column".to_shared_string());
            settings.invoke_reset("student-data.id-column".to_shared_string());
            settings.invoke_reset("student-data.immediate-sign-in-column".to_shared_string());
            settings
                .invoke_reset("student-data.immediate-sign-in-enabled-symbol".to_shared_string());
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

    settings.set_my_studio(MyStudio {
        email: config.my_studio.email.clone().into(),
        company_id: config.my_studio.company_id.clone().into(),
    });

    settings.set_student_data(StudentData {
        filepath: config.student_data.filepath.display().to_shared_string(),
        sheet_name: config.student_data.sheet_name.clone().into(),
        name_column: config.student_data.name_column.into(),
        id_column: config.student_data.id_column.into(),
        immediate_sign_in_column: config.student_data.immediate_sign_in.column.into(),
        immediate_sign_in_enabled_symbol: config
            .student_data
            .immediate_sign_in
            .enabled_symbol
            .clone()
            .into(),
    });
}

fn save_to_config(settings: &Settings, config: &mut Config) {
    config.theme = match settings.get_theme() {
        Theme::System => config::Theme::System,
        Theme::Dark => config::Theme::Dark,
        Theme::Light => config::Theme::Light,
    };

    config.my_studio.email = settings.get_my_studio().email.into();
    config.my_studio.company_id = settings.get_my_studio().company_id.into();

    config.student_data.filepath = settings.get_student_data().filepath.to_string().into();
    config.student_data.sheet_name = settings.get_student_data().sheet_name.into();
    config.student_data.name_column = settings.get_student_data().name_column.try_into().unwrap();
    config.student_data.id_column = settings.get_student_data().id_column.try_into().unwrap();
    config.student_data.immediate_sign_in.column = settings
        .get_student_data()
        .immediate_sign_in_column
        .try_into()
        .unwrap();
    config.student_data.immediate_sign_in.enabled_symbol = settings
        .get_student_data()
        .immediate_sign_in_enabled_symbol
        .into();
}
