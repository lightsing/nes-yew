# nes-yew

[![Crates.io](https://img.shields.io/crates/v/nes-yew)](https://crates.io/crates/nes-yew)
[![docs.rs](https://docs.rs/nes-yew/badge.svg)](https://docs.rs/nes-yew)

> Yew component library for nes.css derived from [nes-react](https://github.com/bschulte/nes-react)

## Install

```bash
cargo add nes-yew
```

## Demo

<https://lightsing.github.io/nes-yew/>

## Usage

If you want to use the default font family for nes.css (Press Start 2P), make sure to include that in your app in whatever way you choose.
To use Google Font APIs, just stick a

```html
<link
  href="https://fonts.googleapis.com/css?family=Press+Start+2P"
  rel="stylesheet"
/>
```

in your `index.html`.

```rust
use nes_yew::*;

#[function_component]
fn App() -> Html {
    html! {
        <Container>{"We're using containers from nes.css!"}</Container>
    }
}
```

## Development

`git clone git@github.com:lightsing/nes-yew.git`

`npm install` in both the in /example.

`trunk serve` then also in /example.

## License

MIT © [lightsing](https://github.com/lightsing)