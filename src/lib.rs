#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use metrics::{register_counter, register_gauge, Unit};

pub mod buffer;
pub mod config;
mod file;

pub use file::Log;

pub fn init_metrics(mut targets: Vec<String>) {
    for target in targets.drain(..) {
        register_gauge!("maximum_bytes_per",
                        Unit::Bytes,
                        "soft maximum size of the file, in bytes",
                        "target" => target.clone());
        register_gauge!("maximum_line_size_bytes",
                        Unit::Bytes,
                        "maximum line size of the file, in bytes",
                        "target" => target.clone());
        register_counter!("bytes_written",
                          Unit::Bytes,
                          "total bytes written to the file",
                          "target" => target.clone());
        register_gauge!("current_target_size_bytes",
                          Unit::Bytes,
                          "current size in bytes of the target file",
                          "target" => target.clone());
        register_counter!("file_rotated",
                          Unit::Count,
                          "number of times the underlying file has been rotated",
                          "target" => target.clone());
        register_gauge!("file_rotation_slop",
                        Unit::Bytes,
                        "total number of bytes over the soft maximum at rotation",
                        "target" => target);
    }
}
