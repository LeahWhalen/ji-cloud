import "@elements/core/hebrew-buttons/hebrew-buttons";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Core / Hebrew buttons",
};


interface Args {
    short: boolean,
}

const DEFAULT_ARGS: Args = {
    short: true,
};

export const Ji = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <div style="padding:30px;">
            <hebrew-buttons ${argsToAttrs(props)}></hebrew-buttons>
        </div>
    `;
};

Ji.args = DEFAULT_ARGS;
