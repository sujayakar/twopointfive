#!/usr/bin/env python3
"""After `bun run build`: reduce dist/index.html to Artifact page content (title + styles + body + module script) at
dist/artifact/index.html and drop the embedded-buffer glTF next to it (the artifact host serves JSON/JS/text, not .bin).
Prints the chunk file name to pass in the Artifact `files` map. Usage: python3 tools/artifact-page.py"""
import re, os, json, base64
root = os.path.dirname(os.path.dirname(os.path.abspath(__file__))) + '/'
s = open(root + 'dist/index.html', encoding='utf-8').read()
head = s[s.find('<head>') + 6:s.find('</head>')]; body = s[s.find('<body>') + 6:s.find('</body>')]
title = re.search(r"<title>.*?</title>", head, re.S).group(0)
styles = "\n".join(re.findall(r"<style[^>]*>.*?</style>", head, re.S))
scripts = "\n".join(re.findall(r"<script[^>]*src=[^>]*></script>", head, re.S)).replace(' crossorigin', '')
chunk = re.search(r'src="\./([^"]+)"', scripts).group(1)
os.makedirs(root + 'dist/artifact/assets/ual', exist_ok=True)
open(root + 'dist/artifact/index.html', 'w', encoding='utf-8').write(f"{title}\n{styles}\n{body.strip()}\n{scripts}\n")
g = json.load(open(root + 'public/assets/ual/AnimationLibrary_Godot_Standard.gltf'))
uri = g['buffers'][0]['uri']
if not uri.startswith('data:'):
    data = open(root + 'public/assets/ual/' + uri, 'rb').read()
    g['buffers'][0]['uri'] = 'data:application/octet-stream;base64,' + base64.b64encode(data).decode('ascii')
json.dump(g, open(root + 'dist/artifact/assets/ual/AnimationLibrary_Godot_Standard.gltf', 'w'), separators=(',', ':'))
print(chunk)
