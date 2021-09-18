use libc::{c_char, c_int};

#[link(name = "khaiii")]
extern {
    pub fn khaiii_version() -> *const c_char;
    pub fn khaiii_open(rsc_dir: *const c_char, opt_str: *const c_char) -> c_int;
    pub fn khaiii_close(handle: c_int);
    pub fn khaiii_analyze(handle: c_int, input: *const c_char, opt_str: *const c_char) -> *mut khaiii_word_t;
    pub fn khaiii_free_results(handle: c_int, results: *mut khaiii_word_t);
    pub fn khaiii_last_error(handle: c_int) -> *const c_char;
    pub fn khaiii_analyze_bfr_errpatch(handle: c_int, input: *mut c_char, opt_str: *mut c_char, output: *mut i16) -> c_int;
    pub fn khaiii_set_log_level(name: *const c_char, level: *const c_char) -> c_int;
    pub fn khaiii_set_log_levels(name_level_pairs: *const c_char) -> c_int;
}

#[repr(C)]
pub struct khaiii_morph_t {
    pub lex: *const c_char,
    pub tag: *const c_char,
    pub begin: c_int,
    pub length: c_int,
    pub reserved: [c_char; 8],
    pub next: *mut khaiii_morph_t,
}

#[repr(C)]
pub struct khaiii_word_t {
    pub begin: c_int,
    pub length: c_int,
    pub reserved: [c_char; 8],
    pub morphs: *mut khaiii_morph_t,
    pub next: *mut khaiii_word_t,
}