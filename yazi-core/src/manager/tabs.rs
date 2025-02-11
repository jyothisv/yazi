use std::ops::{Deref, DerefMut};

use yazi_boot::BOOT;
use yazi_shared::fs::Url;

use crate::{manager::Manager, tab::Tab};

pub struct Tabs {
	pub idx:          usize,
	pub(super) items: Vec<Tab>,
}

impl Tabs {
	pub fn make() -> Self {
		let mut tabs = Self { idx: 0, items: vec![Tab::from(Url::from(&BOOT.cwd))] };
		if let Some(file) = &BOOT.file {
			tabs.items[0].reveal(Url::from(BOOT.cwd.join(file)));
		}

		Manager::_refresh();
		tabs
	}

	#[inline]
	pub(super) fn absolute(&self, rel: isize) -> usize {
		if rel > 0 {
			(self.idx + rel as usize).min(self.items.len() - 1)
		} else {
			self.idx.saturating_sub(rel.unsigned_abs())
		}
	}

	#[inline]
	pub(super) fn set_idx(&mut self, idx: usize) {
		if self.idx == idx {
			return;
		}

		// Reset the preview of the previous active tab
		if let Some(active) = self.items.get_mut(self.idx) {
			active.preview.reset_image();
		}

		self.idx = idx;
		Manager::_refresh();
		Manager::_peek(true);
	}
}

impl Tabs {
	#[inline]
	pub fn active(&self) -> &Tab { &self.items[self.idx] }

	#[inline]
	pub(super) fn active_mut(&mut self) -> &mut Tab { &mut self.items[self.idx] }
}

impl Deref for Tabs {
	type Target = Vec<Tab>;

	fn deref(&self) -> &Self::Target { &self.items }
}

impl DerefMut for Tabs {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.items }
}
