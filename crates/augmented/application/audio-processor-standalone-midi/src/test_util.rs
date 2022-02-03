use assert_no_alloc::{assert_no_alloc, reset_violation_count, violation_count, AllocDisabler};

#[global_allocator]
static A: AllocDisabler = AllocDisabler;

pub(crate) fn assert_allocation_count<T>(count: usize, f: impl FnOnce() -> T) -> T {
    reset_violation_count();
    let result = assert_no_alloc(f);
    let violation_count = violation_count();
    assert_eq!(
        violation_count as usize, count,
        "Section allocated/de-allocated memory N times"
    );
    result
}
