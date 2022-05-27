from pathlib import Path
from jinja2 import FileSystemLoader, Environment
import os

env = Environment(loader=FileSystemLoader("./misc/templates"))
template = env.get_template("docs.html")

root = Path("./gh-pages")
rust_versions = []
python_versions = []

for item in (root / "rust").iterdir():
    rust_versions.append(item.name)
for item in (root / "python").iterdir():
    python_versions.append(item.name)

result = template.render(
    rust_versions=rust_versions,
    python_versions=python_versions,
)
