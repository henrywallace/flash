#!/bin/sh

glove() {
  zip=data/glove.840B.300d.zip
  txt=data/glove.840B.300d.txt
  tiny="$txt.tiny"
  if [ ! -f $zip ] && [ ! -f $txt ]; then
    curl -L -o $zip -C - http://nlp.stanford.edu/data/glove.840B.300d.zip
  fi
  [ -f $txt ] || unzip $zip -d data
  [ -f $tiny ] || head -20000 $txt > $tiny
}

all() {
  glove
}

all
