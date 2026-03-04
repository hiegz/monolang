use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(utf8 = true)]
#[logos(skip r"[ \t\n\f]+")] // skip spaces, tabs, newlines, ...
pub enum Token {
    #[token("let")]
    Let,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
}
