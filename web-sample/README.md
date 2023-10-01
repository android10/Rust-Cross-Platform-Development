## Rust Cross Platform Development: Wev Sample with Vue.js

In order to **get started,** please refer to the the following **article/post** I have written:

- [Rust cross-platform... The Web (Vue.js) part...](https://fernandocejas.com/blog/engineering/2023-10-31-rust-cross-platform-web/).

## The Idea

**A picture worth a thousand words!** Here is a summary of what this project tries to accomplished:

<!-- <p align="center">
  <img src="https://github.com/android10/Rust-Cross-Platform-Development/assets/1360604/04cbde40-1d4e-4f8d-8619-30142e186c25" width="400" alt="rust-cross-platform-project-overview"/>
</p> -->

TODO

And here the **implemantion details:**

<!-- <p align="center">
  <img src="https://github.com/android10/Rust-Cross-Platform-Development/assets/1360604/18d8a3f2-a487-4b2a-9000-1e4e52ab58d3" width="400" alt="rust-cross-platform-project-overview"/>
</p> -->

TODO

## Recommended `nodejs` version

 - If you use [asdf-vm](https://asdf-vm.com/) as **Version Manager**, please check the [.tool-versions](.tool-versions) file.

## Recommended IDE Setup

[VSCode](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur) + [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin).

## Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin) to make the TypeScript language service aware of `.vue` types.

If the standalone TypeScript plugin doesn't feel fast enough to you, Volar has also implemented a [Take Over Mode](https://github.com/johnsoncodehk/volar/discussions/471#discussioncomment-1361669) that is more performant. You can enable it by the following steps:

1. Disable the built-in TypeScript Extension
    1) Run `Extensions: Show Built-in Extensions` from VSCode's command palette
    2) Find `TypeScript and JavaScript Language Features`, right click and select `Disable (Workspace)`
2. Reload the VSCode window by running `Developer: Reload Window` from the command palette.

## Customize configuration

See [Vite Configuration Reference](https://vitejs.dev/config/).

## Local Development

```bash
pnpm install   # Install project dependencies
pnpm dev       # Compile and Hot-Reload for Development 
pnpm build     # Type-Check, Compile and Minify for Production       
```

## Testing and Code quality

It includes, Static Analisys, Lint and Unit, Integration and Functions tests.

### Run Tests with [Vitest](https://vitest.dev/)

```sh
pnpm test
```

### Lint with [ESLint](https://eslint.org/)

```sh
pnpm lint
```

## References

 - https://vitejs.dev/guide/ 
 - https://vitest.dev/guide/ 
 - https://vuejs.org/guide/
 - https://github.com/vuejs/language-tools 
 - https://pinia.vuejs.org/

## License

    Copyright 2023 Fernando Cejas

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.


![https://fernandocejas.com](https://github.com/android10/Sample-Data/blob/master/android10/android10_logo_big.png)

<a href="https://www.buymeacoffee.com/android10" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>
