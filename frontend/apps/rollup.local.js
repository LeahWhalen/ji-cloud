import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";

let {APP_NAME, APP_PORT} = process.env;

if(!APP_NAME) {
    console.error("INVALID APP_NAME!");
    process.exit(1);
}

const path = require('path');

const watchPatterns = [
    `./crates/entry/**/_common/**`,
    `./crates/utils/**`,
	`./crates/components/**`,
    `./crates/entry/${APP_NAME}/**`,
    "../css/plain/dist/**", 
    "../.template_output/**", 
    //technically this happens _after_ the html rebuild
    //and should therefore be watched instead of .template_output
    //but that slows down responding to html changes
    //and responding to _both_ would refresh twice
    //so just reload after a bit if changing css in order to see it
    //(usually css changes are done at the storybook phase, so this is a rare need)
    //TODO - make it an env thing and params to Makefile.toml
    //"../css/tailwind/dist/**", 
    "../../shared/rust/src/**", 
    "../../config/rust/src/**", 
    "../../config/js/dist/**"
].map(x => path.resolve(x));

export default {
    input: {
        index: `./crates/entry/${APP_NAME}/Cargo.toml`,
    },
    output: {
        dir: `./dist/${APP_NAME}/js/`,
        format: "iife",
        sourcemap: true,
    },
    plugins: [

        rust({
            serverPath: "/js/",
            debug: true,
            watchPatterns,
            cargoArgs: ["--features", "local quiet"],
            watch: true,
        }),

        serve({
            contentBase: `dist/${APP_NAME}`,
            open: true,
            historyApiFallback: true,
            port: APP_PORT, 
        }),

        livereload("dist"),
    ],
};
