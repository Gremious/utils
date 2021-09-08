use super::*;

// TODO: Macro, maybe even add into actual hobo? This is on window, hobo is on element so moving the mouse out while clicked doesn't trigger
pub mod callbacks {
	use super::*;

	pub struct WindowMouseupCallback(Closure<dyn FnMut(web_sys::MouseEvent) + 'static>);
	pub struct WindowMousedownCallback(Closure<dyn FnMut(web_sys::MouseEvent) + 'static>);
	pub struct WindowMousemoveCallback(Closure<dyn FnMut(web_sys::MouseEvent) + 'static>);

	impl WindowMouseupCallback {
		pub fn new(mouseup: impl FnMut(web_sys::MouseEvent) + 'static) -> Self {
			let mouseup = Closure::new(mouseup);
			window().add_event_listener_with_callback(web_str::mouseup(), mouseup.as_ref().unchecked_ref()).unwrap();
			Self(mouseup)
		}
	}

	impl Drop for WindowMouseupCallback {
		fn drop(&mut self) {
			window().remove_event_listener_with_callback(web_str::mouseup(), self.0.as_ref().unchecked_ref()).unwrap()
		}
	}

	impl WindowMousedownCallback {
		pub fn new(mousedown: impl FnMut(web_sys::MouseEvent) + 'static) -> Self {
			let mousedown = Closure::new(mousedown);
			window().add_event_listener_with_callback(web_str::mousedown(), mousedown.as_ref().unchecked_ref()).unwrap();
			Self(mousedown)
		}
	}

	impl Drop for WindowMousedownCallback {
		fn drop(&mut self) {
			window().remove_event_listener_with_callback(web_str::mousedown(), self.0.as_ref().unchecked_ref()).unwrap();
		}
	}

	impl WindowMousemoveCallback {
		pub fn new(mousemove: impl FnMut(web_sys::MouseEvent) + 'static) -> Self {
			let mousemove = Closure::new(mousemove);
			window().add_event_listener_with_callback(web_str::mousemove(), mousemove.as_ref().unchecked_ref()).unwrap();
			Self(mousemove)
		}
	}

	impl Drop for WindowMousemoveCallback {
		fn drop(&mut self) {
			window().remove_event_listener_with_callback(web_str::mousemove(), self.0.as_ref().unchecked_ref()).unwrap();
		}
	}
}

/// Generic `bool` component for on-click state-switch-like events
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Flipped(pub bool);

/// Allows you to tell whether it is currently being clicked on (mousedown active).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Clicked(pub bool);

pub trait EleExt: Element {
	// Just a very common operation. Removes 1 level of nested closures.
	// .with(|&element| element.add_component( ... -> .with_component(|&element| ...
	// It shortens by 13 characters physically, and at least 15 emotionally
	fn with_component<T: 'static>(self, f: impl FnOnce(&Self) -> T) -> Self {
		self.add_component(f(&self));
		self
	}

	// Another very common operation
	// Removes 1 level of nested closures over with_component
	// .with_component(|&element| FOO.subscribe(move |state| { ... -> .with_subscription(&FOO, move |&element, state| ...
	fn with_subscription<T: 'static>(self, state: &state::State<T>, mut f: impl FnMut(&Self, &T) + 'static) -> Self where Self: Sized + 'static + Copy {
		self.add_component(state.subscribe(move |state_val| f(&self, state_val)));
		self
	}

	/// Adds an `data-name` attribute to the element with a value of T
	fn name_typed<T: 'static>(self) -> Self {
		if WORLD.is_dead(&self) { log::warn!("mark dead {:?}", self.as_entity()); return self; }
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
	fn on_flip(self, f: impl FnOnce(&Self, bool) + 'static + Copy) -> Self where Self: Sized + 'static + Copy {
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
	/// See: `clicked()`
	fn report_clicked(self) -> Self where Self: Sized + 'static + Copy {
		if self.try_get_cmp::<Clicked>().is_some() {
			// wish i could compile_error here tbh
			log::warn!("Element already has the Clicked component. Did you accidentally call report_clicked() twice? {:#?}", self.as_entity());
		} else {
			self.add_component(Clicked(false));
			self.add_on_mouse_down(move |e| { e.prevent_default(); self.get_cmp_mut::<Clicked>().0 = true; });
			self.add_component(callbacks::WindowMouseupCallback::new(move |_| self.get_cmp_mut::<Clicked>().0 = false));
		}
		self
	}

	/// This will panic at runtime if the `Clicked` component is not present.
	/// Make sure to actually call report_clicked() on the element first.
	fn clicked(self) -> bool {
		self.get_cmp::<Clicked>().0
	}

}
impl<T: Element> EleExt for T {}
