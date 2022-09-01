extern crate proc_macro;

use pyo3::prelude::*;
use std::include_str;
use std::collections::HashMap;


/// The direct Rust equivalent of the Python class of the same name.
#[derive(Debug, FromPyObject)]
pub struct TokenInfo {
    #[pyo3(attribute("type"))]
    pub token_type: usize,  /// type
    pub string: String, /// The token itself
    pub start: (usize,usize),  /// Start (line,col)
    pub end: (usize,usize),  /// End (line,col)
    pub line: String
}

/// We need to deal with not just a token list, but also the set of constants since the constants can change
#[derive(Debug)]
pub struct TokenSet {
    /// The tokens extracted from the Python file.
    pub tokens: Vec<TokenInfo>,

    /// A mapping of token type ids to string descriptions. This is functionally an enum, but the specific
    /// numbers seem to change, so we load it from Python to build a lookup table.
    pub token_types: HashMap<usize, String>,
}

impl TokenSet {
    pub fn get_token_type(&self, token_type: usize) -> Option<&String> {
        self.token_types.get(&token_type)
    }
}

//(usize,String,(usize,usize),(usize,usize),String)

/// Takes a string of bytes and returns the Python-tokenized version of it.
pub fn tokenize(input: &str) -> PyResult<TokenSet> {

    let pymodule_code = include_str!("make_tokens.py");

    Python::with_gil(|py| -> PyResult<TokenSet> {
        // We want to call tokenize.tokenize from Python.
        let pymodule = PyModule::from_code(py, pymodule_code, "make_tokens.py", "make_tokens")?;
        let t = pymodule.getattr("tokenize")?;
        let h = pymodule.getattr("get_token_numbers")?;
        assert!(t.is_callable());
        let args = (input,);
        let tokens: Vec<TokenInfo> = t.call1(args)?.extract()?;
        let token_numbers: HashMap<usize, String> = h.call0()?.extract()?;
        
        Ok(TokenSet{
            tokens: tokens,
            token_types: token_numbers,
        })
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
        assert_eq!(result.get_token_type(result.tokens[0].token_type), Some(&"ENCODING".to_string()));
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
