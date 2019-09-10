fn main() {
    cc::Build::new()
        .file("src/make_array.c")
        .compile("libarray.a");
}
