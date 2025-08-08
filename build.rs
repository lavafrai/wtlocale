use slint_build::CompilerConfiguration;

fn main() {
    let slint_config = CompilerConfiguration::new()
        .with_style("native".to_owned());
    slint_build::compile_with_config("ui/wtlocale.slint", slint_config).unwrap();
}