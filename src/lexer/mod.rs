/*
lexer.rs is a lexer for the src language
*/

use std::{fmt::Display, hash::Hash, iter::Iterator, iter::Peekable, str::Chars};

use okstd::prelude::*;

// Identifier
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Variable<'input> {
    // $$ is the process ID of the shell
    ProcessID,
    // $! is the process ID of the last background command
    LastBackgroundProcessID,
    // $? is the exit status of the last command executed
    LastCommandExitStatus,
    // $- is the current option flags as specified upon invocation, by the set built-in command, or by the shell invocation environment
    CurrentOptionFlags,
    // $@ is the positional parameters, starting from one
    PositionalParameters,
    // $# is the number of positional parameters in decimal
    PositionalParametersCount,
    // $0 is the name of the shell or shell script
    ShellName,
    // $1...$9 are the positional parameters, starting from zero
    PositionalParameter(usize),
    // ${parameter} is the value of the variable parameter
    Parameter(&'input str),
    // ${parameter:-word} is the value of the variable parameter if it is set; otherwise, the expansion of word is substituted
    ParameterDefault(&'input str, &'input str),
}

impl<'input> Display for Variable<'input> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::ProcessID => write!(f, "$$"),
            Variable::LastBackgroundProcessID => write!(f, "$!"),
            Variable::LastCommandExitStatus => write!(f, "$?"),
            Variable::CurrentOptionFlags => write!(f, "$-"),
            Variable::PositionalParameters => write!(f, "$@"),
            Variable::PositionalParametersCount => write!(f, "$#"),
            Variable::ShellName => write!(f, "$0"),
            Variable::PositionalParameter(i) => write!(f, "${}", i),
            Variable::Parameter(p) => write!(f, "${}", p),
            Variable::ParameterDefault(p, w) => write!(f, "${}:{}", p, w),
        }
    }
}

// LexicalError
#[derive(Debug, PartialEq, Clone)]
pub enum LexicalError {
    // Unexpected character
    UnexpectedCharacter(char),
    // Unterminated string
    UnterminatedString,
    // Invalid number format
    InvalidNumberFormat,
    // Invalid variable format
    InvalidVariableFormat,
    // Unexpected end of input
    UnexpectedEndOfInput,
    // Invalid state emission
    InvalidStateEmission(State),
}

type Result<T> = std::result::Result<T, LexicalError>;

#[derive(Debug, PartialEq, Clone)]
pub struct Spanned<T, P = Position> {
    pub node: T,
    pub start: usize,
    pub end: usize,
    pub pos: P,
}

impl Display for Spanned<Token<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}, start: {} end: {}",
            self.node, self.start, self.end
        )
    }
}

impl<T, P> Spanned<T, P> {
    pub fn new(node: T, start: usize, end: usize, pos: P) -> Self {
        Spanned {
            node,
            start,
            end,
            pos,
        }
    }
}

impl Spanned<Token<'_>> {
    pub fn len(&self) -> usize {
        self.node.string_repr().chars().count()
    }
}

// Position struct
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Position {
    pub line: usize,
    pub col: usize,
    pub size: usize,
}

// new function for Position
impl Position {
    pub fn new(line: usize, col: usize, size: usize) -> Self {
        Self { line, col, size }
    }
}

// display trait implementation for Position
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "line: {}, col: {}, size: {}",
            self.line, self.col, self.size
        )
    }
}

