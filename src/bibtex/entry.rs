use std::{fmt, str::FromStr};
use crate::bibtex::{error::ParseErrorType, fields::Fields}; 

#[derive(Debug, Clone)]
pub enum EntryKind {
    Article, Book, Booklet, Conference, InBook, InCollection,
    InProceedings, Manual, MasterThesis, Misc, PhDThesis,
    Proceedings, TechReport, Unpublished,
}

impl fmt::Display for EntryKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self {
            Self::Article => "article",
            Self::Book => "book",
            Self::Booklet => "booklet",
            Self::Conference => "conference",
            Self::InBook => "inbook",
            Self::InCollection => "incollection",
            Self::InProceedings => "inproceedings",
            Self::Manual => "manual",
            Self::MasterThesis => "mastersthesis",
            Self::Misc => "misc",
            Self::PhDThesis => "phdthesis",
            Self::Proceedings => "proceedings",
            Self::TechReport => "techreport",
            Self::Unpublished => "unpublished",
        };
        write!(f, "{}", kind)
    }
}

impl FromStr for EntryKind {
    type Err = ParseErrorType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "article"       => Ok(Self::Article),
            "book"          => Ok(Self::Book), 
            "booklet"       => Ok(Self:: Booklet), 
            "conference"    => Ok(Self::Conference), 
            "inbook"        => Ok(Self::InBook), 
            "incollection"  => Ok(Self::InCollection), 
            "inproceedings" => Ok(Self::InProceedings), 
            "manual"        => Ok(Self::Manual), 
            "masterthesis"  => Ok(Self::MasterThesis), 
            "misc"          => Ok(Self::Misc), 
            "phdthesis"     => Ok(Self::PhDThesis), 
            "proceedings"   => Ok(Self::Proceedings), 
            "techreport"    => Ok(Self::TechReport), 
            "unpublished"   => Ok(Self::Unpublished), 
            _ => Err(ParseErrorType::UnknownEntry { entry: s.to_string() })
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub citekey: String, 
    pub kind: EntryKind, 
    pub fields: Fields,
}

impl Entry {
    pub fn new(citekey: String, kind: EntryKind, fields: Fields) -> Self {
        Entry { citekey, kind, fields }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}{{{},\n", self.kind, self.citekey);
        write!(f, "");
        write!(f, "}}")
    }
}