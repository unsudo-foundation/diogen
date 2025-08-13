export type OnEvent = () => void;
export type OnCancel = () => void;

export function on_animation_frame(on_animation_frame: OnEvent): OnCancel {
    let cancelled: boolean = false;
    const on_animation_frame_begin: OnEvent = () => {
        on_animation_frame();
        if (cancelled) {
            return;
        }
        requestAnimationFrame(on_animation_frame_begin);
    };
    requestAnimationFrame(on_animation_frame_begin);
    return () => {
        cancelled = true;
    };
}

export function on_timeout(ms: number, on_timeout: OnEvent): OnCancel {
    const key: NodeJS.Timeout = setTimeout(() => {
        on_timeout();
    }, ms);
    return () => clearTimeout(key);
}

export function on_interval(ms: number, on_interval: OnEvent): OnCancel {
    const key: NodeJS.Timeout = setInterval(() => {
        on_interval();
    }, ms);
    return () => clearInterval(key);
}