use crate::ffi::khaiii_morph_t;
use std::ffi::CStr;

#[derive(Debug)]
pub struct KhaiiiMorph {
    pub lex: String,
    pub tag: String,
    pub begin: usize,
    pub length: usize,
}

impl KhaiiiMorph {
    pub fn make_morphs(results: *mut khaiii_morph_t, align: &Vec<usize>) -> Vec<KhaiiiMorph> {
        let mut morphs: Vec<KhaiiiMorph> = Vec::new();
        let mut pointer: *mut khaiii_morph_t = results;

        unsafe {
            while !pointer.is_null() {
                let begin = (*pointer).begin as usize;
                let length = (*pointer).length as usize;

                let morph = KhaiiiMorph {
                    lex: CStr::from_ptr((*pointer).lex).to_str().unwrap().to_owned(),
                    begin: align[begin],
                    length: align[begin + length - 1] - align[begin] + 1,
                    tag: CStr::from_ptr((*pointer).tag).to_str().unwrap().to_owned(),
                };

                morphs.push(morph);
                pointer = (*pointer).next;
            }
        }

        morphs
    }
}