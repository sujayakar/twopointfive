#!/bin/sh
# bundle the headless page into $1
bun build src/shot.ts --outfile "$1/shot.js" --target browser --loader .wgsl:text && cp shot.html "$1/shot.html"
