use hobo::prelude::*;
pub use crate::__dbg;

#[track_caller]
pub fn spawn_complain<T>(x: impl std::future::Future<Output = anyhow::Result<T>> + 'static) {
	let caller = std::panic::Location::caller();
	wasm_bindgen_futures::spawn_local(async move { if let Err(e) = x.await { log::error!("{} {:?}", caller, e); } });
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct FontTag;

pub fn window() -> web_sys::Window { web_sys::window().expect("no window") }
pub fn document() -> web_sys::Document { window().document().expect("no document") }

/// Generic `bool` component for on-click state-switch-like events
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Flipped(pub bool);

/// Allows you to tell whether it is currently being clicked on (mousedown active).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Clicked(pub bool);

pub trait EleExt: Element {

	// Another very common operation
	// Removes 1 level of nested closures over with_component
	// .with_component(|&element| FOO.subscribe(move |state| { ... -> .with_subscription(&FOO, move |&element, state| ...
	fn with_subscription<T: 'static>(self, state: &state::State<T>, mut f: impl FnMut(&Self, &T) + 'static) -> Self where Self: Sized + Copy + 'static {
		self.add_component(state.subscribe(move |state_val| f(&self, state_val)));
		self
	}

	/// Adds an `data-name` attribute to the element with a value of T
	fn name_typed<T: 'static>(self) -> Self {
		if self.is_dead() { log::warn!("mark dead {:?}", self.as_entity()); return self; }
		let name = core::any::type_name::<T>();
		let name = name.rsplit_once(':').map_or(name, |s| s.1);
		self.attr("data-name", name)
	}

	fn mark_and_name<T: 'static>(self) -> Self {
		self.mark::<T>().name_typed::<T>()
	}

	/// On click, flips a `BoolState` component in the given element and executes a passed-in closure.
	///
	/// Closure parameters are `self`, and the already flipped state `bool`
	fn on_flip(self, mut f: impl FnMut(&Self, bool) + 'static) -> Self where Self: Sized + Copy + 'static {
		if self.try_get_cmp::<Flipped>().is_none() { self.add_component(Flipped(false)); };
		self.add_on_click(move |_| {
			let state = self.try_get_cmp_mut::<Flipped>();
			if let Some(mut state) = state {
				state.0 = !state.0;
				f(&self, state.0);
			}
		});
		self
	}

	/// Adds the `Clicked` component to an element which allows you to tell whether it is currently being clicked on (mousedown active).
	///
	/// Uses the default window (e.g. [web_sys::window()])
	///
	/// See: `clicked()`
	fn report_clicked(self) -> Self where Self: Sized + Copy + 'static {
		self.report_clicked_on_window(window())
	}

	/// Adds the `Clicked` component to an element which allows you to tell whether it is currently being clicked on (mousedown active).
	///
	/// Uses the passed in [web_sys::Window]
	///
	/// See: `clicked()`
	fn report_clicked_on_window(self, window: web_sys::Window) -> Self where Self: Sized + Copy + 'static {
		if self.try_get_cmp::<Clicked>().is_some() {
			return self;
		} else {
			self.add_component(Clicked(false));
			self.add_on_mouse_down(move |e| { e.prevent_default(); self.get_cmp_mut::<Clicked>().0 = true; });
			self.get_cmp_mut_or_default::<Vec<_>>().push(window.on_mouse_up(move |_| self.get_cmp_mut::<Clicked>().0 = false));
		}
		self
	}

	/// This will panic at runtime if the `Clicked` component is not present.
	/// Make sure to actually call report_clicked() on the element first.
	fn clicked(self) -> bool {
		self.try_get_cmp::<Clicked>().and_then(|x| Some(x.0)).unwrap_or(false)
	}

	fn font(self, style: &css::Style) -> Self {
		self.class_typed::<FontTag>(style.clone())
	}

	/// Auto-flips an element if it would be off-screen, by mirroring the top/bottom/left/right positional properties appropriately.
	///
	/// This also counts as setting the prefered position for the element, so you do not need to add it in a class/style yourself.
	///
	/// # Arguments
	///
	/// * `spacing_v` - A top or bottom property with the amount of spacing between the parent and child e.g. Some(css::top!(8 px))
	/// * `spacing_h` - A left or right property with the amount of spacing between the parent and child e.g. Some(css::right!(36 px))
	///
	/// Note that it is not e.g. "100% + 8 px", but only the "margin".
	///
	/// Currently only px units are supported.
	fn flip_if_offscreen(self, spacing_v: Option<css::Property>, spacing_h: Option<css::Property>) {
		let parent = SomeElement(self.as_entity()).get_cmp::<hobo::Parent>().0;
		let element_rect = self.get_cmp::<web_sys::HtmlElement>().get_bounding_client_rect();
		let element_height = element_rect.bottom() - element_rect.top();
		let element_width = element_rect.right() - element_rect.left();
		let parent_rect = parent.get_cmp::<web_sys::HtmlElement>().get_bounding_client_rect();
		let window_height = window().inner_height().unwrap().as_f64().unwrap();
		let window_width = window().inner_width().unwrap().as_f64().unwrap();
		let mut new_style = Vec::new();

		if let Some(v) = spacing_v {
			if let css::Property::Top(css::Dimension::Some(css::Unit::Px(f))) = v {
				let vertical = f.into_inner() as f64;
				let dimension = css::Dimension::Some(css::unit!(100% + vertical px));
				let property = if parent_rect.bottom() + vertical + element_height > window_height {
					css::Property::Bottom(dimension)
				} else {
					css::Property::Top(dimension)
				};
				new_style.push(property);
			} else if let css::Property::Bottom(css::Dimension::Some(css::Unit::Px(f))) = v {
				let vertical = f.into_inner() as f64;
				let dimension = css::Dimension::Some(css::unit!(100% + vertical px));
				let property = if parent_rect.top() - vertical - element_height < 0. {
					css::Property::Top(dimension)
				} else {
					css::Property::Bottom(dimension)
				};
				new_style.push(property);
			} else {
				log::warn!("Flip on element with a non-pixel position! (or not top/bottom?)")
			}
		}

		if let Some(h) = spacing_h {
			if let css::Property::Left(css::Dimension::Some(css::Unit::Px(f))) = h {
				let horizontal = f.into_inner() as f64;
				let dimension = css::Dimension::Some(css::unit!(100% - horizontal px));
				let property = if parent_rect.right() + horizontal + element_width > window_width {
					css::Property::Right(dimension)
				} else {
					css::Property::Left(dimension)
				};
				new_style.push(property);
			} else if let css::Property::Right(css::Dimension::Some(css::Unit::Px(f))) = h {
				let horizontal = f.into_inner() as f64;
				let dimension = css::Dimension::Some(css::unit!(100% - horizontal px));
				let property = if parent_rect.left() - horizontal - element_width < 0. {
					css::Property::Left(dimension)
				} else {
					css::Property::Right(dimension)
				};
				new_style.push(property);
			} else {
				log::warn!("Flip on element with a non-pixel position! (or not left/right?)")
			}
		}

		self.set_style(new_style);
	}

	fn hide_signal<S>(self, signal: S) -> Self where
		S: futures_signals::signal::Signal<Item=bool> + 'static,
		Self: 'static + Copy,
	{
		self.component_collection(signal.subscribe(move |x| if x { self.set_style(css::display!(none)) } else { self.remove_style() }));
		self
	}

	fn component_collection<C: 'static>(self, x: C) -> Self { self.set_component_collection(x); self }

	fn set_component_collection<C: 'static>(&self, x: C) { self.get_cmp_mut_or_default::<Vec<C>>().push(x) }

	/// The chaining counterpart of [set_on_slide](Self::set_on_slide).
	fn on_slide(self, f: impl FnMut(f32) + 'static) -> Self where Self: Sized + Copy + 'static {
		self.add_on_slide(f);
		self
	}

	/// Provides a closure which triggers on mouse move, only while the element is clicked.
	/// It captures a normalized `f32` which indicates where the mouse currently is on the element.
	///
	/// This is a non-chaining function. For the chaining counterpart, see [on_slide](Self::on_slide).
	fn add_on_slide(self, mut f: impl FnMut(f32) + 'static) where Self: Sized + Copy + 'static {
		self
			.report_clicked()
			.set_component_collection(window().on_mouse_move(move |event: web_sys::MouseEvent| {
				if !self.clicked() { return; }
				let mouse_x = event.client_x() as f32; // Current mouse position
				let container_element = self.get_cmp::<web_sys::Element>();
				let left = container_element.get_bounding_client_rect().x() as f32;
				let width = container_element.client_width() as f32;
				let position = f32::clamp((mouse_x - left) / width, 0.0, 1.0);
				f(position);
			}));
	}

	fn with_on_slide(self, mut f: impl FnMut(&Self, f32) + 'static) -> Self where Self: Sized + Copy + 'static {
		self.on_slide(move |e| f(&self, e))
	}

	/// The chaining counterpart of [set_on_first_flow](Self::set_on_first_flow).
	fn on_next_flow(self, f: impl FnOnce() + 'static) -> Self where Self: Sized + Copy + 'static {
		self.set_on_next_flow(f);
		self
	}

	/// Provides a closure which triggers once, after the next reflow completes.
	///
	/// In practice, when creating an element with `.on_next_flow(|| ... )`,
	/// it will trigger immediately after the page's first flow.
	///
	/// However, if used in conjunction with a function that is called multiple times, e.g.
	/// ```
	/// 	window().on_resize(move |_| element.set_on_next_flow(|| /* ... */ ))
	/// ```
	/// it will re-trigger after each reflow.
	///
	/// This is a non-chaining function. For the chaining counterpart, see [on_first_flow](Self::on_first_flow).
	fn set_on_next_flow(self, f: impl FnOnce() + 'static) where Self: Sized + Copy + 'static {
		window().request_animation_frame(Closure::once_into_js(f).unchecked_ref()).unwrap();
	}
}

impl<T: Element> EleExt for T {}

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

#[macro_export]
macro_rules! __dbg {
	() => {
		log::info!("[{}:{}]", file!(), line!());
	};
	($val:expr) => {
		match $val {
			tmp => {
				log::info!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!($val), &tmp);
				tmp
			}
		}
	};
	($val:expr,) => { $crate::dbg!($val) };
	($($val:expr),+ $(,)?) => {
		($($crate::dbg!($val)),+,)
	};
}
