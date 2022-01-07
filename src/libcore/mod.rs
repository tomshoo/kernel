// src/core/mod.rs
//
// This is the mod.rs for the core module.

// Memory allocation
pub mod allocator;

// Arch-related modules
pub mod arch;

// Devices and drivers
pub mod dev;

// External functions
pub mod external;

// Libraries for working with filesystems
pub mod fs;

// TUI and GUI modules
pub mod graphics;

// I/O modules
pub mod io;

// Mathematics, measurements, and time-keeping
pub mod math;

// Task-execution
//pub mod task;