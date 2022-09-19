use std::sync::Once;

/// This optional function can be called once to show more more helpful errors on panic as
/// well as to provide standard logging output to the console in general.
///
/// Any subsequent calls to this function after the first call do nothing.
pub fn init_once() {
    static INITIALIZE_LOGGING: Once = Once::new();
    INITIALIZE_LOGGING.call_once(|| {
        wasm_logger::init(wasm_logger::Config::default());
        console_error_panic_hook::set_once();
    })
}
