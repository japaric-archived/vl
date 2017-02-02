//! Exceptions

use core::intrinsics;
use core::ptr;

#[cfg(feature = "semihosting")]
use cortex_m::Exception;
use cortex_m::StackFrame;

/// Default exception handler
#[naked]
pub extern "C" fn default_handler() {
    extern "C" fn handler(_sf: &StackFrame) -> ! {
        hprintln!("EXCEPTION {:?} @ PC=0x{:08x}", Exception::current(), _sf.pc);

        unsafe {
            bkpt!();
        }

        loop {}
    }

    unsafe {
        asm!("mrs r0, MSP
              ldr r1, [r0, #20]
              b $0"
             :
             : "i"(handler as extern "C" fn(&StackFrame) -> !)
             :
             : "volatile");

        intrinsics::unreachable();
    }
}

#[doc(hidden)]
#[export_name = "_start"]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static mut _ebss: u32;
        static mut _sbss: u32;

        static mut _edata: u32;
        static mut _sdata: u32;

        static _sidata: u32;

        fn main(argc: isize, argv: *const *const u8) -> isize;
    }

    ::r0::zero_bss(&mut _sbss, &mut _ebss);
    ::r0::init_data(&mut _sdata, &mut _edata, &_sidata);

    main(0, ptr::null());

    loop {
        ::cortex_m::asm::wfi()
    }
}
