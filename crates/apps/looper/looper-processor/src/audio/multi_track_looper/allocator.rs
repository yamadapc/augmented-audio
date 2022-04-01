/// Disables allocations on certain sections.
///
/// This is disabled on iOS for now, because due to variable buffer sizes the graph processor will
/// always allocate on its first run (TODO: we should figure out how to estimate the maximum size or
/// avoid having to resize)
#[cfg(not(target_os = "ios"))]
#[global_allocator]
static A: assert_no_alloc::AllocDisabler = assert_no_alloc::AllocDisabler;
