#!/bin/bash

for f in [0-9]*.xls; do
	[ -e "$f" ] || continue
	/Applications/LibreOffice.app/Contents/MacOS/soffice --headless --convert-to xlsx "$f" --outdir .
done
