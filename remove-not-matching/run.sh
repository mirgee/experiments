#!/bin/bash

mkdir -p ./artifacts/bindings-a
mkdir -p ./artifacts/bindings-b
mkdir -p ./artifacts/bindings-c
mkdir -p ./artifacts/bindings-d

mkdir -p ./artifacts/some-nonsensea-a
mkdir -p ./artifacts/some-nonsensea-b
mkdir -p ./artifacts/some-nonsensea-c

echo "a" > ./artifacts/bindings-a/a
echo "b" > ./artifacts/bindings-b/b
echo "c" > ./artifacts/bindings-c/c
echo "d" > ./artifacts/bindings-d/d

echo "nonsense" > ./artifacts/some-nonsensea-a/a
echo "nonsense" > ./artifacts/some-nonsensea-b/b
echo "nonsense" > ./artifacts/some-nonsensea-c/c

mkdir some-other-file
cp -r ./artifacts ./some-other-file
