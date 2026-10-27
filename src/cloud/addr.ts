// THE ADDRESSES A BACKEND MAY HAVE, written once: the page (cloud.ts) and the dev server's side (tools/cloud/local.ts) must agree on what a backend on this
// machine is, because a backend on this machine is the one the page sends to with no notice. Nothing else lives here, and nothing is imported.
/** a backend on this machine, as a page reaches it: http, a loopback host, a port, at most one trailing slash */
export const LOOPBACK = /^http:\/\/(127\.0\.0\.1|localhost|\[::1\])(:\d+)?\/?$/;
/** a hosted backend's address */
export const HTTPS_ADDR = /^https:\/\/[\w.-]+(:\d+)?\/?$/;
/** the host name of a page served from this machine */
export const LOCAL_HOST = /^(localhost|127\.0\.0\.1|\[::1\])$/;
/** an address without its trailing slashes */
export const bare = (u: string): string => u.trim().replace(/\/+$/, '');
