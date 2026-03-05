use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(utf8 = true)]
#[logos(skip r"[ \t\n\f]+")] // skip spaces, tabs, newlines, ...
pub enum Token {
    #[regex(r"-?\d+i8")]
    Integer8,
    #[regex(r"-?\d+i16")]
    Integer16,
    #[regex(r"-?\d+i32")]
    Integer32,
    #[regex(r"-?\d+i64")]
    Integer64,
    #[regex(r"-?\d+u8")]
    Unsigned8,
    #[regex(r"-?\d+u16")]
    Unsigned16,
    #[regex(r"-?\d+u32")]
    Unsigned32,
    #[regex(r"-?\d+u64")]
    Unsigned64,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("/")]
    Slash,
    #[token("*")]
    Star,
}
