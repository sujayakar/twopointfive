// ONE DEED UNDER WAY, BY A HAND ON THE REAL PAGE (rust/game_rules/src/sam/deed.rs): Sam at a locked door in the bazaar, real keys. What he is in the middle of is put down
// before the next thing begins, and nothing is left hanging: his rows come back, and his hands are free to draw.
//   bun run tools/qa/pages.ts deed-page      (PORT=<dev server> node tools/qa/deed-page.cjs: one boot, about two minutes on the software adapter; no picture is drawn)
// the pick: E begins it · E again or a step lets go and the lock keeps what was done · begun again it goes on from there · no other key, digit or button ends it or begins anything ·
//   E held on it is spent on letting go (no ring under it) · picked whole it is done of itself
// the kick: by the ring, after the pick is let go; and by a run into the door · the ease: begun, let go, begun again · the door hand: no takedown is on offer while it is on the leaf
// Exit 0 = every check passed.   DEED_BREAK=1: the pick is never let go before the kick is asked for (must FAIL).   Never seen on a GPU.
const { page, NOISE } = require('./page.cjs');
const P = page({ name: 'deed-page', detail: 'json' }), { check, note } = P, BREAK = process.env.DEED_BREAK === '1';
(async () => {
  const { page: w, errors, go, ready, step } = await P.open({ size: [1280, 720], errors: { also: /Invalid|error:/, skip: NOISE }, init: [() => { window.__testVisible = true; }] });
  await go('paused&quality=verylow&mute&cloud=0&seed=1&cards=0&levelFile=bazaar.json', { timeout: 600000 });
  await ready(); note('ready');
  await w.evaluate(() => { window.__draw.engine.render = () => {}; const c = window.__draw.game.commands; c.god(1); c.unseen(1); c.ai(0); });
  const run = n => step(n, [], 'none'), down = k => w.keyboard.down(k), up = k => w.keyboard.up(k);
  const tap = async (k, n = 3) => { await down(k); await run(n); await up(k); await run(3); };
  const see = () => w.evaluate(() => { const g = window.__draw.game, v = g.view(), m = g.map(), p = m.men.find(x => x.id === 1), near = m.doors.map(d => [Math.hypot(d.hx - p.x, d.hz - p.z), d]).sort((a, b) => a[0] - b[0])[0][1];
    return { x: p.x, z: p.z, rows: v.interact.rows.map(r => `${r.label}: ${r.opts.map(o => o.text).join(' / ')}`), done: Number(/(\d+)% picked|picking… (\d+)%/.exec(v.interact.rows.map(r => `${r.label} ${r.opts.map(o => o.text).join(' ')}`).join(' '))?.slice(1).find(Boolean) ?? 0), ring: v.hand && v.hand.wedges.length ? v.hand.wedges.map(x => `${x.key} ${x.text}`) : [],
      up: v.kit.slots.find(k => k.up)?.thing, locked: near.locked, broken: near.broken, open: Math.abs(Math.atan2(near.uz, near.ux)) > 0.2 }; });
  const aim = async (x, y, z) => { const p = await w.evaluate(q => window.__draw.cam.project(q, 1280, 720, 1), [x, y, z]); await w.mouse.move(Math.max(5, Math.min(1275, p[0] ?? p.x)), Math.max(5, Math.min(715, p[1] ?? p.y))); await run(3); };
  const stand = async (x, z) => { for (let i = 0; i < 6; i++) { await aim(x, 0, z); await w.evaluate(() => window.__draw.game.commands.act('teleport', 0, '')); await run(20); const s = await see(); if (Math.hypot(s.x - x, s.z - z) < 0.6) return true; } return false; };
  const load = async () => { await w.evaluate(() => window.__draw.game.commands.load(window.__draw.game.saves().rows[0].key)); await run(5); };
  const wedge = async re => { await down('e'); await run(30); const x = (await see()).ring.find(t => re.test(t)); if (x) await tap(x[0], 2); await up('e'); await run(3); return !!x; };
  const PICKING = /picking…/, OFFERED = /pick the lock/, row = s => s.rows.join(' · ');
  await run(30);
  let at = true; for (const [x, z] of [[12.75, 12.5], [12.75, 18.5], [12.75, 16.8]]) at = await stand(x, z) && at;
  await aim(12.75, 1, 16); await run(20);
  let s = await see(); check(at && s.locked && OFFERED.test(row(s)), 'he stands at a locked door and the pick is on offer', row(s));
  await w.evaluate(() => window.__draw.game.commands.save()); await run(5);

  // the pick, let go and begun again
  const pickOne = async () => { await load(); await tap('e'); await run(60); return see(); };
  s = await pickOne(); check(PICKING.test(row(s)) && s.done > 10, 'E begins the pick', row(s)); const one = s.done;
  await tap('e'); await run(10); s = await see(); check(OFFERED.test(row(s)) && s.done >= one, 'E again lets go, and the lock keeps what was done', s.done);
  await tap('e'); await run(60); s = await see(); check(PICKING.test(row(s)) && s.done > one + 10, 'begun again, it goes on from there', s.done); const two = s.done;
  await tap('w', 4); await run(10); s = await see(); check(OFFERED.test(row(s)) && s.done >= two, 'a step lets go', row(s));
  await tap('1'); await run(30); s = await see(); check(s.up === 'pistol', 'his hands are free: he draws', s.up);
  const held = []; for (const k of ['1', '3', 'c', 'v', 'r', 'q', 'f']) { await pickOne(); await tap(k, 4); await run(20); s = await see(); if (!PICKING.test(row(s)) || s.up !== 'hands') held.push(k); await tap('e'); await run(15); s = await see(); if (!OFFERED.test(row(s))) held.push(`${k}, then E`); }
  for (const button of ['left', 'right']) { await pickOne(); await w.mouse.down({ button }); await run(6); await w.mouse.up({ button }); await run(20); s = await see(); if (!PICKING.test(row(s))) held.push(`the ${button} button`); }
  check(!held.length, 'no other key, digit or button ends the pick or begins anything under it', held);
  await pickOne(); await down('e'); await run(30); s = await see(); await up('e'); await run(6); check(OFFERED.test(row(s)) && !s.ring.length, 'E held on it is spent on letting go: no ring under it', s.ring);

  // the kick: by the ring, and by a run into the door
  await pickOne(); let asked = false; if (!BREAK) { await tap('e'); await run(10); asked = await wedge(/kick it in/); } else await tap('3');
  await run(6); s = await see(); check(asked && /kicking it in…/.test(row(s)), 'the pick let go, the ring kicks it', row(s));
  await run(110); s = await see(); check(s.broken && s.open && !/kicking/.test(row(s)) && s.rows.length > 0, 'the door is in, and his rows are back', row(s));
  await tap('1'); await run(30); s = await see(); check(s.up === 'pistol', 'he draws after it', s.up);
  await pickOne(); await tap('s', 2); await run(4); await tap('c'); await run(40); await down('s'); await run(25); await up('s'); await run(6); await down('Shift'); await down('w'); await run(50); await up('w'); await up('Shift'); await run(90);
  s = await see(); check(s.broken && s.rows.length > 0 && !/kicking|picking/.test(row(s)), 'a run into it after a pick let go kicks it, and his rows are back', row(s));

  // the pick whole, and the ease
  await load(); await tap('e'); await run(330); s = await see(); check(!s.locked && /push it open/.test(row(s)), 'picked whole, it is done of itself', row(s));
  const eased = async () => { const ok = await wedge(/ease it open/); await run(20); const r = await see(); return ok && /easing it open…/.test(row(r)); };
  check(await eased(), 'the ring eases it'); await tap('e'); await run(10); s = await see(); check(/pull it shut|push it open/.test(row(s)) && !/easing/.test(row(s)), 'E lets go of the leaf where it is', row(s));
  check(await eased(), 'and eases it again'); await tap('e'); await tap('1'); await run(30); s = await see(); check(s.up === 'pistol', 'he draws after it', s.up);

  // the door hand: a takedown has no row while it is on the leaf
  await load(); for (const [x, z] of [[9, 18.5], [6.75, 17]]) await stand(x, z); await aim(6.75, 1, 16); await run(10);
  await tap('e'); await run(1); s = await see(); const under = row(s); await w.mouse.down({ button: 'left' }); await run(4); await w.mouse.up({ button: 'left' }); await run(120); s = await see();
  check(!/grab him|knock him out/.test(under), 'no takedown is on offer while his hand is on the leaf', under);
  check(s.rows.length > 0, 'and his rows are back after it', row(s)); await tap('1'); await run(30); s = await see(); check(s.up === 'pistol', 'he draws after it', s.up);
  check(errors.length === 0, 'no page, console or GPU error', errors);
  await P.end();
})().catch(e => { console.error(e); process.exit(2); });
