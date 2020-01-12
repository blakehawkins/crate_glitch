`crate_glitch` is a [matrix.org](https://matrix.org) bot which monitors messages in a channel and provides links based
on simple pattern matching.

Execute as `$ crate_glitch <config_file>` (default `config.yaml`), with config like the following:

```yaml
---
token: ""
password: ""
account: "crates.io"
room: "#rust:matrix.org"
listen_to: "!crate "
prepend_with: "https://crates.io/crates/"
```

The bot logs in using the account and credentials provided, and begins listening in the provided room.
By default, the bot catches messages like `!crate serde` and provides a link to crates.io/crates/serde.

Note:

- The bot account must already be in the channel configured.
- Login attempts to use the provided token, or falls back to password-auth if the token fails.
  In either case, the service will print the token to stdout.
  If you don't have a token, use password-auth and copy the emitted token into your config for next time.

