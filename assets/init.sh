#!/bin/bash

curl \
	-L \
	https://github.com/sirhcel/embedded-vintage-fonts/raw/efde4f6250b8127a954e2055b1b12b81e1824fc6/data/font8x16_1bpp.raw \
	-o font8x16_1bpp.raw

for byte in $(\
	for row in `seq 0 6`; do \
		for horiz in `seq 1 29; echo 0;`; do \
			xxd -c1 -b font8x16_1bpp.raw \
				| tail -n +$((16*30*$row+1)) \
				| head -n $((16*30)) \
				| awk "NR%30==$horiz"; \
		done \
	done \
		| awk '{ print $2 }' \
		| head -n $((16*(126-32+1))) \
); do
	printf "%02x" $((2#$byte))
done \
	| xxd -r -p > font8x16_remapped_1bpp.raw

rm font8x16_1bpp.raw

curl \
	-L \
	https://github.com/sirhcel/embedded-vintage-fonts/raw/efde4f6250b8127a954e2055b1b12b81e1824fc6/LICENSE-MIT \
	-o FONT_CREDIT
