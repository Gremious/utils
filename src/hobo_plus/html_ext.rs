use hobo::{prelude::*, create as e};
use hobo::signal::{Mutable, MutableSignal, SignalExt};
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, AsElement)]
pub struct Toggleable<E: hobo::AsElement + Clone + Copy + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static>(E);

pub trait ToggleableExt<E: hobo::AsElement + Clone + Copy + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static>: std::ops::Deref<Target = Toggleable<E>> + hobo::AsElement + Copy + Sized + 'static {
	/// Takes in a closure of (self, current toggle state as fired by the mutable) and executes it.
	fn set_on_toggle(self, mut f: impl FnMut(bool) + 'static) -> Self {
		self.add_bundle(self.get_cmp::<ToggleState>().signal().subscribe(move |x| f(x.0)));
		self
	}
	fn on_toggle(self, f: impl FnMut(bool) + 'static) -> Self { self.set_on_toggle(f); self }
	fn with_on_toggle(self, mut f: impl FnMut(&Self, bool) + 'static) -> Self { self.on_toggle(move |e| f(&self, e)) }

	fn value(&self) -> bool {
		self.get_cmp::<ToggleState>().lock_ref().0
	}

	fn set_value(&self, v: bool) {
		self.get_cmp::<ToggleState>().lock_mut().0 = v;
	}

	fn toggle(&self) {
		self.get_cmp::<ToggleState>().lock_mut().0.tap_mut(|x| { *x = !*x; });
	}

	fn value_signal(self) -> impl hobo::signal::Signal<Item = bool> {
		self.get_cmp::<ToggleState>().signal_ref(|x| x.0)
	}

	fn toggle_on_click(self) -> Self { self.on_click(move |_| self.toggle()) }
}

// There's checkboxes, radials, and switches. And maybe even different types of those.
// They all have a bunch of shared functionality,
// the on_flip function didn't really do it very well, and I still wanted some consistency.
// So I thought hey, the "StringValue" trait was very cool, maybe we can similarly do a trait for "Toggleables"?
impl<E: hobo::AsElement + Clone + Copy + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static> Toggleable<E> {
	pub fn new(element: E, default: bool) -> Self {
		Self(element)
			.component(Mutable::new(Toggle(default)))
	}
}
