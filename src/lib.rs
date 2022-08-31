extern crate proc_macro;

use std::cmp::PartialEq;

use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::include_str;

/// All the numeric values tokens can have. Taken from here:
/// https://github.com/python/cpython/blob/main/Lib/token.py
#[derive(Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[pyclass]
#[repr(usize)]
pub enum TokenType {
    ENDMARKER = 0,
    NAME = 1,
    NUMBER = 2,
    STRING = 3,
    NEWLINE = 4,
    INDENT = 5,
    DEDENT = 6,
    LPAR = 7,
    RPAR = 8,
    LSQB = 9,
    RSQB = 10,
    COLON = 11,
    COMMA = 12,
    SEMI = 13,
    PLUS = 14,
    MINUS = 15,
    STAR = 16,
    SLASH = 17,
    VBAR = 18,
    AMPER = 19,
    LESS = 20,
    GREATER = 21,
    EQUAL = 22,
    DOT = 23,
    PERCENT = 24,
    LBRACE = 25,
    RBRACE = 26,
    EQEQUAL = 27,
    NOTEQUAL = 28,
    LESSEQUAL = 29,
    GREATEREQUAL = 30,
    TILDE = 31,
    CIRCUMFLEX = 32,
    LEFTSHIFT = 33,
    RIGHTSHIFT = 34,
    DOUBLESTAR = 35,
    PLUSEQUAL = 36,
    MINEQUAL = 37,
    STAREQUAL = 38,
    SLASHEQUAL = 39,
    PERCENTEQUAL = 40,
    AMPEREQUAL = 41,
    VBAREQUAL = 42,
    CIRCUMFLEXEQUAL = 43,
    LEFTSHIFTEQUAL = 44,
    RIGHTSHIFTEQUAL = 45,
    DOUBLESTAREQUAL = 46,
    DOUBLESLASH = 47,
    DOUBLESLASHEQUAL = 48,
    AT = 49,
    ATEQUAL = 50,
    RARROW = 51,
    ELLIPSIS = 52,
    COLONEQUAL = 53,
    OP = 54,
    AWAIT = 55,
    ASYNC = 56,
    TYPE_IGNORE = 57,
    TYPE_COMMENT = 58,
    SOFT_KEYWORD = 59,
    // These aren't used by the C tokenizer but are needed for tokenize.py
    ERRORTOKEN = 60,
    COMMENT = 61,
    NL = 62,
    ENCODING = 63,
    N_TOKENS = 64,
    // Special definitions for cooperation with parser
    NT_OFFSET = 256    
}

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

impl TokenInfo {
    fn get_token_type(&self) -> Result<TokenType, TryFromPrimitiveError<TokenType>> {
        TokenType::try_from(self.token_type)
    }
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
        assert_eq!(result[0].get_token_type().unwrap(), TokenType::ENCODING);
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
