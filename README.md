<div align="center">

![logo](https://i.imgur.com/tBm6IJA.png)

# osu-skin-flipper

A nice little tool to flip osu! skin numbers.

![GitHub release (latest by date)](https://img.shields.io/github/v/release/damaredayo/osu-skin-flipper)

</div>

## Usage

- Download the latest release from the [releases page](https://github.com/damaredayo/osu-skin-flipper/releases)
- Click `Browse` in the GUI and navigate to the directory of the osu! skin you wish to flip.
- Click `Start` and the application will flip the skin for you and automatically import it into osu!, just as long as the Default Association for `.osk` files is set to osu!.

<div align="center">

# Example

![Example](https://i.imgur.com/THI8N0u.png)

![Example](https://i.imgur.com/pR76FQE.png)

</div>

## Building

osu-skin-flipper is written in Rust, so to build you will need to install the [Rust toolchain](https://rustup.rs/).

Once you have the Rust toolchain installed, you can build the application with the following command:

```bash
cargo build --release
```

Where it will then be located in `./target/release`.

## License

osu-skin-flipper is licensed under the [MIT License](https://opensource.org/license/mit/)
