#!/usr/bin/env zsh
ghcid "--command=stack ghci src/*.hs test/*.hs" -T=main
