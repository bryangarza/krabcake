// FIXME we haven't implemented raw pointers (i.e. \bot Tag and SharedRW Item)
// so the test output we currently have for this is bogus.

use test_dependencies::{kc_as_raw, kc_borrow_mut, print_stack_of, print_tag_of};

// This is slightly modified from the presentation in the POPL 2020 paper; we
// want to observe the stack state as we run.

#[allow(non_upper_case_globals)]
static mut l: *const i32 = std::ptr::null();

fn main() {
    unsafe {
        let mut local = 5;
        l = std::ptr::addr_of!(local);
        print_stack_of(b"l\0".as_ptr(), l);

        //let raw_pointer = kc_as_raw!(kc_borrow_mut!(local)); //as *mut i32;
        let temp = kc_borrow_mut!(local);
        let raw_pointer = kc_as_raw!(temp);

        print_stack_of(b"l\0".as_ptr(), l);
        let result = {
            let arg1 = kc_borrow_mut!(*raw_pointer);
            print_tag_of(b"arg1\0".as_ptr(), arg1);
            print_stack_of(b"l\0".as_ptr(), l);
            let arg2 = kc_borrow_mut!(*raw_pointer);
            print_tag_of(b"arg2\0".as_ptr(), arg2);
            print_stack_of(b"l\0".as_ptr(), l);
            example1(
                arg1,
                arg2,
            )
        };
        println!("{}", result);
    }
}

fn example1(x: &mut i32, y: &mut i32) -> i32 {
    unsafe { print_stack_of(b"l\0".as_ptr(), l); }
    *x = 42;
    *y = 13;
    return *x;
}
