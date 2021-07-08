# rustnot

_Don't rust. Stand up._

This tool is just a simple programm running in the background and remindining you to
stand up and sit back down via desktop notifications.

Reason for this is I have a desk which can be used while standing or sitting and while
working long hours at my PC I tend to forget to change positions once in a while to
relax my back.

### Config

Upon start the tool tries to read the durations for sitting and standing from a
`rustnot_config.toml` file. If this file does not exist it will be created with a
default config. If the file cannot be read the default config is also loaded.

The default config is:
- 45 min sitting
- 15 min standing

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
