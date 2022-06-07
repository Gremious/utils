//# [dependencies]
//# palette = "0.6"
//# css-color-parser = "0.1"
//# clipboard = "0.5"

// this is a cargo-play script
// takes css from your clipboard and fixes it up and puts it back into your clipboard
// don't know if we should use rust-script instead?

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn main() {
	let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
	let clipboard_content = clipboard.get_contents().unwrap();
	let mut new_clipboard_content = String::new();

	for line in clipboard_content.lines() {
		if line.starts_with("css::") {
			new_clipboard_content.push_str(&format!("{}\n", line));
			continue;
		}
		let line = line.trim();
		for prop in line.split(';') {
			let prop = prop.trim();
			if prop.is_empty() { continue; }
			let mut s = prop.split(':');
			let property_name = s.next().unwrap();
			let property_name = property_name.trim().replace('-', "_");
			let value = s.next().unwrap().trim();
			let value_res = if let Some(color) = value.parse::<css_color_parser::Color>().ok() {
				format!("0x{:02X}_{:02X}_{:02X}_{:02X}", color.r, color.g, color.b, (color.a * 255.) as u8)
			} else {
				value.replace("px", " px")
			};
			new_clipboard_content.push_str(&format!("css::{}!({}),\n", property_name, value_res));
		}
	}

	clipboard.set_contents(new_clipboard_content).unwrap();
}
