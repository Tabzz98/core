/*
 *	MetaCall Library by Parra Studios
 *	A library for providing a foreign function interface calls.
 *
 *	Copyright (C) 2016 - 2021 Vicente Eduardo Ferrer Garcia <vic798@gmail.com>
 *
 *	Licensed under the Apache License, Version 2.0 (the "License");
 *	you may not use this file except in compliance with the License.
 *	You may obtain a copy of the License at
 *
 *		http://www.apache.org/licenses/LICENSE-2.0
 *
 *	Unless required by applicable law or agreed to in writing, software
 *	distributed under the License is distributed on an "AS IS" BASIS,
 *	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *	See the License for the specific language governing permissions and
 *	limitations under the License.
 *
 */

use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_float, c_int, c_long, c_short, c_void};

#[link(name = "metacall")] // requires libmetacall to be in $PATH
extern "C" {
    fn metacall_initialize() -> c_int;
    fn metacall_load_from_file(
        tag: *const c_char,
        paths: *mut *const u8,
        size: usize,
        handle: *mut *mut c_void,
    ) -> c_int;
    fn metacall_function(cfn: *const c_char) -> *mut c_void;
    fn metacall_destroy() -> c_int;
    fn metacallfv_s(func: *mut c_void, args: *mut *mut c_void, size: usize) -> *mut c_void;
    fn metacall_value_create_short(s: c_short) -> *mut c_void;
    fn metacall_value_create_int(i: c_int) -> *mut c_void;
    fn metacall_value_create_long(l: c_long) -> *mut c_void;
    fn metacall_value_create_float(f: c_float) -> *mut c_void;
    fn metacall_value_create_double(d: c_double) -> *mut c_void;
    fn metacall_value_create_bool(b: c_int) -> *mut c_void;
    fn metacall_value_create_string(st: *const c_char, ln: usize) -> *mut c_void;
    fn metacall_value_create_char(st: c_char) -> *mut c_void;
    fn metacall_value_destroy(v: *mut c_void);
    fn metacall_value_id(v: *mut c_void) -> c_int;
    fn metacall_value_to_string(v: *mut c_void) -> *mut c_char;
    fn metacall_value_to_char(v: *mut c_void) -> c_char;
    fn metacall_value_to_short(v: *mut c_void) -> c_short;
    fn metacall_value_to_int(v: *mut c_void) -> c_int;
    fn metacall_value_to_long(v: *mut c_void) -> c_long;
    fn metacall_value_to_bool(v: *mut c_void) -> c_int;
    fn metacall_value_to_float(v: *mut c_void) -> c_float;
    fn metacall_value_to_double(v: *mut c_void) -> c_double;
}

#[derive(Debug)]
pub struct Error(String);

/// Enum of all possible Metacall types to allow for safe conversion between them and c_types
#[derive(Debug)]
pub enum Any {
    Null,              // from c_null
    Short(i16),        // from c_short
    Int(i32),          // from c_int
    Long(i64),         // from c_long
    Float(f32),        // from c_float
    Double(f64),       // from c_double
    Bool(bool),        // from c_bool
    Char(char),        // from c_char
    Str(String),       // from *const u8 (null terminated)
    Array(Vec<Any>),   // from *mut *mut c_void
    Buffer(Vec<u8>),   // from *const u8 (non-null terminated) (raw binary data)
    Pointer(Box<Any>), // from *mut c_void
    Function(Box<fn(Any) -> Any>), // from a C function pointer
                       // METACALL_FUTURE
}

impl From<c_short> for Any {
    fn from(val: c_short) -> Self {
        Any::Short(val)
    }
}
impl From<c_int> for Any {
    fn from(val: c_int) -> Self {
        Any::Int(val)
    }
}
impl From<c_long> for Any {
    fn from(val: c_long) -> Self {
        Any::Long(val)
    }
}
impl From<c_char> for Any {
    fn from(val: c_char) -> Self {
        Any::Char(val as u8 as char)
    }
}
impl From<bool> for Any {
    fn from(val: bool) -> Self {
        Any::Bool(val)
    }
}
impl From<c_float> for Any {
    fn from(val: c_float) -> Self {
        Any::Float(val)
    }
}
impl From<c_double> for Any {
    fn from(val: c_double) -> Self {
        Any::Double(val)
    }
}

pub fn initialize() -> Result<(), &'static str> {
    if unsafe { metacall_initialize() } != 0 {
        Err("Metacall failed to initialize")
    } else {
        Ok(())
    }
}

pub fn load_from_file(
    tag: &str,
    scripts: impl IntoIterator<Item = impl AsRef<str>>,
) -> Result<(), &'static str> {
    // allocate a safe C String
    let ctag = CString::new(tag).expect("Conversion to C String failed");
    let owned_scripts: Vec<_> = scripts
        .into_iter()
        .map(|x| CString::new(x.as_ref()).expect("Conversion to C String failed"))
        .collect();
    let mut ref_c_scripts: Vec<_> = owned_scripts
        .iter()
        .map(|s| s.as_ptr())
        .map(|p| p as *const u8)
        .collect();
    if unsafe {
        metacall_load_from_file(
            ctag.as_ptr(),
            ref_c_scripts.as_mut_ptr(),
            ref_c_scripts.len(),
            std::ptr::null_mut(),
        )
    } != 0
    {
        return Err("MetaCall failed to load script");
    }
    Ok(())
}

