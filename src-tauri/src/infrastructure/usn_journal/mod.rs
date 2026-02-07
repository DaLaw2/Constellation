//! NTFS USN Journal Infrastructure
//!
//! Low-level access to the NTFS Change Journal using the unprivileged API.

mod frn;
mod path_resolver;
mod reader;
mod volume;

pub use frn::get_file_reference_number;
pub use path_resolver::resolve_path_by_frn;
pub use reader::{read_journal_records, RawUsnRecord};
pub use volume::{is_ntfs, VolumeHandle};
