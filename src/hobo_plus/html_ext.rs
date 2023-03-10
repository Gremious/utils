use hobo::{prelude::*, create as e};
use hobo::signals::signal::{Mutable, MutableSignal, SignalExt};
use super::entity_ext::AsEntityExt;
pub use tap::prelude::*;

pub trait AExt: AsElement + Copy {
	#[must_use] #[inline] fn untrusted<'a>(self) -> Self { self.set_untrusted(); self }
	#[inline] fn set_untrusted<'a>(self) { self.attr(web_str::target(), web_str::_blank()).set_attr(web_str::rel(), "noopener noreferrer"); }
}

impl AExt for e::A {}

/// Generic `bool` component for checbox/switch like events.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Toggle(bool);
type ToggleState = Mutable<Toggle>;

// There's checkboxes, radials, and switches. And maybe even different types of those.
// They all have a bunch of shared functionality,
// the on_flip function didn't really do it very well, and I still wanted some consistency.
// So I thought hey, the "StringValue" trait was very cool, maybe we can similarly do a trait for "Toggleables"?
pub trait Toggleable: AsElement + Copy + Sized + 'static {
	/// Sets up the necessary mutables/components.
	fn toggleable(self, default: bool) -> Self { self.set_toggleable(default); self }
	fn set_toggleable(self, default: bool) { let _ = self.get_cmp_mut_or(|| Mutable::new(Toggle(default))); }

	/// Takes in a closure of (self, current toggle state as fired by the mutable) and executes it.
	fn set_on_toggle(self, mut f: impl FnMut(bool) + 'static) -> Self {
		let state = self.try_get_cmp::<ToggleState>().expect("No Toggle Mutable found. Did you call `set_/toggleable`?");
		self.add_bundle(state.signal().subscribe(move |x| f(x.0)));
		self
	}
	fn on_toggle(self, f: impl FnMut(bool) + 'static) -> Self { self.set_on_toggle(f); self }
	fn with_on_toggle(self, mut f: impl FnMut(&Self, bool) + 'static) -> Self { self.on_toggle(move |e| f(&self, e)) }

	fn value(&self) -> bool {
		self.get_cmp::<ToggleState>().lock_ref().0
	}

	fn set_value(&self, v: bool) {
		self.get_cmp::<ToggleState>().set(Toggle(v));
	}

	fn toggle(&self) {
		self.get_cmp::<ToggleState>().lock_mut().tap_mut(|x| { x.0 = !x.0; });
	}

	fn value_signal(self) -> hobo::signals::signal::Map<MutableSignal<Toggle>, fn (Toggle) -> bool> {
		fn lift_toggle(x: Toggle) -> bool { x.0 }

		self.get_cmp::<ToggleState>().signal().map(lift_toggle)
	}

	fn toggle_on_click(self) -> Self { self.on_click(move |_| self.toggle()) }
}
