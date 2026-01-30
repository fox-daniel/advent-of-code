import { readFile } from "fs/promises";

const PATH_TO_INPUT = "../aoc2/input/input.txt";
const input = await readFile(PATH_TO_INPUT, "utf-8");

const lines = input.trim().split("\n");

const part1 = (lines) => {
  let letter_counts = new Map();
  let current_twos;
  let current_threes;
  const [twos, threes] = lines.reduce(
    ([a, b], current) => {
      current_twos = 0;
      current_threes = 0;
      letter_counts.clear();
      [...current].forEach((c) =>
        letter_counts.set(c, (letter_counts.get(c) || 0) + 1),
      );
      // console.log(letter_counts);
      letter_counts.entries().forEach(([_, v]) => {
        if (v === 2) {
          current_twos = 1;
        }
        if (v == 3) {
          current_threes = 1;
        }
      });
      return [a + current_twos, b + current_threes];
    },
    [0, 0],
  );
  return twos * threes;
};

const part2 = (lines) => {
  const common = new Map();
  let len;
  let subid;
  let id;
  for (id of lines) {
    // console.debug(`   id: ${id}`);
    len = id.length;
    for (let idx = 0; idx < len; idx++) {
      subid = id.slice(0, idx) + id.slice(idx + 1, len);
      // console.debug(`subid: ${subid}`);
      if (common.get(subid) && common.get(subid) != id) {
        return subid;
      } else {
        common.set(subid, id);
      }
    }
  }
};

console.log(`part1: ${part1(lines)}`);
console.log(`part2: ${part2(lines)}`);
