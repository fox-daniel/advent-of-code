import { describe, it, expect } from "vitest";
import { Claim, overlap } from "./index.js";

describe("aoc3 tests", () => {
  it("claim constructor parsers", () => {
    const line = "#1 @ 2,3: 4x5";
    const claim = new Claim(line);
    expect(claim.id).toBe(1);
    expect(claim.x).toBe(2);
    expect(claim.y).toBe(3);
    expect(claim.width).toBe(4);
    expect(claim.height).toBe(5);
  });
  it("claim contains top left corner", () => {
    const line = "#1 @ 2,3: 4x5";
    const claim = new Claim(line);
    expect(claim.contains([2, 3]));
  });
  it("claim contains bottom right corner", () => {
    const line = "#1 @ 2,3: 4x5";
    const claim = new Claim(line);
    expect(claim.contains([5, 7]));
  });
  it("claim does not contain a location", () => {
    const line = "#1 @ 2,3: 4x5";
    const claim = new Claim(line);
    expect(!claim.contains([1, 3]));
  });
  it("claims overlap", () => {
    let line = "#1 @ 2,3: 4x5";
    const claim = new Claim(line);
    line = "#2 @ 4,5: 1x1";
    const other_claim = new Claim(line);
    expect(overlap(claim, other_claim));
  });
  it("claims do not overlap", () => {
    let line = "#1 @ 2,3: 4x5";
    const claim = new Claim(line);
    line = "#2 @ 6,5: 1x1";
    const other_claim = new Claim(line);
    expect(!overlap(claim, other_claim));
  });
});
