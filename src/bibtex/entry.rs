use std::str::FromStr;
use crate::bibtex::error::ParseError;

#[derive(Debug)]
pub enum EntryKind {
    Article, 
    Book, 
    Booklet, 
    Conference, 
    InBook, 
    InCollection, 
    InProceedings, 
    Manual, 
    MasterThesis, 
    Misc, 
    PhDThesis, 
    Proceedings, 
    TechReport, 
    Unpublished, 
}

impl FromStr for EntryKind {
    type Err = ParseError;

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
            _ => Err(ParseError::UnknownEntryKind { kind: s.to_string() })
        }
    }
}


#[derive(Debug)]
pub struct Entry {
    citekey: String, 
    kind: EntryKind, 
    fields: Fields,
}