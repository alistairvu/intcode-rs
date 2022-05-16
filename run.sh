#!/bin/sh

cargo run -q $(python compiler.py "input/$1")
