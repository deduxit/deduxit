#!/usr/bin/env python3
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
"""The Rocq library's inventory: its build, its warnings, its assumptions.

For step 86's audit (plan/notes/audit/method.md, the inventories). Copies
the library's sources into WORK_DIR, so that its checkout is only read,
and prints Markdown:
- each file's compile time, with every warning on, and the warnings;
- the files a build with `-mangle-names` refuses: their proofs rely on a
  name Rocq generated;
- `Print Assumptions` of every theorem and lemma, against the ones
  `tests/Assumptions.v` asks about;
- how the sources require the standard library and declare hints;
- the modules the certificates of deduxit's exporter import.

usage: rocq.py ROCQ_LIBRARY DEDUXIT_ROOT WORK_DIR

Run it where `rocq` and the standard library are at hand (the library's
devshell), in a capped scope.
"""

import os
import re
import shutil
import subprocess
import sys
import time
from collections import Counter

LIBRARY = ["-Q", "theories", "Deduxit"]
TESTS = [
    ("tests", "tests-linear", "Deduxit.Linear.Tests"),
    ("tests/Intuitionistic", "tests-intuitionistic", "Deduxit.Intuitionistic.Tests"),
    ("tests/Modal", "tests-modal", "Deduxit.Modal.Tests"),
]
STATEMENT = re.compile(
    r"^\s*(?:#\[[^\]]*\]\s*)?(?:(?:Local|Global|Program)\s+)*"
    r"(Theorem|Lemma|Corollary|Proposition|Fact|Remark)\s+([A-Za-z_][A-Za-z0-9_']*)"
)
MODULE = re.compile(r"^\s*Module\s+([A-Za-z_][A-Za-z0-9_']*)\s*\.")
SECTION = re.compile(r"^\s*Section\s+([A-Za-z_][A-Za-z0-9_']*)\s*\.")
END = re.compile(r"^\s*End\s+([A-Za-z_][A-Za-z0-9_']*)\s*\.")
WARNING = re.compile(r"\[([a-z0-9-]+(?:,[a-z0-9-]+)*)\]\s*\Z")
ERROR_LINE = re.compile(r"line (\d+), characters")
CLOSED = "Closed under the global context"


def rocq(work, arguments):
    """Runs `rocq compile` in the copy; returns seconds, status and output."""
    start = time.monotonic()
    result = subprocess.run(
        ["rocq", "compile", *arguments],
        cwd=work,
        capture_output=True,
        text=True,
        check=False,
    )
    return time.monotonic() - start, result.returncode, result.stdout, result.stderr


def project_files(work):
    """Returns the files `_RocqProject` lists, in its order."""
    with open(os.path.join(work, "_RocqProject"), encoding="utf-8") as f:
        return [line.strip() for line in f if line.strip().endswith(".v")]


def copy_sources(library, work):
    """Copies the library's `.v` files and its project file into `work`."""
    shutil.rmtree(work, ignore_errors=True)
    os.makedirs(work)
    shutil.copy(os.path.join(library, "_RocqProject"), work)
    for d, _, names in os.walk(os.path.join(library, "theories")):
        for name in names:
            if name.endswith(".v"):
                target = os.path.join(work, os.path.relpath(d, library))
                os.makedirs(target, exist_ok=True)
                shutil.copy(os.path.join(d, name), target)
    for source, copy, _ in TESTS:
        os.makedirs(os.path.join(work, copy))
        for name in sorted(os.listdir(os.path.join(library, source))):
            if name.endswith(".v"):
                shutil.copy(
                    os.path.join(library, source, name), os.path.join(work, copy)
                )


def warnings_of(text):
    """Returns the names of the warnings in a compiler's output."""
    names = []
    for line in text.splitlines():
        found = WARNING.search(line.strip())
        if found:
            names.append(found.group(1).split(",")[0])
    return names


def statements(path):
    """Yields the theorems and lemmas of a file with the modules around each."""
    stack = []
    with open(path, encoding="utf-8") as f:
        for line in f:
            if MODULE.match(line):
                stack.append(("module", MODULE.match(line).group(1)))
            elif SECTION.match(line):
                stack.append(("section", SECTION.match(line).group(1)))
            elif END.match(line) and stack:
                stack.pop()
            else:
                found = STATEMENT.match(line)
                if found:
                    modules = [name for kind, name in stack if kind == "module"]
                    yield ".".join([*modules, found.group(2)])


def answers(output):
    """Splits the output of several `Print Assumptions` into its answers."""
    result = []
    for line in output.splitlines():
        if line.strip() == CLOSED:
            result.append(None)
        elif line.startswith(("Axioms:", "Section Variables:")):
            result.append([line.strip()])
        elif result and result[-1] is not None and line.strip():
            result[-1].append(line.strip())
    return result


