#!/bin/bash

# Copyright © 2017–2019 University of Malta

# Copying and distribution of this file, with or without modification,
# are permitted in any medium without royalty provided the copyright
# notice and this notice are preserved. This file is offered as-is,
# without any warranty.

set -e

if [ -e public ]; then
	rm -r public
fi
mkdir public{,/gmp,/mpfr,/mpc}
cp cdoc-src/files/* public
CSS="--css-ref ../normalize.css --css-ref ../rustdoc.css --css-ref ../light.css"
makeinfo gmp*/doc/gmp.texi --html --split=chapter --output=public/gmp $CSS
makeinfo mpfr*/doc/mpfr.texi --html --split=chapter --output=public/mpfr $CSS
makeinfo mpc*/doc/mpc.texi --html --split=chapter --output=public/mpc $CSS
rustdoc cdoc-src/index.md --markdown-no-toc --output public \
	--markdown-css normalize.css \
	--markdown-css rustdoc.css \
	--markdown-css light.css \
	--html-before-content cdoc-src/before-content.html \
	--html-after-content cdoc-src/after-content.html
for l in gmp mpfr mpc; do
	L=$(echo $l | tr '[a-z]' '[A-Z]')
	for f in public/$l/*.html; do
		sed -i.rm~ \
		    's/..\/dir\/index.html\|dir.html#Top/..\/index.html/g' "$f"
		sed -i.rm~ -e '/<body/r cdoc-src/before-content-c.html' "$f"
		sed -n -i.rm~ -e '/<\/body>/r cdoc-src/after-content.html' \
		    -e 1x -e '2,${x;p}' -e '${x;p}' "$f"
		sed -i.rm~ 's,\(class="crate\)\(">'$L'</a>\),\1 current\2,' "$f"
		if [ $(basename $f) != index.html ]; then
			sed -i.rm~ 's,\(class="location">\)\(</p\),\1<a href="index.html">'$L'</a>\2,' "$f"
		fi
	done
done
find public -name \*.rm~ -delete
