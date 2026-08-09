use oboe_sys as ffi;

/**
 * Check if the device supports AAudio MMAP (memory-mapped) data path.
 *
 * MMAP provides lower latency by bypassing the AudioFlinger mixer.
 * This checks the device's system property `aaudio.mmap_policy`.
 */
pub fn is_mmap_supported() -> bool {
    unsafe { ffi::oboe_OboeExt_isMMapSupported() }
}

/**
 * Check if MMAP is currently enabled for this process.
 *
 * MMAP can be enabled/disabled via [`set_mmap_enabled`].
 * This queries the current `aaudio.mmap_policy` setting.
 */
pub fn is_mmap_enabled() -> bool {
    unsafe { ffi::oboe_OboeExt_isMMapEnabled() }
}

/**
 * Enable or disable MMAP data path for subsequent stream opens.
 *
 * This has no effect on already opened streams.
 * Only affects the current application process.
 *
 * Returns `true` if the operation succeeded, `false` if MMAP is not supported.
 */
pub fn set_mmap_enabled(enabled: bool) -> bool {
    unsafe { ffi::oboe_OboeExt_setMMapEnabled(enabled) == 0 }
}
