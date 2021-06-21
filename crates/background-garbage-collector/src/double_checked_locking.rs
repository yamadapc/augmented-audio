// "CppCon 2015: Live Lock-free or Deadlock" Part 1
// https://www.youtube.com/watch?v=lVBvHbJsg5Y

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct Singleton {}
    impl Singleton {
        fn thread_safe_method(&self) {}
    }

    #[test]
    fn test_double_checked_locking_use_case() {
        fn use_singleton() {
            static SINGLETON: Singleton = Singleton {};
            // ^^^ Is it thread safe to initialize this static?
            SINGLETON.thread_safe_method();
        }
        use_singleton();
        assert!(true);
    }

    #[test]
    fn test_thread_unsafe_static_initialization() {
        static mut SINGLETON_PTR: *mut Singleton = std::ptr::null_mut();
        unsafe {
            if SINGLETON_PTR == std::ptr::null_mut() {
                // would acquire mutex

                // would load again
                if SINGLETON_PTR == std::ptr::null_mut() {
                    SINGLETON_PTR = Box::into_raw(Box::new(Singleton {}));
                }

                // would release mutex

                // doesn't work due to initialization/set ptr order
            }
        }
        assert!(true);
    }

    #[test]
    fn test_atomic_static_initialization() {
        static mut SINGLETON_PTR: AtomicPtr<Singleton> = AtomicPtr::new(std::ptr::null_mut());
        unsafe {
            if SINGLETON_PTR.load(Ordering::Relaxed) == std::ptr::null_mut() {
                // would acquire mutex

                // would load again (non-atomic load)
                if SINGLETON_PTR.load(Ordering::Relaxed) == std::ptr::null_mut() {
                    let singleton = Box::into_raw(Box::new(Singleton {}));
                    SINGLETON_PTR.store(singleton, Ordering::Relaxed);
                }

                // would release mutex

                // ... Doesn't work because CPU caches aren't necessary flushed to memory despite
                //     atomic instructions - non-atomic load doesn't work
            }
        }
        assert!(true);
    }

    #[test]
    fn test_atomic_static_initialization_with_ordering() {
        static mut SINGLETON_PTR: AtomicPtr<Singleton> = AtomicPtr::new(std::ptr::null_mut());
        unsafe {
            if SINGLETON_PTR.load(Ordering::Acquire) == std::ptr::null_mut() {
                // would acquire mutex

                // would load again (non-atomic load)
                if SINGLETON_PTR.load(Ordering::Acquire) == std::ptr::null_mut() {
                    let singleton = Box::into_raw(Box::new(Singleton {}));
                    SINGLETON_PTR.store(singleton, Ordering::Release);
                }

                // would release mutex

                // ... Works due to "Acquire" / "Release" ordering
                // Before "acquire" read, all "release" WRITES will be visible
            }
        }
        assert!(true);
    }

    // QUESTIONS:
    // 1. Is memory order applicable BETWEEN atomic variables? Or only for 1 variable at a time
}
