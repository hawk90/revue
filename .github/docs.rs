[build.rs]
# Don't run build.rs in docs.rs - it sets env vars that may not work
# This is needed because our lib.rs uses env!() for VERSION, GIT_SHA, etc.
skip-build = true
