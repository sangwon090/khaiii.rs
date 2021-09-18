use crate::{KhaiiiWord, ffi::*};
use crate::error::KhaiiiError;
use std::ffi::{CString, CStr};
use std::slice;
use libc::c_short;

pub struct KhaiiiApi {
    handle: i32,
}

impl KhaiiiApi {
    pub fn new() -> KhaiiiApi {
        KhaiiiApi { handle: -1 }
    }

    pub fn version(&self) -> String {
        return unsafe {
            CStr::from_ptr(khaiii_version()).to_str().unwrap().to_owned()
        };
    }

    pub fn open(&mut self, rsc_dir: String, opt_str: String) -> Result<(), KhaiiiError>{
        self.close();

        let rsc_dir_ptr = CString::new(rsc_dir).unwrap().into_raw();
        let opt_str_ptr = CString::new(opt_str).unwrap().into_raw();

        self.handle = unsafe {
            khaiii_open(rsc_dir_ptr, opt_str_ptr)
        };

        unsafe {
            CString::from_raw(rsc_dir_ptr);
            CString::from_raw(opt_str_ptr);
        }

        if self.handle < 0 {
            return Err(KhaiiiError::new(self.last_error()))
        }

        Ok(())
    }

    pub fn close(&mut self) {
        if self.handle >= 0 {
            unsafe {
                khaiii_close(self.handle);
            }
        }

        self.handle = -1;
    }

    pub fn analyze(&self, in_str: String, opt_str: String) -> Result<Vec<KhaiiiWord>, KhaiiiError> {
        let in_str_ptr = CString::new(in_str.clone()).unwrap().into_raw();
        let opt_str_ptr = CString::new(opt_str.clone()).unwrap().into_raw();
        let results = unsafe {
            khaiii_analyze(self.handle, in_str_ptr, opt_str_ptr)
        };
        
        if results.is_null() {
            unsafe {
                CString::from_raw(in_str_ptr);
                CString::from_raw(opt_str_ptr);
            }

            return Err(KhaiiiError::new(self.last_error()));
        }

        let align = KhaiiiApi::get_align(&in_str);
        let words: Vec<KhaiiiWord> = KhaiiiWord::make_words(in_str, results, &align);

        unsafe {
            khaiii_free_results(self.handle, results);
            CString::from_raw(in_str_ptr);
            CString::from_raw(opt_str_ptr);
        }

        Ok(words)
    }

    pub fn analyze_bfr_errpatch(&self, in_str: String, opt_str: String) -> Result<Vec<i16>, KhaiiiError> {
        let in_str_ptr = CString::new(in_str.clone()).unwrap().into_raw();
        let opt_str_ptr = CString::new(opt_str.clone()).unwrap().into_raw();
        let output: *mut c_short = vec![0i16; in_str.len() + 3].as_mut_ptr();

        let result = unsafe {
            khaiii_analyze_bfr_errpatch(self.handle, in_str_ptr, opt_str_ptr, output)
        };

        if result < 2 {
            unsafe {
                CString::from_raw(in_str_ptr);
                CString::from_raw(opt_str_ptr);
            }

            return Err(KhaiiiError::new(self.last_error()));
        }

        let output_arr = unsafe {
            slice::from_raw_parts(output, result as usize)
        }.to_vec();

        unsafe {
            CString::from_raw(in_str_ptr);
            CString::from_raw(opt_str_ptr);
        }

        Ok(output_arr)
    }


    pub fn last_error(&self) -> String {
        return unsafe {
            CStr::from_ptr(khaiii_last_error(self.handle)).to_str().unwrap().to_owned()
        }
    }

    pub fn set_log_level(&self, name: String, level: String) -> Result<(), KhaiiiError> {
        let name_ptr = CString::new(name).unwrap().into_raw();
        let level_ptr = CString::new(level).unwrap().into_raw();

        let result = unsafe {
            khaiii_set_log_level(name_ptr, level_ptr)
        };

        unsafe {
            CString::from_raw(name_ptr);
            CString::from_raw(level_ptr);
        }

        if result < 0 {
            return Err(KhaiiiError::new(self.last_error()))
        }

        Ok(())
    }

    pub fn set_log_levels(&self, name_level_pairs: String) -> Result<(), KhaiiiError> {
        let name_level_pairs_ptr = CString::new(name_level_pairs).unwrap().into_raw();

        let result = unsafe {
            khaiii_set_log_levels(name_level_pairs_ptr)
        };

        unsafe {
            CString::from_raw(name_level_pairs_ptr);
        }

        if result < 0 {
            return Err(KhaiiiError::new(self.last_error()))
        }

        Ok(())
    }

    fn get_align(in_str: &String) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        for (i, c) in in_str.chars().enumerate() {
            result.append(&mut vec![i; c.len_utf8()].to_owned());
        }

        result
    }
}