# 

<div align="center">

[![](https://dudochkin-victor.github.io/assets/yew-components/logo.svg)](#top)
# YEW Material Components

[![API Docs][docrs-badge]][docrs-url]
[![Crates.io][crates-badge]][crates-url]
[![Code coverage][codecov-badge]][codecov-url]
[![Tests][tests-badge]][tests-url]
[![MPL-2.0 licensed][license-badge]][license-url]
[![Gitter chat][gitter-badge]][gitter-url]
[![loc][loc-badge]][loc-url]
</div>

[docrs-badge]: https://img.shields.io/docsrs/ymc?style=flat-square
[docrs-url]: https://docs.rs/uymc/
[crates-badge]: https://img.shields.io/crates/v/ymc.svg?style=flat-square
[crates-url]: https://crates.io/crates/ymc
[license-badge]: https://img.shields.io/badge/license-MPL--2.0-blue.svg?style=flat-square
[license-url]: https://github.com/angular-rust/yew-components/blob/master/LICENSE
[gitter-badge]: https://img.shields.io/gitter/room/angular_rust/community.svg?style=flat-square
[gitter-url]: https://gitter.im/angular_rust/community
[tests-badge]: https://img.shields.io/github/workflow/status/angular-rust/yew-components/tests?label=tests&logo=github&style=flat-square
[tests-url]: https://github.com/angular-rust/yew-components/actions/workflows/tests.yml
[codecov-badge]: https://img.shields.io/codecov/c/github/angular-rust/yew-components?logo=codecov&style=flat-square&token=OWZIWBTGII
[codecov-url]: https://codecov.io/gh/angular-rust/yew-components
[loc-badge]: https://img.shields.io/tokei/lines/github/angular-rust/yew-components?style=flat-square
[loc-url]: https://github.com/angular-rust/yew-components

Material Design Components for the Yew framework.

**Angular Rust** is a high productivity, `platform-agnostic` frontend framework for the [Rust language](https://www.rust-lang.org/). It now supports desktop and web development. Angular Rust currently uses GTK for desktop development and WebAssembly for web development. We are planning to add support for mobile development.

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

* [Manual, Docs, etc](https://angular-rust.github.io/)
* [Samples](https://github.com/angular-rust/ux-samples)
* [Apps using Angular Rust](https://github.com/angular-rust/yew-components/wiki/Apps-in-the-Wild)
* [Articles Featuring Angular Rust](https://github.com/angular-rust/yew-components/wiki/Articles)


## Community

 [![](https://img.shields.io/badge/Facebook-1877F2?style=for-the-badge&logo=facebook&logoColor=white)](https://www.facebook.com/groups/angular.rust) 
 [![](https://img.shields.io/badge/Stack_Overflow-FE7A16?style=for-the-badge&logo=stack-overflow&logoColor=white)](https://stackoverflow.com/questions/tagged/angular-rust) 
 [![](https://img.shields.io/badge/YouTube-FF0000?style=for-the-badge&logo=youtube&logoColor=white)](https://www.youtube.com/channel/UCBJTkSl_JWShuolUy4JksTQ) 
 [![](https://img.shields.io/badge/Medium-12100E?style=for-the-badge&logo=medium&logoColor=white)](https://medium.com/@angular.rust) 
 [![](https://img.shields.io/gitter/room/angular_rust/angular_rust?style=for-the-badge)](https://gitter.im/angular_rust/community)


## Contributing

We believe the wider community can create better code. The first tool for improving the community is to tell the developers about the project by giving it a star. More stars - more members.

 [![](https://dudochkin-victor.github.io/assets/star-me-wide.svg)](https://github.com/angular-rust/yew-components#top)

Angular Rust is a community effort and we welcome all kinds of contributions, big or small, from developers of all backgrounds. We want the Angular Rust community to be a fun and friendly place, so please review our [Code of Conduct](CODE_OF_CONDUCT.md) to learn what behavior will not be tolerated.

### New to Angular Rust?

Start learning about the framework by helping us improve our [documentation](https://angular-rust.github.io/). Pull requests which improve test coverage are also very welcome.

### Looking for inspiration?

Check out the community curated list of awesome things related to Angular Rust / WebAssembly at [awesome-angular-rust](https://github.com/angular-rust/awesome-angular-rust).

### Confused about something?

Feel free to drop into our [Gitter chatroom](https://gitter.im/angular_rust/community) or open a [new "Question" issue](https://github.com/angular-rust/yew-components/issues/new/choose) to get help from contributors. Often questions lead to improvements to the ergonomics of the framework, better documentation, and even new features!

### Ready to dive into the code?

After reviewing the [Contributing Code Guidelines](CONTRIBUTING.md), check out the ["Good First Issues"](https://github.com/angular-rust/yew-components/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) (they are eager for attention!). Once you find one that interests you, feel free to assign yourself to an issue and don't hesitate to reach out for guidance, the issues vary in complexity.

### Let's help each other!

Come help us on the [issues that matter that the most](https://github.com/angular-rust/yew-components/labels/%3Adollar%3A%20Funded%20on%20Issuehunt) and receive a small cash reward for your troubles. We use [Issuehunt](https://issuehunt.io/r/angular-rust/yew-components/) to fund issues from our Open Collective funds. If you really care about an issue, you can choose to add funds yourself! 

### Found a bug?

Please [report all bugs!](https://github.com/angular-rust/yew-components/issues/new/choose) We are happy to help support developers fix the bugs they find if they are interested and have the time.

## Todo
- [ ] Documentation

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
