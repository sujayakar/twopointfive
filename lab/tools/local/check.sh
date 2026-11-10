#!/bin/sh
# No GPU here: every method compiles and draws a few small frames on the software adapter. Prints CHECK OK, or what failed.
#   once: npm i playwright-core in $P; P=<folder for the site and the pictures> (default /tmp/lab-local); chromium at /usr/bin/chromium
P=${P:-/tmp/lab-local}; D=$(cd "$(dirname "$0")" && pwd); mkdir -p "$P/site"; cp "$D/one.cjs" "$D/serve.cjs" "$P/"
(cd "$D/../.." && sh tools/dev.sh "$P/site" >/dev/null) || exit 1
curl -s -o /dev/null http://localhost:5999/ || (cd "$P" && setsid node serve.cjs "$P/site" 5999 >/dev/null 2>&1 &) ; sleep 1
cd "$P" && node one.cjs c1.png '{"w":160,"h":90,"dist":24,"hour":14,"look":[8,4,1.5],"pitch":0.7,"samples":2,"method":"learnedup","tick":600}' c2.png '{"w":160,"h":90,"dist":34,"hour":21.5,"cut":2.55,"look":[0,0.6,0],"doors":0.8,"samples":2,"method":"norays","tick":600}' c3.png '{"w":160,"h":90,"dist":24,"hour":14,"samples":1,"method":"reference","tick":600}' c4.png '{"w":96,"h":54,"dist":24,"hour":14,"samples":1,"method":"raw","tick":600}' c5.png '{"w":96,"h":54,"dist":24,"hour":14,"samples":1,"method":"nocache","tick":600}' c6.png '{"w":96,"h":54,"dist":24,"hour":14,"samples":1,"method":"bilinear","tick":600}' && echo CHECK OK
