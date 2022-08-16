pub fn log(data: String) {
    web_sys::console::log_3(
        &"%cin Rust".to_string().into(),
        &"background:#faad14;border-radius: 0.5em;color: white;font-weight: bold;padding: 2px 0.5em"
            .to_string()
            .into(),
        &data.into(),
    )
}
