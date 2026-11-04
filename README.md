# Please Use My RAM

**Your RAM is just sitting there. Doing nothing. Let's fix that.**

A website that borrows your RAM in 1 GiB chunks. Returns are accepted 50% of the time.

Live at **https://pleaseusemyram.cleslie.uk**

## Why

RAM is expensive right now and AI companies are buying all of it.
An AI executive would like to know why your RAM isn't pulling its weight.

I can't prove this. I'd rather not ask. Your RAM is at least busy now.

## Performance

- Written in Rust and compiled to WebAssembly. The only thing faster than this code is how quickly your other browser tabs will stop responding.
- Each allocation gets its own Web Worker, so your RAM is used in parallel.
- Every byte is filled with the highest quality pseudorandom noise I could find, because the operating system's memory compression is smart. You can be sure that your RAM is actually being used, not just reserved.
- Zero dependencies at runtime.
- When benchmarked against your 40+ extension VS Code and every other Electron app, Please Use My RAM was found to be **at least as memory hungry**.

## Usage

Click **+1** to make your computer warmer.

Click **-1** to try and give a GiB back.

Keep clicking at your own risk. I accept no responsibility for:

- Your browser deciding it has had enough
- Your operating system deciding it has had enough
- You deciding you have had enough

## Build

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
wasm-pack build --target web --release --out-dir web/pkg
python -m http.server -d web 8080
```

## FAQ

**Is this safe?**
Define safe.

**Will it actually use my RAM?**
Yes.

**Does it work on my phone?**
It will try. Your phone will say no.

**Has Sam Altman endorsed this?**
No. Thankfully I am not affiliated with him, with OpenAI, or with anyone else who might want your memory. This is a joke. Please don't sue me.

**Can I have my RAM back?**
Maybe, maybe not. The **-1** button has a 50% chance to do nothing.

## License

[WTFPL](LICENSE). Do what the fuck you want to.