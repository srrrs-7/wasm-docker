.PHONY: run serve dev pjt build test fmt

run:
	deno run --allow-read ./src/main.ts
dev:
	deno task dev
pjt:
	deno task wasmbuild new
build:
	deno task wasmbuild
test:
	deno test ./src/main_test.ts
fmt:
	deno fmt