#!/usr/bin/env python3
"""PreToolUse guard for the Dark Castle RPG project (Claude Code hook).

Second line of defence behind `permissions.deny` in `.claude/settings.json`.
Permission rules match command *text*; this hook tokenizes the command and
blocks dangerous intent regardless of flag order, wrappers or `bash -c`.

It is a heuristic, NOT a security boundary: variables, scripts, interpreters
(`python -c`), aliases and programs that write files on their own (the game
itself) can still get past it. See D-012 in docs/DECISIONS.md.

Protocol (Claude Code hooks): JSON on stdin; exit 2 + stderr = deny;
JSON `permissionDecision: "ask"` on stdout = ask the user; exit 0 = no opinion.
"""

from __future__ import annotations

import json
import os
import re
import shlex
import sys

PROTECTED_DIRS = ("Md-usario", "saves")
MAX_JOBS = 4

SEPARATORS = {"&&", "||", ";", "|", "|&", "&", "\n", "(", ")", "{", "}", "!"}
WRAPPERS = {
    "sudo", "env", "timeout", "time", "nice", "nohup", "stdbuf", "command",
    "builtin", "noglob", "exec", "xargs", "then", "do", "else", "if", "while",
    "until",
}
SHELLS = {"bash", "sh", "zsh", "dash"}
WRITE_TOOLS = {
    "rm", "rmdir", "unlink", "shred", "truncate", "touch", "chmod", "chown",
    "chgrp", "ln", "dd", "install", "tee", "mv",
}
GIT_OPTS_WITH_VALUE = {"-C", "-c", "--git-dir", "--work-tree", "--namespace"}
ASSIGN_RE = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*=")
NUMERIC_ARG_RE = re.compile(r"[+-]?[\d.]+[smhd]?")


class Verdict(Exception):
    def __init__(self, kind: str, reason: str) -> None:
        super().__init__(reason)
        self.kind = kind  # "deny" | "ask"
        self.reason = reason


def deny(reason: str) -> None:
    raise Verdict("deny", reason)


def ask(reason: str) -> None:
    raise Verdict("ask", reason)


# ---------------------------------------------------------------- paths

def project_dir() -> str:
    return os.path.realpath(os.environ.get("CLAUDE_PROJECT_DIR") or os.getcwd())


def resolve(path: str, cwd: str) -> str:
    path = os.path.expanduser(path.strip("'\""))
    if not os.path.isabs(path):
        path = os.path.join(cwd, path)
    return os.path.realpath(path)


def protected_hit(path: str, cwd: str) -> str | None:
    root = project_dir()
    real = resolve(path, cwd)
    for name in PROTECTED_DIRS:
        base = os.path.join(root, name)
        if real == base or real.startswith(base + os.sep):
            return name
    return None


def mentions_protected(token: str, cwd: str) -> str | None:
    if not token or token.startswith("-"):
        value = token.split("=", 1)[1] if "=" in token else ""
        return protected_hit(value, cwd) if value else None
    return protected_hit(token, cwd)


def is_project_target(path: str, cwd: str) -> bool:
    return resolve(path, cwd) == os.path.join(project_dir(), "target")


# ---------------------------------------------------------- tokenizing

def tokenize(command: str) -> list[str]:
    # Newlines and `backticks` start new commands; line continuations do not.
    command = command.replace("\\\n", " ").replace("\n", " ; ").replace("`", " ; ")
    lexer = shlex.shlex(command, posix=True, punctuation_chars=";&|()<>")
    lexer.whitespace_split = True
    lexer.commenters = ""
    try:
        return list(lexer)
    except ValueError:
        # Unbalanced quotes: fall back to a crude split rather than fail open.
        return re.split(r"\s+", command.strip())


def split_segments(tokens: list[str]) -> list[list[str]]:
    segments: list[list[str]] = [[]]
    for tok in tokens:
        if tok in SEPARATORS or (tok and set(tok) <= set(";&|()")):
            segments.append([])
        else:
            segments[-1].append(tok)
    return [s for s in segments if s]


