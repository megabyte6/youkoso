use slint_build::{CompilerConfiguration, compile_with_config};

fn main() {
    let config = CompilerConfiguration::new()
        .with_style(option_env!("SLINT_STYLE").unwrap_or("native").to_owned());
    compile_with_config("ui/app.slint", config).expect("Slint build failed");
}
