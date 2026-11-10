# A LEARNED HAND ON THE UPSCALER, A PIXEL AT A TIME. As up.py, but a tiny network at every output pixel weighs the nine samples round it one by one, and says how much to trust
# history and how far it may stand off. What is shown is still sums of real samples. Against: the upscaler as it is by hand, its best constants, and up.py's network of patches.
#   python3 lab/learn/pix.py <data dir> <out dir> [minutes]
import sys, os, json, time, glob, random
import numpy as np, torch, torch.nn.functional as F
D, OUT = sys.argv[1], sys.argv[2]; MIN = float(sys.argv[3]) if len(sys.argv) > 3 else 12; os.makedirs(OUT, exist_ok=True); dev = 'cuda' if torch.cuda.is_available() else 'cpu'; torch.manual_seed(1); random.seed(1)
KX = 0.25; LUMA = torch.tensor([.2126, .7152, .0722], device=dev).view(1, 3, 1, 1)
lum = lambda c: (c * LUMA).sum(1, keepdim=True)
tm = lambda c: c / (1 + lum(c) * KX)
ycc = lambda c: torch.cat([.25 * c[:, :1] + .5 * c[:, 1:2] + .25 * c[:, 2:], .5 * (c[:, :1] - c[:, 2:]), -.25 * c[:, :1] + .5 * c[:, 1:2] - .25 * c[:, 2:]], 1)
def load(name, w, h): return torch.from_numpy(np.fromfile(os.path.join(D, name), np.float16).reshape(h, w, 4).astype(np.float32)).permute(2, 0, 1)
class Seq:
    def __init__(s, meta):
        s.name = os.path.basename(meta)[:-5]; j = json.load(open(meta)); (s.rw, s.rh), (s.ow, s.oh) = j['render'], j['output']; s.n = len(j['meta']); base = s.name.rsplit('-', 1)[0]
        s.col = torch.stack([load(f'{s.name}-{f}.color', s.rw, s.rh) for f in range(s.n)]).clamp(-100, 6e4).half(); s.aux = torch.stack([load(f'{s.name}-{f}.aux', s.rw, s.rh)[:2] for f in range(s.n)]); s.truth = torch.stack([load(f'{base}-{f}.truth', s.ow, s.oh)[:3] for f in range(s.n)]).clamp(0, 6e4).half(); s.pic0 = load(f'{s.name}-0.pic', s.ow, s.oh).clamp(0, 6e4); s.jit = torch.tensor([m['jitter'] for m in j['meta']])
def catmull(h, uv):                         # what the shader's five taps come to, near enough: bicubic, held above nothing where it must be
    return F.grid_sample(h, uv.permute(0, 2, 3, 1) * 2 - 1, mode='bicubic', padding_mode='border', align_corners=False)
def prep(col, aux, jit, ow, oh):
    """this frame's samples seen from every output pixel: as the shader does it"""
    B, _, rh, rw = col.shape; d = col.device; ys, xs = torch.meshgrid(torch.arange(oh, device=d), torch.arange(ow, device=d), indexing='ij'); so = torch.stack([xs, ys]).float() + .5; r = torch.tensor([rw / ow, rh / oh], device=d).view(2, 1, 1); s = so * r; jj = jit.view(B, 2, 1, 1); q = torch.floor(s[None] - jj).long()
    rgb, t = col[:, :3], col[:, 3:4]; flat = torch.cat([ycc(tm(rgb)), t], 1).flatten(2); taps, ds = [], []; best = torch.full((B, 1, oh, ow), 1e9, device=d); bi = torch.zeros((B, oh, ow), dtype=torch.long, device=d); prox = torch.zeros((B, 1, oh, ow), device=d)
    for dy in (-1, 0, 1):
        for dx in (-1, 0, 1):
            tx = (q[:, 0] + dx).clamp(0, rw - 1); ty = (q[:, 1] + dy).clamp(0, rh - 1); ix = ty * rw + tx; v = torch.gather(flat, 2, ix.flatten(1)[:, None].expand(-1, 4, -1)).view(B, 4, oh, ow); dd = (torch.stack([tx, ty], 1).float() + .5 + jj - s[None]) / r[None]; taps.append(v[:, :3]); ds.append((dd * dd).sum(1, keepdim=True)); a = dd.abs(); prox = torch.maximum(prox, (1.35 - a[:, :1]).clamp(0, 1) * (1.35 - a[:, 1:]).clamp(0, 1)); nt = v[:, 3:4]; m = nt < best; best = torch.where(m, nt, best); bi = torch.where(m[:, 0], ix, bi)
    tt = torch.gather(t.flatten(2), 2, ((q[:, 1].clamp(0, rh - 1)) * rw + q[:, 0].clamp(0, rw - 1)).flatten(1)[:, None]).view(B, 1, oh, ow); off = (q.float() + .5 + jj - s[None]) / r[None]; deep = ((tt - best) / best.clamp(min=.1)).clamp(0, 2); taps = torch.stack(taps, 1); ds = torch.stack(ds, 1); mean = taps.mean(1); sd = (taps.pow(2).mean(1) - mean * mean).clamp(min=1e-9).sqrt(); mv = torch.gather(aux.flatten(2), 2, bi.flatten(1)[:, None].expand(-1, 2, -1)).view(B, 2, oh, ow); wh = torch.tensor([ow, oh], device=d).view(1, 2, 1, 1); uv = so[None] / wh + mv
    return dict(taps=taps, ds=ds, mean=mean, sd=sd, prox=prox, off=off, deep=deep, uv=uv, inside=((uv > 0) & (uv < 1)).all(1, keepdim=True), px=mv * wh)
