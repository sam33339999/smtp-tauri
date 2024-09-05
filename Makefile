.PHONY: tailwind dev build

tailwind:
	npx tailwindcss -i ./src/global.css -o ./output.css --watch

dev:
	pnpm tauri dev

build:
	pnpm tauri build