use std::io;

use bstr::{BStr, ByteSlice};

use crate::{
    commit,
    mutable::{encode, NL},
    Commit,
};

impl Commit {
    /// Returns a convenient iterator over all extra headers.
    pub fn extra_headers(&self) -> commit::ExtraHeaders<impl Iterator<Item = (&BStr, &BStr)>> {
        commit::ExtraHeaders::new(self.extra_headers.iter().map(|(k, v)| (k.as_bstr(), v.as_bstr())))
    }
    /// Serializes this instance to `out` in the git serialization format.
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        encode::trusted_header_id(b"tree", &self.tree, &mut out)?;
        for parent in &self.parents {
            encode::trusted_header_id(b"parent", parent, &mut out)?;
        }
        encode::trusted_header_signature(b"author", &self.author, &mut out)?;
        encode::trusted_header_signature(b"committer", &self.committer, &mut out)?;
        if let Some(encoding) = self.encoding.as_ref() {
            encode::header_field(b"encoding", encoding, &mut out)?;
        }
        for (name, value) in &self.extra_headers {
            let has_newline = value.find_byte(b'\n').is_some();
            if has_newline {
                encode::header_field_multi_line(name, value, &mut out)?;
            } else {
                encode::trusted_header_field(name, value, &mut out)?;
            }
        }
        out.write_all(NL)?;
        out.write_all(&self.message)
    }
}
