#!/bin/sh

cargo run -q $(cargo run -q --bin compile input/$1) $2
