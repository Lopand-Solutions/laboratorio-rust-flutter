# Laboratorio: un módulo de Rust llamado desde Flutter

Repositorio del laboratorio de [Lopand Academy](https://academy.lopandsolutions.com/tutoriales/laboratorio-rust-flutter).
Un generador de cuevas para un juego 2D, escrito en Rust y llamado desde una app de Flutter con
[flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) v2. Con pruebas, un comprobador y el reto
«rómpelo y arréglalo».

## Ramas

- **`inicio`**: el crate con las firmas, los comentarios y las pruebas; los cuerpos de las funciones son `todo!()`.
  Desde aquí escribes el generador siguiendo la lección.
- **`main`**: la solución, tal como está en la lección.

```bash
git clone -b inicio https://github.com/Lopand-Solutions/laboratorio-rust-flutter.git
cd laboratorio-rust-flutter
```

## Qué hay

| Ruta | Qué es |
| --- | --- |
| `rust/src/api/cuevas.rs` | El generador y sus pruebas. Es lo que escribes |
| `flutter/lib/main.dart` | La pantalla que dibuja la cueva |
| `flutter/integration_test/cuevas_test.dart` | La prueba de que el puente funciona en un dispositivo |
| `tool/comprobar_laboratorio.sh` | Revisa la parte de Rust. No necesita Flutter |
| `tool/crear_app.sh` | Crea `cuevas_app/` con `flutter_rust_bridge_codegen create` y le copia tus archivos |

## Córrelo

Primero la lógica, sin Flutter:

```bash
cd rust
cargo test
cd ..
./tool/comprobar_laboratorio.sh
```

Con las pruebas en verde, la app (necesitas Flutter y `cargo install flutter_rust_bridge_codegen`):

```bash
./tool/crear_app.sh
cd cuevas_app
flutter run
```

Si cambias `rust/src/api/cuevas.rs` después de crear la app, cópialo otra vez a `cuevas_app/rust/src/api/` y corre
`flutter_rust_bridge_codegen generate` dentro de `cuevas_app/`.

## Cómo saber si quedó

```bash
./tool/comprobar_laboratorio.sh
```

Corre `cargo test`, `cargo clippy -- -D warnings` y `cargo fmt --check`, y rompe tu código en una copia (los tres
retos de la lección) para ver que tus pruebas lo detectan. Si ya creaste `cuevas_app/`, revisa que use tu
`cuevas.rs` y que el puente de Dart esté generado.

## Requisitos

Rust 1.85 o superior (edición 2024). Para la app: Flutter y `flutter_rust_bridge_codegen` 2.13.0, la versión con la
que se probó la lección.

Código libre para usar y adaptar (licencia MIT).
