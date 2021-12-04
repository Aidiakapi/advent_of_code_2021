use std::{
    alloc::{Allocator, Global, Layout},
    ptr::NonNull,
};

pub fn init_boxed_array<T: Default, const S: usize>() -> Box<[T; S]> {
    assert_ne!(std::mem::size_of::<T>(), 0);

    struct InitState<T, const S: usize> {
        mem: NonNull<[T; S]>,
        init_count: isize,
    }

    impl<T, const S: usize> Drop for InitState<T, S> {
        fn drop(&mut self) {
            unsafe {
                for i in (0..self.init_count).rev() {
                    self.mem.cast::<T>().as_ptr().offset(i).drop_in_place()
                }
                Global.deallocate(self.mem.cast(), Layout::new::<[T; S]>());
            }
        }
    }

    let mem: NonNull<[T; S]> = Global
        .allocate(Layout::new::<[T; S]>())
        .expect("failed to allocate")
        .cast();

    let mut state = InitState::<T, S> { mem, init_count: 0 };

    for i in 0..S as isize {
        unsafe {
            mem.cast::<T>().as_ptr().offset(i).write(T::default());
            state.init_count = i;
        }
    }

    let array = unsafe { Box::from_raw(mem.as_ptr()) };
    std::mem::forget(state);
    array
}
