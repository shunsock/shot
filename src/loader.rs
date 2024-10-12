mod loader_error;

use loader_error::LoaderError;

#[derive(Debug, PartialEq)]
pub struct Loader {
    pub source_code: String,
    pub source_code_vector: Vec<String>,
}

impl Loader {
    pub fn new(content: String) -> Self {
        Loader {
            source_code: content.clone(),
            source_code_vector: content.split("\n").map(|s| s.to_string()).collect(),
        }
    }

    pub fn load(content: Option<String>, file_path: Option<String>) -> Result<Self, LoaderError> {
        let content_option_is_used: bool = content.is_some();
        let file_path_option_is_used: bool = file_path.is_some();

        if content_option_is_used && file_path_option_is_used {
            return Err(LoaderError::TooManyOptions);
        }

        if !content_option_is_used && !file_path_option_is_used {
            return Err(LoaderError::TooFewOptions);
        }

        // ソースコードが指定されている場合
        if let Some(content) = content {
            return Ok(Loader::load_from_content(content));
        }

        // ファイルパスが指定されている場合
        Loader::load_from_file(file_path.unwrap())
    }

    fn load_from_content(content: String) -> Self {
        Loader {
            source_code: content.clone(),
            source_code_vector: content.split("\n").map(|s| s.to_string()).collect(),
        }
    }

    fn load_from_file(file_path: String) -> Result<Self, LoaderError> {
        let content = std::fs::read_to_string(file_path.clone());

        let source_code: String = match content {
            Ok(content) => content,
            Err(_) => {
                return Err(LoaderError::FileNotFound { file_path });
            }
        };

        Ok(Loader {
            source_code: source_code.clone(),
            source_code_vector: source_code.split("\n").map(|s| s.to_string()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_content() {
        let content = "1 + 1".to_string();
        let source_code_loader = Loader::load_from_content(content);

        assert_eq!(source_code_loader.source_code, "1 + 1".to_string());
    }

    #[test]
    fn test_load_from_file() {
        let source_code_loader = Loader::load_from_file("tests/loader/exist.shot".to_string());

        assert_eq!(
            source_code_loader.unwrap().source_code,
            "1 + 1;\n2 + 2;\n\n".to_string()
        );
    }

    #[test]
    fn test_load_from_file_not_found() {
        let source_code_loader =
            Loader::load_from_file("tests/loader/source_code_loader_not_found.txt".to_string());

        assert_eq!(
            source_code_loader,
            Err(LoaderError::FileNotFound {
                file_path: "tests/loader/source_code_loader_not_found.txt".to_string()
            })
        );
    }
}
