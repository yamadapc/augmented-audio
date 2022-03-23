use assert_no_alloc::{assert_no_alloc, reset_violation_count, violation_count, AllocDisabler};

#[global_allocator]
static A: AllocDisabler = AllocDisabler;
