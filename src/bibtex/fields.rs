use std::str::FromStr;
use crate::bibtex::error::ParseError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// An enum for the different kinds of BibTex field 
pub enum Field {
    Address, Annote, Author, BookTitle, Chapter, Edition, Editor,
    HowPublished, Institution, Journal, Month, Note, Number,
    Organization, Pages, Publisher, School, Series, Title, Type,
    Volume, Year, Doi, Issn, Isbn, Url,
}

impl FromStr for Field {
    type Err = ParseError;

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
            _              => Err(ParseError::UnknownFieldKind { kind: s.to_string() })
        }
    }
}

impl Field {
    fn is_non_standard_field(&self) -> bool {
        matches!(self, Self::Doi | Self::Issn | Self::Isbn | Self::Url)
    }

    fn is_standard_field(&self) -> bool {
        !self.is_non_standard_field()
    }
}
