#[cfg(any(target_os = "linux", target_os = "android"))]
#[test]
fn test_statx() {
    use posish::fs::{FsWord, StatFs, PROC_SUPER_MAGIC};

    // Ensure these all have consistent types.
    let t: StatFs = unsafe { std::mem::zeroed() };
    let _s: FsWord = t.f_type;
    let _u: FsWord = PROC_SUPER_MAGIC;

    // Ensure that after all the platform-specific dancing we have to do, this
    // constant comes out with the correct value.
    #[cfg(not(target_env = "musl"))]
    {
        assert_eq!(
            i128::from(PROC_SUPER_MAGIC),
            i128::from(libc::PROC_SUPER_MAGIC)
        );
    }

    assert_eq!(PROC_SUPER_MAGIC, 0x0000_9fa0);
}
