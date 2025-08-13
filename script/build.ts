(async () => {
    (await Bun.build({
        entrypoints: [
            "src/win/win-binding.ts"
        ],
        outdir: "target/js/",
        minify: true,
        bytecode: true
    }));
    console.log("Build completed.");
})();