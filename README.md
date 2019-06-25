# Rust allocator backed by ESP-IDF

This is a memory allocator for Rust, backed by the [ESP-IDF](https://docs.espressif.com/projects/esp-idf/en/latest/).


This is intended to be used on an ESP32, linked against the ESP-IDF. For more information see:

* https://github.com/ctron/rust-esp-container/
* https://github.com/ctron/rust-esp-template
* https://github.com/MabezDev/rust-xtensa.git
* https://quickhack.net/nom/blog/2019-05-14-build-rust-environment-for-esp32.html

## Usage

Add the following to your main, application project:

~~~rust
extern crate esp_idf_alloc;

#[global_allocator]
static A: esp_idf_alloc::EspIdfAllocator = esp_idf_alloc::EspIdfAllocator;
~~~

## Using with `alloc`

Also be sure to link in the `alloc` create, as you might want this. Add the following to your `Xargo.toml`:

~~~toml
[target.xtensa-esp32-none-elf.dependencies]
alloc={}
~~~
