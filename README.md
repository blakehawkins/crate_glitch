`crate_glitch` is a [matrix.org](https://matrix.org) bot which monitors messages in a channel and provides links based
on simple pattern matching.

Execute as `$ crate_glitch <config_file>` (default `config.yaml`), with config like the following:

```yaml
---
token: ""
password: ""
account: "@crates.io:matrix.org"
room: "#rust:matrix.org"
listen_to: "!crate"
prepend_with: "https://crates.io/crates/"
```

The bot logs in using the account and credentials provided, and begins listening in the provided room.
By default, the bot catches messages like `!crate serde` and provides a link to crates.io/crates/serde.

Note:

- The bot account must already be in the channel configured.
