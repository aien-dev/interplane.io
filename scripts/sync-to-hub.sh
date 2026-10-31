#!/bin/sh
# scripts/sync-to-hub.sh: rebuild interplane.io from the protocol repository's public record and
# copy the result to the web server on the hub Pi. Runs from cron on the Spark once an hour.
#
#   REPO   clean clone of aien-dev/interplane.io on branch main   (default ~/workspace/web/interplane.io-live)
#   HUB    ssh host that serves the site                           (default hub)
#   WEBDIR document root on HUB                                    (default /var/www/interplane.io)
#   LOGDIR where this script keeps its log and the last facts file (default ~/.local/share/interplane-site)
#
# Rules: never run while the Spark quiet flag is held (chip runs own the machine then); never build
# a dirty checkout; if the facts step fails, rebuild from the last good facts so the site still
# follows a source change; refuse to publish a page that fails `site-builder verify`.
set -eu

REPO=${REPO:-"$HOME/workspace/web/interplane.io-live"}
HUB=${HUB:-hub}
WEBDIR=${WEBDIR:-/var/www/interplane.io}
LOGDIR=${LOGDIR:-"$HOME/.local/share/interplane-site"}
mkdir -p "$LOGDIR"
LOG="$LOGDIR/sync.log"
stamp() { date -u +%Y-%m-%dT%H:%M:%SZ; }
say() { printf '%s %s\n' "$(stamp)" "$*" | tee -a "$LOG"; }

if command -v quietlock >/dev/null 2>&1 && ! quietlock check >/dev/null 2>&1; then
  say "skip: Spark quiet flag held"
  exit 0
fi

if [ ! -d "$REPO/.git" ]; then
  say "clone into $REPO"
  git clone -q https://github.com/aien-dev/interplane.io.git "$REPO"
fi
cd "$REPO"
if [ -n "$(git status --porcelain)" ]; then
  say "refuse: $REPO is not clean"
  exit 2
fi
git fetch -q origin main
git checkout -q main
git merge -q --ff-only origin/main
HEAD=$(git rev-parse --short=12 HEAD)

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$REPO/target}"
cargo build -q --locked --release -p site-builder
BIN="$CARGO_TARGET_DIR/release/site-builder"

if "$BIN" facts "$LOGDIR/facts.json" >>"$LOG" 2>&1; then
  say "facts refreshed"
else
  if [ -s "$LOGDIR/facts.json" ]; then
    say "facts step failed; building with the last good facts"
  else
    say "facts step failed and no earlier facts exist; nothing published"
    exit 3
  fi
fi

rm -rf "$LOGDIR/dist"
"$BIN" build "$LOGDIR/dist" "$LOGDIR/facts.json" >>"$LOG" 2>&1
"$BIN" verify "$LOGDIR/dist" >>"$LOG" 2>&1 || { say "refuse: verify failed for $HEAD"; exit 4; }

NEW=$(cat "$LOGDIR/dist/index.html" "$LOGDIR/dist/facts.json" | cksum | cut -d' ' -f1)
OLD=$(cat "$LOGDIR/published.cksum" 2>/dev/null || echo none)
if [ "$NEW" = "$OLD" ]; then
  say "unchanged ($HEAD); nothing to publish"
  exit 0
fi

rsync -a --delete --exclude '.well-known' "$LOGDIR/dist/" "$HUB:$WEBDIR/" >>"$LOG" 2>&1
echo "$NEW" >"$LOGDIR/published.cksum"
say "published site $HEAD to $HUB:$WEBDIR"
