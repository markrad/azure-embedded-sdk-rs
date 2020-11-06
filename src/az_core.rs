use azsys;

pub fn precondition_failed_set_callback(callback: azsys::az_precondition_failed_fn) {
    unsafe { azsys::az_precondition_failed_set_callback(callback) };
}

pub fn precondition_failed_get_callback() -> Option<unsafe extern "C" fn()> {
    unsafe { azsys::az_precondition_failed_get_callback() }
}

pub fn get_span_from_str(s: &str) -> azsys::az_span {
    let result: azsys::az_span = azsys::az_span {
        _internal: azsys::az_span__bindgen_ty_1 {
            ptr: s.as_ptr() as *mut u8,
            size: s.len() as i32,
        }
    };

    result
}

pub fn get_span_from_vector(v: &Vec<u8>) -> azsys::az_span {
    let result: azsys::az_span = azsys::az_span {
        _internal: azsys::az_span__bindgen_ty_1 {
            ptr: v.as_ptr() as *mut u8,
            size: v.capacity() as i32,
        }
    };

    result
}

pub fn get_empty_span() -> azsys::az_span {
    let result: azsys::az_span = azsys::az_span {
        _internal: azsys::az_span__bindgen_ty_1 {
            ptr: std::ptr::null_mut(),
            size: 0,
        }
    };

    result
}

pub fn get_span_size(span: &azsys::az_span) -> i32 {
    span._internal.size
}
/*
pub struct az_span {
    inner: azsys::az_span;
}

impl az_span {
    pub fn from_str(s: &str) -> az_span {
        let result: azsys::az_span = azsys::az_span {
            _internal: azsys::az_span__bindgen_ty_1 {
                ptr: s.as_ptr() as *mut u8,
                size: s.len() as i32,
            }
        };
    
        result
    }
}
*/

#[cfg(test)] 
mod tests {
    use super::*;
    #[test]
    fn set_callback() {
        precondition_failed_set_callback(Option::Some(callback));
    }
    #[test]
    fn get_callback() {
        precondition_failed_set_callback(Option::Some(callback));
        let check: azsys::az_precondition_failed_fn = precondition_failed_get_callback();
        assert!(check.is_some());
    }

    unsafe extern "C" fn callback() {
        panic!();
    }
}