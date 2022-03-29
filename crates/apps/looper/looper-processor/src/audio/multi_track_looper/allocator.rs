use assert_no_alloc::AllocDisabler;

#[global_allocator]
static A: AllocDisabler = AllocDisabler;
