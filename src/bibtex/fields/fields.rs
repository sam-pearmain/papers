use std::{collections::HashMap, str::FromStr};
use url::Url;

use crate::bibtex::error::ParseErrorType;

#[derive(Debug, PartialEq, Clone, Hash)]
/// An enum for the different kinds of BibTex field 
pub enum Field {
    Address, Annote, Author, BookTitle, Chapter, Edition, Editor,
    HowPublished, Institution, Journal, Month, Note, Number,
    Organization, Pages, Publisher, School, Series, Title, Type,
    Volume, Year, Doi, Issn, Isbn, Url,
}

impl FromStr for Field {
    type Err = ParseErrorType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "address"      => Ok(Self::Address),
            "annote"       => Ok(Self::Annote),
            "author"       => Ok(Self::Author),
            "booktitle"    => Ok(Self::BookTitle),
            "chapter"      => Ok(Self::Chapter),
            "edition"      => Ok(Self::Edition),
            "editor"       => Ok(Self::Editor),
            "howpublished" => Ok(Self::HowPublished),
            "institution"  => Ok(Self::Institution),
            "journal"      => Ok(Self::Journal),
            "month"        => Ok(Self::Month),
            "note"         => Ok(Self::Note),
            "number"       => Ok(Self::Number),
            "organization" => Ok(Self::Organization),
            "pages"        => Ok(Self::Pages),
            "publisher"    => Ok(Self::Publisher),
            "school"       => Ok(Self::School),
            "series"       => Ok(Self::Series),
            "title"        => Ok(Self::Title),
            "type"         => Ok(Self::Type),
            "volume"       => Ok(Self::Volume),
            "year"         => Ok(Self::Year),
            "doi"          => Ok(Self::Doi),
            "issn"         => Ok(Self::Issn),
            "isbn"         => Ok(Self::Isbn),
            "url"          => Ok(Self::Url),
            _              => Err(ParseErrorType::UnknownField { field: s.to_string() })
        }
    }
}

#[derive(Debug)]
pub enum FieldValue {
    String { s: String }, 
    Number { n: usize }, 
    Pages  { p: Pages }, 
    Month  { m: Month }, 
    Year   { y: Year }, 
    Url    { u: Url }, 
}

impl FieldValue {
    fn string(s: &str) -> Self {
        todo!()
    }

    fn number(s: &str) -> Self {
        todo!()
    }

    fn pages(s: &str) -> Self {
        todo!()
    }

    fn month(s: &str) -> Self {
        todo!()
    }

    fn url(s: &str) -> Result<Self, url::ParseError> {
        let url = Url::parse(s)?;
        Ok(Self::Url { u: url })
    }
}

pub struct Fields {
    fields: HashMap<Field, FieldValue>
}

impl Fields {
    pub fn new() -> Self {
        Fields { fields: HashMap::new() }
    }

    pub fn add(&mut self, field: Field, value: &str) {
        match field {
            Field::
        }
    } 
}