/// One frame's word to Sam's animator, before any state poses him; down: nothing fed.
pub fn stealth_feed(g: &mut crate::game::Game) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    if crate::condition::down(g, g.pl[pl].ch) {
        g.ch[c].anim.stealth = None;
        return;
    }
    let wanding = g.pl[pl].wanding().is_some();
    let cable = wanding;
    // the same test, in the same order: sam_work only when all the rest are false
    let p = &g.pl[pl];
    let taken = (p.deed.is_some() && !wanding)
        || p.tow.is_some()
        || p.wall.is_some()
        || p.sprinting
        || crate::handwork::sam_work(g).is_some();
    let f = g.ch[c].anim.stealth.get_or_insert(crate::stealthpose::StealthFeed { free: false, cable: false });
    f.cable = cable;
    // kit_in_hand only when the tests before it pass, and after the row above is written
    let free = !taken
        && !wanding
        && crate::kitanim::kit_in_hand(g, &crate::seats::here(g)).is_none()
        && g.sets[g.seat].hand.throw_preview.is_none();
    g.ch[c].anim.stealth.as_mut().unwrap().free = free;
}