def join_redirects(seg: list[str]) -> tuple[list[str], list[str]]:
    """Return (words, redirect_targets). shlex splits `>` from its target."""
    words: list[str] = []
    targets: list[str] = []
    i = 0
    while i < len(seg):
        tok = seg[i]
        if tok and set(tok) <= set("<>&|") and set(tok) & set("<>"):
            # `2>&1` arrives as "2", ">&", "1": drop the fd number before it.
            if words and words[-1].isdigit():
                words.pop()
            nxt = seg[i + 1] if i + 1 < len(seg) else ""
            fd_dup = tok.endswith("&") and (nxt.isdigit() or nxt == "-")
            if ">" in tok and nxt and not fd_dup:
                targets.append(nxt)
            i += 2
            continue
        words.append(tok)
        i += 1
    return words, [t for t in targets if t != "/dev/null"]


# ------------------------------------------------------------- checks

def check_env(assignments: list[str], cwd: str) -> None:
    for a in assignments:
        name, _, value = a.partition("=")
        if name in {"CARGO_TARGET_DIR", "CARGO_BUILD_TARGET_DIR"} and not is_project_target(value, cwd):
            deny(f"{name} must point to the project target/ (got {value!r}); see 'Compilação segura' in CLAUDE.md")
        if name in {"CARGO_BUILD_JOBS"} and (not value.isdigit() or int(value) > MAX_JOBS):
            deny(f"CARGO_BUILD_JOBS above {MAX_JOBS} is forbidden (got {value!r})")


def check_rm(args: list[str]) -> None:
    for a in args:
        if a == "--":
            break
        if a == "--recursive" or (a.startswith("-") and not a.startswith("--") and set(a[1:]) & {"r", "R"}):
            deny("recursive rm is blocked by project policy; delete specific files or ask the user")


def check_find(args: list[str]) -> None:
    if "-delete" in args:
        deny("find -delete is blocked by project policy")
    for i, a in enumerate(args):
        if a in {"-exec", "-execdir", "-ok", "-okdir"} and i + 1 < len(args):
            prog = os.path.basename(args[i + 1])
            if prog in {"rm", "rmdir", "shred", "unlink", "mv"}:
                deny(f"find {a} {prog} is blocked by project policy")


def git_subcommand(args: list[str]) -> tuple[str, list[str]]:
    i = 0
    while i < len(args):
        a = args[i]
        if a in GIT_OPTS_WITH_VALUE:
            i += 2
        elif a.startswith("-"):
            i += 1
        else:
            return a, args[i + 1:]
    return "", []


