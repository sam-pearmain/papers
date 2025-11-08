use std::str::FromStr;
use crate::bibtex::error::ParseError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// An enum for the different kinds of BibTex field 
pub enum FieldKind {
    Address, Annote, Author, BookTitle, Chapter, Edition, Editor,
    HowPublished, Institution, Journal, Month, Note, Number,
    Organization, Pages, Publisher, School, Series, Title, Type,
    Volume, Year, Doi, Issn, Isbn, Url,
}

impl FromStr for FieldKind {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "address"      => Ok(FieldKind::Address),
            "annote"       => Ok(FieldKind::Annote),
            "author"       => Ok(FieldKind::Author),
            "booktitle"    => Ok(FieldKind::BookTitle),
            "chapter"      => Ok(FieldKind::Chapter),
            "edition"      => Ok(FieldKind::Edition),
            "editor"       => Ok(FieldKind::Editor),
            "howpublished" => Ok(FieldKind::HowPublished),
            "institution"  => Ok(FieldKind::Institution),
            "journal"      => Ok(FieldKind::Journal),
            "month"        => Ok(FieldKind::Month),
            "note"         => Ok(FieldKind::Note),
            "number"       => Ok(FieldKind::Number),
            "organization" => Ok(FieldKind::Organization),
            "pages"        => Ok(FieldKind::Pages),
            "publisher"    => Ok(FieldKind::Publisher),
            "school"       => Ok(FieldKind::School),
            "series"       => Ok(FieldKind::Series),
            "title"        => Ok(FieldKind::Title),
            "type"         => Ok(FieldKind::Type),
            "volume"       => Ok(FieldKind::Volume),
            "year"         => Ok(FieldKind::Year),
            "doi"          => Ok(FieldKind::Doi),
            "issn"         => Ok(FieldKind::Issn),
            "isbn"         => Ok(FieldKind::Isbn),
            "url"          => Ok(FieldKind::Url),
            _              => Err(ParseError::UnknownFieldKind { king: s.to_string() })
        }
    }
}

impl FieldKind {
    fn is_non_standard_field(&self) -> bool {
        matches!(self, Self::Doi | Self::Issn | Self::Isbn | Self::Url)
    }

    fn is_standard_field(&self) -> bool {
        !self.is_non_standard_field()
    }
}