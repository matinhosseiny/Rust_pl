// Adapted from file parser.rs at:
// https://github.com/kenpratt/rusty_scheme

use let_lang_scanner::*;
use let_lang_exp::*;      // needed for building ASTs

use std::fmt;
use std::slice;

pub fn parse(tokens: &Vec<Token>) -> Result<LetLangExp, ParseErr> {
    Parser::parse(tokens)
}

pub struct ParseErr {
    message: String,
}

impl fmt::Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }}
impl fmt::Debug for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }}

macro_rules! parse_err {
    ($($arg:tt)*) => (
        return Err(ParseErr { message: format!($($arg)*)})
    )
}

// Parser datatype is struct with one field.
#[derive(Clone)]
struct Parser<'a> {
    tokens: slice::Iter<'a, Token>,
}

impl<'a> Parser<'a> {
    // This is a constructor for a Parser object.
    // Takes a reference to a token vector as input.
    // Builds Parser and then calls parse_let_lang_exp().
    // "parser" must be mutable b/c the tokens field is updated.
    fn parse(tokens: &Vec<Token>) -> Result<LetLangExp, ParseErr> {
        let mut parser = Parser { tokens: tokens.iter() };
        let ast_root = parser.parse_let_lang_exp();
        let option_next_tok = parser.tokens.next();
        match option_next_tok {
            Some(tok) => parse_err!("Toplevel: Extra input at end of parse: {:?}", tok),
            _          => ast_root,
        }
    }
    fn parse_let_lang_exp(&mut self) -> Result<LetLangExp, ParseErr> {
        let option_peek: Option<&Token> = self.tokens.clone().next();
        match option_peek {
                Some(peek_token) => self.parse_lle_work(peek_token.clone()),
                None             => parse_err!("Unexpected end of input")
            }
    }
    fn parse_lle_work(&mut self, peek_tok: Token) -> Result<LetLangExp, ParseErr> {
//        println!("Peek token: {:?}", peek_tok);  // for debugging
        match peek_tok {  // try returns Err(ParseErr) on early return
            Token::Integer(_n)    => {
                                      let e: LetLangExp = self.parse_const()?;
                                      Ok(e)},
            Token::Boolean(_b)    => {
                                      let e = self.parse_bool()?;
                                      Ok(e)},
            Token::Minus          => { // diff_exp
                                      let e = self.parse_diff()?;
                                      Ok(e)
                                    },
            Token::IsZero         => { // iszero exp
                                      let e = self.parse_iszero()?;
                                      Ok(e)
                                    },
            Token::If             => { // If-then-else expression
                                      let e = self.parse_if_then_else()?;
                                      Ok(e)
                                    },
            Token::Identifier(_s) => { // identifier cannot be a reserved word
                                      let e = self.parse_var()?;
                                      Ok(e)
                                    },
            Token::Let            => { // let-in expression
                                      let e = self.parse_let_in()?;
                                      Ok(e)
                                    },
                            _ => parse_err!("lle: Unexpected token type"),
            }
    }
    // build AST fragment for const
    fn parse_const(&mut self) -> Result<LetLangExp, ParseErr> {
        let val: i32;
        let option_tok: Option<&Token> = self.tokens.next();
        // Extract the token from Some() if found, else return with Err(ParseErr)
        let tok = match option_tok {
                    Some(t) => t,
                    _       => parse_err!("parse_integer: Int expected but EOI found."),
                    };
        // extract the integer value
        match tok.clone() {
            Token::Integer(i) => val = i,
            _                 => parse_err!("parse_const: Int token expected."),
        };
        Ok(LetLangExp::new_const_exp(val))  // Final returned value
    }
   // build AST fragment for boolean
   fn parse_bool(&mut self) -> Result<LetLangExp, ParseErr> {
        let val: bool;
        let option_tok: Option<&Token> = self.tokens.next();
        // Extract the token from Some() if found, else return with Err(ParseErr)
        let tok = match option_tok {
                    Some(t) => t,
                    _       => parse_err!("parse_bool: Bool expected but EOI found."),
                    };
        // extract the boolean value
        match tok.clone() {
            Token::Boolean(b) => val = b,
            _                 => parse_err!("parse_bool: Identifier token expected."),
        };
        Ok(LetLangExp::new_boolean(val))
    }
    // build AST fragment for variable
    fn parse_var(&mut self) -> Result<LetLangExp, ParseErr> {
        let var: String;
        let option_tok: Option<&Token> = self.tokens.next();
        // Extract the token from Some() if found, else return with Err(ParseErr)
        let tok = match option_tok {
                    Some(t) => t,
                    _       => parse_err!("parse_var: Ident expected but EOI found."),
                    };
        // extract the variable name string
        match tok.clone() {
            Token::Identifier(s) => var = s,
            _                    => parse_err!("parse_var: Identifier token expected."),
        };
        Ok(LetLangExp::new_var_exp(&var))
    }

