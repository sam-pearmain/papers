use std::str::FromStr;
use super::error::ParseRomanError;
use Numeral::{I, V, X, L, C, D, M};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Numeral {
    I, V, X, L, C, D, M
}

impl Numeral {
    fn value(&self) -> u32 {
        match self {
            Self::I => 1, 
            Self::V => 5, 
            Self::X => 10, 
            Self::L => 50, 
            Self::C => 100, 
            Self::D => 500, 
            Self::M => 1000, 
        }
    }
}

impl TryFrom<char> for Numeral {
    type Error = ParseRomanError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'I' => Ok(Self::I),
            'V' => Ok(Self::V),
            'X' => Ok(Self::X),
            'L' => Ok(Self::L),
            'C' => Ok(Self::C),
            'D' => Ok(Self::D),
            'M' => Ok(Self::M),
            _   => Err(ParseRomanError::InvalidCharacter(c))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Roman {
    value: u32, 
    numerals: Vec<Numeral>, 
}

impl Roman {
    pub const MAX_VALUE: u32 = 3999;
    pub const MIN_VALUE: u32 = 1; 
}

impl FromStr for Roman {
    type Err = ParseRomanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numerals = s.chars()
            .map(Numeral::try_from)
            .collect::<Result<Vec<Numeral>, _>>()?;

        if numerals.is_empty() {
            return Err(ParseRomanError::EmptyInput)
        }

        let mut value = 0;
        let mut iter = numerals.iter().peekable();

        while let Some(current) = iter.next() {
            let current_value = current.value();
            let next_value = iter.peek().map_or(0, |n| n.value());

            if next_value > current_value {
                value += next_value - current_value;
                iter.next();
            } else {
                value += current_value;
            }
        }

        Ok(Roman { value, numerals })
    }
}

const NUMERALS: [(u32, &'static [Numeral]); 13] = [
    (1000, &[M]),
    (900, &[C, M]),
    (500, &[D]),
    (400, &[C, D]),
    (100, &[C]),
    (90, &[X, C]),
    (50, &[L]),
    (40, &[X, L]),
    (10, &[X]),
    (9, &[I, X]),
    (5, &[V]),
    (4, &[I, V]),
    (1, &[I]),
];

impl TryFrom<u32> for Roman {
    type Error = ParseRomanError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < Roman::MIN_VALUE || value > Roman::MAX_VALUE {
            return Err(ParseRomanError::OutsideRange);
        }
        
        let mut remainder = value;
        let mut numerals = Vec::new();

        for &(val, nums) in NUMERALS.iter() {
            while remainder >= val {
                remainder -= val;
                numerals.extend_from_slice(nums);
            }
        }

        Ok(Roman { value, numerals })
    }
}

macro_rules! impl_try_from_roman {
    ( $t:ty ) => {
        impl TryFrom<$t> for Roman {
            type Error = ParseRomanError;

            fn try_from(value: $t) -> Result<Self, Self::Error> {
                let value = u32::try_from(value)
                    .map_err(|_| ParseRomanError::OutsideRange)?;
                Roman::try_from(value)
            }
        }
    };
}

impl_try_from_roman!(u8);
impl_try_from_roman!(u16);
impl_try_from_roman!(u64);
impl_try_from_roman!(usize);
impl_try_from_roman!(i8);
impl_try_from_roman!(i16);
impl_try_from_roman!(i32);
impl_try_from_roman!(i64);
impl_try_from_roman!(isize);
