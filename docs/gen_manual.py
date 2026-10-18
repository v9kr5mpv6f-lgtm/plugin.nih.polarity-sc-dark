#!/usr/bin/env python3
"""Builds docs/MANUAL.html (the manual the HELP button opens) from docs/MANUAL.md.

    python3 docs/gen_manual.py           write docs/MANUAL.html
    python3 docs/gen_manual.py --check   fail if docs/MANUAL.html is not what this would write

The plug-in embeds MANUAL.html in its binary, so the page has to be one self-contained file: no
network, no images to lose. There are no third-party dependencies (standard library only), and the
output is deterministic (no dates, no random ids) so `--check` can compare bytes in CI.

It refuses to write a manual that is broken in ways a reader would hit:
  * the "Version x.y.z" line differs from plugin/Cargo.toml,
  * a heading id the plug-in or the README links to is missing,
  * an in-page link points at an id that does not exist,
  * the search box fails its self-test (needs Node; reported as SKIPPED without it, never faked).
"""

import html
import json
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = HERE / "MANUAL.md"
OUTPUT = HERE / "MANUAL.html"
CARGO_TOML = ROOT / "plugin" / "Cargo.toml"

# Sections the rest of the project links to or relies on.
REQUIRED_IDS = [
    "quick-start",
    "installing",
    "how-slackwater-works",
    "controls",
    "modes-in-depth",
    "troubleshooting-and-questions",
    "technical-notes",
    "glossary",
]

CALLOUTS = {"Tip": "tip", "Note": "note", "Careful": "careful", "Advanced": "advanced"}


class ManualError(Exception):
    pass


# ---------------------------------------------------------------------------------------------
# Inline markup


def inline(text):
    """Turns the inline subset (code, bold, italic, links) of one line of Markdown into HTML."""
    codes = []

    def stash(match):
        codes.append(html.escape(match.group(1), quote=False))
        return "\x00%d\x00" % (len(codes) - 1)

    text = re.sub(r"`([^`]+)`", stash, text)
    text = html.escape(text, quote=False)
    text = re.sub(r"\*\*(.+?)\*\*", r"<strong>\1</strong>", text)
    text = re.sub(r"(?<![*\w])\*(?!\s)(.+?)(?<!\s)\*(?![*\w])", r"<em>\1</em>", text)

    def link(match):
        label, target = match.group(1), match.group(2)
        if target.startswith("#"):
            return '<a href="%s">%s</a>' % (target, label)
        return '<a href="%s" rel="noopener" target="_blank">%s</a>' % (target, label)

    text = re.sub(r"\[([^\]]+)\]\(([^)\s]+)\)", link, text)
    return re.sub(r"\x00(\d+)\x00", lambda m: "<code>%s</code>" % codes[int(m.group(1))], text)


def slugify(text):
    slug = re.sub(r"[^a-z0-9]+", "-", re.sub(r"[`*_]", "", text).lower()).strip("-")
    return slug or "section"


# ---------------------------------------------------------------------------------------------
# Block structure


