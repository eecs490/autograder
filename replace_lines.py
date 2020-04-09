#! /usr/bin/env python
import argparse
from pathlib import Path
from typing import List


def main(
    replace_start: str, replace_end: str, from_path: Path, to_path: Path, output: Path,
):
    with from_path.open() as from_file, to_path.open() as to_file, output.open(
        "w"
    ) as outfile:
        replacing = False
        for i, (from_line, to_line) in enumerate(
            zip(from_file.readlines(), to_file.readlines())
        ):
            if to_line.strip() == replace_start:
                replacing = True
            if to_line.strip() == replace_end:
                replacing = False
            if replacing:
                outfile.write(from_line)
            else:
                outfile.write(to_line)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--replace-start")
    parser.add_argument("--replace-end")
    parser.add_argument("--from-path", type=Path)
    parser.add_argument("--to-path", type=Path)
    parser.add_argument("--output", type=Path)
    main(**vars(parser.parse_args()))
