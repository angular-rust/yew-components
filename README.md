# YEW Material Components

[![API Docs][docrs-badge]][docrs-url]
[![Crates.io][crates-badge]][crates-url]
[![MPL-2.0 licensed][license-badge]][license-url]
[![Gitter chat][gitter-badge]][gitter-url]
[![Rustc Version 1.45+][rust-badge]][rust-url]

[docrs-badge]: https://img.shields.io/docsrs/ymc?style=flat-square
[docrs-url]: https://docs.rs/ymc/
[crates-badge]: https://img.shields.io/crates/v/ymc.svg?style=flat-square
[crates-url]: https://crates.io/crates/ymc
[license-badge]: https://img.shields.io/badge/license-MPL--2.0-blue.svg?style=flat-square
[license-url]: https://github.com/angular-rust/yew-components/blob/master/LICENSE
[gitter-badge]: https://img.shields.io/gitter/room/angular_rust/angular_rust.svg?style=flat-square
[gitter-url]: https://gitter.im/angular_rust/angular_rust
[rust-badge]: https://img.shields.io/badge/rustc-1.45-lightgrey.svg?style=flat-square
[rust-url]: https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html

Angular Rust is a high productivity, frontend web framework for the [Rust language](https://www.rust-lang.org/).

Material Design Components for the Yew framework

Yew Material Components is a components library for [Yew framework](https://yew.rs/) which is a wrapper around [Material Design Components](https://github.com/material-components/material-components-web) exposing Yew components. All modern browsers are supported.


## Getting started
### Installation

Currently, this library is available from [crates.io](https://crates.io/). Add it using `cargo-edit`
```
cargo add ymc
```

Cargo features are used to pick the components. See [features](#features)   

`Cargo.toml`:
```toml
[dependencies]
ymc = { version = "0.1", features = ["full"] }
```
Material icons and a Material font can also be imported for full functionality.  
`index.html`:
```html
<link href="https://fonts.googleapis.com/css?family=Roboto:300,400,500" rel="stylesheet">
<link href="https://fonts.googleapis.com/css?family=Material+Icons&display=block" rel="stylesheet">
```

It's also important to note that you need `viewport` `meta` tag for the Material Components to be responsive.
```html
<meta name="viewport" content="width=device-width, initial-scale=1.0">
```

### Features

Following are all the cargo features available (each feature corresponds to its respective component):

- [x] `banner`
- [x] `button`
- [x] `card`
- [x] `checkbox`
- [x] `chips`
- [x] `circular-progress-four-color`
- [x] `circular-progress`
- [x] `data-table`
- [x] `dialog`
- [x] `drawer`
- [x] `fab`
- [x] `formfield`
- [x] `icon-button-toggle`
- [x] `icon-button`
- [x] `icon`
- [x] `image-list`
- [x] `linear-progress`
- [x] `list`
- [x] `menu`
- [x] `radio`
- [x] `select`
- [x] `slider`
- [x] `slider`
- [x] `snackbar`
- [x] `switch`
- [x] `tabs`
- [x] `textarea`
- [x] `textfield`
- [x] `tooltip`
- [x] `top-app-bar-fixed`
- [x] `top-app-bar`

`full` feature enables all the components

## Adding additional components
This is still a work in progress... feel free to add additional components.

To port a component that hasn't been ported yet open [TODO](https://github.com/angular-rust/yew-components/blob/main/TODO.md)

Implement a component you need and add it to the src/ directory.

## Theming

These components respect the theming applied to Material Design Components using stylesheets. [Learn about how to theme Material Design Components.](https://github.com/material-components/material-components-web-components/blob/master/docs/theming.md)

## Learn More

* [Manual, Samples, Docs, etc](https://angular-rust.github.io/)
* [Apps using Angular Rust](https://github.com/angular-rust/yew-components/wiki/Apps-in-the-Wild)
* [Articles Featuring Angular Rust](https://github.com/angular-rust/yew-components/wiki/Articles)

## Community

* [Gitter](https://gitter.im/angular_rust/community)
* [StackOverflow](https://stackoverflow.com/questions/tagged/angular-rust)

## Bugs ##

If you find an issue, let me know [here](https://github.com/angular-rust/yew-components/issues/new).

## Contributing

Your contributions are welcome. I openly welcome community contributions, including bug fixes and new features. Please feel free to [fork the project](https://github.com/theukedge/recent-contributors-widget/fork) and submit a pull request.

* [Contributing Code Guidelines](https://github.com/angular-rust/yew-components/blob/main/CONTRIBUTING.md)
* [Angular Rust Contributors](https://github.com/angular-rust/yew-components/graphs/contributors)


## Alternatives

- https://github.com/Follpvosten/yew-mdc
- https://github.com/AlephAlpha/muicss-yew

## Resources

- https://github.com/material-components/material-components-web
- https://github.com/material-components/material-components-web-react
- https://material-components.github.io/material-components-web-catalog/#/
- https://github.com/jamesmfriedman/rmwc
- https://github.com/pgbross/vue-material-adapter
- https://github.com/material-components/material-components-web/blob/master/docs/integrating-into-frameworks.md
- https://github.com/material-components/material-components-web/blob/master/docs/code/architecture.md
