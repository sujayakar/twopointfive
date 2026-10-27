// The game page's side of the SESSION JOURNAL (game/journal.ts): F10 starts a recording (or asks for one — it waits for a moment it may start on and says why on the
// HUD tag), F10 again stops it, and the file — twopointsix-session-<date>.json — goes to whoever listens at the one door (host/savefile.ts: no download since 2026-10-27, `?downloads=1` = as it was) and, if the shot server is up (tools/shot-server.ts :5174), is POSTed to
// /session?name=… so it lands straight in tools/qa/sessions/, where `bun run tools/qa/replay.ts --all` picks it up. The recorder itself is the Game's (headless too);
// this file only owns the key, the delivery and the console hook (__journal, main.ts).
import type { Game } from '../game/game';
import type { SessionFile } from '../game/journal';
import { checkSessionFile } from '../game/journal';
import { saveFile, savedWords, savedEnd, onThisMachine, type Went } from '../host/savefile';

export class SessionRec {
  /** the commit the dev server says it serves (tools/serve.ts /qa/commit), fetched once; absent off a static build */
  commit: string | undefined;
  lastName = ''; lastBytes = 0;
  constructor(private game: Game, private note: (s: string) => void) {
    game.journal.onStop = (file, why) => { if (file) this.deliver(file, why); else this.note(`REC did not start — ${why}`); };
    try { void fetch('/qa/commit', { cache: 'no-store' }).then(r => r.ok ? r.text() : '').then(t => { const c = t.trim(); if (/^[0-9a-f]{7,40}$/.test(c)) this.commit = c; }).catch(() => {}); } catch { /* no dev server */ }
  }
  /** F10: idle → ask for a recording; pending → give up; recording → stop (deferred to the end of this frame's update; a second press will not wait) */
  toggle(): string {
    const J = this.game.journal;
    if (J.state === 'idle') { J.request({ commit: this.commit, userAgent: typeof navigator !== 'undefined' ? navigator.userAgent : undefined, hud: true }); const s = J.why ? `REC waits — ${J.why}` : 'REC — F10 stops; hands off the debugger (`) while it runs'; this.note(s); return s; }
    if (J.state === 'pending') { J.stop('cancelled'); this.note('REC cancelled'); return 'cancelled'; }
    if (J.stopping) { J.stop('stopped (forced)', true); return 'forced'; }
    J.stop('stopped'); return 'stopping';
  }
  /** to the door's listener (host/savefile.ts) + POST to the shot server's /session sink; the name carries the date and the length */
  deliver(file: SessionFile, why: string): void {
    const bad = checkSessionFile(file); if (bad) { this.note(`REC: the file failed its own check — ${bad}`); return; }
    const body = JSON.stringify(file); const bpf = file.frames ? Math.round((body.length - JSON.stringify(file.save).length) / file.frames) : 0;
    file.meta.bytesPerFrame = bpf;
    const name = `twopointsix-session-${file.recordedAt.replace(/[:.]/g, '-').replace(/Z$/, '')}-${Math.round(file.meta.seconds)}s`;
    const text = JSON.stringify(file);
    this.lastName = name + '.json'; this.lastBytes = text.length;
    let w: Went = { went: false, local: false }; try { w = saveFile('session', this.lastName, new Blob([text], { type: 'application/json' }), { meta: { commit: this.commit },
      // (later: it arrived, or it was let go after all. The note says so unless a newer journal has gone out since)
      done: arrived => { if (name + '.json' === this.lastName) this.note(savedEnd('The recording', arrived)); } }); } catch { /* headless */ }
    // (the shot server is a thing of a page served from this machine: a page opened from anywhere else asks nothing of the visitor's own localhost. With
    // no listener at the door it is the one place the journal can land: the note says where it is, or that it is nowhere)
    const here = onThisMachine(), lone = w.went === false && !w.local;
    if (here) try { const port = (window as unknown as { __shotPort?: number }).__shotPort ?? 5174; void fetch(`http://localhost:${port}/session?name=${encodeURIComponent(name)}`, { method: 'POST', body: text, headers: { 'Content-Type': 'application/json' } }).then(r => r.ok ? r.text() : '').then(t => { if (t) this.note(`REC → ${t.split(' ')[0]}`); else if (lone) this.note("The recording wasn't saved."); }, () => { if (lone) this.note("The recording wasn't saved, because no shot server is up."); }); } catch { /* no sink */ }
    // (no file lands in a downloads folder any more, host/savefile.ts: the note says where the journal went. The file's numbers are a developer's: the console)
    const facts = `${file.frames} frames / ${file.meta.seconds} s · ${(text.length / 1024).toFixed(0)} KB (${bpf} B/frame) · ${why}`;
    if (w.local) this.note(`REC saved — ${facts}`); else { try { console.info(`REC: ${facts}`); } catch { /* */ } const own = /^stopped/.test(why) ? '' : ` It stopped by itself (${why}).`; this.note((lone && here ? `REC stopped after ${Math.round(file.meta.seconds)} s.` : savedWords('The recording', w.went)) + own); }
  }
}
