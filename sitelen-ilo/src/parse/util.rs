
/// Functions for checking *sitelen pona* characters.
/// 
/// Refer to [the UCSUR proposal](https://www.kreativekorp.com/ucsur/charts/sitelen.html) for more info.
pub(super) trait CharSitelenPonaExt: Copy {
    fn is_sp_word(self) -> bool;
    fn is_sp_punct(self) -> bool;
}

impl CharSitelenPonaExt for char {
    fn is_sp_word(self) -> bool {
        matches!(self, '\u{F1900}'..='\u{F1988}' | '\u{F19A0}'..='\u{F19A3}')
    }

    fn is_sp_punct(self) -> bool {
        matches!(self, '\u{F1990}' | '\u{F1991}' | '\u{F199C}' | '\u{F199D}')
    }
}