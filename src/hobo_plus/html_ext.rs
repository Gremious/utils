use hobo::{prelude::*, create as e};

pub trait AExt: AsElement + Copy {
	#[must_use] #[inline] fn untrusted<'a>(self) -> Self { self.set_untrusted(); self }
	#[inline] fn set_untrusted<'a>(self) { self.attr(web_str::target(), web_str::_blank()).set_attr(web_str::rel(), "noopener noreferrer"); }
}

impl AExt for e::A {}
