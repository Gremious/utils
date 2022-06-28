pub fn setup(crate_name: &str) {
	env_logger::builder()
		.filter(None, log::LevelFilter::Info)
		.filter(Some(crate_name), #[cfg(debug_assertions)] log::LevelFilter::Trace, #[cfg(not(debug_assertions))] log::LevelFilter::Debug)
		.format(|buf, record| {
			use std::io::Write;

			let mut dimmed = buf.style();
			dimmed.set_color(env_logger::fmt::Color::Rgb(126, 126, 126));

			let mut level_style = buf.style();
			match record.level() {
				log::Level::Trace => &mut dimmed,
				log::Level::Warn => level_style.set_color(env_logger::fmt::Color::Yellow),
				log::Level::Error => level_style.set_color(env_logger::fmt::Color::Red),
				_ => &mut level_style,
			};

			writeln!(buf, "[{time} {level}  {module} {file}:{line}] {args}",
				time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
				level = level_style.value(record.level()),
				module = record.module_path().unwrap_or("module?"),
				file = dimmed.value(record.file().unwrap_or("file?")),
				line = dimmed.value(record.line().unwrap_or(0)),
				args = record.args(),
			)
		})
		.init();
}
