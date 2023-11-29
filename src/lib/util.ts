import type { Action } from "svelte/action";
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

// const MIN_TIME_MS = 111
// const MIN_DIST_PX = 20

interface SwipeParams {
    thresholdProvider: () => number
}

export const swipe: Action<HTMLElement, SwipeParams> = (node, { thresholdProvider }) => {
    function dispatch<T>(event: string, detail?: T) {
        node.dispatchEvent(new CustomEvent(event, { detail }));
    }

    let x: number;
    let y: number;
    let moved = 0;
    let startTime: number;
    let swiping = false;

    // function isValidSwipe() {
    //     const duration = Date.now() - startTime;
    //     return duration >= MIN_TIME_MS && Math.abs(moved) >= MIN_DIST_PX;
    // }

    function handleDown(e: MouseEvent) {
        e.stopImmediatePropagation();
        e.stopPropagation();
        e.preventDefault()
        startTime = Date.now();
        moved = 0;
        swiping = true;
        x = e.clientX;
        y = e.clientY;
        dispatch("swipeStart", { x, y });
        window.addEventListener("mousemove", handleMove);
        window.addEventListener("mouseup", handleUp);
    }

    function handleMove(e: MouseEvent) {
        if(!swiping) return;
        e.stopImmediatePropagation();
        e.stopPropagation();
        const dx = e.clientX - x;
        const dy = e.clientY - y;
        x = e.clientX;
        y = e.clientY;
        dispatch("swipeMove", { x, y, dx, dy });
        moved += dx;
    }

    function handleUp(_event: MouseEvent) {
        window.removeEventListener("mousemove", handleMove);
        window.removeEventListener("mouseup", handleUp);

        swiping = false;

        const duration = Date.now() - startTime

        let direction: number;
        if(duration > 250) {
            direction = Math.sign(Math.round(moved / thresholdProvider()));
        }
        else {
            direction = Math.sign(moved)
        }

        dispatch("swipeEnd", { direction });
    }

    node.addEventListener("mousedown", handleDown);

    return {
        destroy() {
            node.removeEventListener("mousedown", handleDown);
        },
    }
};
