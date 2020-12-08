#!/usr/bin/env python3

import json
import sys
import os

from jinja2 import Environment, FileSystemLoader

TYPE_MAP = {
    "rdf:langString": "string",
    "xsd:anyURI": "URL",
    "xsd:boolean": "bool",
    "xsd:dateTime": "Date",
    "xsd:string": "string",
    "rfc:bcp47": "Language",
    "rfc:rfc2045": "MimeType",
    "rfc:rfc5988": "RelativeLink",
    "xsd:duration": "Duration",
    "xsd:float": "float",
    "xsd:nonNegativeInteger": "uint",
}

NAMED_UNIONS = {
    ("CollectionPage", "Link"): "PageOrLink",
    ("Image", "Link"): "ImageOrLink",
    ("Link", "Collection"): "CollectionOrLink",
    ("Link", "Object"): "ObjectOrLink",
    ("Object", "Link", "Date", "bool"): "Various",
    ("Object", "string"): "ObjectOrString",
    ("OrderedCollection", "Collection"): "MaybeOrderedCollection",
    ("URL", "Link"): "URLOrLink",
    ("string", "URL"): "StringOrURL",
}

GLOBAL_OPTIONS = {}


def parse_one(typ):
    if isinstance(typ, dict):
        return typ["name"]
    if typ in TYPE_MAP.keys():
        return TYPE_MAP[typ]
    return typ


def parse_domain(d):
    u = d["unionOf"]
    if isinstance(u, dict):
        return [u["name"]]
    if isinstance(u, list):
        return [parse_one(typ) for typ in u]
    return [parse_one(u)]


def sanitize(prop):
    return prop.replace("-", "_")


def pre_generate_step(path):
    dirname, filename = os.path.split(os.path.abspath(path))
    env = Environment(
        loader=FileSystemLoader([".", "templates", dirname]), **GLOBAL_OPTIONS
    )
    prefix, extension = os.path.splitext(filename)
    # env.filters["format_list"] = format_list
    return (prefix, env)


if __name__ == "__main__":
    astream = json.loads(open(sys.argv[1]).read())
    props = {}
    domains = {}
    bases = {}
    activities = []
    actors = []

    # Accept both orders for a key
    GENEROUS_NAMED_UNIONS = {k: v for k, v in NAMED_UNIONS.items()}
    keylist = list(GENEROUS_NAMED_UNIONS.keys())
    for k in keylist:
        if len(k) == 2:
            GENEROUS_NAMED_UNIONS[tuple(reversed(k))] = GENEROUS_NAMED_UNIONS[k]

    for k in astream["sections"]["activityTypes"]["members"]:
        activity = sanitize(k["id"].split("#")[1])
        activities.append(activity)
        bases[activity] = [k["subClassOf"]["name"]]

    for k in astream["sections"]["actorTypes"]["members"]:
        actor = sanitize(k["id"].split("#")[1])
        actors.append(actor)
        bases[actor] = [k["subClassOf"]["name"]]

    for k in astream["sections"]["properties"]["members"]:
        prop = sanitize(k["id"].split("#")[1])
        domain = k["domain"]
        prange = k["range"]
        props[prop] = {"domain": domain, "range": prange}
        domain_list = parse_domain(domain)
        prange = tuple(parse_domain(prange))
        if prange == ("string", "string"):
            prange = ("string",)
        if len(prange) > 1:
            prange = GENEROUS_NAMED_UNIONS[prange]
        else:
            prange = prange[0]

        for domain in domain_list:
            if domain not in domains.keys():
                domains[domain] = {}
            domains[domain][prop] = prange

    prefix, env = pre_generate_step(sys.argv[1])
    tree = {
        "unions": NAMED_UNIONS,
        "tables": domains,
        "actors": actors,
        "activities": activities,
        "bases": bases,
    }
    print(env.get_template("astream_template.fbs").render(tree))
