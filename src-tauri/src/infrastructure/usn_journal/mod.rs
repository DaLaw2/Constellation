//! NTFS USN Journal Infrastructure
//!
//! Low-level access to the NTFS Change Journal using the unprivileged API.

#[cfg(windows)]
mod frn;
#[cfg(windows)]
mod path_resolver;
#[cfg(windows)]
mod reader;
#[cfg(windows)]
mod volume;

#[cfg(windows)]
pub use frn::get_file_reference_number;
#[cfg(windows)]
pub use path_resolver::resolve_path_by_frn;
#[cfg(windows)]
pub use reader::{read_journal_records, RawUsnRecord};
#[cfg(windows)]
pub use volume::{is_ntfs, VolumeHandle};
