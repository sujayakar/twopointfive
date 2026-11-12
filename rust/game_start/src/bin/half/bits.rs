//! A word's bits, for the page and for the shaders.
use crate::typescript::{said, WRITTEN};
use game::bits::{pascal, BitsTable};

pub fn files() -> Vec<(&'static str, String)> { vec![("src/draw/bits.ts", typescript()), ("src/render/shaders/bits.wgsl", shaders())] }

const FROM: &str = "a declaration (`bits!`). To change a bit, change it there";
fn typescript() -> String {
    let mut l = vec![format!("{WRITTEN} from {FROM}.")];
    for BitsTable { name, doc, bits } in game_draw::BITS {
        l.extend([String::new(), format!("/** {} */", said(doc))]);
        // (a level's generator asks for a box's bit by the name in its hand)
        let listed = *name == "BoxFlag";
        l.push(if listed { format!("export const {name} = {{") } else { format!("export const enum {name} {{") });
        for (n, v, doc) in *bits { l.push(format!("  /** {} */", said(doc))); l.push(format!("  {}{} {v},", pascal(n).collect::<String>(), if listed { ":" } else { " =" })); }
        l.push(if listed { "} as const;".into() } else { "}".into() });
    }
    l.join("\n") + "\n"
}
/// the words a shader reads, each under a short name
const SHADED: [(&str, &BitsTable); 2] = [("GEO", &game_draw::boxes::pack::geo_flag::TABLE), ("GB", &game_draw::boxes::glass::pane_flag::TABLE)];
fn shaders() -> String {
    let mut l = vec![format!("{WRITTEN} from {FROM}.")];
    for (short, BitsTable { doc, bits, .. }) in SHADED {
        l.extend([String::new(), format!("// {}", said(doc))]);
        for (n, v, doc) in *bits { l.push(format!("const {short}_{n}: u32 = {v}u;   // {}", said(doc))); }
    }
    l.join("\n") + "\n"
}
