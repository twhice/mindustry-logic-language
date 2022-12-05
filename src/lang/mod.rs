use crate::error::Err;
use crate::io::Meg;
mod en_us;
mod zh_cn;

pub enum Language {
    Chinese,
    English,
}

pub fn get_errmeg(err: &Err) -> String {
    match crate::LANGUAGE {
        Language::Chinese => zh_cn::get_errmeg(err),
        Language::English => en_us::get_errmeg(err),
    }
}

pub fn get_buildin_meg(meg: &Meg) -> String {
    match crate::LANGUAGE {
        Language::Chinese => zh_cn::get_buildin_meg(meg),
        Language::English => en_us::get_buildin_meg(meg),
    }
}
