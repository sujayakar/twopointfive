//! WHAT A HOST THAT IS NOT RUST HANDS A START: the level file's text, the address's words and the character's files, as ONE run of bytes that
//! crosses into the module once (the page; the headless host). The start reads no file and fetches nothing: whoever stands a game has the bytes.
//!
//! The bytes: a count, then each part as its length and its bytes (every number 32 bits, little end first). The parts, in `PARTS`' order; a part
//! that may be left out is handed empty.
use crate::gltf::GltfFile;
use crate::json;
use crate::host::kept::Kept;
use crate::stand::{stand, Assets, StartOpts, Stood};
use alloc::{format, string::String, vec::Vec};

/// the parts a host hands, in order (a host's packer names them by these places)
pub const PARTS: [&str; 7] = ["the level file's text", "the address's words (empty: none)", "the mannequin's gltf", "the mannequin's buffer", "the cast's gltf (empty: none)", "the look's cast's gltf (empty: none)",
    "what the machine keeps (empty: nothing)"];

/// the parts of a handed run of bytes; an error says what is wrong with the bytes
pub fn parts(b: &[u8]) -> Result<Vec<&[u8]>, String> {
    let word = |at: usize| -> Result<usize, String> { b.get(at..at + 4).map(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]) as usize).ok_or_else(|| format!("the bytes end at {} inside a number", b.len())) };
    let n = word(0)?;
    if n != PARTS.len() { return Err(format!("{} parts were handed and a start takes {}", n, PARTS.len())); }
    let (mut at, mut out) = (4, Vec::with_capacity(n));
    for name in PARTS {
        let len = word(at)?;
        out.push(b.get(at + 4..at + 4 + len).ok_or_else(|| format!("the bytes end inside {}", name))?);
        at += 4 + len;
    }
    if at != b.len() { return Err(format!("{} bytes follow the last part", b.len() - at)); }
    Ok(out)
}

/// STAND A GAME FROM WHAT A HOST HANDED. `o.link` and `o.kept` are the handed address's words and the handed record (what `o` held of them is not read). A run of bytes that is not a
/// start's, a text that is not text, a record that is none, or a level file that does not read stops with its reason, as `stand` does for a file it does not carry.
pub fn stand_handed(b: &[u8], o: &StartOpts) -> Stood {
    let p = parts(b).unwrap_or_else(|e| panic!("a start was handed bytes that are not a start's: {}", e));
    let text = |i: usize| core::str::from_utf8(p[i]).unwrap_or_else(|_| panic!("{} is not text", PARTS[i]));
    let some = |i: usize| if p[i].is_empty() { None } else { Some(GltfFile { text: text(i), bin: None }) };
    let data = json::parse(text(0)).unwrap_or_else(|e| panic!("{}: {}", PARTS[0], e));
    let assets = Assets { mannequin: GltfFile { text: text(2), bin: Some(p[3]) }, cast: some(4), cast_look: some(5) };
    let link = if p[1].is_empty() { None } else { Some(String::from(text(1))) };
    let kept = Kept::read(text(6)).unwrap_or_else(|e| panic!("{}", e));
    stand(&data, assets, &StartOpts { link, kept, ..o.clone() })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn pack(parts: &[&[u8]]) -> Vec<u8> { let mut b = (parts.len() as u32).to_le_bytes().to_vec(); for p in parts { b.extend((p.len() as u32).to_le_bytes()); b.extend(*p); } b }
    #[test]
    fn the_parts_come_back_as_handed_and_bytes_that_are_not_a_starts_are_refused() {
        let b = pack(&[b"{}", b"", b"m", &[1, 2, 3], b"", b"look", b"c"]);
        assert_eq!(parts(&b).unwrap(), [&b"{}"[..], b"", b"m", &[1, 2, 3], b"", b"look", b"c"]);
        assert!(parts(&pack(&[b"{}", b""])).unwrap_err().contains("2 parts"));
        assert!(parts(&b[..b.len() - 1]).unwrap_err().contains("what the machine keeps"));
        let mut more = b.clone(); more.push(0);
        assert!(parts(&more).unwrap_err().contains("1 bytes follow"));
        assert!(parts(&[1, 0]).is_err());
    }
}