def check_git(args: list[str], cwd: str) -> None:
    if any(a == "--output" or a.startswith("--output=") for a in args):
        deny("git --output writes arbitrary files; use stdout instead")
    sub, rest = git_subcommand(args)
    flags = set(rest)

    def has_short(letter: str) -> bool:
        return any(a.startswith("-") and not a.startswith("--") and letter in a[1:] for a in rest)

    if sub == "reset" and flags & {"--hard", "--merge", "--keep"}:
        deny("git reset --hard/--merge/--keep is forbidden without explicit authorization")
    if sub == "clean" and not (flags & {"-n", "--dry-run"} or has_short("n")):
        deny("git clean is forbidden without explicit authorization")
    if sub == "push":
        if (flags & {"--force", "--mirror", "--delete", "--prune"} or has_short("f") or has_short("d")
                or any(a.startswith("--force") for a in rest)
                or any(a.startswith("+") or a.startswith(":") for a in rest)):
            deny("forced/deleting git push is forbidden without explicit authorization")
        ask("git push requires explicit user authorization")
    if sub == "checkout" and ("--" in rest or "." in rest or flags & {"--force", "-f"} or has_short("f")):
        deny("git checkout that discards working-tree changes is forbidden")
    if sub == "switch" and (flags & {"--discard-changes", "--force", "-f"}):
        deny("git switch --discard-changes/--force is forbidden")
    if sub == "restore" and ("--worktree" in flags or "-W" in flags or not (flags & {"--staged", "-S"})):
        deny("git restore of the working tree discards changes; only `git restore --staged` is allowed")
    if sub == "stash" and rest[:1] and rest[0] in {"drop", "clear"}:
        deny("git stash drop/clear is forbidden without explicit authorization")
    if sub == "stash" and not (rest[:1] and rest[0] in {"list", "show"}) and (
        flags & {"--include-untracked", "--all"} or has_short("u") or has_short("a")
    ):
        deny("git stash -u/-a would remove untracked user files (Md-usario/) from the working tree")
    if sub == "branch" and (flags & {"-D", "--force", "-f", "-M", "-C"}
                            or ({"--delete", "-d"} & flags and {"--force", "-f"} & flags)):
        deny("forced git branch delete/overwrite is forbidden")
    if sub == "worktree" and rest[:1] and rest[0] in {"add", "remove", "prune", "move"}:
        deny("git worktree add/remove/prune needs explicit user authorization ('Compilação segura' item 3)")
    if sub in {"rebase", "filter-branch", "filter-repo", "update-ref", "replace"}:
        deny(f"git {sub} rewrites history/refs and needs explicit user authorization")
    if sub == "reflog" and rest[:1] and rest[0] in {"expire", "delete"}:
        deny("git reflog expire/delete is forbidden")
    if sub == "gc" and any(a.startswith("--prune") for a in rest):
        deny("git gc --prune is forbidden")
    if sub in {"add", "rm", "mv", "checkout", "restore", "stash", "clean"}:
        for a in rest:
            hit = mentions_protected(a, cwd)
            if hit:
                deny(f"git {sub} on {hit}/ is forbidden: user files stay out of git operations")


def check_cargo(args: list[str], cwd: str) -> None:
    sub = next((a for a in args if not a.startswith("-") and not a.startswith("+")), "")
    if sub == "clean":
        deny("cargo clean is forbidden ('Compilação segura' item 4)")
    for i, a in enumerate(args):
        value = None
        if a == "--target-dir" and i + 1 < len(args):
            value = args[i + 1]
        elif a.startswith("--target-dir="):
            value = a.split("=", 1)[1]
        if value is not None and not is_project_target(value, cwd):
            deny(f"--target-dir must be the project target/ (got {value!r})")
        jobs = None
        if a in {"-j", "--jobs"} and i + 1 < len(args):
            jobs = args[i + 1]
        elif a.startswith("--jobs="):
            jobs = a.split("=", 1)[1]
        elif re.fullmatch(r"-j\d+", a):
            jobs = a[2:]
        if jobs is not None and (not jobs.isdigit() or int(jobs) > MAX_JOBS):
            deny(f"more than {MAX_JOBS} cargo jobs is forbidden (got {jobs!r})")
    if sub == "run" and "--example" in args:
        idx = args.index("--example")
        if idx + 1 < len(args) and args[idx + 1] == "autoplay":
            deny("autoplay writes to the real saves/slot1.ron; blocked until stage 3 isolates its save directory")
    if any(a.startswith("--example=autoplay") for a in args) and sub == "run":
        deny("autoplay writes to the real saves/slot1.ron; blocked until stage 3 isolates its save directory")
    if "--release" in args or "-r" in args or any(
        a == "--profile" or a.startswith("--profile=") for a in args
    ):
        ask("release/dist builds are heavy: confirm the user asked for it and `free -h` shows >= 6 GiB")


def check_protected_writes(prog: str, args: list[str], redirects: list[str], cwd: str) -> None:
    for t in redirects:
        hit = protected_hit(t, cwd)
        if hit:
            deny(f"writing to {hit}/ is forbidden by project policy")
    hits = [mentions_protected(a, cwd) for a in args]
    if not any(hits):
        return
    name = next(h for h in hits if h)
    if prog in WRITE_TOOLS:
        deny(f"{prog} on {name}/ is forbidden by project policy")
    if prog in {"cp", "rsync"}:
        positional = [a for a in args if not a.startswith("-")]
        into_target = any(
            a in {"-t", "--target-directory"} and i + 1 < len(args) and protected_hit(args[i + 1], cwd)
            for i, a in enumerate(args)
        ) or any(a.startswith("--target-directory=") and mentions_protected(a, cwd) for a in args)
        if into_target or (positional and protected_hit(positional[-1], cwd)):
            deny(f"{prog} into {name}/ is forbidden by project policy")
    if prog in {"sed", "perl"} and any(a.startswith("-i") or a == "--in-place" or a.startswith("--in-place=") for a in args):
        deny(f"in-place edit of {name}/ is forbidden by project policy")