def parse(markdown):
    """Returns (title, version_line, blocks); every block is a plain tuple the renderer understands."""
    lines = markdown.replace("\r\n", "\n").split("\n")
    blocks = []
    i = 0

    def is_table_sep(line):
        return bool(re.match(r"^\|?\s*:?-{3,}:?\s*(\|\s*:?-{3,}:?\s*)*\|?\s*$", line))

    def cells(line):
        line = line.strip()
        if line.startswith("|"):
            line = line[1:]
        if line.endswith("|"):
            line = line[:-1]
        return [c.strip() for c in line.split("|")]

    while i < len(lines):
        line = lines[i]
        if not line.strip():
            i += 1
        elif line.startswith("```"):
            i += 1
            code = []
            while i < len(lines) and not lines[i].startswith("```"):
                code.append(lines[i])
                i += 1
            i += 1
            blocks.append(("code", "\n".join(code)))
        elif re.match(r"^#{1,4} ", line):
            level = len(line) - len(line.lstrip("#"))
            blocks.append(("heading", level, line[level:].strip()))
            i += 1
        elif re.match(r"^---+\s*$", line):
            blocks.append(("rule",))
            i += 1
        elif line.startswith("|") and i + 1 < len(lines) and is_table_sep(lines[i + 1]):
            header = cells(line)
            i += 2
            rows = []
            while i < len(lines) and lines[i].startswith("|"):
                rows.append(cells(lines[i]))
                i += 1
            blocks.append(("table", header, rows))
        elif line.startswith(">"):
            quote = []
            while i < len(lines) and lines[i].startswith(">"):
                quote.append(lines[i][1:].lstrip(" ") if lines[i][1:2] == " " else lines[i][1:])
                i += 1
            blocks.append(("quote", "\n".join(quote)))
        elif re.match(r"^(- |\* )", line):
            items = []
            while i < len(lines) and re.match(r"^(- |\* )", lines[i]):
                items.append(lines[i][2:].strip())
                i += 1
            blocks.append(("ul", items))
        elif re.match(r"^\d+\. ", line):
            items = []
            while i < len(lines) and re.match(r"^\d+\. ", lines[i]):
                items.append(re.sub(r"^\d+\. ", "", lines[i]).strip())
                i += 1
            blocks.append(("ol", items))
        else:
            para = []
            while (
                i < len(lines)
                and lines[i].strip()
                and not re.match(r"^(#{1,4} |```|> |>$|- |\* |\d+\. |\||---+\s*$)", lines[i])
            ):
                para.append(lines[i].strip())
                i += 1
            blocks.append(("para", " ".join(para)))
    return blocks


def render_blocks(blocks, ids):
    out = []
    for block in blocks:
        kind = block[0]
        if kind == "para":
            out.append("<p>%s</p>" % inline(block[1]))
        elif kind == "ul":
            out.append("<ul>%s</ul>" % "".join("<li>%s</li>" % inline(t) for t in block[1]))
        elif kind == "ol":
            out.append("<ol>%s</ol>" % "".join("<li>%s</li>" % inline(t) for t in block[1]))
        elif kind == "code":
            out.append("<pre><code>%s</code></pre>" % html.escape(block[1], quote=False))
        elif kind == "rule":
            out.append("<hr>")
        elif kind == "table":
            head = "".join("<th>%s</th>" % inline(c) for c in block[1])
            body = "".join(
                "<tr>%s</tr>" % "".join("<td>%s</td>" % inline(c) for c in row) for row in block[2]
            )
            out.append(
                '<div class="table-wrap"><table><thead><tr>%s</tr></thead><tbody>%s</tbody></table></div>'
                % (head, body)
            )
        elif kind == "quote":
            paragraphs = [p.strip() for p in re.split(r"\n\s*\n|\n>\s*\n", block[1]) if p.strip()]
            cls = "callout"
            m = re.match(r"\*\*(\w+):\*\*", paragraphs[0]) if paragraphs else None
            if m and m.group(1) in CALLOUTS:
                cls += " " + CALLOUTS[m.group(1)]
            inner = "".join("<p>%s</p>" % inline(" ".join(p.split("\n"))) for p in paragraphs)
            out.append('<aside class="%s">%s</aside>' % (cls, inner))
        else:
            raise ManualError("unknown block %r" % (kind,))
    return "".join(out)


# ---------------------------------------------------------------------------------------------
# Whole page


def unique_id(base, used):
    candidate, n = base, 2
    while candidate in used:
        candidate = "%s-%d" % (base, n)
        n += 1
    used.add(candidate)
    return candidate


