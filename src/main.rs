#![feature(custom_test_frameworks)]
#![test_runner(crate::testexec)]
#![reexport_test_harness_main = "testmain"]
#![no_main]
#![no_std]

use core::panic::PanicInfo;
mod vgabuff;
mod ser;


#[no_mangle]
pub extern "C" fn _start() -> !
{
	println!("WELCOME TO LIBERTYOS");

	#[cfg(test)]
	testmain();
	loop {}
}

#[cfg(not(test))] // PANIC HANDLER FOR RELEASE
#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	println!("{}", info);
	loop {}
}

#[cfg(test)]
#[panic_handler] // PANIC HANDLER FOR DEBUG/TESTING
fn panic(info: &PanicInfo) -> !
{
	serprintln!("[PANIC]\n");
	serprintln!("[ERR]: {}\n", info);
	exitqemu(QEMUExitCode::Failure);
	loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QEMUExitCode
{
	Success = 0x10,
	Failure = 0x11,
}

pub fn exitqemu(exitcode: QEMUExitCode)
{
	use x86_64::instructions::port::Port; //TODO: Use x64.
	unsafe
	{
		let mut port = Port::new(0xf4);
		port.write(exitcode as u32);
	}
}

//TODO: FIX ISSUES WITH TESTS NOT COMPILING
#[cfg(test)]
fn testexec(tests: &[&dyn Fn()])
{
	serprintln!("[LIBERTYOS] EXECUTING {} TESTS", tests.len());
	for test in tests
	{		test();
	}
	exitqemu(QEMUExitCode::Success);
}

#[test_case]
fn trivassert()
{
	serprint!("[TEST] TRIVIAL ASSERTION... ");
	assert_eq!(1, 1);
	serprintln!("[TEST] SUCCESS");
}
