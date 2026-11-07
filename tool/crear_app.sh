#!/usr/bin/env bash
# Crea cuevas_app/: una app de Flutter con flutter_rust_bridge, tu cuevas.rs y la pantalla que dibuja la cueva.
# Necesita Flutter y flutter_rust_bridge_codegen (cargo install flutter_rust_bridge_codegen).
set -euo pipefail
cd "$(dirname "$0")/.."
command -v flutter >/dev/null || { echo "Falta Flutter: https://docs.flutter.dev/get-started/install"; exit 1; }
command -v flutter_rust_bridge_codegen >/dev/null || { echo "Falta el generador: cargo install flutter_rust_bridge_codegen"; exit 1; }
if [ -e cuevas_app ]; then
  echo "Ya existe cuevas_app/. Bórrala si quieres empezar de cero."
  exit 1
fi

flutter_rust_bridge_codegen create cuevas_app
# Tu módulo de Rust, junto al ejemplo de la plantilla.
cp rust/src/api/cuevas.rs cuevas_app/rust/src/api/
printf 'pub mod cuevas;\npub mod simple;\n' > cuevas_app/rust/src/api/mod.rs
(cd cuevas_app/rust && cargo add anyhow)
# La pantalla y la prueba de integración. La prueba de la plantilla buscaba el greet de ejemplo: ya no aplica.
cp flutter/lib/main.dart cuevas_app/lib/main.dart
rm -f cuevas_app/integration_test/simple_test.dart
cp flutter/integration_test/cuevas_test.dart cuevas_app/integration_test/
(cd cuevas_app && flutter_rust_bridge_codegen generate)

echo
echo "Listo. Para verla:  cd cuevas_app && flutter run"
echo "Prueba del puente:  cd cuevas_app && flutter test integration_test/cuevas_test.dart -d macos"
