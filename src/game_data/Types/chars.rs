#[repr(u16)]
#[derive(Copy, Clone)]  // Add these
pub enum CharType {
    CharA = 0,
    CharB = 1,
    CharC = 2,
    CharD = 3,
    CharE = 4,
    CharF = 5,
    CharG = 6,
    CharH = 7,
    CharI = 8,
    CharJ = 9,
    CharK = 10,
    CharL = 11,
    CharM = 12,
    CharN = 13,
    CharO = 14,
    CharP = 15,
    CharQ = 16,
    CharR = 17,
    CharS = 18,
    CharT = 19,
    CharU = 20,
    CharV = 21,
    CharW = 22,
    CharX = 23,
    CharY = 24,
    CharZ = 25,
    Char0 = 26,
    Char1 = 27,
    Char2 = 28,
    Char3 = 29,
    Char4 = 30,
    Char5 = 31,
    Char6 = 32,
    Char7 = 33,
    Char8 = 34,
    Char9 = 35,
    CharPeriod = 36,
    CharQuestion = 37,
    CharExclamation = 38,
    CharSemicolon = 39,
    CharColon = 40,
    CharHashtag = 41,
    CharPrecent = 42,
    CharCloseBracket = 43,
    CharOpenBracket = 44,
    CharForwardSlash = 45,
    CharMinus = 46,
    CharSpace = 47,
}

impl CharType {
    /// Get the ID value of the enum variant
    pub fn get_id(&self) -> u16 {
        *self as u16
    }

    pub fn get_total_chars() -> u16 {
        return 47;
    }

    /// Convert an ID to its corresponding CharType enum variant
    /// Returns None if the ID is out of range
    pub fn from_id(id: u16) -> Option<Self> {
        if id >= Self::get_total_chars() {
            return None;
        }
        // SAFETY: We've validated that id is within the valid range [0, 46]
        // and CharType is repr(u16) with sequential values starting from 0
        unsafe { Some(std::mem::transmute(id)) }
    }

    /// Convert a character to its corresponding CharType enum variant
    pub fn from_char(c: char) -> Self {
        match c {
            'A' | 'a' => CharType::CharA,
            'B' | 'b' => CharType::CharB,
            'C' | 'c' => CharType::CharC,
            'D' | 'd' => CharType::CharD,
            'E' | 'e' => CharType::CharE,
            'F' | 'f' => CharType::CharF,
            'G' | 'g' => CharType::CharG,
            'H' | 'h' => CharType::CharH,
            'I' | 'i' => CharType::CharI,
            'J' | 'j' => CharType::CharJ,
            'K' | 'k' => CharType::CharK,
            'L' | 'l' => CharType::CharL,
            'M' | 'm' => CharType::CharM,
            'N' | 'n' => CharType::CharN,
            'O' | 'o' => CharType::CharO,
            'P' | 'p' => CharType::CharP,
            'Q' | 'q' => CharType::CharQ,
            'R' | 'r' => CharType::CharR,
            'S' | 's' => CharType::CharS,
            'T' | 't' => CharType::CharT,
            'U' | 'u' => CharType::CharU,
            'V' | 'v' => CharType::CharV,
            'W' | 'w' => CharType::CharW,
            'X' | 'x' => CharType::CharX,
            'Y' | 'y' => CharType::CharY,
            'Z' | 'z' => CharType::CharZ,
            '0' => CharType::Char0,
            '1' => CharType::Char1,
            '2' => CharType::Char2,
            '3' => CharType::Char3,
            '4' => CharType::Char4,
            '5' => CharType::Char5,
            '6' => CharType::Char6,
            '7' => CharType::Char7,
            '8' => CharType::Char8,
            '9' => CharType::Char9,
            '.' => CharType::CharPeriod,
            '?' => CharType::CharQuestion,
            '!' => CharType::CharExclamation,
            ';' => CharType::CharSemicolon,
            ':' => CharType::CharColon,
            '#' => CharType::CharHashtag,
            '%' => CharType::CharPrecent,
            ']' => CharType::CharCloseBracket,
            '[' => CharType::CharOpenBracket,
            '/' => CharType::CharForwardSlash,
            '-' => CharType::CharMinus,
            _ => CharType::CharSpace,  // Default to 'A' if no match found
        }
    }
}
