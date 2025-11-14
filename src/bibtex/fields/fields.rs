use std::{collections::HashMap, str::FromStr};
use url::Url;

use crate::bibtex::error::{ParseError, ParseErrorType};
use crate::bibtex::fields::{date::{Month, Year}, pages::Pages, numbers::Number};
use crate::bibtex::fields::error::ParseFieldError;

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

#[derive(Debug, Clone)]
pub enum FieldValue {
    String { s: String }, 
    Number { number: Number }, 
    Pages  { pages: Pages }, 
    Month  { month: Month }, 
    Year   { year: Year }, 
    Url    { url: Url }, 
}

impl FieldValue {
    fn string(s: &str) -> Self {
        todo!()
    }

    fn number(s: &str) -> Self {
        todo!()
    }

    fn pages(s: &str) -> Result<Self, ParseFieldError> {
        let pages = s.parse::<Pages>()?;
        Ok(Self::Pages { pages })
    }

    fn month(s: &str) -> Result<FieldValue, ParseFieldError> {
        let month = Month::from_str(s)?;
        Ok(Self::Month { month })
    }

    fn url(s: &str) -> Result<Self, url::ParseError> {
        let url = Url::parse(s)?;
        Ok(Self::Url { url })
    }
}

#[derive(Debug, Clone)]
pub struct Fields {
    fields: HashMap<Field, FieldValue>
}

impl Fields {
    pub fn new() -> Self {
        Fields { fields: HashMap::new() }
    }

    pub fn add(&mut self, field: Field, value: &str) -> Result<(), ParseError> {
        match field {
            
        }
    } 
}