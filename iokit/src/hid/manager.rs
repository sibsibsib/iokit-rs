use core_foundation::base::{CFRelease, TCFType, kCFAllocatorDefault};

pub use iokit_sys::hid::manager::*;
use iokit_sys::types::IOOptionBits;

pub struct IOHIDManager(IOHIDManagerRef);

impl Drop for IOHIDManager {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl IOHIDManager {
    pub fn new() -> Option<Self> {
        let m = unsafe {
            IOHIDManagerCreate(kCFAllocatorDefault,
                               IOHIDManagerOptions::kIOHIDManagerOptionNone as IOOptionBits)
        };

        if m.is_null() {
            None
        } else {
            Some(IOHIDManager(m))
        }
    }
}

impl_TCFType!(IOHIDManager, IOHIDManagerRef, IOHIDManagerGetTypeID);
