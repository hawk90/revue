# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.31.5](https://github.com/hawk90/revue/compare/v2.31.4...v2.31.5) (2026-01-16)


### Bug Fixes

* **reactive:** add Subscription handle to prevent subscriber memory leak ([#189](https://github.com/hawk90/revue/issues/189)) ([c50fdee](https://github.com/hawk90/revue/commit/c50fdee94c7c7073ccacba327114d3f8dad45b3d)), closes [#141](https://github.com/hawk90/revue/issues/141)

## [2.31.4](https://github.com/hawk90/revue/compare/v2.31.3...v2.31.4) (2026-01-16)


### Bug Fixes

* **reactive:** add recursion guard to prevent stack overflow in effect callbacks ([#187](https://github.com/hawk90/revue/issues/187)) ([5d814ee](https://github.com/hawk90/revue/commit/5d814ee0407cf4ccfbef5ffa7821b40deafc5ebb)), closes [#142](https://github.com/hawk90/revue/issues/142)

## [2.31.3](https://github.com/hawk90/revue/compare/v2.31.2...v2.31.3) (2026-01-16)


### Bug Fixes

* **event:** handle FocusGained, FocusLost, and Paste events ([#186](https://github.com/hawk90/revue/issues/186)) ([3d24d7c](https://github.com/hawk90/revue/commit/3d24d7c96cd9042ffdff135a27f2279942c7ba34)), closes [#143](https://github.com/hawk90/revue/issues/143)

## [2.31.2](https://github.com/hawk90/revue/compare/v2.31.1...v2.31.2) (2026-01-16)


### Bug Fixes

* **reactive:** prevent data race in Computed concurrent recomputation ([#173](https://github.com/hawk90/revue/issues/173)) ([ab172d0](https://github.com/hawk90/revue/commit/ab172d0752da7e2af9e4548f5c461302e6c746a0)), closes [#140](https://github.com/hawk90/revue/issues/140)

## [2.31.1](https://github.com/hawk90/revue/compare/v2.31.0...v2.31.1) (2026-01-16)


### Bug Fixes

* **core:** improve error handling and centralize border rendering ([#136](https://github.com/hawk90/revue/issues/136)) ([f5b1262](https://github.com/hawk90/revue/commit/f5b126293389badbcbf9a1c838d923ee40cde7e9)), closes [#133](https://github.com/hawk90/revue/issues/133) [#135](https://github.com/hawk90/revue/issues/135)

## [2.31.0](https://github.com/hawk90/revue/compare/v2.30.0...v2.31.0) (2026-01-16)


### Features

* **a11y:** add screen reader backend integration ([#130](https://github.com/hawk90/revue/issues/130)) ([a954da0](https://github.com/hawk90/revue/commit/a954da0770377fe5ae7c76267b2571400ee75b15)), closes [#26](https://github.com/hawk90/revue/issues/26)

## [2.30.0](https://github.com/hawk90/revue/compare/v2.29.0...v2.30.0) (2026-01-16)


### Features

* add IME, RTL/BiDi, and Sixel/iTerm2 image protocol support ([#128](https://github.com/hawk90/revue/issues/128)) ([92619d3](https://github.com/hawk90/revue/commit/92619d342f26f39ddbb13de817620ceb3951f988)), closes [#7](https://github.com/hawk90/revue/issues/7) [#8](https://github.com/hawk90/revue/issues/8) [#9](https://github.com/hawk90/revue/issues/9)

## [2.29.0](https://github.com/hawk90/revue/compare/v2.28.0...v2.29.0) (2026-01-16)


### Features

* add Time-Travel Debugging, Custom Events, and Gesture Support ([#126](https://github.com/hawk90/revue/issues/126)) ([14740cd](https://github.com/hawk90/revue/commit/14740cd46ae34f7704baba4aa01529d428fa5033))

## [2.28.0](https://github.com/hawk90/revue/compare/v2.27.0...v2.28.0) (2026-01-16)


### Features

* add Context API and Performance Profiler ([#124](https://github.com/hawk90/revue/issues/124)) ([4835fd6](https://github.com/hawk90/revue/commit/4835fd6bc6594c6dd005f3e575e480f5f5973b13))

## [2.27.0](https://github.com/hawk90/revue/compare/v2.26.0...v2.27.0) (2026-01-15)


### Features

* implement HTTP client enhancements, mocking utilities, batched updates, and component picker ([#122](https://github.com/hawk90/revue/issues/122)) ([418cbd5](https://github.com/hawk90/revue/commit/418cbd54ac981bd666320098bed05e3307e6c0a0))

## [2.26.0](https://github.com/hawk90/revue/compare/v2.25.0...v2.26.0) (2026-01-15)


### Features

* **widget:** add CodeEditor and RichTextEditor widgets ([#119](https://github.com/hawk90/revue/issues/119)) ([358d48a](https://github.com/hawk90/revue/commit/358d48a0291fca7080d1b26f907200db0c813780)), closes [#10](https://github.com/hawk90/revue/issues/10) [#11](https://github.com/hawk90/revue/issues/11)
* **widget:** enhance Canvas with arcs, polygons, transforms, and layers ([#120](https://github.com/hawk90/revue/issues/120)) ([c7b69a7](https://github.com/hawk90/revue/commit/c7b69a72c35dfb3cd9d0cde235e2895d681c7a16)), closes [#6](https://github.com/hawk90/revue/issues/6)

## [2.25.0](https://github.com/hawk90/revue/compare/v2.24.0...v2.25.0) (2026-01-15)


### Features

* **widget:** add advanced LogViewer widget ([#117](https://github.com/hawk90/revue/issues/117)) ([dc7c152](https://github.com/hawk90/revue/commit/dc7c152291a687032372248a2986123b88f61345))

## [2.24.0](https://github.com/hawk90/revue/compare/v2.23.0...v2.24.0) (2026-01-15)


### Features

* **widget:** add Sidebar Navigation widget ([#115](https://github.com/hawk90/revue/issues/115)) ([0a68994](https://github.com/hawk90/revue/commit/0a68994f6959db5d6346c7f1c89fcbff5cbddfd6)), closes [#35](https://github.com/hawk90/revue/issues/35)

## [2.23.0](https://github.com/hawk90/revue/compare/v2.22.0...v2.23.0) (2026-01-15)


### Features

* **widget:** add CSV and JSON viewer widgets ([#113](https://github.com/hawk90/revue/issues/113)) ([7d8d9e3](https://github.com/hawk90/revue/commit/7d8d9e3c4ec6193e0ed9545953ddd2483a997780)), closes [#33](https://github.com/hawk90/revue/issues/33) [#34](https://github.com/hawk90/revue/issues/34)

## [2.22.0](https://github.com/hawk90/revue/compare/v2.21.0...v2.22.0) (2026-01-15)


### Features

* **widget:** add Combobox widget with multi-select and filtering ([#111](https://github.com/hawk90/revue/issues/111)) ([9a61343](https://github.com/hawk90/revue/commit/9a6134353cb426dee8b30c09ef3541c3ad75af52)), closes [#20](https://github.com/hawk90/revue/issues/20)

## [2.21.0](https://github.com/hawk90/revue/compare/v2.20.0...v2.21.0) (2026-01-15)


### Features

* **widget:** add ToastQueue for centralized toast management ([#107](https://github.com/hawk90/revue/issues/107)) ([61baed0](https://github.com/hawk90/revue/commit/61baed02ed55cbf7ea50361ca97f78b918364058))

## [2.20.0](https://github.com/hawk90/revue/compare/v2.19.0...v2.20.0) (2026-01-15)


### Features

* **widget:** add Popover widget for anchor-positioned overlays ([#106](https://github.com/hawk90/revue/issues/106)) ([6228aab](https://github.com/hawk90/revue/commit/6228aab28cb5b780f994233bc7e45b175a8b7c54))


### Bug Fixes

* **test:** eliminate race condition in worker pool priority test ([#108](https://github.com/hawk90/revue/issues/108)) ([ba7e519](https://github.com/hawk90/revue/commit/ba7e519875a5da092283aa72ab594bf752cb28ec))

## [2.19.0](https://github.com/hawk90/revue/compare/v2.18.0...v2.19.0) (2026-01-15)


### Features

* **core:** add debounce and throttle utilities ([#104](https://github.com/hawk90/revue/issues/104)) ([bfc561d](https://github.com/hawk90/revue/commit/bfc561dd2dbcb83fa58f221cbc01941494a3eca7))

## [2.18.0](https://github.com/hawk90/revue/compare/v2.17.0...v2.18.0) (2026-01-15)


### Features

* **widget:** add EmptyState widget for no-data scenarios ([#102](https://github.com/hawk90/revue/issues/102)) ([91351ab](https://github.com/hawk90/revue/commit/91351abd1ed51ff98438c30ee9a3b95d52f0224a))

## [2.17.0](https://github.com/hawk90/revue/compare/v2.16.0...v2.17.0) (2026-01-09)


### Features

* **widget:** add chart widgets (PieChart, ScatterChart, Histogram, BoxPlot) ([#99](https://github.com/hawk90/revue/issues/99)) ([9ab48d4](https://github.com/hawk90/revue/commit/9ab48d4ed649d7d161f1edb98487e9c8f1cb603c)), closes [#12](https://github.com/hawk90/revue/issues/12) [#13](https://github.com/hawk90/revue/issues/13) [#14](https://github.com/hawk90/revue/issues/14) [#15](https://github.com/hawk90/revue/issues/15) [#36](https://github.com/hawk90/revue/issues/36)

## [2.16.0](https://github.com/hawk90/revue/compare/v2.15.0...v2.16.0) (2026-01-09)


### Features

* **widget:** add TextArea Find/Replace and Multiple Cursors support ([8691ccb](https://github.com/hawk90/revue/commit/8691ccbc5097372bd97e9b7bead7c3045309a268))

## [2.15.0](https://github.com/hawk90/revue/compare/v2.14.0...v2.15.0) (2026-01-08)


### Features

* **widget:** add Footnotes and Admonition support for Markdown ([#96](https://github.com/hawk90/revue/issues/96)) ([c6b83c0](https://github.com/hawk90/revue/commit/c6b83c0ec5a6d4c97fde6fd0364e864e4dd8cf95)), closes [#47](https://github.com/hawk90/revue/issues/47) [#49](https://github.com/hawk90/revue/issues/49)

## [2.14.0](https://github.com/hawk90/revue/compare/v2.13.1...v2.14.0) (2026-01-08)


### Features

* **widget:** add DataGrid tree grid, export, and footer features ([#94](https://github.com/hawk90/revue/issues/94)) ([e282dcc](https://github.com/hawk90/revue/commit/e282dccc20254afbcddfa9ad5cbe7f40f133738d)), closes [#16](https://github.com/hawk90/revue/issues/16) [#17](https://github.com/hawk90/revue/issues/17) [#18](https://github.com/hawk90/revue/issues/18)

## [2.13.1](https://github.com/hawk90/revue/compare/v2.13.0...v2.13.1) (2026-01-08)


### Bug Fixes

* **ci:** run only coverage on main branch push ([#92](https://github.com/hawk90/revue/issues/92)) ([91c3a85](https://github.com/hawk90/revue/commit/91c3a859d4a096c57ca9ee01bd381cbee6f46071))

## [2.13.0](https://github.com/hawk90/revue/compare/v2.12.0...v2.13.0) (2026-01-08)


### Features

* **widget:** add NumberInput, MultiSelect, and RangePicker widgets ([#90](https://github.com/hawk90/revue/issues/90)) ([c19729d](https://github.com/hawk90/revue/commit/c19729dc05f4adf44ba98c46b56a891d78f827af))

## [2.12.0](https://github.com/hawk90/revue/compare/v2.11.0...v2.12.0) (2026-01-08)


### Features

* **widget:** add DataGrid column resize, reorder, and freeze ([#87](https://github.com/hawk90/revue/issues/87)) ([b4f673a](https://github.com/hawk90/revue/commit/b4f673ad638db245e50414d636470fef1119fce6))

## [2.11.0](https://github.com/hawk90/revue/compare/v2.10.0...v2.11.0) (2026-01-07)


### Features

* **widget:** add DateTimePicker widget ([#83](https://github.com/hawk90/revue/issues/83)) ([47ab075](https://github.com/hawk90/revue/commit/47ab075ed0a9dac2412c1241807e19f24c920aa3)), closes [#46](https://github.com/hawk90/revue/issues/46)

## [2.10.0](https://github.com/hawk90/revue/compare/v2.9.1...v2.10.0) (2026-01-07)


### Features

* **widget:** add Card widget for structured content layout ([#79](https://github.com/hawk90/revue/issues/79)) ([3109241](https://github.com/hawk90/revue/commit/31092419cd00463ba5aa8997e660bf283047a632)), closes [#40](https://github.com/hawk90/revue/issues/40)

## [2.9.1](https://github.com/hawk90/revue/compare/v2.9.0...v2.9.1) (2026-01-07)


### Bug Fixes

* **widget:** handle wide characters correctly in Callout and StatusIndicator ([#77](https://github.com/hawk90/revue/issues/77)) ([9c7771c](https://github.com/hawk90/revue/commit/9c7771c18f37542a6739ba0d6ddab2d6f6e47c36))

## [2.9.0](https://github.com/hawk90/revue/compare/v2.8.0...v2.9.0) (2026-01-07)


### Features

* **widget:** add StatusIndicator widget for availability states ([#75](https://github.com/hawk90/revue/issues/75)) ([5f037cf](https://github.com/hawk90/revue/commit/5f037cfb82432c4495e65455b0f81eb0cdd4ca6d))

## [2.8.0](https://github.com/hawk90/revue/compare/v2.7.0...v2.8.0) (2026-01-07)


### Features

* **widget:** add Callout widget for info highlight blocks ([#73](https://github.com/hawk90/revue/issues/73)) ([bc02240](https://github.com/hawk90/revue/commit/bc02240ac2025f8731a9d1b1ab72e5989a280841))

## [2.7.0](https://github.com/hawk90/revue/compare/v2.6.0...v2.7.0) (2026-01-07)


### Features

* **widget:** add Alert widget ([#71](https://github.com/hawk90/revue/issues/71)) ([6eae377](https://github.com/hawk90/revue/commit/6eae3773747fa767d626d914ba83fd76caf854c9)), closes [#41](https://github.com/hawk90/revue/issues/41)

## [2.6.0](https://github.com/hawk90/revue/compare/v2.5.1...v2.6.0) (2026-01-07)


### Features

* **widget:** add Collapsible widget ([#69](https://github.com/hawk90/revue/issues/69)) ([4bb7b73](https://github.com/hawk90/revue/commit/4bb7b7376bd09c76c68ce68f3111208addd22d7f)), closes [#48](https://github.com/hawk90/revue/issues/48)

## [2.5.1](https://github.com/hawk90/revue/compare/v2.5.0...v2.5.1) (2026-01-07)


### Bug Fixes

* **widget:** handle wide characters correctly in draw_text functions ([#67](https://github.com/hawk90/revue/issues/67)) ([85dceb7](https://github.com/hawk90/revue/commit/85dceb72d2072cd29b692b9f7ae532c718d51e65))

## [2.5.0](https://github.com/hawk90/revue/compare/v2.4.0...v2.5.0) (2026-01-06)


### Features

* **widget:** add Justify text alignment ([#65](https://github.com/hawk90/revue/issues/65)) ([bf1c27d](https://github.com/hawk90/revue/commit/bf1c27dd049e726f669f853ba74fba73fc4bf6ba)), closes [#50](https://github.com/hawk90/revue/issues/50)

## [2.4.0](https://github.com/hawk90/revue/compare/v2.3.0...v2.4.0) (2026-01-05)


### Features

* **widget:** add OSC 66 text sizing support for BigText ([#117](https://github.com/hawk90/revue/issues/117)) ([ce22714](https://github.com/hawk90/revue/commit/ce22714cb489e8b6e7051dc85f3b5c14e37e25cf)), closes [#57](https://github.com/hawk90/revue/issues/57)

## [2.3.0](https://github.com/hawk90/revue/compare/v2.2.0...v2.3.0) (2026-01-05)


### Features

* **widget:** add virtual scroll support to DataGrid ([#115](https://github.com/hawk90/revue/issues/115)) ([cdf6022](https://github.com/hawk90/revue/commit/cdf6022a621a92fa070e204f2c60812198fd1152))

## [2.2.0](https://github.com/hawk90/revue/compare/v2.1.0...v2.2.0) (2026-01-05)


### Features

* **examples:** add form_validation example ([#56](https://github.com/hawk90/revue/issues/56)) ([#105](https://github.com/hawk90/revue/issues/105)) ([d4f25c9](https://github.com/hawk90/revue/commit/d4f25c933f4f1891b5b578c1c1e5d9694ff402fc))

## [2.1.0](https://github.com/hawk90/revue/compare/v2.0.4...v2.1.0) (2026-01-05)


### Features

* **form:** refactor Form API with Signal-based reactivity ([#103](https://github.com/hawk90/revue/issues/103)) ([bf955cf](https://github.com/hawk90/revue/commit/bf955cfb7b275094d9bfcec6316563b500293669))


### Bug Fixes

* **docs:** align theme signal documentation with implementation ([#95](https://github.com/hawk90/revue/issues/95)) ([a4ff194](https://github.com/hawk90/revue/commit/a4ff19417473f76eadb6ebfd148214f71961dc59)), closes [#64](https://github.com/hawk90/revue/issues/64)
* **docs:** unify widget count to 80+ across all documentation ([#94](https://github.com/hawk90/revue/issues/94)) ([6dea979](https://github.com/hawk90/revue/commit/6dea97978d7342332b82487c41f7efc331ae21e4)), closes [#60](https://github.com/hawk90/revue/issues/60)

## [2.0.4](https://github.com/hawk90/revue/compare/v2.0.3...v2.0.4) (2026-01-05)


### Bug Fixes

* **ci:** use stable rust toolchain instead of non-existent 1.100 ([7fb6f37](https://github.com/hawk90/revue/commit/7fb6f37fbbb9d57a7a8e1115f28b74412a3d42a8))
* consistent lock poisoning recovery across codebase ([ba8003e](https://github.com/hawk90/revue/commit/ba8003e8a2f34a693239330259dd7fd6d99fbbb3))
* consistent lock poisoning recovery across codebase ([1809de5](https://github.com/hawk90/revue/commit/1809de56ec67a3cbb8b36fd26ae78d077e51cfef)), closes [#63](https://github.com/hawk90/revue/issues/63)

## [2.0.3](https://github.com/hawk90/revue/compare/v2.0.2...v2.0.3) (2026-01-05)


### Bug Fixes

* **examples:** handle unicode properly in text_editor ([7a83d19](https://github.com/hawk90/revue/commit/7a83d197e310a44aaaf2c28d53719b69800c57f4)), closes [#66](https://github.com/hawk90/revue/issues/66)

## [2.0.2](https://github.com/hawk90/revue/compare/v2.0.1...v2.0.2) (2026-01-04)


### Bug Fixes

* connect AppBuilder hot_reload/devtools flags to actual functionality ([#76](https://github.com/hawk90/revue/issues/76)) ([87490ea](https://github.com/hawk90/revue/commit/87490ead1759fb6b52f9c545698bd5425def1d06))
* update documentation to match actual API ([#75](https://github.com/hawk90/revue/issues/75)) ([e9568c4](https://github.com/hawk90/revue/commit/e9568c43a25baaf6b4fd6f92643e89748353fec0)), closes [#59](https://github.com/hawk90/revue/issues/59)

## [2.0.1](https://github.com/hawk90/revue/compare/v2.0.0...v2.0.1) (2026-01-03)


### Bug Fixes

* **ci:** improve release workflow and trusted publishing setup


## [2.0.0](https://github.com/hawk90/revue/compare/v1.0.8...v2.0.0) (2026-01-03)


### ⚠ BREAKING CHANGES

* v2.0.0 - Optional deps, widgets, and maintenance CI ([#26](https://github.com/hawk90/revue/issues/26))

### Features

* v2.0.0 - Optional deps, widgets, and maintenance CI ([#26](https://github.com/hawk90/revue/issues/26)) ([4f39f03](https://github.com/hawk90/revue/commit/4f39f036ae0d8da7b9401b9b7298908a847f275b))


### Bug Fixes

* **ci:** add commitlint config and update typos ignore list ([#30](https://github.com/hawk90/revue/issues/30)) ([c6c6457](https://github.com/hawk90/revue/commit/c6c6457112a02ad80f92b7151a50bb3a095f7d73))
* clippy warnings and format issues ([#28](https://github.com/hawk90/revue/issues/28)) ([f555351](https://github.com/hawk90/revue/commit/f555351a7d0265dce24313429540d32100b8fa26))

## [1.0.8](https://github.com/hawk90/revue/compare/v1.0.7...v1.0.8) (2026-01-03)


### Bug Fixes

* **docs:** correct GitHub URL and remove broken links ([9cb5684](https://github.com/hawk90/revue/commit/9cb56846f44d487c75b818fddee9a26448972e15))
* include examples and benches in published package ([8465bd0](https://github.com/hawk90/revue/commit/8465bd083ecd86e6214dbc1adc0a16257dfa4b85))
* prefix unused variable with underscore ([c0d956c](https://github.com/hawk90/revue/commit/c0d956caef0a1b6ac64731ddc7a03a4dbb11dba5))
* resolve clippy warnings in examples ([12da1f6](https://github.com/hawk90/revue/commit/12da1f645bc105a4c558ae9fd5d212b33585d38f))

## [1.0.7](https://github.com/hawk90/revue/compare/v1.0.6...v1.0.7) (2026-01-03)


### Bug Fixes

* use API token for crates.io publish ([b939f43](https://github.com/hawk90/revue/commit/b939f432eb8eb7b65448630ff812d7be1c9553b1))

## [1.0.6](https://github.com/hawk90/revue/compare/v1.0.5...v1.0.6) (2026-01-03)


### Bug Fixes

* remove environment from publish job for trusted publishing ([2f24c9d](https://github.com/hawk90/revue/commit/2f24c9dc753e50be1b000e2cd90c8ce3241fddc0))

## [1.0.5](https://github.com/hawk90/revue/compare/v1.0.4...v1.0.5) (2026-01-03)


### Bug Fixes

* add codecov token to coverage job ([47b0334](https://github.com/hawk90/revue/commit/47b033498bbb8199db2c16a1a8eff7c06f27e8ca))
* configure trusted publishing for crates.io ([b016f41](https://github.com/hawk90/revue/commit/b016f411a7b4f2ea1de3f77039fb21b7d8ee3956))

## [1.0.4](https://github.com/hawk90/revue/compare/v1.0.3...v1.0.4) (2026-01-03)


### Bug Fixes

* remove binary build for library crate ([0980f08](https://github.com/hawk90/revue/commit/0980f0888cfd88a3bd0f682766180f5d25497cd0))

## [1.0.3](https://github.com/hawk90/revue/compare/v1.0.2...v1.0.3) (2026-01-03)


### Bug Fixes

* mark flaky accessibility test as ignored ([38d001c](https://github.com/hawk90/revue/commit/38d001c1562cc2a96fa7d230d180664ff2246ad0))

## [1.0.2](https://github.com/hawk90/revue/compare/v1.0.1...v1.0.2) (2026-01-03)


### Bug Fixes

* correct Quick Start anchor link in README ([8d98b4b](https://github.com/hawk90/revue/commit/8d98b4b0dca6339ba80d918f2defd2562a601c43))

## [1.0.1](https://github.com/hawk90/revue/compare/v1.0.0...v1.0.1) (2026-01-03)


### Bug Fixes

* add allow(dead_code) to test functions and adjust banner size ([31fa7a9](https://github.com/hawk90/revue/commit/31fa7a9e5c78e78d9c59d5fdc32297ffe5cdd725))
* add allow(dead_code) to test structs ([99e16bc](https://github.com/hawk90/revue/commit/99e16bc201f56fe97ba68eb55f5491721529cfd5))
* add fxhash and zune-jpeg version advisories ([b0c79bd](https://github.com/hawk90/revue/commit/b0c79bd42c92b4c7c8d3b63b46273363ff6f6f20))
* add missing licenses and relax FPS test threshold ([fd054b4](https://github.com/hawk90/revue/commit/fd054b40e4da7787257996939553183e7febc5a2))
* add poison recovery to Effect callback handling ([38dfa12](https://github.com/hawk90/revue/commit/38dfa124c4c5404cb8b1e7fc7762fe0a58990132))
* **ci:** remove missing labels from dependabot ([1889bdd](https://github.com/hawk90/revue/commit/1889bdd2adac56292be1cf500a9f237716c617aa))
* **ci:** skip commitlint on initial push ([950463d](https://github.com/hawk90/revue/commit/950463dd48585c0d860b6c17d78e69213804a194))
* **ci:** update deny.toml for cargo-deny v0.18+ ([7f2ec16](https://github.com/hawk90/revue/commit/7f2ec16fa93ae67d2cd3b5483c12eb5bcbaa7d95))
* **ci:** update deny.toml for cargo-deny v0.18+ compatibility ([fd9fbad](https://github.com/hawk90/revue/commit/fd9fbad20589261d4b694bb140ff3409d742e7d2))
* downgrade zune-jpeg to 0.4.21 for MSRV compatibility ([50f9d10](https://github.com/hawk90/revue/commit/50f9d10e0d325639809f8d3e713e0d016aa07763))
* ignore flaky test and add CI job dependencies ([b1f7bdd](https://github.com/hawk90/revue/commit/b1f7bdd2868a82ccfc7b4e39e87e1142a92bd6ee))
* ignore unmaintained advisories and update CI badges ([35c0db1](https://github.com/hawk90/revue/commit/35c0db1c8820570748979d322870b5a0aad9847d))
* resolve all example warnings for CI ([4fa0360](https://github.com/hawk90/revue/commit/4fa03602991088e826df4c5738936460491b6ba8))
* resolve CI failures across all platforms ([e654e23](https://github.com/hawk90/revue/commit/e654e235a85370f8be49997f08daac3582610212))
* resolve CI failures and reorganize workflow ([2df63d6](https://github.com/hawk90/revue/commit/2df63d632970cf9ac55ab0d534c4b764abc15f69))
* resolve clippy warnings (impl derive, repeat_n, div_ceil) ([e17d8d8](https://github.com/hawk90/revue/commit/e17d8d859e9531bed2aedf7d1a445638f2b7ae39))
* resolve clippy warnings and flaky test issues ([5d3ab22](https://github.com/hawk90/revue/commit/5d3ab22625644f724281901d2cb05c3f954999bd))
* resolve critical bugs and improve code quality ([eef7b15](https://github.com/hawk90/revue/commit/eef7b15947f76335c343389b1db136d0868b29d1))
* resolve doc links and security workflow ([bc12246](https://github.com/hawk90/revue/commit/bc1224663d1f510eb06d174644dc890906428eed))
* resolve doc warnings and update MSRV to 1.85 ([ed7cfbf](https://github.com/hawk90/revue/commit/ed7cfbfaa7ca808cbfe63f8ae155c7f67c15a3a3))
* update to taffy 0.9.2 API and remove dead_code allow ([e919b41](https://github.com/hawk90/revue/commit/e919b41a15f70410313ec35e1962d7433ac4d57c))

## [Unreleased]

## [1.0.0] - 2026-01-02

### Highlights

**Revue 1.0 is here!** After 10 releases of iterative development, Revue is now production-ready.

### Added

- **Production Ready**
  - Stable API with semantic versioning guarantee
  - Complete documentation with tutorials and guides
  - 80%+ test coverage with visual regression testing
  - Cross-platform support (Linux, macOS, Windows)

### Changed

- **API Stabilization**
  - Removed all deprecated APIs for clean 1.0 release
  - Finalized widget constructor patterns
  - Stabilized reactive system API

### Removed

- `Runtime` struct (deprecated since v0.8.0) - use `App::builder()` instead

### Migration from v0.9.0

No breaking changes from v0.9.0. Simply update your `Cargo.toml`:

```toml
[dependencies]
revue = "1.0"
```

## [0.9.0] - 2026-01-02

### Added

- **Documentation**
  - Complete tutorial series (Getting Started, Counter, Todo)
  - Comprehensive guides:
    - Styling Guide - CSS, variables, themes, transitions
    - State Management Guide - Signals, computed, effects, async
    - Testing Guide - Pilot framework, snapshots, visual testing
    - Performance Guide - Optimization, profiling, memory
    - Accessibility Guide - WCAG, focus management, screen readers

### Changed

- Updated tutorials to use correct v0.9 API patterns
- Fixed API examples in documentation

## [0.8.0] - 2026-01-02

### Added

- **API Stabilization**
  - `View` implementation for `Box<dyn View>` enabling boxed views as children
  - Migration guide documentation (`docs/migration/v0.8.0.md`)
  - Property-based tests using proptest for core components

- **Testing Improvements**
  - 11 new property-based tests for Rect, Color, and Signal
  - Proptest integration for fuzz testing

### Changed

- **Widget API Consistency**
  - Fixed gallery example to use correct widget APIs
  - Documented constructor patterns (convenience vs builder)

### Fixed

- Gallery example compilation errors with correct API usage
- Border widget usage patterns (Border::rounded().child())
- Checkbox, Switch, and Gauge API usage

## [0.7.0] - 2026-01-02

### Added

- **Plugin CLI Commands**
  - `revue plugin list` - List installed plugins
  - `revue plugin search <query>` - Search crates.io for plugins
  - `revue plugin install <name>` - Install a plugin
  - `revue plugin info <name>` - Show plugin information
  - `revue plugin new <name>` - Create new plugin project

- **VS Code Extension** (`extensions/vscode/`)
  - Revue CSS syntax highlighting
  - 25+ Rust widget snippets (vstack, hstack, border, button, input, etc.)
  - CSS snippets for Revue-specific properties
  - Language configuration for `.rcss` files

- **Zed Extension** (`extensions/zed/`)
  - Revue CSS language support
  - Syntax highlighting via tree-sitter
  - Language configuration

- **Online Playground** (`playground/`)
  - WASM-based terminal emulator
  - Live code editing with syntax highlighting
  - Example templates (Hello World, Counter, Todo, Dashboard)
  - Share functionality via URL

- **Theme Builder** (`tools/theme-builder/`)
  - Interactive TUI for creating themes
  - Live preview of color changes
  - Preset themes (Tokyo Night, Dracula, Nord, Gruvbox, Catppuccin)
  - CSS export with full widget styles

### Changed

- CLI updated with ureq and serde_json for HTTP API calls
- Plugin ecosystem now supports crates.io discovery

## [0.6.0] - 2026-01-02

*Note: v0.5.0 features were merged into v0.6.0 release*

### v0.5.0 Features (DX & Testing)

- **Visual Regression Testing**
  - `VisualTest` for pixel-perfect UI comparisons
  - `VisualCapture` with color, style, and text capture
  - `VisualDiff` for detailed difference reporting
  - Golden file serialization/deserialization
  - Color tolerance for fuzzy matching
  - `REVUE_UPDATE_VISUALS=1` for updating golden files

- **CI Integration**
  - `CiEnvironment` with auto-detection for GitHub Actions, GitLab CI, CircleCI, Travis CI, Jenkins, Azure Pipelines
  - `TestReport` with markdown generation
  - GitHub Actions annotations for test failures
  - Artifact collection for failed tests
  - Branch, commit, PR detection

- **DevTools**
  - `Inspector` - Widget tree viewer
  - `StateDebugger` - Reactive state viewer
  - `StyleInspector` - CSS style inspector with property sources
  - `EventLogger` - Event stream logger with filtering
  - Tabbed panel UI with position options (Right, Bottom, Left, Overlay)
  - F12 toggle support

- **Performance Profiler**
  - `Profiler` with hierarchical timing
  - `profile()` macro for easy instrumentation
  - Statistics: count, total, min, max, average
  - Global profiler instance
  - Report generation

- **Examples & Gallery**
  - 22+ example applications
  - Widget gallery (`gallery.rs`)
  - Dashboard, IDE, chat, todo examples

### Added

- **Drag & Drop System**
  - `DragContext` for managing drag state and data transfer
  - `Draggable` trait for widgets that can be dragged
  - `DragData` with type-erased payload and MIME types
  - `DropZone` widget for drop targets with visual feedback
  - `SortableList` widget for reorderable lists
  - `DragState` enum: Idle, Dragging, Over, Dropped

- **Resize & Layout**
  - `Resizable` widget wrapper for dynamic sizing
  - `ResizeHandle` with 8 directions (corners + edges)
  - Aspect ratio preservation and grid snapping
  - `Breakpoints` system for responsive layouts (XS, SM, MD, LG, XL)
  - `ResponsiveValue<T>` for breakpoint-aware values
  - `MediaQuery` for width/height-based conditionals

- **Focus Management**
  - Nested focus traps with `push_trap()` / `pop_trap()`
  - Focus restoration with `release_trap_and_restore()`
  - `FocusTrap` helper struct with RAII-style cleanup
  - `FocusTrapConfig` for customizing trap behavior
  - `trap_depth()` for querying nesting level

- **Performance Optimizations**
  - `VirtualList` variable height support with `HeightCalculator`
  - Binary search for O(log n) row lookup in virtual lists
  - `ScrollMode` (Item, Pixel) and `ScrollAlignment` options
  - `jump_to()`, `scroll_by()`, `scroll_position()` methods
  - Lazy loading patterns: `LazyData`, `LazyReloadable`, `LazySync`
  - `PagedData` for paginated datasets
  - `ProgressiveLoader` for chunked loading
  - `RenderBatch` for batched terminal operations
  - `RenderOp` enum for optimized render commands
  - Consecutive cell merging into text operations
  - Object pooling: `ObjectPool<T>`, `SyncObjectPool<T>`
  - `BufferPool` for render buffer reuse
  - `StringPool` / `SyncStringPool` for string interning
  - `VecPool<T>` for vector reuse
  - `Pooled<T>` RAII guard for automatic pool return
  - `PoolStats` for monitoring cache hit rates

### Changed

- Prelude now exports drag/drop, resize, and lazy loading types
- `FocusManager` supports nested traps with stack-based management
- Widget module exports `DropZone`, `SortableList`, `Resizable`
- Layout module exports `Breakpoints`, `ResponsiveValue`, `MediaQuery`
- Patterns module exports lazy loading types and constructors
- Render module exports `RenderBatch`, `RenderOp`, `BatchStats`
- DOM module exports pooling types

## [0.4.0] - 2026-01-02

### Added

- **Thread-Safe Reactive System**
  - `Signal<T>` now uses `Arc<RwLock<T>>` for thread-safety
  - Async hooks: `use_async()`, `use_async_poll()`, `use_async_immediate()`
  - `AsyncState` and `AsyncResult` types for async operations
  - Thread-safe tracker, effect, and computed primitives

- **Accessibility System**
  - High-contrast themes: `HighContrastDark`, `HighContrastLight` (WCAG AAA compliant)
  - `BuiltinTheme::accessibility()` and `is_accessibility()` helpers
  - Focus indicator rendering: `draw_focus_ring()`, `draw_focus_underline()`, `draw_focus_marker()`
  - `FocusStyle` enum: Solid, Rounded, Double, Dotted, Bold, Ascii
  - Screen reader announcements: `announce()`, `announce_now()`, `take_announcements()`
  - Widget-specific helpers: `announce_button_clicked()`, `announce_checkbox_changed()`, etc.
  - Preference functions: `prefers_reduced_motion()`, `is_high_contrast()`

- **Animation Engine**
  - `KeyframeAnimation` - CSS @keyframes style with percentage-based keyframes
  - `CssKeyframe` for defining property values at each percentage
  - `AnimationDirection`: Normal, Reverse, Alternate, AlternateReverse
  - `AnimationFillMode`: None, Forwards, Backwards, Both
  - `Stagger` for staggered animation delays across multiple elements
  - `AnimationGroup` for parallel/sequential animation coordination
  - `Choreographer` for managing multiple named animation sets
  - `widget_animations` module with pre-built effects:
    - `fade_in()`, `fade_out()`, `slide_in_left/right/top/bottom()`
    - `scale_up()`, `scale_down()`, `bounce()`, `shake()`
    - `pulse()`, `blink()`, `spin()`, `cursor_blink()`
    - `toast_enter()`, `toast_exit()`, `modal_enter()`, `modal_exit()`
    - `shimmer()` for loading effects

- **Reduced Motion Support**
  - `should_skip_animation()` - check if animations should be skipped
  - `effective_duration()` - returns zero duration when reduced motion preferred
  - All animations automatically respect user's reduced motion preference
  - `TransitionManager` skips transitions when reduced motion is enabled

### Changed

- Prelude now exports animation types and accessibility functions
- `Signal::new()` returns thread-safe signal usable across threads
- All reactive primitives support concurrent access

## [0.3.0] - 2026-01-02

### Added

- **Plugin System**
  - `Plugin` trait with lifecycle hooks: `on_init`, `on_mount`, `on_tick`, `on_unmount`
  - `PluginContext` for plugin data storage and cross-plugin communication
  - `PluginRegistry` for managing plugin ordering by priority
  - Built-in plugins: `LoggerPlugin`, `PerformancePlugin`
  - `App::builder().plugin()` method for registering plugins
  - Plugin styles collected and merged with app stylesheet

- **Runtime Theme Switching**
  - Signal-based theme system via `use_theme()` returning `Signal<Theme>`
  - Theme functions: `set_theme()`, `set_theme_by_id()`, `toggle_theme()`, `cycle_theme()`
  - Theme registration: `register_theme()`, `get_theme()`, `theme_ids()`
  - `theme_to_css_variables()` for generating CSS variable stylesheets
  - `ThemePicker` widget for interactive theme selection

- **New Patterns**
  - `SearchState` for list filtering with fuzzy/contains/prefix/exact modes
  - `FormState` with field validation, focus navigation, and submit handling
  - `FormField` with validators: required, min/max length, email, numeric, custom
  - `NavigationState` for browser-like history with back/forward navigation
  - `Route` with path matching and parameters
  - `build_breadcrumbs()` helper for navigation trails

- **CLI Enhancements**
  - `revue add <component>` command for generating component templates
    - Components: search, form, navigation, modal, toast, command-palette, table, tabs
  - `revue benchmark` command for running Criterion benchmarks
  - Component templates with full working examples

### Changed

- Prelude now exports theme functions and new pattern types
- Widget module exports `ThemePicker` and `theme_picker()` constructor
- Patterns module exports search, form, and navigation types

## [0.2.0] - 2025-01-02

### Added

- **Testing Infrastructure**
  - Expanded widget snapshot tests from 29 to 65 test cases
  - Criterion benchmarks for DOM, CSS, Layout, and Rendering
  - DOM benchmark measuring incremental build performance

- **Performance Optimizations**
  - Incremental DOM build: reuses existing nodes by ID or position
    - 2% faster for 10 children
    - 36% faster for 50 children
    - 54% faster for 100 children
  - Node-aware transition tracking for partial rendering
  - Style cache preservation for unchanged nodes

- **Transition System Enhancements**
  - `TransitionManager::start_for_node()` for element-specific transitions
  - `TransitionManager::active_node_ids()` for partial rendering optimization
  - `TransitionManager::get_for_node()` and `current_values_for_node()`

- **DOM Renderer Methods**
  - `DomRenderer::invalidate()` to force fresh rebuild
  - `DomRenderer::build_incremental()` for efficient updates

### Changed

- `DomRenderer::build()` now performs incremental updates when possible
- Transition updates now process both legacy and node-aware transitions
- Dirty rect calculation uses node-specific areas for active transitions

### Fixed

- Documentation links with proper module prefixes (`widget::`, `app::`, etc.)
- Escaped brackets in widget documentation (checkbox, radio, switch)
- Ambiguous doc links using `mod@` prefix
- Missing `keys` module export in patterns

## [0.1.0] - 2024-XX-XX

### Added

- Core rendering engine with double buffering
- CSS parser with variables, selectors, transitions, and animations
- Flexbox layout powered by taffy
- Reactive state management (Signal, Computed, Effect)
- 70+ widgets
  - Layout: Stack, Grid, Scroll, Tabs, Accordion, Splitter
  - Input: Input, TextArea, Select, Checkbox, Switch, Slider, ColorPicker
  - Display: Text, RichText, Markdown, Table, Progress, Badge, Image
  - Feedback: Modal, Toast, Notification, Tooltip
  - Data Viz: BarChart, LineChart, Sparkline, Heatmap
  - Advanced: Terminal, Vim mode, AI Streaming, Mermaid diagrams
- Hot reload for CSS files
- Widget inspector (devtools)
- Snapshot testing utilities
- Built-in themes (Dracula, Nord, Monokai, Gruvbox, Catppuccin)
- Kitty graphics protocol support for images
- Unicode and emoji support
- Clipboard integration
- i18n support

[Unreleased]: https://github.com/hawk90/revue/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/hawk90/revue/compare/v0.9.0...v1.0.0
[0.9.0]: https://github.com/hawk90/revue/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/hawk90/revue/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/hawk90/revue/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/hawk90/revue/compare/v0.4.0...v0.6.0
[0.4.0]: https://github.com/hawk90/revue/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/hawk90/revue/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/hawk90/revue/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/hawk90/revue/releases/tag/v0.1.0
