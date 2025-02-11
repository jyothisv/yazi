use std::str::FromStr;

use yazi_config::manager::SortBy;
use yazi_shared::event::Cmd;

use crate::{manager::Manager, tab::Tab, tasks::Tasks};

impl Tab {
	pub fn sort(&mut self, c: Cmd, tasks: &Tasks) {
		if let Some(by) = c.args.first() {
			self.conf.sort_by = SortBy::from_str(by).unwrap_or_default();
		}
		self.conf.sort_sensitive = c.named.contains_key("sensitive");
		self.conf.sort_reverse = c.named.contains_key("reverse");
		self.conf.sort_dir_first = c.named.contains_key("dir-first");

		self.apply_files_attrs();
		Manager::_update_paged();

		tasks.preload_sorted(&self.current.files);
	}
}
