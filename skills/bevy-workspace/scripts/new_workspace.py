#!/usr/bin/env python3
"""Copy the Rust workspace templates and replace their placeholders."""
import argparse
import json
from pathlib import Path
import re


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('destination', type=Path)
    parser.add_argument('--name', help='Package name; defaults to directory name')
    parser.add_argument('--repository', required=True)
    parser.add_argument('--description', default='Bevy game workspace.')
    options = parser.parse_args()
    destination = options.destination.absolute()
    name = options.name or destination.name
    if not re.fullmatch(r'[a-z][a-z0-9_-]*', name):
        parser.error('name must start with a lowercase letter and contain a-z, 0-9, _ or -')
    if destination.exists() or destination.is_symlink():
        parser.error('destination already exists; no files were changed')
    for value in (options.repository, options.description):
        if '\n' in value or '\r' in value:
            parser.error('repository and description must be single-line values')
    values = {
        '__NAME_IDENT__': name.replace('-', '_'),
        '__NAME__': name,
        '__DESCRIPTION__': options.description,
        '__NAME_TOML__': json.dumps(name, ensure_ascii=False),
        '__DESCRIPTION_TOML__': json.dumps(options.description, ensure_ascii=False),
        '__REPOSITORY_TOML__': json.dumps(options.repository, ensure_ascii=False),
    }
    pattern = re.compile('|'.join(re.escape(key) for key in values))
    templates = Path(__file__).resolve().parents[1] / 'templates'
    # Prepare every substitution before creating the destination.
    rendered = []
    skipped = {'target', 'Cargo.lock'}
    for source in sorted(templates.rglob('*')):
        if source.is_file() and not skipped & set(source.relative_to(templates).parts):
            relative = str(source.relative_to(templates)).replace('__NAME__', name)
            content = pattern.sub(lambda match: values[match.group()], source.read_text())
            rendered.append((relative, content, source.stat().st_mode & 0o777))
    destination.mkdir(parents=True, exist_ok=False)
    for relative, content, mode in rendered:
        target = destination / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(content)
        target.chmod(mode)
    print(f'Created {destination}. Run just fmt, just build, just lint, just test and just run there; the first just lint-arch installs the pinned lint nightly through rustup.')


if __name__ == '__main__':
    main()
