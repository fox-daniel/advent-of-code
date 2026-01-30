import { readFile } from "fs/promises";

const PATH_TO_INPUT = "../aoc3/input/input.txt";
const input = await readFile(PATH_TO_INPUT, "utf-8");
const lines = input.trim().split("\n");

function part1(lines) {
  // C = num claims; S = num points in a claim; G = num points in grid;
  // maintain a claimMap {(x,y): num_claims}
  //
  // [NOT IMPLEMENTED]
  // O(S*C*G)
  // iterate through all claims
  //   update claimMap
  // iterate through claimMap values and count positive values
  //
  // [IMPLEMENTED]
  // O(G*C)
  // iterate through all locations
  //   iterate through claims
  //   check if it is in a claim and update claim map
  //   filter out if not in overlap
  // count locations not filtered out
  let [xmax, ymax] = [0, 0];
  let claims = [];
  lines.forEach((str) => {
    const claim = new Claim(str);
    xmax = Math.max(claim.xmax, xmax);
    ymax = Math.max(claim.ymax, ymax);
    claims.push(claim);
  });
  let claim;
  let counts = new Map();
  let key;
  let overlap = 0;
  for (let x = 0; x <= xmax; x++) {
    for (let y = 0; y <= ymax; y++) {
      for (claim of claims) {
        if (claim.contains([x, y])) {
          key = `${x},${y}`;
          counts.set(key, counts.get(key) + 1 || 1);
          if (counts.get(key) > 1) {
            overlap += 1;
            break;
          }
        }
      }
    }
  }
  return overlap;
}

function part2(lines) {
  const claims = new Map();
  const claim_status = new Map();
  let claim;
  lines.forEach((str) => {
    claim = new Claim(str);
    claims.set(claim.id, claim);
    claim_status.set(claim.id, false);
  });
  let other_claim;
  const claim_ids = [...claim_status.keys()];
  const num_claims = claim_ids.length;
  for (let i = 0; i < num_claims; i++) {
    for (let j = i + 1; j < num_claims; j++) {
      claim = claims.get(claim_ids[i]);
      other_claim = claims.get(claim_ids[j]);
      if (overlap(claim, other_claim)) {
        claim_status.set(claim_ids[i], true);
        claim_status.set(claim_ids[j], true);
      }
    }
  }
  const entries = [...claim_status.entries()];
  const filtered = entries.filter(([k, v]) => !v);
  const [k, v] = filtered[0];

  return k;
}

export function overlap(claim, other_claim) {
  return (
    claim.xmin <= other_claim.xmax &&
    claim.xmax >= other_claim.xmin &&
    claim.ymin <= other_claim.ymax &&
    claim.ymax >= other_claim.ymin
  );
}

export class Claim {
  x;
  y;
  width;
  height;
  id;

  constructor(line) {
    // Example line: #123 @ 1,3: 5x6
    let id, x, y, width, height;
    let rest, corner, dimensions;
    [id, rest] = line.split("@");
    this.id = Number(id.trim().slice(1));
    [corner, rest] = rest.trim().split(":");
    [x, y] = corner.split(",");
    this.x = Number(x.trim());
    this.y = Number(y.trim());
    [width, height] = rest.trim().split("x");
    this.width = Number(width.trim());
    this.height = Number(height.trim());
    this.xmin = this.x;
    this.xmax = this.x + this.width - 1;
    this.ymin = this.y;
    this.ymax = this.y + this.height - 1;
  }

  contains([a, b]) {
    return a >= this.xmin && a <= this.xmax && b >= this.ymin && b <= this.ymax;
  }
}

console.log(`part 1: ${part1(lines)}`);
console.log(`part 2: ${part2(lines)}`);
