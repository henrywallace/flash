#!/bin/sh

set -ex

cd "$(git rev-parse --show-toplevel)" || return

# rust
cargo fmt -- --check
cargo clippy --all-targets
cargo test

# playground
for file in $(find playground -maxdepth 1 -name '*.ipynb'); do
  if [ "$(wc -l "$file" | awk '{print $1}')" = 1 ]; then
    continue
  fi
  jq -c . < "$file" > /tmp/nb.ipynb
  mv -f /tmp/nb.ipynb "$file"
done
