#!/usr/bin/env python3
"""Tests for guard.py. Run: python3 -B .claude/hooks/test_guard.py

Each case feeds a fake PreToolUse event to the hook in a subprocess; no command
is ever executed. Expected: "deny" (exit 2), "ask" (JSON ask) or "allow"
(exit 0, no output: normal permission flow continues).
"""

from __future__ import annotations

import json
import os
import shlex
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
GUARD = os.path.join(HERE, "guard.py")

BASH_CASES: list[tuple[str, str]] = [
    # --- rm variants
    ("rm -rf target/x", "deny"),
    ("rm -r target/x", "deny"),
    ("rm -fr target/x", "deny"),
    ("rm -R target/x", "deny"),
    ("rm -v -r target/x", "deny"),
    ("rm --recursive target/x", "deny"),
    ("/bin/rm -rf target/x", "deny"),
    ("command rm -rf target/x", "deny"),
    ("sudo rm -rf target/x", "deny"),
    ("FOO=1 rm -rf target/x", "deny"),
    ("bash -c 'rm -rf target/x'", "deny"),
    ("sh -c \"cd target && rm -rf x\"", "deny"),
    ("eval rm -rf target/x", "deny"),
    ("ls | xargs rm -r", "deny"),
    ("echo ok && rm -rf target/x", "deny"),
    ("find target -name '*.tmp' -delete", "deny"),
    ("find target -exec rm {} \\;", "deny"),
    ("rm target/autoplay/01.png", "allow"),
    # --- git destructive
    ("git reset --hard", "deny"),
    ("git reset -q --hard HEAD", "deny"),
    ("git -C . reset --hard", "deny"),
    ("git clean -fd", "deny"),
    ("git clean -n", "allow"),
    ("git push --force", "deny"),
    ("git push -f origin master", "deny"),
    ("git push origin +master", "deny"),
    ("git push --force-with-lease", "deny"),
    ("git push origin --delete feature", "deny"),
    ("git push origin master", "ask"),
    ("git checkout -- .", "deny"),
    ("git checkout .", "deny"),
    ("git checkout -f master", "deny"),
    ("git checkout -b feature", "allow"),
    ("git restore src/main.rs", "deny"),
    ("git restore --staged src/main.rs", "allow"),
    ("git stash drop", "deny"),
    ("git stash clear", "deny"),
    ("git branch -D feature", "deny"),
    ("git branch -d feature", "allow"),
    ("git worktree add target/wt HEAD", "deny"),
    ("git worktree list", "allow"),
    ("git rebase -i HEAD~3", "deny"),
    ("git diff --output=CLAUDE.md", "deny"),
    ("git log --output CLAUDE.md", "deny"),
    ("git show HEAD --output=x", "deny"),
    ("git diff --stat", "allow"),
    ("git status --short", "allow"),
    ("git add Md-usario/inicio.md", "deny"),
    ("git add saves/slot1.ron", "deny"),
    ("git add CLAUDE.md docs/DECISIONS.md", "allow"),
    ("git commit -m 'fix(save): restore saves on load'", "allow"),
    # --- protected directories via shell
    ("echo x > Md-usario/inicio.md", "deny"),
    ("echo x >> saves/slot1.ron", "deny"),
    ("echo x 2> saves/err.log", "deny"),
    ("tee saves/slot1.ron < /dev/null", "deny"),
    ("sed -i 's/a/b/' Md-usario/inicio.md", "deny"),
    ("mv Md-usario/inicio.md docs/", "deny"),
    ("cp docs/x.md Md-usario/", "deny"),
    ("touch saves/new.ron", "deny"),
    ("chmod 000 saves", "deny"),
    ("rm saves/slot1.ron", "deny"),
    ("truncate -s 0 ./saves/slot1.ron", "deny"),
    ("cat Md-usario/inicio.md", "allow"),
    ("sha256sum Md-usario/* saves/*", "allow"),
    ("ls -la saves", "allow"),
    ("cp saves/slot1.ron target/save-backup.ron", "allow"),
    ("grep -rn saves src", "allow"),
    # --- cargo
    ("cargo clean", "deny"),
    ("cargo clean -p dark_castle_rpg", "deny"),
    ("cargo build --target-dir /tmp/x", "deny"),
    ("cargo test --target-dir=/tmp/x", "deny"),
    ("CARGO_TARGET_DIR=/tmp/x cargo test", "deny"),
    ("export CARGO_TARGET_DIR=/tmp/x; cargo test", "deny"),
    ("env CARGO_TARGET_DIR=/tmp/x cargo check", "deny"),
    (f"CARGO_TARGET_DIR={shlex.quote(ROOT + '/target')} cargo test", "allow"),
    ("cargo build -j 12", "deny"),
    ("cargo build -j12", "deny"),
    ("cargo build --jobs=8", "deny"),
    ("CARGO_BUILD_JOBS=12 cargo build", "deny"),
    ("cargo build -j 2", "allow"),
    ("cargo run --example autoplay", "allow"),
    ("timeout 120 cargo run --example autoplay", "allow"),
    ("cargo run --example=autoplay", "allow"),
    ("cargo build --profile dist", "ask"),
    ("cargo build --release", "ask"),
    ("cargo test 2>&1 | tail -20", "allow"),
    ("cargo clippy --all-targets", "allow"),
    ("cargo fmt --check", "allow"),
    ("cargo run", "allow"),
    ("cargo check > /dev/null 2>&1", "allow"),
    # --- regressions found in the stage 2 review
    ("echo ok\nrm -rf target/x", "deny"),
    ("cargo test \\\n  --all-targets", "allow"),
    ("bash -lc 'rm -rf target/x'", "deny"),
    ("cd saves && rm slot1.ron", "deny"),
    ("cd Md-usario; echo x > inicio.md", "deny"),
    ("cd target && rm autoplay/01.png", "allow"),
    ("timeout -k 5 10 rm -rf target/x", "deny"),
    ("ls | xargs -n 1 rm -r", "deny"),
    ("nice -n 5 cargo build -j 8", "deny"),
    ("echo `rm -rf target/x`", "deny"),
    ("cp -t saves/ target/x.ron", "deny"),
    ("cp --target-directory=Md-usario target/x.md", "deny"),
    ("git stash -u", "deny"),
    ("git stash push --include-untracked", "deny"),
    ("git stash", "allow"),
    ("git stash list", "allow"),
    ("git commit -m 'docs: mention rm -rf; git reset --hard'", "allow"),
    ("git diff -- 'Md-usario/E2-Auditoria-Tecnica.md'", "allow"),
    ("cat 'Md-usario/arquivo com espaço.md' > target/copia.md", "allow"),
    ("echo x > 'saves/nome com espaço.ron'", "deny"),
]

