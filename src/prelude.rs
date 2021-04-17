#[doc(hidden)]
pub use gio::prelude::*;

#[doc(hidden)]
pub use glib::prelude::*;

#[doc(hidden)]
pub use pango::prelude::*;


pub trait Object: std::fmt::Debug + Clone + 'static {}
pub trait Is<T: Object>: AsRef<T> + 'static {}

#[cfg(not(target_arch = "wasm32"))]
pub use crate::legacy::traits::*;

// pub use app_info::AppInfoExtManual;

// pub use application::*;
// pub use converter::*;
// pub use data_input_stream::DataInputStreamExtManual;
// pub use desktop_app_info::DesktopAppInfoExtManual;

// pub use file::FileExtManual;
// pub use inet_address::InetAddressExtManual;
// pub use input_stream::InputStreamExtManual;
// pub use io_stream::IOStreamExtManual;
// pub use list_store::ListStoreExtManual;

// pub use output_stream::OutputStreamExtManual;
// pub use pollable_input_stream::PollableInputStreamExtManual;
// pub use pollable_output_stream::PollableOutputStreamExtManual;
// pub use settings::SettingsExtManual;
// pub use socket::*;
// pub use socket_listener::SocketListenerExtManual;
// pub use unix_input_stream::UnixInputStreamExtManual;
// pub use unix_output_stream::UnixOutputStreamExtManual;
// pub use unix_socket_address::{UnixSocketAddressExtManual, UnixSocketAddressPath};
