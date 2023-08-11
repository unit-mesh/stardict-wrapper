use std::path::PathBuf;
use stardict_lib::{no_cache, StarDict, StarDictStd};

pub struct WrapperDict {
    pub std_dict: StarDictStd,
}

impl WrapperDict {
    pub fn init(path: impl Into<PathBuf>) -> WrapperDict {
        let dict = no_cache(path).unwrap();
        return WrapperDict {
            std_dict: dict,
        };
    }

    pub fn translate(&mut self, word: &str) -> Option<String> {
        if let Ok(result) = self.std_dict.lookup(word) {
            if let None = result {
                return None;
            } else if let Some(defs) = result {
                if defs.len() == 0 {
                    return None;
                }

                let result = String::new();
                for def in &defs[0].segments {
                    return if let Some(pos) = def.text.find(';').or(def.text.find('\n')) {
                        let output = &def.text[..pos];
                        Some(output.into())
                    } else {
                        Some(def.text.clone())
                    }
                }

                return Some(result);
            } else {
                unreachable!()
            };
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        let mut dict = WrapperDict::init("/Users/phodal/Downloads/stardict-ncce-ce-2.4.2");
        let result = dict.translate("对齐");
        assert_eq!(result, Some("hello".to_string()));
    }
}