// display trait implementation for Token
impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Word<'input> {
    Let,
    Const,
    Fn,
    If,
    Else,
    Return,
    Match,
    For,
    While,
    Break,
    Continue,
    True,
    False,
    Null,
    Action,
    Enum,
    Impl,
    Import,
    None,
    Struct,
    Effect,
    When,
    Use,
    From,
    Where,
    Self_,
    Pub,
    Priv,
    Ident(&'input str),
    FnIdent(&'input str),
    Any(&'input str),
}

impl<'input> Word<'input> {
    fn chars(&self) -> Chars<'_> {
        match self {
            Word::Let => "let".chars(),
            Word::Const => "const".chars(),
            Word::Fn => "fn".chars(),
            Word::If => "if".chars(),
            Word::Else => "else".chars(),
            Word::Return => "return".chars(),
            Word::Match => "match".chars(),
            Word::For => "for".chars(),
            Word::While => "while".chars(),
            Word::Break => "break".chars(),
            Word::Continue => "continue".chars(),
            Word::True => "true".chars(),
            Word::False => "false".chars(),
            Word::Null => "null".chars(),
            Word::When => "when".chars(),
            Word::Ident(ident) => ident.chars(),
            Word::FnIdent(ident) => ident.chars(),
            Word::Any(word) => word.chars(),
            Word::Action => "action".chars(),
            Word::Enum => "enum".chars(),
            Word::Impl => "impl".chars(),
            Word::Import => "import".chars(),
            Word::None => "none".chars(),
            Word::Struct => "struct".chars(),
            Word::Effect => "effect".chars(),
            Word::Use => "use".chars(),
            Word::From => "from".chars(),
            Word::Where => "where".chars(),
            Word::Self_ => "self".chars(),
            Word::Pub => "pub".chars(),
            Word::Priv => "priv".chars(),
        }
    }
}

// token types debug
#[derive(Debug, PartialEq, Clone)]
pub enum Token<'input> {
    // Operators
    Pipe,      // |
    Ampersand, // &
    Semicolon, // ;
    Equals,    // =
    // Redirections
    LessThan,    // <
    GreaterThan, // >
    // Identifiers
    Variable(Variable<'input>), // $a-z, $A-Z, $0-9, $_
    // Literals
    Word(Word<'input>),  // a-z, A-Z, 0-9, _
    String(&'input str), // "..."
    // Comments
    Comment(&'input str), // #
    // Numbers
    Integer(i64), // 0-9
    Float(f64),   // 0-9
    // Special
    Eof,          // EOF
    NewLine,      // \n
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Dot,          // .
    Colon,        // :
    Underscore,   // _
    Minus,        // -
    Plus,         // +
    Arrow,        // ->
    FatArrow,     // =>
    Divide,       // /
    Multiply,     // *
    Percent,      // %
    Dollar,       // $
    Exclamation,  // !
    Question,     // ?
    Tilde,        // ~
    At,           // @
    Caret,        // ^
    Shebang,      // #!
}

impl<'input> Token<'input> {
    // deprecated
    #[deprecated(note = "to_chars is deprecated, use to_string instead")]
    fn to_chars(&'input self) -> Chars<'input> {
        match self {
            Token::Pipe => "|".chars(),
            Token::Ampersand => "&".chars(),
            Token::Semicolon => ";".chars(),
            Token::Equals => "=".chars(),
            Token::LessThan => "<".chars(),
            Token::GreaterThan => ">".chars(),
            Token::Variable(_identifier) => {
                // Implement the conversion to chars for Variable
                // based on its fields
                "".chars()
            }
            Token::Word(word) => word.chars(),
            Token::String(string) => string.chars(),
            Token::Comment(comment) => comment.chars(),
            Token::Integer(_number) => "".chars(),
            Token::Float(_number) => "".chars(),
            Token::Eof => "".chars(),
            Token::NewLine => "\n".chars(),
            Token::LeftParen => "(".chars(),
            Token::RightParen => ")".chars(),
            Token::LeftBrace => "{".chars(),
            Token::RightBrace => "}".chars(),
            Token::LeftBracket => "[".chars(),
            Token::RightBracket => "]".chars(),
            Token::Comma => ",".chars(),
            Token::Colon => ":".chars(),
            Token::Underscore => "_".chars(),
            Token::Minus => "-".chars(),
            Token::Plus => "+".chars(),
            Token::Arrow => "->".chars(),
            Token::FatArrow => "=>".chars(),
            Token::Divide => "/".chars(),
            Token::Multiply => "*".chars(),
            Token::Percent => "%".chars(),
            Token::Dollar => "$".chars(),
            Token::Exclamation => "!".chars(),
            Token::Question => "?".chars(),
            Token::Tilde => "~".chars(),
            Token::At => "@".chars(),
            Token::Caret => "^".chars(),
            Token::Dot => ".".chars(),
            Token::Shebang => "#!".chars(),
        }
    }

    fn string_repr(&'input self) -> String {
        match self {
            Token::Pipe => "|".to_string(),
            Token::Ampersand => "&".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Equals => "=".to_string(),
            Token::LessThan => "<".to_string(),
            Token::GreaterThan => ">".to_string(),
            Token::Variable(variable) => variable.to_string(),
            Token::Word(word) => word.chars().collect(),
            Token::String(string) => string.to_string(),
            Token::Comment(comment) => comment.to_string(),
            Token::Integer(number) => number.to_string(),
            Token::Float(number) => number.to_string(),
            Token::Eof => "".to_string(),
            Token::NewLine => "\n".to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::LeftBrace => "{".to_string(),
            Token::RightBrace => "}".to_string(),
            Token::LeftBracket => "[".to_string(),
            Token::RightBracket => "]".to_string(),
            Token::Comma => ",".to_string(),
            Token::Colon => ":".to_string(),
            Token::Underscore => "_".to_string(),
            Token::Minus => "-".to_string(),
            Token::Plus => "+".to_string(),
            Token::Arrow => "->".to_string(),
            Token::FatArrow => "=>".to_string(),
            Token::Divide => "/".to_string(),
            Token::Multiply => "*".to_string(),
            Token::Percent => "%".to_string(),
            Token::Dollar => "$".to_string(),
            Token::Exclamation => "!".to_string(),
            Token::Question => "?".to_string(),
            Token::Tilde => "~".to_string(),
            Token::At => "@".to_string(),
            Token::Caret => "^".to_string(),
            Token::Dot => ".".to_string(),
            Token::Shebang => "#!".to_string(),
        }
    }
}

impl<'input> Iterator for Token<'input> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.to_chars().next()
    }
}

pub struct Lexer<'input> {
    input: &'input str,
    pos: usize,
    line: usize,
    col: usize,
    state: State,
    buffer: String,
    peekable: Peekable<Chars<'input>>,
    last_char: Option<char>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str, pos: usize) -> Self {
        Self {
            input,
            pos,
            line: 0, // Change from 1 to 0
            col: 0,  // Change from 1 to 0
            state: State::Program,
            buffer: String::new(),
            peekable: input.chars().peekable(),
            last_char: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Quotation {
    Single,
    Double,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Comment,
    Eof,
    NewLine,
    String(Quotation),
    Op,
    Variable,
    Word,
    Number,
    Program,
    Shebang,
    Any,
}
macro_rules! set_state {
    ($self:expr, $state:expr;) => {{
        $self.state = $state;
    }};
}
macro_rules! emit {
    ($self:expr, $state:expr => ?) => {{
        let r = $self.emit_buffer()?;
        $self.buffer.clear();
        emit!($self, $state => r)
    }};
    ($self:expr, $state:expr => $token:expr) => {{
        let start = $self.pos;

        for c in $token.string_repr().chars() {
            $self.advance(c);
        }

        let end = $self.pos;
        let pos = Position::new(
            $self.line,
            $self.col - $self.buffer.len(),
            end - start,
        );
        $self.state = $state;
        let token = $token;
        let token = Spanned::new(token, start, end, pos);
        Ok(token)
    }};
}

// Lexer trait implementation
impl<'input> Lexer<'input> {
    pub fn input(&self) -> &'input str {
        self.input
    }

    fn push(&mut self) -> bool {
        let c = self.peekable.next().unwrap();
        self.buffer.push(c);
        let finished = (self.pos as i32) + self.buffer.len() as i32 >= self.input.len() as i32;
        finished
    }

    fn ignore(&mut self) -> bool {
        let c = self.peekable.next().unwrap();
        self.advance(c)
    }

    fn advance(&mut self, c: char) -> bool {
        if self.pos + 1 > self.input.len() {
            return false;
        }
        self.pos += 1;
        self.last_char = Some(c);
        if c == '\n' {
            self.line += 1;
            self.col = 0; // Change from 1 to 0
        } else {
            self.col += 1;
        }
        let finished = self.pos >= self.input.len();
        finished
    }

    fn peek(&mut self) -> Option<char> {
        return self.peekable.peek().copied();
    }

    // emit emit's the current token
    fn emit_buffer(&mut self) -> Result<Token<'input>> {
        let start = self.pos;
        let end = self.pos + self.buffer.len();

        match self.state {
            // these states cannot emit tokens
            State::Program => Err(LexicalError::InvalidStateEmission(State::Program)),
            State::Op => Ok(
                match self
                    .buffer
                    .chars()
                    .next()
                    .ok_or(LexicalError::UnexpectedEndOfInput)?
                {
                    '(' => Token::LeftParen,
                    ')' => Token::RightParen,
                    '{' => Token::LeftBrace,
                    '}' => Token::RightBrace,
                    '>' => Token::GreaterThan,
                    '<' => Token::LessThan,
                    '|' => Token::Pipe,
                    '&' => Token::Ampersand,
                    ';' => Token::Semicolon,
                    ',' => Token::Comma,
                    ':' => Token::Colon,
                    '_' => Token::Underscore,
                    '+' => Token::Plus,
                    '*' => Token::Multiply,
                    '[' => Token::LeftBracket,
                    ']' => Token::RightBracket,
                    '%' => Token::Percent,
                    '@' => Token::At,
                    '/' => Token::Divide,
                    '.' => Token::Dot,
                    '-' => {
                        if self.buffer.len() == 1 {
                            Token::Minus
                        } else if self.buffer == "->" {
                            Token::Arrow
                        } else {
                            return Err(LexicalError::UnexpectedCharacter(
                                self.buffer.chars().next().unwrap(),
                            ));
                        }
                    }
                    '=' => Token::Equals,
                    _ => {
                        return Err(LexicalError::UnexpectedCharacter(
                            self.buffer.chars().next().unwrap(),
                        ))
                    }
                },
            ),
            State::Any => Err(LexicalError::InvalidStateEmission(State::Any)),
            // these states can emit tokens
            State::Comment => {
                let comment = self
                    .input
                    .get(start..end)
                    .ok_or(LexicalError::UnexpectedEndOfInput)?;
                Ok(Token::Comment(comment))
            }
            State::Variable => {
                let variable = self.buffer.clone();
                let identifier = match variable.as_str() {
                    "$$" => Variable::ProcessID,
                    "$?" => Variable::LastCommandExitStatus,
                    "$!" => Variable::LastBackgroundProcessID,
                    "$-" => Variable::CurrentOptionFlags,
                    "$0" => Variable::ShellName,
                    "$#" => Variable::PositionalParametersCount,
                    _ => {
                        if variable.starts_with('$') && variable.len() > 1 {
                            let number = variable[1..]
                                .parse()
                                .map_err(|_| LexicalError::InvalidVariableFormat)?;
                            Variable::PositionalParameter(number)
                        } else {
                            let var = self
                                .input
                                .get(start..end)
                                .ok_or(LexicalError::UnexpectedEndOfInput)?;
                            Variable::Parameter(var)
                        }
                    }
                };

                Ok(Token::Variable(identifier))
            }
            State::Word => {
                let word = self
                    .input
                    .get(start..end)
                    .ok_or(LexicalError::UnexpectedEndOfInput)?;
                let word = match word {
                    "let" => Word::Let,
                    "const" => Word::Const,
                    "fn" => Word::Fn,
                    "if" => Word::If,
                    "else" => Word::Else,
                    "return" => Word::Return,
                    "match" => Word::Match,
                    "for" => Word::For,
                    "while" => Word::While,
                    "break" => Word::Break,
                    "continue" => Word::Continue,
                    "true" => Word::True,
                    "false" => Word::False,
                    "null" => Word::Null,
                    "action" => Word::Action,
                    "enum" => Word::Enum,
                    "impl" => Word::Impl,
                    "import" => Word::Import,
                    "none" => Word::None,
                    "struct" => Word::Struct,
                    "effect" => Word::Effect,
                    "when" => Word::When,
                    "use" => Word::Use,
                    "from" => Word::From,
                    "where" => Word::Where,
                    "self" => Word::Self_,
                    "pub" => Word::Pub,
                    "priv" => Word::Priv,
                    _ => Word::Ident(word),
                };
                Ok(Token::Word(word))
            }
            State::String(quotation) => {
                let last_char = self.buffer.chars().last();
                let quote = if quotation == Quotation::Double {
                    Some('"')
                } else {
                    Some('\'')
                };
                if last_char != quote {
                    return Err(LexicalError::UnterminatedString);
                }
                let string = self
                    .input
                    .get(start..end)
                    .ok_or(LexicalError::UnexpectedEndOfInput)?;
                Ok(Token::String(string))
            }
            State::Number => {
                let number = self.buffer.clone();
                if number.contains('.') {
                    let float = number
                        .parse()
                        .map_err(|_| LexicalError::InvalidNumberFormat)?;

                    Ok(Token::Float(float))
                } else if number.starts_with("0x") {
                    let integer = i64::from_str_radix(&number[2..], 16)
                        .map_err(|_| LexicalError::InvalidNumberFormat)?;
                    Ok(Token::Integer(integer))
                } else {
                    let integer = number
                        .parse()
                        .map_err(|_| LexicalError::InvalidNumberFormat)?;
                    Ok(Token::Integer(integer))
                }
            }
            State::NewLine => Ok(Token::NewLine),
            State::Eof => Ok(Token::Eof),
            State::Shebang => Ok(Token::Shebang),
        }
    }

    fn skip_ws(&mut self) -> Result<()> {
        while let Some(c) = self.peek() {
            match c {
                ' ' => {
                    self.ignore();
                }
                '\t' => {
                    self.ignore();
                }
                '#' => {
                    set_state!(self, State::Comment;);
                    return Ok(());
                }
                '"' => {
                    set_state!(self, State::String(Quotation::Double););
                    return Ok(());
                }
                '\'' => {
                    set_state!(self, State::String(Quotation::Single););
                    return Ok(());
                }
                '$' => {
                    set_state!(self, State::Variable;);
                    return Ok(());
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    set_state!(self, State::Word;);
                    return Ok(());
                }
                '0'..='9' => {
                    set_state!(self, State::Number;);
                    return Ok(());
                }
                '\n' => {
                    set_state!(self, State::NewLine;);
                    return Ok(());
                }
                '(' | ')' | '{' | '}' | '>' | '<' | '|' | '&' | ';' | ',' | ':' | '+' | '*'
                | '.' | '[' | ']' | '%' | '@' | '/' | '-' | '=' | '!' => {
                    set_state!(self, State::Op;);
                    debug!("to state: {:?}", self.state);
                    return Ok(());
                }
                _ => {
                    return Err(LexicalError::UnexpectedCharacter(c));
                }
            }
            if self.pos >= self.input.len() {
                break;
            }
        }
        if self.pos >= self.input.len() {
            set_state!(self, State::Eof;);
        }
        Ok(())
    }

    fn consume_op(&mut self) -> Result<Spanned<Token<'input>>> {
        if let Some(c) = self.peek() {
            debug!("consume_op: {}", c);
            if self.state != State::Op {
                return Err(LexicalError::InvalidStateEmission(self.state));
            }
            match c {
                '(' | ')' | '{' | '}' | '>' | '<' | '|' | '&' | ';' | ',' | ':' | '_' | '+'
                | '.' | '/' | '*' | '[' | ']' | '%' | '@' => {
                    let state = if self.push() { State::Eof } else { State::Any };
                    return emit!(self, state => ?);
                }
                '=' => {
                    self.push();
                    if let Some('>') = self.peek() {
                        self.push();
                        return emit!(self, State::Any => ?);
                    } else {
                        let state = if self.pos == self.input.len() {
                            State::Eof
                        } else {
                            State::Any
                        };
                        return emit!(self, state => ?);
                    }
                } // - and ->
                '-' => {
                    self.push();
                    match self.peek() {
                        Some('>') => {
                            self.push();
                            return emit!(self, State::Any => ?);
                        }
                        Some('0'..='9') => {
                            set_state!(self, State::Number;);
                            return self.consume_number();
                        }
                        _ => {
                            let state = if self.push() { State::Eof } else { State::Any };
                            return emit!(self, state => ?);
                        }
                    }
                }
                '/' => {
                    let state = if self.push() { State::Eof } else { State::Any };
                    match self.peek() {
                        Some(' ') => {
                            return emit!(self, state => ?);
                        }
                        _ => {
                            return emit!(self, state => ?);
                        }
                    }
                } // / and /directory/file
                '!' => {
                    let state = if self.push() { State::Eof } else { State::Any };
                    if let Some('#') = self.peek() {
                        self.push();
                        return emit!(self, State::Any => ?);
                    } else {
                        return emit!(self, state => ?);
                    }
                }
                _ => {
                    return Err(LexicalError::UnexpectedCharacter(c));
                }
            }
        }
        emit!(self, self.state => Token::Eof)
    }

    // comment state
    fn consume_comment(&mut self) -> Result<Spanned<Token<'input>>> {
        loop {
            match self.peek() {
                Some('!') => {
                    let state = if self.push() { State::Eof } else { State::Any };
                    set_state!(self, State::Shebang;);
                    return emit!(self, state => ?);
                }
                Some('\n') => {
                    return emit!(self, State::NewLine => ?);
                }
                // if the option is none, break
                None => {
                    return emit!(self, State::Any => ?);
                }
                _ => {
                    if self.push() {
                        return emit!(self, State::Eof => ?);
                    }
                }
            }
        }
    }

    // consume word
    fn consume_word(&mut self) -> Result<Spanned<Token<'input>>> {
        while let Some(c) = self.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '/' | '_' => {
                    if self.push() {
                        return emit!(self, State::Eof => ?);
                    }
                }
                _ => {
                    break;
                }
            }
        }
        return emit!(self, State::Any => ?);
    }

    // consume number
    fn consume_number(&mut self) -> Result<Spanned<Token<'input>>> {
        debug!("consume_number");
        while let Some(c) = self.peek() {
            match c {
                '0'..='9' => {
                    if self.push() {
                        debug!("finished");
                        return emit!(self, State::Eof => ?);
                    }
                }
                // . is only allowed once
                '.' => {
                    if self.buffer.contains('.') {
                        break;
                    } else if self.push() {
                        // this is a violation as it is not a number
                        // so return error
                        return Err(LexicalError::InvalidNumberFormat);
                    }
                }
                // if the first character is a 0, then the next character can be x
                'x' => {
                    if self.buffer.starts_with('0') {
                        if self.push() {
                            debug!("buffer: {}", self.buffer);
                            return emit!(self, State::Number => ?);
                        }
                    } else {
                        break;
                    }
                }
                // also hex numbers, only if the buffer starts with 0x
                'a'..='f' | 'A'..='F' => {
                    if self.buffer.starts_with("0x") {
                        if self.push() {
                            debug!("buffer: {}", self.buffer);
                            return emit!(self, State::Number => ?);
                        }
                    }
                    // handle scientific notation
                    else if self.buffer.contains(".") && c == 'e' {
                        if self.push() {
                            debug!("buffer: {}", self.buffer);
                            return emit!(self, State::Number => ?);
                        }
                    } else {
                        break;
                    }
                }
                _ => {
                    debug!("breaking");
                    return emit!(self, State::Any => ?);
                }
            }
        }
        return emit!(self, State::Eof => ?);
    }

    fn consume_newline(&mut self) -> Result<Spanned<Token<'input>>> {
        match self.peek() {
            Some('\n') => {
                let state = if self.push() { State::Eof } else { State::Any };
                return emit!(self, state => ?);
            }
            _ => {
                return emit!(self, State::Any => Token::NewLine);
            }
        }
    }

    fn consume_string_literal(&mut self, quotation: Quotation) -> Result<Spanned<Token<'input>>> {
        // loop until the you see the same quotation mark as the one you started with
        // or if you see an escape character
        self.push();
        while let Some(c) = self.peek() {
            match c {
                '"' => {
                    if quotation == Quotation::Double {
                        let state = if self.push() { State::Eof } else { State::Any };
                        return emit!(self, state => ?);
                    }
                }
                '\'' => {
                    if quotation == Quotation::Single {
                        let state = if self.push() { State::Eof } else { State::Any };
                        return emit!(self, state => ?);
                    }
                }
                _ => {
                    if self.push() {
                        self.state = State::Eof;
                        break;
                    }
                }
            }
        }
        return Err(LexicalError::UnexpectedEndOfInput);
    }

    fn consume_variable(&mut self) -> Result<Spanned<Token<'input>>> {
        // ignore $
        self.ignore();
        while let Some(c) = self.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '.' => {
                    if self.push() {
                        return emit!(self, State::Any => ?);
                    }
                }
                _ => {
                    break;
                }
            }
        }
        return emit!(self, State::Op => ?);
    }
}

