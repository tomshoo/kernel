Version History


0.4.0
 - Implemented a basic method for handling CPU exceptions (see src/intr.rs).
 - Enabled the "abi_x86_interrupt" feature.

0.3.3
 - Fixed the issue that would cause the kernel to panic when running the basicboot test.
 - Added the shouldfail test-case to the shouldpanic component of the tests library.
 - Removed the test-harness for the shouldpanic test.

0.3.2
 - Renamed the name of the crate to "libertyos_kernel".
 - Created a new library to handle tests.
 - Added the basicboot test.
 - Removed a TODO note from main.rs, about fixing the issues that prevented the kernel from compiling for testing.
 - Created a lib.rs file, with some basic code to handle the kernel's ever-expanding code.
 - Removed some code from main.rs, as lib.rs now handles part of what main.rs used to handle.
 - Integrated the tests crate into the kernel's code, so that the whole thing compiles. The tests panic, but that should be fixed soon.

0.3.1
 - Added a rust-toolchain file, so that the nightly version of Rust is used by default.
 - Commented out a test-case that was preventing the kernel from compiling.

0.3.0
 - Removed the [profile.dev] section from Cargo.toml.
 - Removed the [profile.release] section from Cargo.toml.
 - Added the CanTest trait to src/main.rs.
 - Added an implementation of the CanTest trait, to src/main.rs.
 - Added the test_simple_println test-case.
 - Renamed the trivassert test case to test_trivassert, so that the test-cases are easily identifiable.
 - Removed the previous method of executing tests, found in the "for test in tests" section of the testexec function.
 - Implemented CanTest's run to the testexec function.
 - Removed the text that was printed when executing the test_trivassert test-case.
 - Fixed the issues that prevented "cargo test" from compiling successfully. The included tests should work as intended.

0.2.9
 - Added an key to the [package.metadata.bootimage], so that QEMU exits after five minutes of running the trivassert test case.

0.2.8
 - Configured Cargo.toml, so that when running tests (using QEMU), information is output to serial, rather than to the VGA buffer, in a seperate window.
 - Modified the formatting used in the "test-args" attribute of the [package.metadata.bootimage] section. The changes in question serve no functional purpose, but make the file look nicer. :) 
 - Created a separate panic-handler for booting the kernel in testing/debug mode.
 - Created an attribute, for what was the sole panic-handler, so that the kernel uses the original panic-handler, rather than the panic-handler that should be used when running tests in QEMU.

0.2.7
 - Configured bootimage's arguments, in Cargo.toml, so that QEMU prints messages to stdout, in addition to being output to the VGA buffer.

0.2.6
 - Modified the trivassert test case, so that it uses the serprint and serprintln macros
 - Added some details to the Cargo.toml, so that the kernel can be published to crates.io (It has not been published, but should be in the near future)

0.2.5
 - Removed a TODO note, about removing unsafe function in main.rs
 - Added a TODO task, to fix the issues that prevent kernel tests from compiling

0.2.4
 - Added serprint macro
 - Added serprintln macro
 - Added an expect message for the _print function of the ser module