def past(p, hist, hn): h = catmull(torch.cat([hist, hn], 1), p['uv']); return torch.cat([h[:, :1].clamp(min=0), h[:, 1:3]], 1), h[:, 3:].clamp(min=0)
def step(p, hist, hn, ctl=None, pix=None):
    """one frame of the upscaler, as taau.wgsl has it. ctl: 4 channels a patch, 0 = as it is; pix: 12 channels a pixel"""
    zero = torch.zeros_like(p['prox']); c = ctl if ctl is not None else zero.expand(-1, 4, -1, -1); sharp, width, trust, slow = (c[:, i:i + 1] for i in range(4)); mv = p['px'].norm(dim=1, keepdim=True); go = mv.clamp(0, 1)
    lw = -p['ds'] * ((4.0 - 2.8 * go) * torch.exp(sharp))[:, None]
    if pix is not None: lw = lw + pix[:, :9, None]; trust = trust + pix[:, 9:10]; width = width + pix[:, 10:11]
    w = torch.exp(lw); cur = (p['taps'] * w).sum(1) / w.sum(1).clamp(min=1e-6)
    if pix is not None: lo = p['taps'].amin(1); hi = p['taps'].amax(1); cur = torch.minimum(torch.maximum(cur + (cur - p['mean']) * pix[:, 11:12] * 0, lo), hi)
    hy, n0 = past(p, hist, hn); n = torch.minimum(n0, (28 - 19 * (mv * .5).clamp(0, 1)) * torch.exp(slow)); ext = (p['sd'] * (2.6 - 1.6 * go) + torch.tensor([.02, .01, .01], device=cur.device).view(1, 3, 1, 1) * p['mean'][:, :1].abs()) * 1.1 * torch.exp(width); dv = hy - p['mean']; k = (dv.abs() / ext.clamp(min=1e-5)).amax(1, keepdim=True); over = k > 1; hc = torch.where(over, p['mean'] + dv / k.clamp(min=1), hy); n = torch.where(over, torch.minimum(n, 28 / (.6 + .4 * k)), n)
    a = ((p['prox'] ** 2 + .02).clamp(0, 1) / (n * (1 - .45 * go) * torch.exp(trust) + 1)).clamp(min=.02); o = torch.where(p['inside'], hc + (cur - hc) * a, cur); n = torch.where(p['inside'], (n + p['prox']).clamp(max=40), torch.full_like(n, .5)); return o, n.clamp(min=.5)
def about(p, hist, hn):
    """what a pixel's network is told (21): its nine samples against their mean, where between them it lies, how far history stands off, how varied, how deep, how fast"""
    hy, n = past(p, hist, hn); m, s = p['mean'], p['sd']; my = m[:, :1].abs() + .05; mv = p['px'].norm(dim=1, keepdim=True); t = p['deep']; d = hy - m
    return torch.cat([((p['taps'][:, :, 0] - m[:, :1]) / my).clamp(-3, 3), p['off'], (d[:, :1] / (s[:, :1] + .02 * my + 1e-3) * .2).clamp(-3, 3), (d[:, :1] / my).clamp(-3, 3), (d[:, 1:].norm(dim=1, keepdim=True) / my).clamp(max=3), (s[:, :1] / my).clamp(max=3), torch.log(my) * .3, n / 28, (mv * .25).clamp(max=3), p['inside'].float(), p['prox'], t], 1)