    // Gets the variable name.
    // Advances input stream by one token.
    // Used in parse_let_in.
    fn get_string(&mut self) -> Result<String, ParseErr> {
        let option_tok: Option<&Token> = self.tokens.next();

        // Extracts the cloned token from the Option
        let tok: Token = match option_tok {
                            Some(token) => token.clone(),
                            _ => parse_err!("get_string1: Unexpected EOI")
                            };
        // Returns the Identifier in result form.
        let var: String =
            match tok {
                Token::Identifier(s) => s,
                _                    => parse_err!("get_string2: Unexpected token type"),
                };
        Ok(var)     // var is a String
        }

    // Matches to the next token if it is a specific type.
    // If successful, the input stream is advanced but the result is not used.
    // Used in parse_diff, parse_iszero, parse_if_then_else, parse_let_in.
    fn match_token(&mut self, tok: &Token)  -> Result<Option<LetLangExp>, ParseErr> {
        let option_tok: Option<&Token> = self.tokens.next();
        match option_tok {
             Some(tok2) => {if tok == tok2 { // checks if the token type matches
                                Ok(None)
                            } else {
                                parse_err!("Expected {:?} but found {:?}", tok, tok2)
                            }},
             _            => parse_err!("Expected {:?} but found EOI", tok)
         }}
    // build AST fragment for diff expression
    fn parse_diff(&mut self) -> Result<LetLangExp, ParseErr> {
        try!(self.match_token(&Token::Minus));   // return with Err(ParseErr) if no match
        try!(self.match_token(&Token::Lparen));
        let e1 = try!(self.parse_let_lang_exp()); // return with Err(ParseErr) if no parse
        try!(self.match_token(&Token::Comma));
        let e2 = try!(self.parse_let_lang_exp());
        try!(self.match_token(&Token::Rparen));
        Ok(LetLangExp::new_diff_exp(&e1, &e2))   // return Ok(LetLangExp::new_diff_exp())
    }
    fn parse_iszero(&mut self) -> Result<LetLangExp, ParseErr> {
        self.match_token(&Token::IsZero)?;
        self.match_token(&Token::Lparen)?;
        let e = self.parse_let_lang_exp()?;
        self.match_token(&Token::Rparen)?;
        Ok(LetLangExp::new_iszero(&e))
    }
    fn parse_if_then_else(&mut self) -> Result<LetLangExp, ParseErr> {
        self.match_token(&Token::If)?;
        let e1 = self.parse_let_lang_exp()?;
        self.match_token(&Token::Then)?;
        let e2 = self.parse_let_lang_exp()?;
        self.match_token(&Token::Else)?;
        let e3 = self.parse_let_lang_exp()?;
        Ok(LetLangExp::new_if_exp(&e1, &e2, &e3))
    }
    fn parse_let_in(&mut self) -> Result<LetLangExp, ParseErr> {
        self.match_token(&Token::Let)?;
        let s = self.get_string()?;               // match variable name
        self.match_token(&Token::Assign)?;        // match "="
        let e1 = self.parse_let_lang_exp()?;
        self.match_token(&Token::In)?;
        let e2 = self.parse_let_lang_exp()?;
        Ok(LetLangExp::new_let_exp(&s, &e1, &e2))
    }
}