// Iterator Trait implementation for self<
impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() && self.state != State::Eof {
            self.state = State::Eof;
            return None;
        } else if self.pos >= self.input.len() {
            return None;
        }

        if self.state == State::Program || self.state == State::Any {
            self.skip_ws().unwrap();
        }
        let res = match self.state {
            State::Op => self.consume_op(),
            State::Comment => self.consume_comment(),
            State::Eof => {
                return None;
            }
            State::NewLine => self.consume_newline(),
            State::String(quotation) => self.consume_string_literal(quotation),
            State::Variable => self.consume_variable(),
            State::Word => self.consume_word(),
            State::Number => self.consume_number(),
            State::Any | State::Program => {
                return None;
            }
            State::Shebang => todo!(),
        };
        debug!(
            ">>> state: {:?}, res: {:?}, pos: {}, line: {}, col: {}",
            self.state, res, self.pos, self.line, self.col
        );
        self.buffer.clear();
        match res {
            Ok(token) => {
                match token.node {
                    Token::Eof => {
                        return None;
                    }
                    _ => {}
                }
                return Some(token);
            }
            _ => {
                return None;
            }
        }
        // Removed the panic! as it's now unreachable.
    }
}

struct TokenStreamDisplay<'input>(Vec<Spanned<Token<'input>>>);

