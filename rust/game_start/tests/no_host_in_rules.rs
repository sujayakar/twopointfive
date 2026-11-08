//! WHAT ONLY A PICTURE SHOWS GOES OUT THE SAME ON EVERY HOST. Whether a host has a particle pool used to decide rows of the game: with none, a shot and a
//! burst made drawn boxes and the frame table numbered differently, so a page and a server parted in destr, the piece table, the sparks and the boxes.
//! A chip, a spark or a puff is now a record that goes out on every host, and a host with nothing to draw drops it.
//! A FAILURE MEANS: a rule is asking again what the host holds.
use game::out_events::OutEvent;
use game_start::play::{Opts, Play};
use game_start::stand;

const DT: f64 = 1.0 / 60.0;

/// THE RULE: a stood game with no page behind it sends a blast's particles out as a page's does.
#[test]
fn a_blast_sends_its_particles_out_on_any_host() {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: 7, ..Default::default() });
    p.sam_unhurt();
    let who = p.g.ch[p.g.pl[p.g.player].ch].id;
    let spec = game::frag::BlastSpec { who, ..Default::default() };
    game::frag::blast(&mut p.g, [20.0, 0.5, 15.0], &spec);
    let mut told = 0;
    for _ in 0..3 {
        for e in stand::frame(&mut p.g, &p.host, DT) {
            if let OutEvent::ParticlesEmit(..) = e { told += 1; }
        }
    }
    assert!(told > 0, "a frag went off and no particle record left the game");
}
