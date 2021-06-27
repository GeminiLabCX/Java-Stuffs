python3 -m pip install -U pip wheel annoy

python3 builder.py 256 10000

python3 bencher.py 256 10000 200 1000

pushd rust
cargo run -q --release -- 256 10000 200 1000
cargo +nightly run -q --release -- 256 10000 200 1000
popd

pushd dart
dart pub get && dart run main.dart 256 10000 200 1000
popd

dotnet run --project dotnet -c Release -- --