impl Display for TokenStreamDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.0 {
            write!(
                f,
                "- {}, {}:{}\n",
                token.node, token.pos.line, token.pos.col
            )?;
        }
        Ok(())
    }
}

impl<'input> From<Vec<Spanned<Token<'input>>>> for TokenStreamDisplay<'input> {
    fn from(tokens: Vec<Spanned<Token<'input>>>) -> Self {
        TokenStreamDisplay(tokens)
    }
}

#[cfg(test)]
mod lexer_prop_tests;
#[cfg(test)]
mod lexer_snap_tests;

pub struct TripleIterator<'input>(Lexer<'input>);

#[derive(Debug, Clone, Default, Copy)]
pub struct Location {
    pub offset: usize,
    pub line: usize,
    pub col: usize,
}

impl Eq for Location {}

// we can ignore the line and col fields, offset is cannonical
impl Hash for Location {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.offset.hash(state);
    }
}
// we can ignore the line and col fields, offset is cannonical
impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.offset.cmp(&other.offset))
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl<'input> TripleIterator<'input> {
    pub fn new(input: &'input str) -> Self {
        TripleIterator(Lexer::new(input, 0))
    }
}

impl From<(usize, usize, usize)> for Location {
    fn from((offset, line, col): (usize, usize, usize)) -> Self {
        Self { offset, line, col }
    }
}

impl<'input> Iterator for TripleIterator<'input> {
    type Item = (Location, Token<'input>, Location);

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.0.next()?;
        debug!("token: {:?}", token);
        let start_pos: Location = (
            token.start,
            token.pos.line,
            token.pos.col.wrapping_sub(token.len()),
        )
            .into();
        let end_pos = (token.end, self.0.line, self.0.col).into();
        Some((start_pos, token.node, end_pos))
    }
}
