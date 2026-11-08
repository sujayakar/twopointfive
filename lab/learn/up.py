# A LEARNED HAND ON THE UPSCALER. The upscaler stays what it is (sums of real samples); a small network says, for each patch of the picture, how far history may stand
# off, how much to trust it, and how sharply to read this frame. Trained through time: frames unrolled, against the same poses standing still at full size.
#   python3 lab/learn/up.py <data dir> <out dir> [minutes]
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
    taps = torch.stack(taps, 1); ds = torch.stack(ds, 1); mean = taps.mean(1); sd = (taps.pow(2).mean(1) - mean * mean).clamp(min=1e-9).sqrt(); mv = torch.gather(aux.flatten(2), 2, bi.flatten(1)[:, None].expand(-1, 2, -1)).view(B, 2, oh, ow); wh = torch.tensor([ow, oh], device=d).view(1, 2, 1, 1); uv = so[None] / wh + mv
    return dict(taps=taps, ds=ds, mean=mean, sd=sd, prox=prox, uv=uv, inside=((uv > 0) & (uv < 1)).all(1, keepdim=True), px=mv * wh)
def past(p, hist, hn): h = catmull(torch.cat([hist, hn], 1), p['uv']); return torch.cat([h[:, :1].clamp(min=0), h[:, 1:3]], 1), h[:, 3:].clamp(min=0)
def step(p, hist, hn, ctl=None):
    """one frame of the upscaler. ctl: 4 channels, 0 = as it is by hand"""
    c = ctl if ctl is not None else torch.zeros_like(p['prox']).expand(-1, 4, -1, -1); sharp, width, trust, slow = (c[:, i:i + 1] for i in range(4))
    w = torch.exp(-p['ds'] * 1.9 * torch.exp(sharp)[:, None]); cur = (p['taps'] * w).sum(1) / w.sum(1).clamp(min=1e-6); hy, n0 = past(p, hist, hn); mv = p['px'].norm(dim=1, keepdim=True)
    n = torch.minimum(n0, (28 - 19 * (mv * .5).clamp(0, 1)) * torch.exp(slow)); ext = (p['sd'] * (2.6 - 1.6 * mv.clamp(0, 1)) + torch.tensor([.02, .01, .01], device=cur.device).view(1, 3, 1, 1) * p['mean'][:, :1].abs()) * torch.exp(width); dv = hy - p['mean']; k = (dv.abs() / ext.clamp(min=1e-5)).amax(1, keepdim=True); over = k > 1; hc = torch.where(over, p['mean'] + dv / k.clamp(min=1), hy); n = torch.where(over, torch.minimum(n, 28 / (.6 + .4 * k)), n)
    a = ((p['prox'] ** 2 + .02).clamp(0, 1) / (n * torch.exp(trust) + 1)).clamp(min=.02); o = torch.where(p['inside'], hc + (cur - hc) * a, cur); n = torch.where(p['inside'], (n + p['prox']).clamp(max=40), torch.full_like(n, .5)); return o, n.clamp(min=.5)
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
            hist, hn = begin(s, 0); e, fl, last = [], [], None
            for f in range(1, s.n):
                p = frame(s, f); hist, hn = step(p, hist, hn, control(net, p, hist, hn, s.oh, s.ow) if net is not None else None); d = rel(hist, truth(s, f)); e.append(d.abs().mean().item())
                if last is not None: fl.append(((d - catmull(last, p['uv'])).abs() * p['inside']).mean().item())
                last = d
            r[s.name] = (100 * np.mean(e), 100 * np.mean(fl))
    return r
def show(name, r): print(name, ' '.join(f'{k} {a:.2f}/{b:.2f}' for k, (a, b) in r.items()), ' mean %.3f / %.3f' % tuple(np.mean(list(r.values()), 0)), flush=True)
show('by hand, seen   (error %/flicker %)', score(None, train)); show('by hand, unseen', score(None, test))
def learn(net, minutes, lr, tag):
    opt = torch.optim.Adam(net.parameters(), lr); t0 = time.time(); it = 0; best = 1e9; keep = None; T = minutes * 60
    while time.time() - t0 < T:
        s = random.choice(train); n = 7; f0 = random.randrange(0, s.n - n - 1); opt.param_groups[0]['lr'] = lr * max(.05, 1 - (time.time() - t0) / T); loss = 0; hist, hn = begin(s, f0)
        if f0 and random.random() < .3: hist = hist + torch.randn_like(hist) * .03; hn = hn * random.random()
        last = None
        for f in range(f0 + 1, f0 + 1 + n):
            p = frame(s, f); hist, hn = step(p, hist, hn, control(net, p, hist, hn, s.oh, s.ow)); d = rel(hist, truth(s, f)); loss = loss + d.abs().mean() + .5 * (d[..., 1:] - d[..., :-1]).abs().mean() + .5 * (d[..., 1:, :] - d[..., :-1, :]).abs().mean()
            if last is not None: loss = loss + .5 * ((d - catmull(last, p['uv'])).abs() * p['inside']).mean()
            last = d
        opt.zero_grad(); (loss / n).backward(); torch.nn.utils.clip_grad_norm_(net.parameters(), 1.); opt.step(); it += 1
        if it % 200 == 0:
            r = score(net, train[::4]); m = np.mean([a + b for a, b in r.values()]); print(f'{tag} {it} steps {time.time() - t0:.0f} s, a quarter of the seen: {m:.3f}', flush=True)      # (chosen by what it has seen: the unseen is only for judging)
            if m < best: best = m; keep = {k: v.clone() for k, v in net.state_dict().items()}
    if keep: net.load_state_dict(keep)
    return net
cst = learn(Const().to(dev), 1.5, 3e-2, 'constants'); print('constants', (torch.tanh(cst.k) * 1.5).flatten().tolist()); show('best constants, seen  ', score(cst, train)); show('best constants, unseen', score(cst, test))
net = learn(Net().to(dev), MIN, 2e-3, 'network')
show('learned, seen  ', score(net, train)); show('learned, unseen', score(net, test)); sd = {k: v.detach().cpu().numpy().astype(np.float32) for k, v in net.state_dict().items()}
json.dump({k: v.flatten().round(5).tolist() for k, v in sd.items()}, open(OUT + '/up.json', 'w')); print('weights', sum(v.size for v in sd.values()))
