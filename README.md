# memflow-microvmi

This connector implements an interface for [libmicrovmi](https://github.com/Wenzel/libmicrovmi).

## Compilation

### Using the crate in a rust project

To use the connector in a rust project just include it in your Cargo.toml

```
memflow-microvmi = "0.1"
```

Make sure to not enable the `inventory` feature when importing multiple
connectors in a rust project without using the memflow connector inventory.
This might cause duplicated exports being generated in your project.

### Building the stand-alone connector for dynamic loading

The stand-alone connector of this library is feature-gated behind the `inventory` feature.
To compile a dynamic library for use with the connector inventory use the following command:

```
cargo build --release --features inventory,[libvmi_backends]
```

Available libvmi backends are currently hyper-v, kvm, virtualbox and xen.

### Installing the library

Alternatively to manually placing the library in the `PATH` the connector can be installed with the `install.sh` script.
It will place it inside `~/.local/lib/memflow` directory. Add `~/.local/lib` directory to `PATH` to use the connector in other memflow projects.

## Arguments

- `name` - the libvmi domain name (default argument; mandatory)
- `type` - the type of the target (hyper-v, kvm, virtualbox or xen; optional)
- `option` - the libvmi init options (optional)

## License

Licensed under MIT License, see [LICENSE](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
