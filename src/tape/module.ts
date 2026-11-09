// WHICH GAME THIS PAGE PLAYS: the name of the module it was handed (host/face.ts), '' until it is. A kept run says the one it was played on: on another, the same hands may go another way.
export const MODULE = { name: '' };
/** a run played on module `played` is of another game than this page's (a run that says none was kept before they did) */
export const otherModule = (played: string | undefined): boolean => MODULE.name !== '' && played !== MODULE.name;
