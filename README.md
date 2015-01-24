Low-level Rust import crate for GObject.

Currently the primary purpose of this library is to support
[Grust](https://github.com/mzabaluev/grust). Eventually the
code for the library should be generated from
[GObject introspection](https://wiki.gnome.org/Projects/GObjectIntrospection)
data using Grust binding generator.

Meanwhile, the function declarations are added as needed for the development
of the Grust core library. If you'd like to reuse this crate outside of
Grust, feel free to add other functions and submit your changes in pull
requests.
