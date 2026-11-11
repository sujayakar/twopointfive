//! WHAT A HOST THAT IS NOT RUST HANDS A START: the level file's text, the address's words and the character's files. The start reads no file and fetches nothing: whoever stands a game has the bytes.
use crate::gltf::GltfFile;
use crate::json;
use crate::host::kept::Kept;
use crate::stand::{stand, Assets, StartOpts, Stood};
use alloc::string::String;

/// the level file's text · the address's words (empty: none) · the mannequin's gltf and its buffer · the cast's gltf and the look's cast's (empty: none) · what the machine keeps (empty: nothing)
pub struct Handed<'a> { pub level: &'a [u8], pub words: &'a str, pub mannequin: &'a [u8], pub mannequin_bin: &'a [u8], pub cast: &'a [u8], pub cast_look: &'a [u8], pub kept: &'a str }

/// STAND A GAME FROM WHAT A HOST HANDED. `o.link` and `o.kept` are the handed address's words and the handed record (what `o` held of them is not read). A text that is not text, a record that is none, or a level
/// file that does not read stops with its reason, as `stand` does for a file it does not carry.
pub fn stand_handed(h: &Handed, o: &StartOpts) -> Stood {
    fn text<'a>(b: &'a [u8], what: &str) -> &'a str { core::str::from_utf8(b).unwrap_or_else(|_| panic!("{what} is not text")) }
    let some = |b, what| if b == b"" { None } else { Some(GltfFile { text: text(b, what), bin: None }) };
    let data = json::parse(text(h.level, "the level file")).unwrap_or_else(|e| panic!("the level file: {e}"));
    let assets = Assets { mannequin: GltfFile { text: text(h.mannequin, "the mannequin's gltf"), bin: Some(h.mannequin_bin) }, cast: some(h.cast, "the cast's gltf"), cast_look: some(h.cast_look, "the look's cast's gltf") };
    let link = if h.words.is_empty() { None } else { Some(String::from(h.words)) };
    let kept = Kept::read(h.kept).unwrap_or_else(|e| panic!("{e}"));
    stand(&data, assets, &StartOpts { link, kept, ..o.clone() })
}
