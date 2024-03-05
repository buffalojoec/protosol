# ProtoSol

A Protobuf fuzzing & testing harness for Solana programs.

## Testing Programs

There are two ways to test a Solana program with Protosol: integration testing
and fuzzing.

To integration test a Solana program, add `protosol` as a dev-dependency, then
create some fixtures and run them within test cases using `process_fixture`.

```
cargo test-sbf
```

To fuzz a Solana program, create one or more fuzz targets similar to the
examples in the [`fuzz` directory](./fuzz/). Then run the fuzzer on your
targets.

```
cargo fuzz <target>
```

## Crate Tests

```
./test
```

```
./fuzz-test
```
