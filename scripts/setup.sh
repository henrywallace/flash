#!/bin/sh

nb_venv() {
  root=$(git rev-parse --show-toplevel)
  cd "$root/playground" || return

  # Install virtualenv, and pip-tools if not already present.
  if [ ! -d venv ]; then
    virtualenv -p "$(command -v python3)" venv
    . venv/bin/activate
    pip install pip-tools
  fi

  # Install requirements.
  . venv/bin/activate
  pip-compile -o requirements.txt requirements.in
  pip-sync
}

nb_venv
