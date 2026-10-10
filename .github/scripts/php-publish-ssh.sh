#!/usr/bin/env bash
# Credentials exist only in protected, post-verification workflow steps.
set -euo pipefail
: "${PHP_DIST_DEPLOY_KEY:?Set the distribution repository deploy key in the packagist environment}"
: "${RUNNER_TEMP:?}"
key_dir=$(mktemp -d "$RUNNER_TEMP/php-publisher.XXXXXX")
trap 'rm -rf "$key_dir"' EXIT
printf '%s\n' "$PHP_DIST_DEPLOY_KEY" > "$key_dir/key"
chmod 600 "$key_dir/key"
# Authenticate host keys through GitHub's HTTPS API, never an unsigned SSH scan.
gh api meta --jq '.ssh_keys[] | "github.com " + .' > "$key_dir/known_hosts"
export GIT_SSH_COMMAND="ssh -i $key_dir/key -o IdentitiesOnly=yes -o UserKnownHostsFile=$key_dir/known_hosts -o StrictHostKeyChecking=yes"
unset PHP_DIST_DEPLOY_KEY
python3 php/bin/release.py "$@"
