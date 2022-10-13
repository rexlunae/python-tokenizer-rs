extern crate proc_macro;

use pyo3::prelude::*;
use std::include_str;
use std::collections::HashMap;


/// The direct Rust equivalent of the Python class of the same name,
/// albeit augmented with the token type text as a string.
#[derive(Clone, Debug, FromPyObject)]
pub struct TokenInfo {
    #[pyo3(attribute("type"))]
    pub token_type: usize,  /// type
    pub string: String, /// The token itself
    pub start: (usize,usize),  /// Start (line,col)
    pub end: (usize,usize),  /// End (line,col)
    pub line: String,
    pub token_text: String,
}

/// We need to deal with not just a token list, but also the set of
/// constants since the constants can change.
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
pub fn tokenize(input: &str) -> PyResult<Vec<TokenInfo>> {

    let pymodule_code = include_str!("make_tokens.py");

    Python::with_gil(|py| -> PyResult<Vec<TokenInfo>> {
        // We want to call tokenize.tokenize from Python.
        let pymodule = PyModule::from_code(py, pymodule_code, "make_tokens.py", "make_tokens")?;
        let t = pymodule.getattr("augment_tokens")?;
        assert!(t.is_callable());
        let args = (input,);
        let tokens: Vec<TokenInfo> = t.call1(args)?.extract()?;
        
        Ok(tokens)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_token_stream() {
        let result = tokenize("#test comment
def foo():
    pass
").unwrap();
        println!("tokens: {:?}", result);
        assert_eq!(result[0].token_text, "COMMENT");
        assert_eq!(result[1].token_text, "NL");
        assert_eq!(result[2].token_text, "NAME");
        assert_eq!(result[3].token_text, "NAME");
        assert_eq!(result[4].token_text, "OP");
        assert_eq!(result[5].token_text, "OP");
        assert_eq!(result[6].token_text, "OP");
        assert_eq!(result[7].token_text, "NEWLINE");
        assert_eq!(result[8].token_text, "INDENT");
        assert_eq!(result[9].token_text, "NAME");
        assert_eq!(result[10].token_text, "NEWLINE");
        assert_eq!(result[11].token_text, "DEDENT");
        assert_eq!(result[12].token_text, "ENDMARKER");
    }

}
