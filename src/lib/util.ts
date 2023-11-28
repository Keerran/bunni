import { linear } from "svelte/easing";
import type { TransitionConfig } from "svelte/transition";

export function sleep(ms: number) {
    return new Promise(res => setTimeout(res, ms));
}

export function popIn(
    node: Element,
    { delay = 0, duration = 400, easing = linear }: TransitionConfig = {}
) {
    const o = +getComputedStyle(node).opacity;
    return {
        delay,
        duration,
        easing,
        css: (t: number) => `
            opacity: ${t * o};
            transform: scale(${1.2 - 0.2 * t});
        `,
    };
}