def feats(p, hist, hn):
    hy, n = past(p, hist, hn); m, s = p['mean'], p['sd']; e = lambda x: torch.log1p(x.clamp(min=0) * 8); mv = p['px'].norm(dim=1, keepdim=True)
    return F.avg_pool2d(torch.cat([e(m[:, :1]), m[:, 1:3], e(s[:, :1]), e((hy[:, :1] - m[:, :1]).abs()), ((hy[:, :1] - m[:, :1]) / (s[:, :1] + .02 * m[:, :1].abs() + 1e-3) * .2).clamp(-3, 3), hy[:, 1:3] - m[:, 1:3], e(hy[:, :1]), (mv * .25).clamp(max=3), n / 28, p['inside'].float()], 1)[..., 1::2, 1::2], 2)      # (four of a patch's sixteen pixels, as the shader looks)
class Const(torch.nn.Module):               # the best that four numbers alone could do: what a network must beat
    def __init__(s): super().__init__(); s.k = torch.nn.Parameter(torch.zeros(1, 4, 1, 1))
    def forward(s, f): return (torch.tanh(s.k) * 1.5).expand(f.shape[0], 4, f.shape[2], f.shape[3])
class Net(torch.nn.Module):
    def __init__(s, c=12):
        super().__init__(); s.a = torch.nn.Conv2d(12, c, 3, padding=1); s.b = torch.nn.Conv2d(c, c, 3, padding=1); s.c = torch.nn.Conv2d(c, 4, 3, padding=1); torch.nn.init.normal_(s.c.weight, std=1e-3); torch.nn.init.zeros_(s.c.bias)
    def forward(s, f): return torch.tanh(s.c(F.relu(s.b(F.relu(s.a(f)))))) * 1.5
class Pix(torch.nn.Module):
    per_pixel = True
    def __init__(s, a=24, b=16):
        super().__init__(); s.a = torch.nn.Conv2d(21, a, 1); s.b = torch.nn.Conv2d(a, b, 1); s.c = torch.nn.Conv2d(b, 12, 1); torch.nn.init.normal_(s.c.weight, std=1e-3); torch.nn.init.zeros_(s.c.bias); s.register_buffer('most', torch.tensor([2.5] * 9 + [2., 1.5, .6]).view(1, 12, 1, 1))
    def forward(s, f): return torch.tanh(s.c(F.relu(s.b(F.relu(s.a(f)))))) * s.most
def go(net, p, hist, hn, s): return step(p, hist, hn, pix=net(about(p, hist, hn))) if getattr(net, 'per_pixel', False) else step(p, hist, hn, control(net, p, hist, hn, s.oh, s.ow)) if net is not None else step(p, hist, hn)
def control(net, p, hist, hn, oh, ow): return F.interpolate(net(feats(p, hist, hn)), size=(oh, ow), mode='bilinear', align_corners=False)
def rel(o, t): return (o - t) / (t[:, :1].abs() + .06)
seqs = [Seq(m) for m in sorted(glob.glob(D + '/*.json'))]; train = [s for s in seqs if not s.name.startswith('T-')]; test = [s for s in seqs if s.name.startswith('T-')]; print(len(train), 'to learn from,', len(test), 'never shown:', [s.name for s in test], flush=True)
truth = lambda s, f: ycc(tm(s.truth[f][None].float().to(dev)))
def begin(s, f0): return (ycc(tm(s.pic0[None, :3].to(dev))), s.pic0[None, 3:].to(dev)) if f0 == 0 else (truth(s, f0), torch.full((1, 1, s.oh, s.ow), 20., device=dev))
def frame(s, f): return prep(s.col[f][None].float().to(dev), s.aux[f][None].to(dev), s.jit[f][None].float().to(dev), s.ow, s.oh)
def score(net, which):
    r = {}
    with torch.no_grad():
        for s in which:
            hist, hn = begin(s, 0); e, fl, last = [], [], None; rounds = 5 if 'still' in s.name else 1
            for f in list(range(1, s.n)) * rounds:
                p = frame(s, f); hist, hn = go(net, p, hist, hn, s); d = rel(hist, truth(s, f)); e.append(d.abs().mean().item())
                if last is not None: fl.append(((d - catmull(last, p['uv'])).abs() * p['inside']).mean().item())
                last = d
            r[s.name] = (100 * np.mean(e), 100 * np.mean(fl))
    return r
