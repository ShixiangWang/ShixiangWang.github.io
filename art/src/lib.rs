//! # Art
//!
//! XYZ ...................

pub mod kinds {
    //! Define color type

    /// Main color
    pub enum  PrimaryColor {
        Red,
        Yellow,
        Blue
    }

    /// Sec color
    #[derive(Debug, PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    //! Utils, only for color now
    use crate::kinds::*;
    
    /// Set colors
    /// ```rust
    /// use art::utils::mix;
    /// use art::kinds::*;
    /// assert!(matches(mix(PrimaryColor::Yellow, PrimaryColor::Blue), SecondaryColor::Green));
    /// ```
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}