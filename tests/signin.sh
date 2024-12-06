#!/usr/bin/env bash

curl -X POST \
  "http://localhost:1234/auth/signin" \
  -H "Content-Type: application/json" \
  -d '{ "signature": "signature", "message": "message", "nonce": "nonce" }' 2>&1
