#!/bin/bash

set -euo pipefail

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <day nr>"
    echo "Example: $0 09"
    exit 1
fi

if [ "${#1}" -ne 2 ]; then
    echo "Error: day nr must be two digits, for example 09."
    exit 1
fi


touch input/$1.txt
