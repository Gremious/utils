// log levels should be configured via RUST_LOG env var
// smth like `RUST_LOG=info,my_crate=trace` where `info` is the level for all targets
// and `my_crate=trace` is the level for just `my_crate`
// and we can also do submodules like `RUST_LOG=trace,my_crate::foo=info`
#[allow(unused_variables)]
pub fn setup(crate_name: &str) {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(
		#[cfg(debug_assertions)] format!("info,{crate_name}=debug"),
		#[cfg(not(debug_assertions))] "info",
	))
		.format(|buf, record| {
			use std::io::Write;

			let mut dimmed = buf.style();
			dimmed.set_color(env_logger::fmt::Color::Rgb(126, 126, 126));

			let mut level_style = buf.style();
			match record.level() {
				log::Level::Trace => level_style.set_color(env_logger::fmt::Color::Rgb(169, 169, 169)),
				log::Level::Debug => level_style.set_color(env_logger::fmt::Color::Cyan),
				log::Level::Info => level_style.set_color(env_logger::fmt::Color::Green),
				log::Level::Warn => level_style.set_color(env_logger::fmt::Color::Yellow),
				log::Level::Error => level_style.set_color(env_logger::fmt::Color::Red),
			};

			let level = match record.level() {
				log::Level::Trace => "TRACE",
				log::Level::Debug => "DEBUG",
				log::Level::Info  => "INFO ",
				log::Level::Warn  => "WARN ",
				log::Level::Error => "ERROR",
			};

			let mut magenta = buf.style();
			magenta.set_color(env_logger::fmt::Color::Magenta);

			writeln!(buf, "[{time} {level} {module}] {args} {at} {file}:{line}",
				time = dimmed.value(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f")),
				level = level_style.value(level),
				module = magenta.value(record.module_path().unwrap_or("module?")),
				at = dimmed.value("@"),
				file = dimmed.value(record.file().unwrap_or("file?")),
				line = dimmed.value(record.line().unwrap_or(0)),
				args = record.args(),
			)
		})
		.init();
}