def analyze(command: str, cwd: str, depth: int = 0) -> None:
    if depth > 4:
        deny("command nesting too deep to analyze safely")
    for seg in split_segments(tokenize(command)):
        words, redirects = join_redirects(seg)
        assignments: list[str] = []
        while words and (ASSIGN_RE.match(words[0]) or os.path.basename(words[0]) in WRAPPERS
                         or words[0] in {"export", "declare", "local", "readonly"}):
            head = words.pop(0)
            if ASSIGN_RE.match(head):
                assignments.append(head)
            elif os.path.basename(head) in {"env", "sudo", "xargs", "nice", "stdbuf", "timeout"}:
                # Drop options and numeric values (`-n 1`, `-k 5 10`, `30s`).
                while words and (words[0].startswith("-") or NUMERIC_ARG_RE.fullmatch(words[0])):
                    words.pop(0)
        check_env(assignments, cwd)
        if words and words[0] == "cd":
            # Later segments run in the new directory (`cd saves && rm x`).
            cwd = resolve(words[1], cwd) if len(words) > 1 and words[1] != "-" else cwd
            continue
        if not words:
            for t in redirects:
                if protected_hit(t, cwd):
                    deny("writing to a protected directory is forbidden by project policy")
            continue
        prog = os.path.basename(words[0])
        args = words[1:]
        if prog in SHELLS:
            # `-c`, `-lc`, `-ec`...: the next argument is the script.
            idx = next((i for i, a in enumerate(args)
                        if a.startswith("-") and not a.startswith("--") and "c" in a[1:]), None)
            if idx is not None and idx + 1 < len(args):
                analyze(args[idx + 1], cwd, depth + 1)
        if prog == "eval":
            analyze(" ".join(args), cwd, depth + 1)
        if prog == "rm":
            check_rm(args)
        elif prog == "find":
            check_find(args)
        elif prog == "git":
            check_git(args, cwd)
        elif prog == "cargo":
            check_cargo(args, cwd)
        check_protected_writes(prog, args, redirects, cwd)


def check_file_tool(tool_input: dict, cwd: str) -> None:
    path = tool_input.get("file_path") or tool_input.get("notebook_path")
    if isinstance(path, str):
        hit = protected_hit(path, cwd)
        if hit:
            deny(f"editing files in {hit}/ is forbidden by project policy")


def decide(event: dict) -> Verdict | None:
    cwd = event.get("cwd") or os.getcwd()
    tool = event.get("tool_name", "")
    tool_input = event.get("tool_input") or {}
    try:
        if tool == "Bash":
            analyze(str(tool_input.get("command", "")), cwd)
        elif tool in {"Edit", "Write", "MultiEdit", "NotebookEdit"}:
            check_file_tool(tool_input, cwd)
    except Verdict as v:
        return v
    return None


def main() -> int:
    try:
        event = json.load(sys.stdin)
        verdict = decide(event)
    except Exception as exc:  # fail closed: preserving data beats convenience
        print(f"[guard] could not analyze the tool call ({exc!r}); blocked. "
              "Fix .claude/hooks/guard.py or ask the user.", file=sys.stderr)
        return 2
    if verdict is None:
        return 0
    if verdict.kind == "deny":
        print(f"[guard] {verdict.reason}", file=sys.stderr)
        return 2
    print(json.dumps({"hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "ask",
        "permissionDecisionReason": f"[guard] {verdict.reason}",
    }}))
    return 0


if __name__ == "__main__":
    sys.exit(main())