# Raw stdin payloads: the hook must fail closed (deny) or stay neutral (allow).
RAW_CASES: list[tuple[str, str, str]] = [
    ("invalid json", "{not json", "deny"),
    ("empty stdin", "", "deny"),
    ("json array instead of object", "[]", "deny"),
    ("bash tool_input is a string", json.dumps({"tool_name": "Bash", "tool_input": "rm -rf x"}), "deny"),
    ("missing tool_name", json.dumps({"tool_input": {"command": "rm -rf x"}}), "allow"),
    ("missing tool_input", json.dumps({"tool_name": "Bash"}), "allow"),
    ("missing cwd", json.dumps({"tool_name": "Bash", "tool_input": {"command": "rm -rf x"}}), "deny"),
    ("unrelated tool", json.dumps({"tool_name": "Read", "tool_input": {"file_path": "/x"}}), "allow"),
]

FILE_CASES: list[tuple[str, str, str]] = [
    ("Edit", os.path.join(ROOT, "Md-usario", "inicio.md"), "deny"),
    ("Write", os.path.join(ROOT, "Md-usario", "novo.md"), "deny"),
    ("Write", os.path.join(ROOT, "saves", "slot1.ron"), "deny"),
    ("NotebookEdit", os.path.join(ROOT, "saves", "x.ipynb"), "deny"),
    ("Edit", os.path.join(ROOT, "docs", "DECISIONS.md"), "allow"),
    ("Write", os.path.join(ROOT, "docs", "saves-notes.md"), "allow"),
]


def run(event: dict) -> str:
    return run_raw(json.dumps(event))


def run_raw(payload: str) -> str:
    env = dict(os.environ, CLAUDE_PROJECT_DIR=ROOT)
    proc = subprocess.run(
        [sys.executable, "-B", GUARD], input=payload,
        capture_output=True, text=True, env=env, timeout=10, cwd=ROOT,
    )
    if proc.returncode == 2:
        return "deny"
    if proc.returncode == 0 and proc.stdout.strip():
        decision = json.loads(proc.stdout)["hookSpecificOutput"]["permissionDecision"]
        return decision
    if proc.returncode == 0:
        return "allow"
    return f"error({proc.returncode}): {proc.stderr.strip()}"


def autoplay_without_isolation() -> str:
    """A project whose autoplay lacks the save override must stay blocked."""
    with tempfile.TemporaryDirectory() as fake_root:
        os.makedirs(os.path.join(fake_root, "examples"))
        with open(os.path.join(fake_root, "examples", "autoplay.rs"), "w", encoding="utf-8") as f:
            f.write("fn main() { /* writes to saves/ */ }\n")
        env = dict(os.environ, CLAUDE_PROJECT_DIR=fake_root)
        event = {"tool_name": "Bash", "cwd": fake_root,
                 "tool_input": {"command": "cargo run --example autoplay"}}
        proc = subprocess.run([sys.executable, "-B", GUARD], input=json.dumps(event),
                              capture_output=True, text=True, env=env, timeout=10)
        return "deny" if proc.returncode == 2 else f"exit {proc.returncode}"


def main() -> int:
    failures = 0
    total = 0
    for command, expected in BASH_CASES:
        total += 1
        got = run({"tool_name": "Bash", "cwd": ROOT, "tool_input": {"command": command}})
        if got != expected:
            failures += 1
            print(f"FAIL bash {command!r}: expected {expected}, got {got}")
    for tool, path, expected in FILE_CASES:
        total += 1
        key = "notebook_path" if tool == "NotebookEdit" else "file_path"
        got = run({"tool_name": tool, "cwd": ROOT, "tool_input": {key: path}})
        if got != expected:
            failures += 1
            print(f"FAIL {tool} {path}: expected {expected}, got {got}")
    for label, payload, expected in RAW_CASES:
        total += 1
        got = run_raw(payload)
        if got != expected:
            failures += 1
            print(f"FAIL raw {label}: expected {expected}, got {got}")
    total += 1
    got = autoplay_without_isolation()
    if got != "deny":
        failures += 1
        print(f"FAIL autoplay without save isolation: expected deny, got {got}")
    print(f"{total - failures}/{total} passed")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
