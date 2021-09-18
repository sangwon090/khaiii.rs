use crate::ffi::{khaiii_word_t};
use crate::KhaiiiMorph;

#[derive(Debug)]
pub struct KhaiiiWord {
    pub lex: String,
    pub begin: usize,
    pub length: usize,
    pub morphs: Vec<KhaiiiMorph>,
}

impl KhaiiiWord {
    pub fn make_words(in_str: String, results: *mut khaiii_word_t, align: &Vec<usize>) -> Vec<KhaiiiWord> {
        let mut words: Vec<KhaiiiWord> = Vec::new();
        let mut pointer: *mut khaiii_word_t = results;

        unsafe {
            while !pointer.is_null() {
                let begin = (*pointer).begin as usize;
                let length = (*pointer).length as usize;
                let end = begin + (*pointer).length as usize;
                
                let word = KhaiiiWord {
                    begin: align[begin],
                    length: align[begin + length - 1] - align[begin] + 1,
                    lex: String::from(&in_str[begin..end]),
                    morphs: KhaiiiMorph::make_morphs((*pointer).morphs, &align),
                };
    
                words.push(word);
                pointer = (*pointer).next;
            }
        }

        words
    }
}