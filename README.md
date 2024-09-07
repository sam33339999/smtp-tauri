# smtp-tauri

![smtp-tauri](./assets/screenshot.png)

> 透過 tauri 建構的一個小工具，用於測試 SMTP 發信功能；<br/>
> 本專案使用 `Vue3` + `Vite` + `Tauri` + `Rust` 構建。<br/><br/>
>
> - PrimeVue: UI Framework for Vue3 
> - TailwindCSS: A utility-first CSS framework for rapid UI development.


## 開發...

> 由於使用了 `Tauri` 並且覺得指令有點稍微的麻煩；故使用 Makefile 簡化指令。
```bash
# development.
make dev

# build to binary file.
make build

# with develop, use tailwindcss should be upload.
make tailwind
```
- [SMTP 筆記](./SMTP_NOTE.md)
- Learn more about Tauri commands at [Tauri commands](https://tauri.app/v1/guides/features/command)