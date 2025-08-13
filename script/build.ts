(async () => {
    (await Bun.build({
        entrypoints: [
            "src/win/bind.ts"
        ],
        outdir: "target/js/",
        minify: true,
        bytecode: true
    }));
    console.log("Build completed.");
})();