def build(markdown):
    blocks = parse(markdown)
    if not blocks or blocks[0][:2] != ("heading", 1):
        raise ManualError("the manual must start with a '# Title' line")
    title = blocks[0][2]

    version = None
    for block in blocks[1:4]:
        if block[0] == "para":
            m = re.match(r"Version (\d+\.\d+\.\d+)\b", block[1])
            if m:
                version = m.group(1)
                version_line = block[1]
                break
    if version is None:
        raise ManualError("the line after the title must start with 'Version x.y.z'")

    cargo = CARGO_TOML.read_text(encoding="utf-8")
    m = re.search(r'^version\s*=\s*"([^"]+)"', cargo, re.M)
    if not m or m.group(1) != version:
        raise ManualError(
            "manual says Version %s but plugin/Cargo.toml says %s" % (version, m and m.group(1))
        )

    # Split into sections: an intro, then one per '##' heading, each with its '###' parts.
    used = set()
    sections = []  # dicts: id, title, subs[(id, title)], blocks
    intro = []
    current = None
    heading_ids = {}
    for block in blocks[1:]:
        if block[0] == "heading" and block[1] == 2:
            current = {"id": unique_id(slugify(block[2]), used), "title": block[2], "subs": [], "blocks": []}
            sections.append(current)
            heading_ids[block[2]] = current["id"]
        elif block[0] == "heading" and block[1] >= 3 and current is not None:
            sid = unique_id(slugify(block[2]), used)
            if block[1] == 3:
                current["subs"].append((sid, block[2]))
            current["blocks"].append(("heading", block[1], block[2], sid))
        elif current is None:
            if block[0] == "para" and block[1] == version_line:
                continue  # shown under the title instead
            intro.append(block)
        else:
            current["blocks"].append(block)

    all_ids = set(used)
    for required in REQUIRED_IDS:
        if required not in all_ids:
            raise ManualError("required section id '%s' is missing" % required)

    # In-page links must land somewhere.
    for m in re.finditer(r"\]\(#([^)\s]+)\)", markdown):
        if m.group(1) not in all_ids:
            raise ManualError("link to #%s has no matching heading" % m.group(1))

    def render_section_blocks(items):
        out = []
        pending = []
        for item in items:
            if item[0] == "heading":
                if pending:
                    out.append(render_blocks(pending, all_ids))
                    pending = []
                level = item[1]
                out.append('<h%d id="%s">%s</h%d>' % (level, item[3], inline(item[2]), level))
            else:
                pending.append(item)
        if pending:
            out.append(render_blocks(pending, all_ids))
        return "".join(out)

    toc = []
    body = []
    plain = {}  # section id -> its visible text, for the search self-test
    for section in sections:
        toc.append(
            '<li data-for="%s"><a href="#%s">%s</a>%s</li>'
            % (
                section["id"],
                section["id"],
                inline(section["title"]),
                ""
                if not section["subs"]
                else "<ul>%s</ul>"
                % "".join('<li><a href="#%s">%s</a></li>' % (sid, inline(st)) for sid, st in section["subs"]),
            )
        )
        inner = render_section_blocks(section["blocks"])
        body.append(
            '<section data-section="%s"><h2 id="%s">%s</h2>%s</section>'
            % (section["id"], section["id"], inline(section["title"]), inner)
        )
        text = re.sub(r"<[^>]+>", " ", section["title"] + " " + inner)
        plain[section["id"]] = html.unescape(re.sub(r"\s+", " ", text)).strip()

    intro_html = render_blocks(intro, all_ids)
    page = PAGE.format(
        title=html.escape(title, quote=False),
        version=html.escape(version_line, quote=False),
        intro=intro_html,
        toc="".join(toc),
        body="".join(body),
        css=CSS,
        script=SCRIPT,
    )
    return page, plain


# ---------------------------------------------------------------------------------------------
# Search self-test


SEARCH_EXPECTATIONS = [
    # (query, section ids that must match, section ids that must not)
    ("sidechain", ["modes-in-depth", "controls", "troubleshooting-and-questions"], ["installing"]),
    ("xattr", ["installing"], ["glossary", "recipes"]),
    ("hi-freq rolloff", ["controls", "technical-notes"], ["installing"]),
    ("PINK noise", ["modes-in-depth", "glossary"], ["installing"]),
    ("latency", ["controls", "technical-notes", "glossary"], ["installing"]),
    ("sidechain latency", ["glossary"], ["installing"]),
    ("zzzqqq", [], ["controls", "installing", "glossary"]),
    ("", ["controls", "installing", "glossary"], []),
]


