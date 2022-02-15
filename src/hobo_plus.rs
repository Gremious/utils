use hobo::{enclose as e, prelude::*};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct FontTag;

pub fn window() -> web_sys::Window { web_sys::window().expect("no window") }
pub fn document() -> web_sys::Document { window().document().expect("no document") }

/// Generic `bool` component for on-click state-switch-like events
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Flipped(pub bool);

/// Allows you to tell whether it is currently being clicked on (mousedown active).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Clicked(pub bool);

pub trait EleExt: Element {

	// Another very common operation
	// Removes 1 level of nested closures over with_component
	// .with_component(|&element| FOO.subscribe(move |state| { ... -> .with_subscription(&FOO, move |&element, state| ...
	fn with_subscription<T: 'static>(self, state: &state::State<T>, mut f: impl FnMut(&Self, &T) + 'static) -> Self where Self: Sized + 'static + Copy {
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
	fn on_flip(self, mut f: impl FnMut(&Self, bool) + 'static) -> Self where Self: Sized + 'static + Copy {
		if self.try_get_cmp::<Flipped>().is_none() { self.add_component(Flipped(false)); };
		self.add_on_click(move |_| {
			let mut state = self.get_cmp_mut::<Flipped>();
			state.0 = !state.0;
			f(&self, state.0);
		});
		self
	}

	/// Adds the `Clicked` component to an element which allows you to tell whether it is currently being clicked on (mousedown active).
	///
	/// Uses the default window (e.g. [web_sys::window()])
	///
	/// See: `clicked()`
	fn report_clicked(self) -> Self where Self: Sized + 'static + Copy {
		self.report_clicked_on_window(window())
	}

	/// Adds the `Clicked` component to an element which allows you to tell whether it is currently being clicked on (mousedown active).
	///
	/// Uses the passed in [web_sys::Window]
	///
	/// See: `clicked()`
	fn report_clicked_on_window(self, window: web_sys::Window) -> Self where Self: Sized + 'static + Copy {
		if self.try_get_cmp::<Clicked>().is_some() {
			// wish i could compile_error here tbh
			log::warn!("Element already has the Clicked component. Did you accidentally call report_clicked() twice? {:#?}", self.as_entity());
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
		self.get_cmp::<Clicked>().0
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
	*cb.borrow_mut() = Some(Closure::wrap(Box::new(e!((cb, window) move |timestamp| {
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