def build(work, files):
    """Compiles the library with every warning on; prints times and warnings."""
    print("## The build, with every warning on (`-w all`)\n")
    print("| file | s | warnings |\n|---|---:|---|")
    total = 0.0
    seen = Counter()
    failed = []
    details = []
    for file in files:
        seconds, status, out, err = rocq(work, [*LIBRARY, "-w", "all", file])
        total += seconds
        names = Counter(warnings_of(out + err))
        seen.update(names)
        if status:
            failed.append(file)
        if names:
            details.append((file, out + err))
        shown = ", ".join(f"{name} ({n})" for name, n in names.most_common())
        print(f"| `{file}` | {seconds:.1f} | {shown}{' FAILED' if status else ''} |")
    print(f"\nTotal: {total:.0f} s, one file after another.\n")
    for source, copy, logical in TESTS:
        for name in sorted(os.listdir(os.path.join(work, copy))):
            file = os.path.join(copy, name)
            seconds, status, out, err = rocq(
                work, [*LIBRARY, "-Q", copy, logical, "-w", "all", file]
            )
            names = Counter(warnings_of(out + err))
            seen.update(names)
            shown = ", ".join(f"{w} ({n})" for w, n in names.most_common())
            print(
                f"- `{source}/{name}`: {seconds:.1f} s{', ' + shown if shown else ''}"
                f"{' FAILED' if status else ''}"
            )
    print("\nWarnings by name:\n")
    for name, n in seen.most_common():
        print(f"- `{name}`: {n}")
    if not seen:
        print("- none")
    for file, text in details[:12]:
        print(f"\nThe first warnings of `{file}`:\n\n```\n{text[:1500]}\n```")
    return failed


def mangled(work, files):
    """Compiles each file with generated names mangled; prints the refusals."""
    print("\n## The build with `-mangle-names`\n")
    print(
        "Each file alone, against the library as built above. A file that"
        " fails names a hypothesis or variable Rocq generated.\n"
    )
    refused = 0
    for file in files:
        # The output goes apart, under the name Rocq requires of it.
        scratch = os.path.join(work, "mangled", file[: -len(".v")] + ".vo")
        os.makedirs(os.path.dirname(scratch), exist_ok=True)
        _, status, out, err = rocq(
            work, [*LIBRARY, "-mangle-names", "M_", "-o", scratch, file]
        )
        if status:
            refused += 1
            first = (err or out).strip().splitlines()
            print(f"- `{file}`: {' '.join(first[:6])[:400]}")
    print(f"\n{refused} of {len(files)} files refused.")


def assumptions(library, work, files):
    """Asks `Print Assumptions` of every theorem and lemma; prints the answers."""
    print("\n## The assumptions of every theorem and lemma\n")
    with open(os.path.join(library, "tests", "Assumptions.v"), encoding="utf-8") as f:
        asked = set(re.findall(r"^Print Assumptions (\S+)\.\s*$", f.read(), re.M))
    scratch = os.path.join(work, "assumptions")
    os.makedirs(scratch)
    total = closed = 0
    open_ones, unresolved, per_file = [], [], []
    for file in files:
        parts = file[len("theories/") : -len(".v")].split("/")
        module = ".".join(["Deduxit", *parts])
        names = [f"{module}.{name}" for name in statements(os.path.join(work, file))]
        total += len(names)
        covered = sum(1 for name in names if name in asked)
        per_file.append((file, len(names), covered))
        while names:
            path = os.path.join(scratch, "_".join(parts) + ".v")
            with open(path, "w", encoding="utf-8") as f:
                f.write(
                    f"From {'.'.join(['Deduxit', *parts[:-1]])} Require {parts[-1]}.\n"
                )
                for name in names:
                    f.write(f"Print Assumptions {name}.\n")
            _, status, out, err = rocq(work, [*LIBRARY, path])
            if not status:
                for name, answer in zip(names, answers(out), strict=False):
                    if answer is None:
                        closed += 1
                    else:
                        open_ones.append((name, answer))
                break
            found = ERROR_LINE.search(err)
            line = int(found.group(1)) if found else 0
            if not 2 <= line <= len(names) + 1:
                unresolved += [(name, err.strip()[:200]) for name in names]
                break
            unresolved.append((names.pop(line - 2), "not found under this name"))
    print(
        f"{total} theorems and lemmas in the library; `tests/Assumptions.v` asks"
        f" about {len(asked)}. Asked here of all: {closed} closed under the"
        f" global context, {len(open_ones)} with assumptions,"
        f" {len(unresolved)} not resolved.\n"
    )
    for name, answer in open_ones:
        print(f"- `{name}`: {' '.join(answer)[:300]}")
    for name, why in unresolved:
        print(f"- not resolved: `{name}` ({why})")
    print(
        "\n| file | theorems and lemmas | in `tests/Assumptions.v` |\n|---|---:|---:|"
    )
    for file, count, covered in per_file:
        print(f"| `{file}` | {count} | {covered} |")


