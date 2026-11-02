/* eslint-disable */
/**
 * Generated `api` utility.
 *
 * THIS CODE IS AUTOMATICALLY GENERATED.
 *
 * To regenerate, run `npx convex dev`.
 * @module
 */

import type {
  ApiFromModules,
  FilterApi,
  FunctionReference,
} from "convex/server";
import type * as crons from "../crons.js";
import type * as files from "../files.js";
import type * as games from "../games.js";
import type * as http from "../http.js";
import type * as limits from "../limits.js";
import type * as modal from "../modal.js";
import type * as notes from "../notes.js";
import type * as prefs from "../prefs.js";
import type * as runners from "../runners.js";
import type * as tapes from "../tapes.js";
import type * as telemetry from "../telemetry.js";

/**
 * A utility for referencing Convex functions in your app's API.
 *
 * Usage:
 * ```js
 * const myFunctionReference = api.myModule.myFunction;
 * ```
 */
declare const fullApi: ApiFromModules<{
  crons: typeof crons;
  files: typeof files;
  games: typeof games;
  http: typeof http;
  limits: typeof limits;
  modal: typeof modal;
  notes: typeof notes;
  prefs: typeof prefs;
  runners: typeof runners;
  tapes: typeof tapes;
  telemetry: typeof telemetry;
}>;
export declare const api: FilterApi<
  typeof fullApi,
  FunctionReference<any, "public">
>;
export declare const internal: FilterApi<
  typeof fullApi,
  FunctionReference<any, "internal">
>;
