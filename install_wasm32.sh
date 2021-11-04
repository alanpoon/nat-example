#!/bin/bash
STR=$(rustup target list)
SUB='wasm32-unknown-unknown (installed)'

if [[ "$STR" == *"$SUB"* ]]; then
  echo "It's there."
elif
  rustup target add wasm32-unknown-unknown
fi