// WHAT RUNS BY THE CLOCK. The reaper is a mutation that reads the live games and nothing else, so a deployment with no game pays two small reads a turn;
// it calls a runner only where a game ended or a sweep did not finish (games.ts `reap`, runners.ts `sweep`).
import { cronJobs } from "convex/server";
import { internal } from "./_generated/api";

const crons = cronJobs();
crons.interval("reap game servers", { seconds: 30 }, internal.games.reap, {});
export default crons;
