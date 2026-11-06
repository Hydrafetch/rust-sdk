import json
import sys


def convert(node):
    if isinstance(node, list):
        return [convert(n) for n in node]
    if not isinstance(node, dict):
        return node
    out = {k: convert(v) for k, v in node.items() if k not in ("nullable", "example", "examples")}
    if out.get("format") == "uuid":
        out.pop("format")
    if "$ref" in out:
        out["$ref"] = out["$ref"].replace("#/components/schemas/", "#/$defs/")
    if node.get("nullable"):
        if "type" in out and "allOf" not in out and "$ref" not in out:
            out["type"] = [out["type"], "null"]
        else:
            inner = {k: v for k, v in out.items() if k not in ("description", "default")}
            keep = {k: v for k, v in out.items() if k in ("description", "default")}
            if inner.get("type") == "object" and "allOf" in inner:
                inner.pop("type")
            return {**keep, "anyOf": [inner, {"type": "null"}]}
    if out.get("type") == "object" and "allOf" in out and "properties" not in out:
        out.pop("type")
    return out


spec = json.load(sys.stdin)
schemas = convert(spec["components"]["schemas"])
json.dump({"$schema": "http://json-schema.org/draft-07/schema#", "$defs": schemas}, sys.stdout, indent=1)
