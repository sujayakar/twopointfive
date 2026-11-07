//! THIS REPOSITORY'S OWN FILES, for a native host and the tests: the character's (public/assets) and a level's (levels/). The start reads no file
//! itself; whoever stands a game hands it the bytes, and on a machine with this checkout these are where they are.
use crate::gltf::GltfFile;
use crate::stand::Assets;
use std::path::PathBuf;
use std::string::String;
use std::vec::Vec;

/// the checkout's root (this crate is `rust/game_start` of it)
pub fn root() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..") }

/// The character's files, read once.
pub struct CharacterFiles { mannequin: String, mannequin_bin: Vec<u8>, cast: String, cast_look: String }
impl CharacterFiles {
    /// the mannequin and the cast the page boots with (`public/assets/ual`, `public/assets/chars/cast.gltf`)
    pub fn read() -> CharacterFiles {
        let p = root().join("public/assets");
        let text = |f: &str| std::fs::read_to_string(p.join(f)).unwrap_or_else(|e| panic!("{}: {}", p.join(f).display(), e));
        CharacterFiles { mannequin: text("ual/AnimationLibrary_Godot_Standard.gltf"), mannequin_bin: std::fs::read(p.join("ual/AnimationLibrary_Godot_Standard.bin")).expect("the mannequin's buffer"), cast: text("chars/cast.gltf"), cast_look: text("chars/cast-gwot.gltf") }
    }
    pub fn assets(&self) -> Assets<'_> { Assets { mannequin: GltfFile { text: &self.mannequin, bin: Some(&self.mannequin_bin) }, cast: Some(GltfFile { text: &self.cast, bin: None }), cast_look: Some(GltfFile { text: &self.cast_look, bin: None }) } }
}

/// a level file of `levels/` (or a path), as text
pub fn level_text(name: &str) -> String {
    let p = if std::path::Path::new(name).exists() { PathBuf::from(name) } else { root().join("levels").join(name) };
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {}", p.display(), e))
}
