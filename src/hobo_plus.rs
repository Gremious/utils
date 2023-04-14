use hobo::prelude::*;
use futures::future::FutureExt;
pub use crate::__dbg;
pub use entity_ext::AsEntityExt;
pub use element_ext::{ChildrenDiff, ChildrenDiffConfig, ChildrenDiffConfigBuilder, AsElementExt, FontTag, Clicked};
pub use html_ext::{ AExt, Toggleable };

mod html_ext;
mod entity_ext;
mod element_ext;
pub mod file_select;

#[track_caller]
pub fn spawn_complain<T>(x: impl std::future::Future<Output = anyhow::Result<T>> + 'static) {
	let caller = std::panic::Location::caller();
	wasm_bindgen_futures::spawn_local(x.map(|res| if let Err(e) = res {
		let lvl = log::Level::Error;
		if lvl <= log::STATIC_MAX_LEVEL && lvl <= log::max_level() {
			log::__private_api_log(
				log::__log_format_args!("{:?}", e),
				lvl,
				&(log::__log_module_path!(), log::__log_module_path!(), caller.file(), caller.line()),
				log::__private_api::Option::None,
			);
		}
	}));
}

pub fn window() -> web_sys::Window { web_sys::window().expect("no window") }
pub fn document() -> web_sys::Document { window().document().expect("no document") }

fn closure_mut<T: wasm_bindgen::convert::FromWasmAbi + 'static> (closure: impl FnMut(T) + 'static) -> Closure<dyn FnMut(T)> {
	Closure::wrap(Box::new(closure) as Box<dyn FnMut(T) + 'static>)
}

pub fn animation(f: impl FnMut(f64) -> bool + 'static) {
	animation_with_window(window(), f);
}

// run a function every frame until it returns false
// fn argument is delta milliseconds
// skips the first frame immediately after because it's not possible to calculate time delta
pub fn animation_with_window(window: web_sys::Window, mut f: impl FnMut(f64) -> bool + 'static) {
	use std::{cell::RefCell, rc::Rc};

	// this weird refcelling is necessary for "recursion"
	let cb = Rc::new(RefCell::new(None as Option<Closure<dyn FnMut(f64) + 'static>>));
	let mut last_timestamp = None;
	*cb.borrow_mut() = Some(Closure::wrap(Box::new(hobo::enclose!((cb, window) move |timestamp| {
		if window.closed().unwrap_or(true) { let _drop = cb.borrow_mut().take(); return; }
		let last_timestamp = if let Some(x) = last_timestamp.as_mut() { x } else {
			window.request_animation_frame(cb.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
			last_timestamp = Some(timestamp);
			return;
		};
		let delta_t = timestamp - *last_timestamp;
		*last_timestamp = timestamp;
		if f(delta_t) {
			window.request_animation_frame(cb.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
		} else {
			let _drop = cb.borrow_mut().take();
		}
	})) as Box<dyn FnMut(f64) + 'static>));
	window.request_animation_frame(cb.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
}

// basically just copy of rust std's dbg!
#[macro_export]
macro_rules! __dbg {
	() => { log::info!("[{}:{}]", file!(), line!()); };
	($val:expr) => { match $val { tmp => { log::info!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!($val), &tmp); tmp } } };
	($val:expr,) => { $crate::dbg!($val) };
	($($val:expr),+ $(,)?) => { ($($crate::dbg!($val)),+,) };
}