def habits(work, files):
    """Counts how the sources require libraries, declare hints and end proofs."""
    patterns = {
        "`From Stdlib Require`": r"^\s*From Stdlib\b.*\bRequire\b",
        "`From Coq Require`": r"^\s*From Coq\b.*\bRequire\b",
        "`Require` without `From`": r"^\s*Require\b",
        "`Require Import` or `Export` (opens a module)": r"\bRequire (Import|Export)\b",
        "hint declared with a locality attribute": r"^\s*#\[(local|export|global)\]\s*Hint\b",
        "hint declared without one": r"^\s*(Local |Global )?Hint\b",
        "`Qed.`": r"\bQed\.",
        "`Defined.`": r"\bDefined\.",
        "`Set Implicit Arguments` and the like": r"^\s*(Set|Unset) ",
        "`Ltac` definitions": r"^\s*(Local )?Ltac\b",
        "`Notation` or `Infix`": r"^\s*(#\[[^\]]*\]\s*)?(Local |Global )?(Notation|Infix)\b",
        "`intuition`, `firstorder` or `tauto`": r"\b(intuition|firstorder|tauto)\b",
        "`lia`": r"\blia\b",
        "`auto` or `eauto`": r"\be?auto\b",
        "lines over 80 characters": r"^.{81,}",
    }
    counts = Counter()
    requires = Counter()
    for file in files:
        with open(os.path.join(work, file), encoding="utf-8") as f:
            for line in f:
                for name, pattern in patterns.items():
                    if re.search(pattern, line):
                        counts[name] += 1
                found = re.match(
                    r"^\s*From (\S+) Require (?:Import |Export )?(.*)\.", line
                )
                if (
                    found
                    and found.group(1) != "Deduxit"
                    and "Deduxit" not in found.group(1)
                ):
                    for module in found.group(2).split():
                        requires[f"{found.group(1)}.{module}"] += 1
    print("\n## Habits of the sources (`theories/`)\n")
    print("| what | lines |\n|---|---:|")
    for name in patterns:
        print(f"| {name} | {counts[name]} |")
    print("\nModules of other libraries required, by the files that do:\n")
    for module, n in requires.most_common():
        print(f"- `{module}`: {n}")


def interface(deduxit, files):
    """Prints the library's modules the exporter's certificates import."""
    imported = Counter()
    pattern = re.compile(
        r"From (Deduxit[A-Za-z.]*) Require (?:Import |Export )?([A-Za-z. ]+?)\."
    )
    for d, dirs, names in os.walk(os.path.join(deduxit, "core", "src")):
        dirs[:] = [x for x in dirs if x != "target"]
        for name in names:
            if not name.endswith(".rs"):
                continue
            with open(os.path.join(d, name), encoding="utf-8") as f:
                for root, modules in pattern.findall(f.read()):
                    for module in modules.split():
                        imported[f"{root}.{module}"] += 1
    modules = {
        ".".join(["Deduxit", *file[len("theories/") : -len(".v")].split("/")])
        for file in files
    }
    print("\n## The modules the exporter's certificates import (`core/src`)\n")
    for module, n in sorted(imported.items()):
        mark = "" if module in modules else " (no such module in the library)"
        print(f"- `{module}`: {n} places{mark}")
    rest = sorted(modules - set(imported))
    print(f"\nThe library's {len(rest)} other modules:\n")
    for module in rest:
        print(f"- `{module}`")


def main():
    library, deduxit, work = sys.argv[1], sys.argv[2], sys.argv[3]
    copy_sources(library, work)
    files = project_files(work)
    version = subprocess.run(
        ["rocq", "--version"], capture_output=True, text=True, check=False
    ).stdout.strip()
    print("# The Rocq library's inventory\n")
    print(f"{version}; {len(files)} files of `_RocqProject`.\n")
    failed = build(work, files)
    if failed:
        print(f"\nThe build failed at {failed}; the rest is not run.")
        return
    mangled(work, files)
    assumptions(library, work, files)
    habits(work, files)
    interface(deduxit, files)


main()
