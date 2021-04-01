use chrono::Local;
use env_logger::Builder;
use std::io::Write;

pub fn init_logging() {
    Builder::from_env("RUST_LOG")
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}]: {}",
                Local::now().format("%H:%M:%S.%3f"),
                record.level(),
                record.args()
            )
        })
        .init();
}
