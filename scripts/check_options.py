import json
import re
import sys

PAIRS = {
    "ScrapeOptions": ("ScrapeRequestDto", {"url", "async"}),
    "MapOptions": ("MapRequestDto", {"url"}),
    "SearchOptions": ("SearchRequestDto", {"query"}),
    "ExtractOptions": ("ExtractRequestDto", {"urls"}),
    "CrawlOptions": ("CrawlRequestDto", {"url"}),
    "BatchOptions": ("BatchRequestDto", {"urls"}),
    "ScreenshotOptions": ("ScreenshotRequestDto", {"url"}),
    "RateResultInput": ("ResultFeedbackDto", set()),
    "SearchJobsOptions": ("JobsIndexSearchRequestDto", {"query"}),
    "HiringChangesOptions": ("JobsIndexChangesRequestDto", {"domains", "since"}),
    "NewsOptions": ("NewsRequestDto", {"query"}),
    "Location": ("LocationDto", set()),
}


def camel(name):
    head, *rest = name.split("_")
    return head + "".join(p.title() for p in rest)


def rust_kind(ty):
    ty = re.sub(r"^Option<(.*)>$", r"\1", ty.strip())
    if ty in ("String", "&str"):
        return "string"
    if ty == "bool":
        return "boolean"
    if re.fullmatch(r"[iu](8|16|32|64|size)|f32|f64", ty):
        return "number"
    if ty.startswith("Vec<"):
        return "array"
    return "object_or_enum"


def api_kind(prop, schemas):
    if "$ref" in prop or "allOf" in prop:
        ref = (prop.get("$ref") or prop["allOf"][0]["$ref"]).split("/")[-1]
        return "string" if schemas.get(ref, {}).get("type") == "string" else "object_or_enum"
    t = prop.get("type")
    if prop.get("enum"):
        return "string" if t == "string" else "object_or_enum"
    return {"integer": "number", "number": "number", "string": "string", "boolean": "boolean",
            "array": "array"}.get(t, "object_or_enum")


spec = json.load(open(sys.argv[1]))
schemas = spec["components"]["schemas"]
src = open("src/types.rs").read()
problems = []
for mine, (theirs, positional) in PAIRS.items():
    m = re.search(rf"pub struct {mine} \{{(.*?)\n\}}", src, re.S)
    if not m:
        problems.append(f"{mine}: not found in src/types.rs")
        continue
    fields = {}
    for fm in re.finditer(r"(?:#\[serde\(rename = \"(\w+)\"\)\]\s*)?pub (\w+): ([^,\n]+),", m.group(1)):
        fields[fm.group(1) or camel(fm.group(2))] = rust_kind(fm.group(3))
    props = {k: api_kind(v, schemas) for k, v in schemas[theirs].get("properties", {}).items() if k not in positional}
    for name, kind in props.items():
        if name not in fields:
            problems.append(f"{mine}: {name} is in the API but not here")
        elif fields[name] != kind and not (kind == "string" and fields[name] == "object_or_enum"):
            problems.append(f"{mine}: {name} is {fields[name]} here but {kind} in the API")
    for name in fields:
        if name not in props:
            problems.append(f"{mine}: {name} is here but the API does not take it")
for p in sorted(problems):
    print(p, file=sys.stderr)
sys.exit(1 if problems else 0)
