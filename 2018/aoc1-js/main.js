import { readFile } from "fs/promises";

const PATH_TO_INPUT = "../aoc1/input/input.txt";
const input = await readFile(PATH_TO_INPUT, "utf-8");

const lines = input.trim().split("\n");

const part1 = (lines) => {
  return lines.reduce((accum, num) => accum + Number(num), 0);
};

const part2 = () => {};

console.log(`part1: ${part1(lines)}`);
console.log(`part2: ${part2()}`);
