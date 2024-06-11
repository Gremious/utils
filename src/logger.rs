// log levels should be configured via RUST_LOG env var
// smth like `RUST_LOG=info,my_crate=trace` where `info` is the level for all targets
// and `my_crate=trace` is the level for just `my_crate`
// and we can also do submodules like `RUST_LOG=trace,my_crate::foo=info`
pub fn setup() {
	env_logger::Builder::from_env(env_logger::Env::default())
		.format(|buf, record| {
			use std::io::Write;

			let dimmed = anstyle::Style::new()
				.fg_color(Some(anstyle::Color::Rgb(anstyle::RgbColor(126, 126, 126))));
			let dimmed_reset = dimmed.render_reset();

			let level_style = anstyle::Style::new()
				.fg_color(Some(match record.level() {
					log::Level::Trace => anstyle::Color::Rgb(anstyle::RgbColor(169, 169, 169)),
					log::Level::Debug => anstyle::Color::Ansi(anstyle::AnsiColor::Cyan),
					log::Level::Info => anstyle::Color::Ansi(anstyle::AnsiColor::Green),
					log::Level::Warn => anstyle::Color::Ansi(anstyle::AnsiColor::Yellow),
					log::Level::Error => anstyle::Color::Ansi(anstyle::AnsiColor::Red),
				}));
			let level_style_reset = level_style.render_reset();

			let level = match record.level() {
				log::Level::Trace => "TRACE",
				log::Level::Debug => "DEBUG",
				log::Level::Info  => "INFO ",
				log::Level::Warn  => "WARN ",
				log::Level::Error => "ERROR",
			};

			let magenta = anstyle::Style::new()
				.fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Magenta)));
			let magenta_reset = magenta.render_reset();

			writeln!(buf, "[{dimmed}{time}{dimmed_reset} {level_style}{level}{level_style_reset} {magenta}{module}{magenta_reset}] {args} {dimmed}@{dimmed_reset} {dimmed}{file}{dimmed_reset}:{dimmed}{line}{dimmed_reset}",
				time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
				module = record.module_path().unwrap_or("module?"),
				file = record.file().unwrap_or("file?"),
				line = record.line().unwrap_or(0),
				args = record.args(),
			)
		})
		.init();
}

#[extend::ext(pub)]
impl anyhow::Result<()> { fn log_error(self) { if let Err(e) = self { log::error!("{e}"); } } }
