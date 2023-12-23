# Digraph Toy

![ls digraph](./ls-binary.png)

The goal of this is to explore generating digraph png images in rust. Right now
its not all rust because I didn't feel like setting up a cargo project in order
to use the `image` crate. Maybe I'll do that at some point in the meantime this
generation is extremely manual and requires the `chunky_png` ruby gem as a
shortcut to building the png. 

In its current iteration the workflow looks like:

```console
$ ./build.sh
$ ./digraph-rs <somefile> > digraph.out
$ ruby to_png.rb
$ xdg-open digraph.png
```

The `samples/` dir is currently empty and ignored by git but thats just where I
have been moving interesting results locally.

Ultimately I'll move everything into rust and tie the steps together in the
single `./digraph-rs` call.





