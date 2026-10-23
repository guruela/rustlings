"""Update the rustlings progress section in README.md.

Parses .rustlings-state.txt (rustlings 6.x format):
  line 1: comment, line 2: blank,
  line 3: current exercise name, line 4: blank,
  lines 5+: done exercise names.
"""

import re
from datetime import datetime, timezone
from pathlib import Path

README = Path("README.md")
STATE_FILE = Path(".rustlings-state.txt")

START = "<!-- RUSTLINGS-PROGRESS:START -->"
END = "<!-- RUSTLINGS-PROGRESS:END -->"


def state():
    lines = STATE_FILE.read_text().splitlines()
    if len(lines) < 3:
        return [], None
    current = lines[2].strip() or None
    done = [line for line in lines[4:] if line.strip()]
    return done, current


def categorize(done):
    counts = {}
    for name in done:
        exercise_file = next((p for p in Path("exercises").rglob(name + ".rs")), None)
        category = (
            re.sub(r"^\d+_", "", exercise_file.parent.name)
            if exercise_file
            else "other"
        )
        counts[category] = counts.get(category, 0) + 1
    return counts


def progress_section():
    done, current = state()
    total = len(list(Path("exercises").rglob("*.rs")))
    n = len(done)

    bar_len = 20
    filled = round(n / total * bar_len) if total else 0
    bar = "🟩" * filled + "⬜" * (bar_len - filled)
    pct = f"{n / total * 100:.0f}%" if total else "0%"

    lines = [
        "## Progress",
        "",
        f"**{n} / {total} exercises complete ({pct})**",
        "",
        f"`{bar}`",
        "",
    ]

    if n == total:
        lines.append("All exercises completed!")
    elif current:
        lines.append(f"Currently working on: **{current}**")

    if counts := categorize(done):
        lines += ["", "| Category | Done |", "| --- | --- |"]
        for category in sorted(counts):
            lines.append(f"| {category} | {counts[category]} |")

    updated = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
    lines += ["", f"_Last updated: {updated}_", ""]

    return "\n".join(lines)


def main():
    section = f"{START}\n{progress_section()}{END}"
    if README.exists():
        content = README.read_text()
        if START in content and END in content:
            content = re.sub(f"{START}[\s\S]*?{END}", section, content)
        else:
            content = content.rstrip() + "\n\n" + section
    else:
        content = (
            "# Rustlings\n\nMy progress through the [Rustlings](https://github.com/rust-lang/rustlings) Rust exercises.\n\n"
            + section
        )
    README.write_text(content)


if __name__ == "__main__":
    main()
