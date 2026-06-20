// Minimal ambient types for WinBox 0.2.x (ships no .d.ts) and its CSS.
declare module "winbox" {
  export interface WinBoxParams {
    title?: string;
    width?: number | string;
    height?: number | string;
    x?: number | string;
    y?: number | string;
    minwidth?: number | string;
    minheight?: number | string;
    class?: string | string[];
    background?: string;
    border?: number;
    root?: HTMLElement;
    mount?: HTMLElement;
    onclose?: (force?: boolean) => boolean | void;
    onfocus?: () => void;
    onblur?: () => void;
    [key: string]: unknown;
  }

  export default class WinBox {
    constructor(params?: WinBoxParams);
    constructor(title: string, params?: WinBoxParams);
    id: string;
    body: HTMLElement;
    close(force?: boolean): boolean | void;
    focus(): this;
    blur(): this;
    minimize(state?: boolean): this;
    maximize(state?: boolean): this;
    setTitle(title: string): this;
  }
}

declare module "winbox/src/js/winbox.js" {
  export { default } from "winbox";
}

declare module "winbox/dist/css/winbox.min.css";
