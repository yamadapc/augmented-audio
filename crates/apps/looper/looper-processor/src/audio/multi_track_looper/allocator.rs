use assert_no_alloc::AllocDisabler;

/// Disables allocations on certain sections.
/// This is disabled on iOS
#[cfg(not(target_os = "ios"))]
#[global_allocator]
static A: AllocDisabler = AllocDisabler;
