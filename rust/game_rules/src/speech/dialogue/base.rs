//! What the parts of `dialogue` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// INVENTED: DialogueCtx.tokens as the table `expand` reads, keyed by the words of TOKENS (`sawWhere`, as Token::as_str), a null token None. The TypeScript hands the one object to expand (dialogue.ts:683, 726, 971, 995, 1311, 1331); Rust has a struct there and a map here.
pub fn ctx_tokens_map(t: &crate::dialogue::DialogueCtxTokens) -> JsMap<String, Option<String>> {
    let mut o = JsMap::new();
    let rows: [(&str, &Option<String>); 23] = [("partner", &t.partner), ("victim", &t.victim), ("body", &t.body), ("missing", &t.missing), ("room", &t.room), ("sawWhere", &t.saw_where), ("heardWhere", &t.heard_where), ("route", &t.route), ("partnerRoute", &t.partner_route), ("post", &t.post), ("door", &t.door), ("caller", &t.caller), ("checker", &t.checker), ("checked", &t.checked), ("clearRoom", &t.clear_room), ("breakerRoom", &t.breaker_room), ("checkEta", &t.check_eta), ("minutes", &t.minutes), ("n", &t.n), ("up", &t.up), ("badge", &t.badge), ("code", &t.code), ("padDoor", &t.pad_door)];
    for (k, v) in rows { o.set(String::from(k), v.clone()); }
    o
}
