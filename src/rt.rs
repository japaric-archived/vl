use core::mem;

#[lang = "start"]
extern "C" fn start(main: *const u8,
                    _argc: isize,
                    _argv: *const *const u8)
                    -> isize {
    // TODO initialization code goes here
    ::led::initialize();

    unsafe {
        (mem::transmute::<_, fn()>(main))();
    }

    0
}
