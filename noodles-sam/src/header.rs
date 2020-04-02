#[allow(clippy::module_inception)]
pub mod header;
mod program;
mod read_group;
mod record;
mod reference_sequence;

use std::{convert::TryFrom, error, fmt, str::FromStr};

pub use self::{program::Program, read_group::ReadGroup, reference_sequence::ReferenceSequence};

pub use self::record::Record;

#[derive(Debug, Default)]
pub struct Header {
    header: header::Header,
    reference_sequences: Vec<ReferenceSequence>,
    read_groups: Vec<ReadGroup>,
    programs: Vec<Program>,
    comments: Vec<String>,
}

impl Header {
    pub fn header(&self) -> &header::Header {
        &self.header
    }

    pub fn reference_sequences(&self) -> &[ReferenceSequence] {
        &self.reference_sequences
    }

    pub fn read_groups(&self) -> &[ReadGroup] {
        &self.read_groups
    }

    pub fn programs(&self) -> &[Program] {
        &self.programs
    }

    pub fn comments(&self) -> &[String] {
        &self.comments
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRecord(record::ParseError),
    InvalidHeader(header::ParseError),
    InvalidReferenceSequence(reference_sequence::ParseError),
    InvalidReadGroup(read_group::ParseError),
    InvalidProgram(program::ParseError),
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRecord(e) => write!(f, "{}", e),
            Self::InvalidHeader(e) => write!(f, "{}", e),
            Self::InvalidReferenceSequence(e) => write!(f, "{}", e),
            Self::InvalidReadGroup(e) => write!(f, "{}", e),
            Self::InvalidProgram(e) => write!(f, "{}", e),
        }
    }
}

impl FromStr for Header {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut header = Header::default();

        for line in s.lines() {
            let record = line.parse().map_err(ParseError::InvalidRecord)?;

            match record {
                Record::Header(fields) => {
                    header.header =
                        header::Header::try_from(&fields[..]).map_err(ParseError::InvalidHeader)?;
                }
                Record::ReferenceSequence(fields) => {
                    let reference_sequence = ReferenceSequence::try_from(&fields[..])
                        .map_err(ParseError::InvalidReferenceSequence)?;
                    header.reference_sequences.push(reference_sequence);
                }
                Record::ReadGroup(fields) => {
                    let read_group =
                        ReadGroup::try_from(&fields[..]).map_err(ParseError::InvalidReadGroup)?;
                    header.read_groups.push(read_group);
                }
                Record::Program(fields) => {
                    let program =
                        Program::try_from(&fields[..]).map_err(ParseError::InvalidProgram)?;
                    header.programs.push(program);
                }
                Record::Comment(comment) => {
                    header.comments.push(comment);
                }
            }
        }

        Ok(header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let raw_header = "\
@HD\tVN:1.6\tSO:coordinate
@SQ\tSN:sq0\tLN:1
@SQ\tSN:sq1\tLN:2
@RG\tID:rg0
@PG\tID:pg0\tPN:noodles
@CO\tnoodles_sam::header::tests::test_from_str
";

        let header: Header = raw_header.parse().unwrap();

        assert_eq!(header.reference_sequences().len(), 2);

        assert_eq!(header.read_groups().len(), 1);

        assert_eq!(header.programs().len(), 1);

        assert_eq!(header.comments.len(), 1);
        assert_eq!(
            &header.comments[0],
            "noodles_sam::header::tests::test_from_str"
        );
    }
}