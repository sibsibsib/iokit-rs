use std::ffi::{CStr, CString};
use std::mem;

use libc::c_char;

use core_foundation::base::TCFType;
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::CFString;

use iokit_sys::*;
use iokit_sys::base::*;
use iokit_sys::types::{io_object_t, io_service_t, io_iterator_t};

use mach::KernReturn;

pub struct IOObject(io_object_t);

impl Drop for IOObject {
    fn drop(&mut self) {
        self.release().unwrap();
    }
}

impl TIOObject<io_object_t> for IOObject {
    #[inline]
    fn as_concrete_io_object_t(&self) -> io_object_t {
        self.0
    }

    #[inline]
    fn as_io_object_t(&self) -> io_object_t {
        self.as_concrete_io_object_t()
    }
}

pub struct IOIterator(io_iterator_t);

impl Drop for IOIterator {
    fn drop(&mut self) {
        self.release().unwrap();
    }
}

impl Iterator for IOIterator {
    type Item = IOObject;

    fn next(&mut self) -> Option<IOObject> {
        unsafe {
            let result = IOIteratorNext(self.as_concrete_io_object_t());

            if result != 0 {
                Some(IOObject(result))
            } else {
                None
            }
        }
    }
}

impl IOIterator {
    pub fn reset(&self) {
        unsafe { IOIteratorReset(self.as_concrete_io_object_t()) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { IOIteratorIsValid(self.as_concrete_io_object_t()) != 0 }
    }
}

impl TIOObject<io_iterator_t> for IOIterator {
    #[inline]
    fn as_concrete_io_object_t(&self) -> io_iterator_t {
        self.0
    }

    #[inline]
    fn as_io_object_t(&self) -> io_object_t {
        self.as_concrete_io_object_t()
    }
}

pub struct IOService(io_service_t);

impl Drop for IOService {
    fn drop(&mut self) {
        self.release().unwrap();
    }
}

impl IOService {
    pub fn get_matching_service(matching: CFDictionary) -> Option<IOService> {
        unsafe {
            let result = IOServiceGetMatchingService(kIOMasterPortDefault,
                                                     matching.as_concrete_TypeRef());

            if result != 0 {
                Some(IOService(result))
            } else {
                None
            }
        }
    }

    pub fn get_matching_services(matching: CFDictionary) -> Result<IOIterator, KernReturn> {
        unsafe {
            let mut io_iterator_t: io_iterator_t = mem::uninitialized();

            let result = IOServiceGetMatchingServices(kIOMasterPortDefault,
                                                      matching.as_concrete_TypeRef(),
                                                      &mut io_iterator_t);

            if result == KERN_SUCCESS {
                Ok(IOIterator(io_iterator_t))
            } else {
                Err(KernReturn::from(result))
            }
        }
    }
}

impl TIOObject<io_service_t> for IOService {
    #[inline]
    fn as_concrete_io_object_t(&self) -> io_service_t {
        self.0
    }

    #[inline]
    fn as_io_object_t(&self) -> io_object_t {
        self.as_concrete_io_object_t()
    }
}

pub trait TIOObject<concrete_io_object_t> {
    /// Returns the object as its concrete `io_object_t`.
    fn as_concrete_io_object_t(&self) -> concrete_io_object_t;

    /// Returns the object as a raw `io_object_t`.
    fn as_io_object_t(&self) -> io_object_t;

    fn release(&self) -> Result<(), KernReturn> {
        unsafe {
            let result = IOObjectRelease(self.as_io_object_t());

            if result == KERN_SUCCESS {
                Ok(())
            } else {
                Err(KernReturn::from(result))
            }
        }
    }

    fn retain(&self) -> Result<(), KernReturn> {
        unsafe {
            let result = IOObjectRetain(self.as_io_object_t());

            if result == KERN_SUCCESS {
                Ok(())
            } else {
                Err(KernReturn::from(result))
            }
        }
    }

    fn get_class(&self) -> Result<String, KernReturn> {
        unsafe {
            let mut buf = Vec::<c_char>::with_capacity(128);

            let result = IOObjectGetClass(self.as_io_object_t(), buf.as_mut_ptr());

            if result == KERN_SUCCESS {
                Ok(String::from(CStr::from_ptr(buf.as_ptr()).to_str().unwrap().to_string()))
            } else {
                Err(KernReturn::from(result))
            }
        }
    }

    fn copy_class(&self) -> Option<CFString> {
        unsafe {
            let result = IOObjectCopyClass(self.as_io_object_t());

            if result.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(result))
            }
        }
    }

    fn copy_superclass_for_class(&self, class_name: CFString) -> Option<CFString> {
        unsafe {
            let result = IOObjectCopySuperclassForClass(class_name.as_concrete_TypeRef());

            if result.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(result))
            }
        }
    }

    fn copy_bundle_identifier_for_class(&self, class_name: CFString) -> Option<CFString> {
        unsafe {
            let result = IOObjectCopyBundleIdentifierForClass(class_name.as_concrete_TypeRef());

            if result.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(result))
            }
        }
    }

    fn conforms_to(&self, class_name: *mut c_char) -> bool {
        unsafe { IOObjectConformsTo(self.as_io_object_t(), class_name) != 0 }
    }

    fn is_equal_to(&self, object: IOObject) -> bool {
        unsafe { IOObjectIsEqualTo(self.as_io_object_t(), object.as_io_object_t()) != 0 }
    }

    fn get_kernel_retain_count(&self) -> u32 {
        unsafe { IOObjectGetKernelRetainCount(self.as_io_object_t()) }
    }

    fn get_user_retain_count(&self) -> u32 {
        unsafe { IOObjectGetUserRetainCount(self.as_io_object_t()) }
    }

    fn get_retain_count(&self) -> u32 {
        unsafe { IOObjectGetRetainCount(self.as_io_object_t()) }
    }
}
