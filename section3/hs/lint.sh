#!/usr/bin/env zsh
fswatch -0 **/*.hs | xargs -0 -n1 -I{} ormolu --mode inplace {}
