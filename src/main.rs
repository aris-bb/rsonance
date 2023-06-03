#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// when compiling to native
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "rsonance",
        native_options,
        Box::new(|cc| Box::new(rsonance::App::new(cc))),
    )
}

// when compiling to WebAssembly
#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "rsonance",
            web_options,
            Box::new(|cc| Box::new(rsonance::App::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