// Possible types as variants in Rust
pub fn metacall<'a>(
    func: &str,
    args: impl IntoIterator<Item = &'a Any>,
) -> Result<Any, &'static str> {
    let c_function = CString::new(func).expect("Conversion to C String failed");
    unsafe {
        // let c_func = metacall_function(c_function.as_ptr());
        let c_func: *mut c_void = metacall_function(c_function.as_ptr());
        if c_func.is_null() {
            return Err("Function Not Found");
        }
        let mut c_args: Vec<*mut c_void> = args
            .into_iter()
            .map(|arg| match arg {
                Any::Short(x) => metacall_value_create_short(*x),
                Any::Int(x) => metacall_value_create_int(*x),
                Any::Long(x) => metacall_value_create_long(*x),
                Any::Float(x) => metacall_value_create_float(*x),
                Any::Double(x) => metacall_value_create_double(*x),
                Any::Bool(x) => metacall_value_create_bool(*x as c_int),
                Any::Char(x) => metacall_value_create_char(*x as c_char),
                Any::Str(x) => {
                    let st = CString::new(x.as_str()).expect("can't convert to c str");
                    metacall_value_create_string(st.as_ptr(), x.len())
                }
                _ => todo!(),
            })
            .collect();
        let ret: *mut c_void = metacallfv_s(c_func, c_args.as_mut_ptr(), c_args.len());
        let mut rt = Any::Null;
        if !ret.is_null() {
            /* TODO: This should be done by an enum or something mimicking the enum in metacall.h */
            match metacall_value_id(ret) {
                0 => {
                    rt = Any::Bool(metacall_value_to_bool(ret) != 0);
                }
                1 => {
                    rt = Any::Char(metacall_value_to_char(ret) as u8 as char);
                }
                2 => {
                    rt = Any::Short(metacall_value_to_short(ret));
                }
                3 => {
                    rt = Any::Int(metacall_value_to_int(ret));
                }
                4 => {
                    rt = Any::Long(metacall_value_to_long(ret));
                }
                5 => {
                    rt = Any::Float(metacall_value_to_float(ret));
                }
                6 => {
                    rt = Any::Double(metacall_value_to_double(ret));
                }
                7 => {
                    let st = std::ffi::CStr::from_ptr(metacall_value_to_string(ret));
                    rt = Any::Str(String::from(
                        st.to_str().expect("couldn't convert CStr to &str"),
                    ));
                }
                8 => {
                    // METACALL_BUFFER
                }
                9 => {
                    // METACALL_ARRAY
                }
                10 => {
                    // METACALL_MAP
                }
                11 => {
                    // METACALL_PTR
                }
                12 => {
                    // METACALL_FUTURE
                }
                13 => {
                    // METACALL_FUNCTION
                }
                14 => {
                    rt = Any::Null;
                }
                _ => {}
            }
            metacall_value_destroy(ret);
        }
        for arg in c_args {
            metacall_value_destroy(arg);
        }
        Ok(rt)
    }
}

pub fn destroy() {
    unsafe {
        metacall_destroy();
    }
}

/// Doc test to check if the code can build an run
#[cfg(test)]
mod tests {
    struct Defer<F: FnOnce()>(Option<F>);

    impl<F: FnOnce()> Drop for Defer<F> {
        fn drop(&mut self) {
            if let Some(f) = self.0.take() {
                f()
            }
        }
    }

    /// Defer execution of a closure until the constructed value is dropped
    /// Works at the end of the scope or manual drop() function
    pub fn defer<F: FnOnce()>(f: F) -> impl Drop {
        Defer(Some(f))
    }

    #[test]
    fn test_defer() {
        use std::cell::RefCell;

        let i = RefCell::new(0);

        {
            let _d = defer(|| *i.borrow_mut() += 1);
            assert_eq!(*i.borrow(), 0);
        }

        assert_eq!(*i.borrow(), 1);
    }

    #[test]
    fn test_metacall() {
        let _d = defer(crate::destroy);

        match crate::initialize() {
            Err(e) => {
                println!("{}", e);
                panic!();
            }
            _ => println!(" Hello World Metacall created "),
        }

        let scripts = ["test.mock"];

        if let Err(e) = crate::load_from_file("mock", &scripts) {
            println!("{}", e);
            panic!();
        }

        match crate::metacall("new_args", &[crate::Any::Str("a".to_string())]) {
            Ok(ret) => match ret {
                crate::Any::Str(value) => {
                    assert_eq!("Hello World".to_string(), value);
                    println!("Result: {}", value);
                }
                _ => {
                    assert_eq!(0, 1);
                    panic!();
                }
            },
            Err(e) => {
                println!("{}", e);
                assert_eq!(0, 1);
                panic!();
            }
        }
    }
}
