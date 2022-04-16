# rs-hotreloadexample
Adapted and updated live reload example from https://fasterthanli.me/articles/so-you-want-to-live-reload-rust

## Updates since the article was written

The biggest improvement appears to be that appears the "breakaround" is no longer required! `libloading` seems to have made some changes (including declaring `Library::new` as `unsafe`) that properly closes the lib on drop. I've tested it with valgrind to confirm that the memory leaks have disappeared, and couldn't find any issues. 

Other nice updates include: 

- `notify` has an easier to use api
- the target folder has a ./deps folder you can check for write events to, this means you can just watch the lib folder and rebuild the main folder. i.e

```sh
# from ./libgreet
cargo-watch -- cargo build --manifest-file-path=../Cargo.toml
```

The article is a really great read and there is *a lot* to learn from it.

