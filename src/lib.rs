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
            match result {
                None => {
                    return None;
                }
                Some(defs) => {
                    if defs.len() == 0 {
                        return return None;
                    }

                    let result = String::new();
                    for def in &defs[0].segments {
                        return if let Some(pos) = def.text.find(';') {
                            let output = &def.text[..pos];
                            Some(output.into())
                        } else {
                            let first_result = def.text.as_str();
                            Some(first_result.into())
                        }
                    }

                    return Some(result);
                }
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
        let mut dict = WrapperDict::init("/Users/phodal/Downloads/stardict-langdao-ce-gb-2.4.2");
        let result = dict.translate("员工");
        assert_eq!(result, Some("hello".to_string()));
    }
}