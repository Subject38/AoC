import requests
from argparse import ArgumentParser
from os import environ

parser = ArgumentParser()

parser.add_argument("day")
parser.add_argument("-v", "--verbose", help="verbose logging", action="store_true")
args = parser.parse_args()
day: str = args.day
verbose: bool = args.verbose

with open(f"{day}/input.txt", "w") as f:
    problem_input = requests.get(f"https://adventofcode.com/2023/day/{int(day[-2:])}/input", cookies={"session": environ["ADVENT_SESSION"]}).text
    if verbose:
        print(problem_input)
    f.write(problem_input)
