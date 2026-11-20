# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

"""Validates every document of a directory against the wire schema.

Usage: check.py SCHEMA DIRECTORY. A file FORM-CASE.json must match
`$defs/FORM`. Every form that names a top-level document (one whose
`required` holds `version`) needs a document, unless the command cannot
write it (`UNWRITTEN`), and each document with a key added must fail.
Prints one line per failure and exits 1 on any.
"""

import json
import sys
from pathlib import Path

from jsonschema import Draft202012Validator
from jsonschema.exceptions import best_match
from referencing import Registry, Resource

# The top-level forms the command never writes on its own, each with why.
# The library writes them, and the command nests each in a document it
# does write, where the schema checks their keys.
UNWRITTEN = {
    # Nested in every ordinary form; `seq json` writes linear sequents.
    "ordinary_sequent": "the command writes it only nested",
    # The command writes the outcome, which holds the disproof's keys.
    "disproof": "the command writes it as an unprovable outcome",
    # Nested in a not-valid ordinary outcome as `countermodel.model`.
    "kripke_model": "the command writes it only nested",
    # Nested in an ordinary outcome decided by `--method table`.
    "truth_table": "the command writes it only nested",
    # Nested in an ordinary outcome decided by `--method tableau`.
    "tableau": "the command writes it only nested",
    # A value of the library's protocol, which no command runs; the
    # bindings' tests hold every kind's form to the schema by name.
    "ordinary_statement": "only the protocol writes it",
    "modal_statement": "only the protocol writes it",
    # Nested in every modal outcome; no command writes a bare one.
    "modal_sequent": "the command writes it only nested",
    # `check` reads it; `resolve --format json` writes its resolvents
    # inside the answer.
    "clause_refutation": "the command reads it, and writes its parts nested",
}


def main():
    """Validates the documents and returns the exit status."""
    schema_path, directory = Path(sys.argv[1]), Path(sys.argv[2])
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    Draft202012Validator.check_schema(schema)
    base = schema["$id"]
    registry = Registry().with_resource(base, Resource.from_contents(schema))
    forms = schema["$defs"]
    failures = []
    seen = set()
    documents = sorted(directory.glob("*.json"))
    if not documents:
        failures.append(f"{directory}: no document")
    for document in documents:
        form = document.stem.split("-", 1)[0]
        if form not in forms:
            failures.append(f"{document.name}: no form {form} in the schema")
            continue
        seen.add(form)
        try:
            value = json.loads(document.read_text(encoding="utf-8"))
        except json.JSONDecodeError as error:
            failures.append(f"{document.name}: not JSON: {error}")
            continue
        validator = Draft202012Validator(
            {"$ref": f"{base}#/$defs/{form}"}, registry=registry
        )
        for error in sorted(validator.iter_errors(value), key=lambda e: e.json_path):
            deepest = best_match(error.context) if error.context else error
            failures.append(
                f"{document.name}: {deepest.json_path}: {deepest.message[:300]}"
            )
        # A key the writer adds later must fail, so the schema says what
        # each form holds.
        if isinstance(value, dict):
            grown = dict(value, unknown_key=0)
            if validator.is_valid(grown):
                failures.append(f"{document.name}: an unknown key passes")
    for form, definition in forms.items():
        top = "version" in definition.get("required", [])
        if top and form not in seen and form not in UNWRITTEN:
            failures.append(f"{form}: no document of this form")
        if form in UNWRITTEN and form in seen:
            failures.append(f"{form}: has a document, so leave it out of UNWRITTEN")
    for line in failures:
        print(line)
    print(
        f"{len(documents)} documents, {len(seen)} of {len(forms)} forms, "
        f"{len(failures)} failures"
    )
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
