interface Coords {
    x: number;
    y: number;
}

interface CoordDiff {
    dx: number;
    dy: number;
}

interface SwipeEvent {
    direction: number;
}

declare namespace svelteHTML {
    interface HTMLAttributes<_> {
        "on:swipeStart"?: (event: CustomEvent<Coords>) => void;
        "on:swipeMove"?: (event: CustomEvent<Coords & CoordDiff>) => void;
        "on:swipeFailed"?: (event: CustomEvent<undefined>) => void;
        "on:swipeEnd"?: (event: CustomEvent<SwipeEvent>) => void;
    }
}
