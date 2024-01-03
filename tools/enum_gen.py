from dataclasses import dataclass
import argparse
import collections
import dataclasses
import glob
import itertools
import logging
import os
import re

from jinja2 import Environment, FileSystemLoader


@dataclass
class Metadata:
    namespace: str
    hfn: str
    cfn: str
    prefix: str
    typename: str
    stream_ifdef: str
    proto_prefix: str
    extra_header: str


@dataclass
class Value:
    name: str
    value: str | None = None
    comment: str | None = None


@dataclass
class Enum:
    metadata: Metadata
    values: list[Value]

    @property
    def name(self):
        return self.metadata.typename.removeprefix("xed_").removesuffix("_enum_t")


def camel_case(name: str) -> str:
    return re.sub("[ _]*", "", name.title())


def ident(id: str) -> str:
    id = id.removeprefix("XED_").removesuffix("_ENUM_T").removesuffix("_T")

    if not id[0].isalpha():
        return "_" + id

    return id


KEYWORDS = [
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    # 2018+
    "async",
    "await",
    "dyn",
    # reserved
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",
    # 2018+
    "try",
]


def parse(lines: list[str]) -> Enum:
    names = [field.name for field in dataclasses.fields(Metadata)]
    md = {}
    values = collections.OrderedDict()

    for line in lines:
        line = line.strip()

        if line.startswith("#"):
            continue

        match line.split():
            case [name, value]:
                if name in names:
                    md[name] = value
                else:
                    values[name] = Value(name, value)
            case [name]:
                values[name] = Value(name)
            case [name, value]:
                values[name] = Value(name, value)
            case [name, "///<", *comment]:
                values[name] = Value(name, comment=" ".join(comment))
            case [name, value, "///<", *comment]:
                values[name] = Value(name, value, " ".join(comment))

    return Enum(Metadata(**md), values.values())


def update_mod(file: str, mod: str):
    if os.path.exists(file):
        with open(file, "r", encoding="utf-8") as r:
            if f"mod {mod};" in r.read():
                return

    with open(file, "a" if os.path.exists(file) else "w", encoding="utf-8") as w:
        w.write(f"mod {mod};\npub use self::{mod}::*;\n\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog="enum_gen", description="generate enum binding"
    )

    parser.add_argument(
        "-d",
        "--debug",
        action="store_const",
        const=logging.DEBUG,
        dest="log_level",
        help="show debug message",
    )
    parser.add_argument(
        "-v",
        "--verbose",
        action="store_const",
        const=logging.INFO,
        dest="log_level",
        help="show verbose message",
    )
    parser.add_argument(
        "-o",
        "--output",
        default=None,
        metavar="FILE",
        help="output to file or directory",
    )
    parser.add_argument("filename", nargs="*", help="enum description file")

    args = parser.parse_args()

    logging.basicConfig(level=args.log_level)

    env = Environment(
        loader=FileSystemLoader(os.path.join(os.path.dirname(__file__), "templates"))
    )

    env.filters["upper"] = lambda s: s.upper()
    env.filters["camel_case"] = camel_case
    env.filters["ident"] = ident

    tpl = env.get_template("enum.rs.jinja2")

    for file in itertools.chain.from_iterable(map(glob.glob, args.filename)):
        with open(file, encoding="utf-8") as f:
            enum = parse(f.readlines())

            logging.debug(f"parsed: {enum}")

            code = tpl.render(enum=enum)

            if args.output:
                filename = (
                    os.path.join(args.output, enum.name) + ".rs"
                    if os.path.isdir(args.output)
                    else args.output
                )

                with open(filename, "w", encoding="utf-8") as w:
                    w.write(code)

                if os.path.isdir(args.output):
                    update_mod(os.path.join(args.output, "mod.rs"), enum.name)
            else:
                print(code)
