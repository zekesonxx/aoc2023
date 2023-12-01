#!/usr/bin/env bash

if [[ $# -ne 1 ]]; then
	echo "usage: newday.sh <day #>"
fi

sed -e "s/DAYNUMBER/$1/g" < src/template.rs >> "src/day$1.rs"