def show(name, r): print(name, ' '.join(f'{k} {a:.2f}/{b:.2f}' for k, (a, b) in r.items()), ' mean %.3f / %.3f' % tuple(np.mean(list(r.values()), 0)), flush=True)
show('by hand, seen   (error %/flicker %)', score(None, train)); show('by hand, unseen', score(None, test))
def learn(net, minutes, lr, tag):
    opt = torch.optim.Adam(net.parameters(), lr); t0 = time.time(); it = 0; best = 1e9; keep = None; T = minutes * 60
    states = {}
    while time.time() - t0 < T:
        s = random.choice(train); n = 5; opt.param_groups[0]['lr'] = lr * max(.05, 1 - (time.time() - t0) / T); loss = 0; st = states.get(s.name)
        if st is None or st[0] + n >= s.n:
            if st is not None and 'still' in s.name and random.random() < .85: st = (0, st[1], st[2])
            else: f0 = random.randrange(0, s.n - n - 1); hist, hn = begin(s, f0); q = random.random(); st = (f0, hist, hn) if q < .4 else (f0, frame(s, f0)['mean'], torch.full_like(hn, .5)) if q < .8 else (f0, hist + torch.randn_like(hist) * .03, hn * random.random())
        f0, hist, hn = st; last = None
        for f in range(f0 + 1, f0 + 1 + n):
            p = frame(s, f); hist, hn = go(net, p, hist, hn, s); d = rel(hist, truth(s, f)); loss = loss + d.abs().mean() + .5 * (d[..., 1:] - d[..., :-1]).abs().mean() + .5 * (d[..., 1:, :] - d[..., :-1, :]).abs().mean()
            if last is not None: loss = loss + .5 * ((d - catmull(last, p['uv'])).abs() * p['inside']).mean()
            last = d
        states[s.name] = (f0 + n, hist.detach(), hn.detach())
        opt.zero_grad(); (loss / n).backward(); torch.nn.utils.clip_grad_norm_(net.parameters(), 1.); opt.step(); it += 1
        if it % 200 == 0:
            r = score(net, train[::4]); m = np.mean([a + b for a, b in r.values()]); print(f'{tag} {it} steps {time.time() - t0:.0f} s, a quarter of the seen: {m:.3f}', flush=True)      # (chosen by what it has seen: the unseen is only for judging)
            if m < best: best = m; keep = {k: v.clone() for k, v in net.state_dict().items()}
    if keep: net.load_state_dict(keep)
    return net
def part(r, k): q = [v for n, v in r.items() if (k in n) == True] if k != 'fast' else [v for n, v in r.items() if 'slow' not in n and 'still' not in n]; return '%s %.3f / %.3f' % ((k,) + tuple(np.mean(q, 0))) if q else ''
def tell(name, net): r = score(net, test); show(name + ', unseen', r); print('   ', name, ' '.join(part(r, k) for k in ('fast', 'slow', 'still')), flush=True)
tell('by hand', None); cst = learn(Const().to(dev), 1.5, 3e-2, 'constants'); print('constants', (torch.tanh(cst.k) * 1.5).flatten().tolist()); tell('best constants', cst)
net = learn(Net().to(dev), MIN * .4, 2e-3, 'patches'); tell('patches', net)
pix = learn(Pix().to(dev), MIN, 2e-3, 'pixels'); tell('pixels', pix)
small = learn(Pix(12, 8).to(dev), MIN * .5, 2e-3, 'small pixels'); tell('small pixels', small)
for n, m in (('pix', pix), ('pixsmall', small), ('patch', net)): sd = {k: v.detach().cpu().numpy().astype(np.float32) for k, v in m.state_dict().items()}; json.dump({k: v.flatten().round(5).tolist() for k, v in sd.items()}, open(OUT + f'/{n}.json', 'w')); print(n, 'weights', sum(v.size for v in sd.values()))
