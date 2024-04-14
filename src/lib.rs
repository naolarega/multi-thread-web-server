//! # Multi thread web server
//! A simple http web server with multi thread capabilities.
//!
//! ## Usage
//! ``` rust
//! use multi_thread_web_server::{
//!     HttpStatus,
//!     Server
//! };
//!
//! let mut server = Server::new();
//!
//! server.get("/", |_, mut res| {
//!     res.send(
//!         HttpStatus::Ok,
//!         Some("Hello World")
//!     );
//! });
//!
//! server.serve("0.0.0.0:8080").unwrap();
//! ```
pub mod core;

pub use core::server::*;
