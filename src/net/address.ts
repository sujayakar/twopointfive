// THE ADDRESS OF A NETWORKED GAME: its words, as a page in one (page.ts) and the lobby that sends a page there (lobby.ts) write and read them.
/** the address's words: the server's WebSocket address, the game's id, the seat's key (read once, then kept for the tab under KEEP) */
export const NETPLAY = 'net', NET_GAME = 'netgame', NET_KEY = 'netkey';
/** …and four for whoever tries it on one machine or on one good network: `netpredict=0` draws his own man where the game has him (foresee.ts FORESEE.on off);
 *  `netlag=<ms>`, `netjitter=<ms>` and `netloss=<percent>` make this page's connection to the server that much worse, each way (NETWAY: delay.ts) */
/** …and one the lobby put there (lobby.ts JOIN: the code of the co-op game this is, so the page in the game can show it and a reload finds the game) */
export const NET_JOIN = 'join';
export const NET_PREDICT = 'netpredict', NET_LAG = 'netlag', NET_JITTER = 'netjitter', NET_LOSS = 'netloss';
