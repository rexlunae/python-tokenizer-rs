extern crate proc_macro;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::include_str;

/// The direct Rust equivalent of the Python class of the same name.
#[derive(Debug, FromPyObject)]
pub struct TokenInfo {
    #[pyo3(attribute("type"))]
    token_type: usize,  /// type
    string: String, /// The token itself
    start: (usize,usize),  /// Start (line,col)
    end: (usize,usize),  /// End (line,col)
    line: String
}

//(usize,String,(usize,usize),(usize,usize),String)

/// Takes a string of bytes and returns the Python-tokenized version of it.
pub fn tokenize(input: &str) -> PyResult<Vec<TokenInfo>> {

    let pymodule_code = include_str!("make_tokens.py");
    println!("module: {}", pymodule_code);

    Python::with_gil(|py| -> PyResult<Vec<TokenInfo>> {
        // We want to call tokenize.tokenize from Python.
        let pymodule = PyModule::from_code(py, pymodule_code, "make_tokens.py", "make_tokens")?;
        let t = pymodule.getattr("tokenize")?;
        assert!(t.is_callable());
        let args = (input,);
        let tokens: Vec<TokenInfo> = t.call1(args)?.extract()?;
        
        println!("tokens: {:?}", tokens);

        Ok(tokens)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_encoding() {
        let result = tokenize(r#"
def foo():
    pass
"#).unwrap();
        assert_eq!(result[0].token_type, 62);
    }


    #[test]
    fn string_makes_token_stream() {
        let result = tokenize(r#"
def foo():
    pass
"#);
        println!("tokens: {:?}", result);
        //assert_eq!(result, 4);
    }
}
