#![cfg(any(feature = "xml", feature = "xml2json", feature = "xq"))]
use std::io::{BufReader, Chain, Read};

static READER_PREFIX: &[u8] = b"<ignored>";
static READER_SUFFIX: &[u8] = b"</ignored>";

type RootNodePreservingReader<R> =
    Chain<Chain<BufReader<&'static [u8]>, R>, BufReader<&'static [u8]>>;

pub fn wrap_xml_reader<R: Read>(reader: R) -> RootNodePreservingReader<R> {
    let prefix_reader = BufReader::new(READER_PREFIX);
    let suffix_reader = BufReader::new(READER_SUFFIX);
    prefix_reader.chain(reader).chain(suffix_reader)
}