def search_selftest(plain):
    """Runs the page's own search function (extracted from the page) against its own text."""
    start = SCRIPT.index("/*SEARCH-START*/")
    end = SCRIPT.index("/*SEARCH-END*/")
    function_source = SCRIPT[start:end]
    payload = {"source": function_source, "sections": plain, "cases": SEARCH_EXPECTATIONS}
    runner = (
        "const p = JSON.parse(require('fs').readFileSync(0, 'utf8'));"
        "const f = new Function(p.source + '; return sectionMatches;')();"
        "let bad = [];"
        "for (const [q, yes, no] of p.cases) {"
        "  for (const id of yes) if (!f(p.sections[id], q)) bad.push('\"' + q + '\" should match ' + id);"
        "  for (const id of no) if (f(p.sections[id], q)) bad.push('\"' + q + '\" should NOT match ' + id);"
        "}"
        "if (bad.length) { console.error(bad.join('\\n')); process.exit(1); }"
    )
    try:
        result = subprocess.run(
            ["node", "-e", runner], input=json.dumps(payload), text=True, capture_output=True
        )
    except FileNotFoundError:
        print("search self-test: SKIPPED (node not found)")
        return
    if result.returncode != 0:
        raise ManualError("search self-test failed:\n" + result.stderr)
    print("search self-test: passed")


# ---------------------------------------------------------------------------------------------
# Template


CSS = """
:root{color-scheme:dark light;--bg:#0a0b14;--panel:#10122a;--ink:#e7eaff;--muted:#9ea6c8;--line:#2b3050;--accent:#9fc4ff;--amber:#f2b35e;--code:#161a38}
@media (prefers-color-scheme:light){:root{--bg:#f7f8fd;--panel:#ffffff;--ink:#14162b;--muted:#4a5170;--line:#d6daee;--accent:#2f5fb3;--amber:#a2650e;--code:#eef1fb}}
*{box-sizing:border-box}
html{scroll-behavior:smooth;scroll-padding-top:5rem}
body{margin:0;background:var(--bg);color:var(--ink);font:17px/1.65 "Sora","Segoe UI",system-ui,-apple-system,sans-serif;font-weight:300}
a{color:var(--accent)}
.layout{display:grid;grid-template-columns:18rem minmax(0,1fr);max-width:78rem;margin:0 auto}
nav{position:sticky;top:0;align-self:start;max-height:100vh;overflow:auto;padding:1.5rem 1rem 2rem 1.5rem;border-right:1px solid var(--line);font-size:.9rem}
nav .brand{font-weight:200;font-size:1.5rem;letter-spacing:.04em;margin:0 0 .25rem}
nav .ver{color:var(--muted);font-size:.8rem;margin:0 0 1rem}
nav input{width:100%;padding:.55rem .8rem;border-radius:1.2rem;border:1px solid var(--line);background:var(--panel);color:var(--ink);font:inherit;font-size:.9rem}
nav ul{list-style:none;margin:.75rem 0 0;padding:0}
nav li{margin:.2rem 0}
nav li ul{margin:.1rem 0 .5rem .9rem;font-size:.82rem}
nav a{text-decoration:none;color:var(--ink)}
nav a:hover{color:var(--accent)}
nav li ul a{color:var(--muted)}
#count{color:var(--muted);font-size:.8rem;margin:.5rem 0 0;min-height:1.1rem}
main{padding:2rem 2rem 6rem;max-width:50rem}
h1{font-weight:200;font-size:2.6rem;letter-spacing:.02em;margin:.2rem 0 .3rem}
h2{font-weight:200;font-size:1.9rem;margin:3rem 0 .6rem;padding-top:1.2rem;border-top:1px solid var(--line)}
h3{font-weight:400;font-size:1.25rem;margin:2rem 0 .4rem;color:var(--accent)}
h4{font-weight:400;font-size:1.05rem;margin:1.5rem 0 .3rem}
.sub{color:var(--muted);margin:0 0 1.5rem}
p{margin:.8rem 0}
ul,ol{padding-left:1.4rem}
li{margin:.3rem 0}
code{font-family:"DM Mono",ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;font-size:.88em;background:var(--code);padding:.1em .4em;border-radius:.3em}
pre{background:var(--code);padding:1rem;border-radius:.6rem;overflow:auto}
pre code{background:none;padding:0}
.table-wrap{overflow-x:auto;margin:1rem 0}
table{border-collapse:collapse;width:100%;font-size:.93rem}
th,td{text-align:left;vertical-align:top;padding:.55rem .75rem;border-bottom:1px solid var(--line)}
th{font-weight:400;color:var(--muted);border-bottom:2px solid var(--line);white-space:nowrap}
.callout{margin:1.2rem 0;padding:.2rem 1rem;border-left:3px solid var(--accent);background:var(--panel);border-radius:0 .6rem .6rem 0}
.callout.tip{border-color:var(--accent)}
.callout.note{border-color:var(--muted)}
.callout.careful{border-color:var(--amber)}
.callout.advanced{border-color:var(--accent);background:var(--code)}
hr{border:0;border-top:1px solid var(--line);margin:2rem 0}
section[hidden],li[hidden]{display:none}
@media (max-width:52rem){.layout{display:block}nav{position:static;max-height:none;border-right:0;border-bottom:1px solid var(--line)}main{padding:1.2rem 1.2rem 4rem}}
@media print{nav{display:none}.layout{display:block}body{background:#fff;color:#000;font-size:11pt}a{color:#000}h2{break-after:avoid}.callout,table{break-inside:avoid}}
"""

