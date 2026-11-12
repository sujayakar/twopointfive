//! Names both sides spell: an enum's words, as a list and a type.
use crate::typescript::WRITTEN;
use game_draw::{Named, NAMES};

pub fn files() -> Vec<(&'static str, String)> { vec![("src/draw/names.ts", typescript())] }
fn typescript() -> String {
    let mut l = vec![format!("{WRITTEN} from an enum's words (`Words`). To change a name, change it there.")];
    for Named { list, ty, words, doc } in NAMES {
        l.extend([String::new(), format!("/** {doc}, by its number in Rust */"), format!("export const {list} = [{}] as const;", words.iter().map(|w| format!("'{w}'")).collect::<Vec<_>>().join(", ")), format!("export type {ty} = (typeof {list})[number];")]);
    }
    l.join("\n") + "\n"
}
