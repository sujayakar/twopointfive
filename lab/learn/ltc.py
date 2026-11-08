# LINEARLY TRANSFORMED COSINES, fitted here: for each roughness and each slant of the eye, the linear map that turns a cosine lobe into the coat's (GGX, height-correlated
# masking), and how much light it sends back (F0 = 1, and what Fresnel adds). A lamp's image in a coat is then one exact polygon integral.
#   python3 lab/learn/ltc.py lab/src/ltc.json
import sys, json
import numpy as np
from scipy.optimize import minimize
N = 16; S = 40; u = (np.arange(S) + .5) / S; U1, U2 = [a.ravel() for a in np.meshgrid(u, u)]
def V_of(t): return np.array([np.sin(t), 0, np.cos(t)])
def brdf(V, L, a):                      # times the cosine; and the chance GGX half-vector sampling gives L
    H = V + L; H /= np.linalg.norm(H, axis=-1, keepdims=True); nl, nv, nh, vh = L[:, 2], V[2], np.clip(H[:, 2], 0, 1), np.clip((H * V).sum(-1), 1e-6, 1); a2 = a * a; d = nh * nh * (a2 - 1) + 1; D = a2 / (np.pi * d * d); ok = nl > 1e-4; nlc = np.maximum(nl, 1e-4); G = .5 / (nlc * np.sqrt(nv * nv * (1 - a2) + a2) + nv * np.sqrt(nlc * nlc * (1 - a2) + a2)); return np.where(ok, D * G * nlc, 0), np.where(ok, D * nh / (4 * vh), 1e-9), vh
def ggx_dirs(V, a): ph = 2 * np.pi * U1; ct = np.sqrt((1 - U2) / (1 + (a * a - 1) * U2)); st = np.sqrt(np.maximum(0, 1 - ct * ct)); H = np.stack([st * np.cos(ph), st * np.sin(ph), ct], -1); return 2 * (H @ V)[:, None] * H - V
def mat(p, X, Y, Z): m11, m22, m13 = p; return np.stack([X, Y, Z], 1) @ np.array([[m11, 0, m13], [0, m22, 0], [0, 0, 1]])
def ltc_pdf(M, L): Mi = np.linalg.inv(M); w = L @ Mi.T; l = np.linalg.norm(w, axis=-1); c = np.maximum(w[:, 2] / l, 0) / np.pi; return c * abs(np.linalg.det(Mi)) / l ** 3
def ltc_dirs(M): r = np.sqrt(U1); ph = 2 * np.pi * U2; l = np.stack([r * np.cos(ph), r * np.sin(ph), np.sqrt(np.maximum(0, 1 - U1))], -1) @ M.T; return l / np.linalg.norm(l, axis=-1, keepdims=True)
def err(M, V, a, norm):
    e = 0
    for L in (ltc_dirs(M), ggx_dirs(V, a)):
        f, pb, _ = brdf(V, L, a); f = f / norm; pl = ltc_pdf(M, L); up = L[:, 2] > 0; e += (np.abs(f - pl * 1.0) ** 3 / (pl + pb) * up).mean()
    return e
out = np.zeros((N, N, 6)); last_row = None
for i in range(N - 1, -1, -1):
    rough = max(i / (N - 1), .06); a = max(rough * rough, .0036); prev = None
    for j in range(N):
        x = j / (N - 1); ct = 1 - x * x; t = min(np.arccos(max(ct, 0)), 1.552); V = V_of(t); L = ggx_dirs(V, a); f, pb, vh = brdf(V, L, a); w = f / pb; norm = max(w.mean(), 1e-5); fr = (w * (1 - vh) ** 5).mean(); avg = (w[:, None] * L).mean(0); avg[1] = 0; avg /= np.linalg.norm(avg)
        if j == 0: Z = np.array([0, 0, 1.]); X = np.array([1., 0, 0]); Y = np.array([0, 1., 0]); p0 = (a if last_row is None else last_row, ) * 2 + (0.,)
        else: Z = avg; Y = np.array([0, 1., 0]); X = np.cross(Y, Z); p0 = prev
        if j == 0: r = minimize(lambda q: err(mat((max(q[0], 1e-3), max(q[0], 1e-3), 0.), X, Y, Z), V, a, norm), [p0[0]], method='Nelder-Mead', options=dict(xatol=1e-5, fatol=1e-9, maxiter=120)); p = (max(r.x[0], 1e-3), max(r.x[0], 1e-3), 0.); last_row = p[0]
        else: r = minimize(lambda q: err(mat((max(q[0], 1e-3), max(q[1], 1e-3), q[2]), X, Y, Z), V, a, norm), p0, method='Nelder-Mead', options=dict(xatol=1e-5, fatol=1e-9, maxiter=260)); p = (max(r.x[0], 1e-3), max(r.x[1], 1e-3), r.x[2])
        prev = p; M = mat(p, X, Y, Z); Mi = np.linalg.inv(M); Mi /= Mi[1, 1]; out[i, j] = [Mi[0, 0], Mi[0, 2], Mi[2, 0], Mi[2, 2], norm, fr]
    print(i, rough, np.round(out[i, [0, 8, 15]], 3).tolist(), flush=True)
json.dump([round(float(v), 5) for v in out.ravel()], open(sys.argv[1], 'w'), separators=(',', ':'))
# a check: a square lamp's image by the fit, and by brute force
rng = np.random.default_rng(1)
def poly(Mi, pts):
    P = pts @ Mi.T; P = P / np.linalg.norm(P, axis=-1, keepdims=True); s = 0
    for k in range(len(P)): A, B = P[k], P[(k + 1) % len(P)]; c = np.clip(A @ B, -1, 1); th = np.arccos(c); s += np.cross(A, B)[2] * (th / max(np.sin(th), 1e-6))
    return abs(s) / (2 * np.pi)
worst = []
for _ in range(60):
    i, j = rng.integers(3, 14), rng.integers(0, 13); rough = i / (N - 1); a = rough * rough; ct = 1 - (j / (N - 1)) ** 2; V = V_of(min(np.arccos(ct), 1.552)); c = np.array([-V[0] * 2 + rng.normal() * .6, rng.normal() * .6, 1.6 + rng.random()]); h = .25 + rng.random() * .6; pts = np.array([[c[0] - h, c[1] - h, c[2]], [c[0] + h, c[1] - h, c[2]], [c[0] + h, c[1] + h, c[2]], [c[0] - h, c[1] + h, c[2]]]); q = rng.random((40000, 2)) * 2 - 1; X = np.stack([c[0] + q[:, 0] * h, c[1] + q[:, 1] * h, np.full(40000, c[2])], -1); d = np.linalg.norm(X, axis=-1); L = X / d[:, None]; f, _, _ = brdf(V, L, a); true = (f * L[:, 2] / d ** 2).mean() * 4 * h * h; m = out[i, j]; Mi = np.array([[m[0], 0, m[1]], [0, 1, 0], [m[2], 0, m[3]]]); worst.append(poly(Mi, pts) * m[4] / max(true, 1e-9))
print('fit over brute force, 60 lamps: median %.3f, 10th %.3f, 90th %.3f' % tuple(np.percentile(worst, [50, 10, 90])))
