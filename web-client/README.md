# web-client
This project was created using `bun init` in bun v1.0.2. [Bun](https://bun.sh) is a fast all-in-one JavaScript runtime.

## Installation
Since this package contains typescript and css, an install and build step is required.

The current approach is to leverage build scripts (see build.rs in this package), to build js and css for the application.
Additionally, we need to ensure that node/npm/bun packages have been installed. This is accomplished by hasing the package.json during cargo build. If the hash has changed it will bun install dependencies.

An alternative approach would be to leverage git hooks (post-merge) to bun install if package.json has changed.
Since git hooks are not checked in, it would require either a manual step or hooking into cargo build to auto setup git hooks.

If you need to manually install (you shouldn't need to though)...
```bash
bun install
```

You can manually build the js and css (and watch) w/ the following commands
```bash
bun build ./src/client/common.js --outdir ./out --watch
bunx tailwindcss -i ./src/client/common.css -o ./out/common.css --watch
```


