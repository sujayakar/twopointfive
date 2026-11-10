/** an element, its attributes and its children */
export function el<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  attributes: Partial<Record<string, string | boolean | ((event: Event) => void)>> = {},
  ...children: (Node | string | null)[]
): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  for (const [key, value] of Object.entries(attributes)) {
    if (value === undefined || value === false) continue;
    if (typeof value === 'function') node.addEventListener(key, value);
    else if (key === 'html') node.innerHTML = String(value);
    else node.setAttribute(key, value === true ? '' : value);
  }
  for (const child of children) if (child !== null) node.append(child);
  return node;
}
export const clear = (node: Node): void => node.childNodes.length ? (node as Element).replaceChildren() : undefined;