SCRIPT = """
/*SEARCH-START*/
function normalise(s) {
  return String(s).toLowerCase().normalize('NFD').replace(/[\\u0300-\\u036f]/g, '');
}
function sectionMatches(text, query) {
  var terms = normalise(query).split(/\\s+/).filter(Boolean);
  if (!terms.length) return true;
  var haystack = normalise(text);
  return terms.every(function (term) { return haystack.indexOf(term) !== -1; });
}
/*SEARCH-END*/
(function () {
  var input = document.getElementById('q');
  var count = document.getElementById('count');
  var sections = Array.prototype.slice.call(document.querySelectorAll('section[data-section]'));
  var tocItems = Array.prototype.slice.call(document.querySelectorAll('nav li[data-for]'));
  function apply() {
    var query = input.value;
    var shown = 0;
    sections.forEach(function (section) {
      var match = sectionMatches(section.textContent, query);
      section.hidden = !match;
      if (match) shown += 1;
    });
    tocItems.forEach(function (item) {
      var section = document.querySelector('section[data-section="' + item.getAttribute('data-for') + '"]');
      item.hidden = section ? section.hidden : false;
    });
    count.textContent = query.trim()
      ? (shown === 0 ? 'No sections match.' : shown + (shown === 1 ? ' section matches.' : ' sections match.'))
      : '';
  }
  input.addEventListener('input', apply);
  input.addEventListener('keydown', function (event) {
    if (event.key === 'Escape') { input.value = ''; apply(); }
  });
})();
"""

PAGE = """<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<style>{css}</style>
</head>
<body>
<div class="layout">
<nav aria-label="Contents">
<p class="brand">slackwater</p>
<p class="ver">Manual</p>
<input id="q" type="search" placeholder="Search the manual" aria-label="Search the manual" autocomplete="off">
<p id="count" aria-live="polite"></p>
<ul>{toc}</ul>
</nav>
<main>
<h1>{title}</h1>
<p class="sub">{version}</p>
{intro}
{body}
</main>
</div>
<script>{script}</script>
</body>
</html>
"""


def main(argv):
    check = "--check" in argv
    try:
        page, plain = build(SOURCE.read_text(encoding="utf-8"))
        search_selftest(plain)
    except ManualError as err:
        print("gen_manual: refused: %s" % err, file=sys.stderr)
        return 1

    data = page.encode("utf-8")
    if check:
        current = OUTPUT.read_bytes() if OUTPUT.exists() else b""
        if current != data:
            print(
                "gen_manual: docs/MANUAL.html is out of date; run `python3 docs/gen_manual.py` and commit it",
                file=sys.stderr,
            )
            return 1
        print("docs/MANUAL.html is up to date")
        return 0

    OUTPUT.write_bytes(data)  # bytes, so the file is LF on every platform
    print("wrote %s (%d bytes)" % (OUTPUT.relative_to(ROOT), len(data)))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
