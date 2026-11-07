#!/usr/bin/env bash
# Comprueba el laboratorio «Un módulo de Rust llamado desde Flutter». Uso: ./tool/comprobar_laboratorio.sh
# Revisa solo la parte de Rust (no necesita Flutter): pruebas, clippy, formato, y rompe tu código en una copia
# para ver que tus pruebas lo detectan. No toca tus archivos. Funciona con el bash 3.2 de macOS.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1
RAIZ=$(pwd)
ARCHIVO=rust/src/api/cuevas.rs
bien=0
mal=0
ok() { printf '  ✓ %s\n' "$1"; bien=$((bien + 1)); }
no() { printf '  ✗ %s\n' "$1"; mal=$((mal + 1)); }

command -v cargo >/dev/null || { echo "No encuentro cargo. Instala Rust con rustup: https://rustup.rs"; exit 1; }

echo "1. Tus pruebas"
pruebas_ok=0
if salida=$(cd rust && cargo test --quiet 2>&1); then
  pruebas_ok=1
  ok "cargo test en verde ($(printf '%s\n' "$salida" | grep -m1 'test result' | sed 's/test result: //'))"
else
  no "cargo test en rojo: arréglalo antes de seguir"
  printf '%s\n' "$salida" | grep -E "panicked at|not yet implemented|FAILED|error\[" | head -8 | sed 's/^/      /'
fi

echo "2. Clippy y formato"
if (cd rust && cargo clippy --quiet --all-targets -- -D warnings >/dev/null 2>&1); then
  ok "cargo clippy sin advertencias"
else
  no "cargo clippy encontró advertencias: corre (cd rust && cargo clippy --all-targets) para verlas"
fi
if (cd rust && cargo fmt --check >/dev/null 2>&1); then
  ok "cargo fmt --check: el formato está bien"
else
  no "falta formato: corre (cd rust && cargo fmt)"
fi

echo "3. Tus pruebas detectan los errores de los retos (en una copia)"
if [ "$pruebas_ok" -eq 0 ]; then
  no "primero pon tus pruebas en verde: romper código que ya falla no dice nada"
fi
# Cada reto: la línea original, la línea rota y qué rompe. Los mismos tres de la lección.
romper() {
  local original=$1 roto=$2 que=$3 tmp codigo
  [ "$pruebas_ok" -eq 1 ] || return
  codigo=$(cat "$ARCHIVO")
  case "$codigo" in
    *"$original"*) ;;
    *) no "no encuentro en $ARCHIVO «$original»: ¿la cambiaste? ($que)"; return ;;
  esac
  tmp=$(mktemp -d)
  mkdir -p "$tmp/rust"
  cp -R rust/Cargo.toml rust/src "$tmp/rust/"
  [ -f rust/Cargo.lock ] && cp rust/Cargo.lock "$tmp/rust/"
  printf '%s\n' "${codigo/"$original"/"$roto"}" > "$tmp/$ARCHIVO"
  # Reusa lo ya compilado en rust/target para no compilar todo otra vez.
  if (cd "$tmp/rust" && CARGO_TARGET_DIR="$RAIZ/rust/target/comprobador" cargo test --quiet >/dev/null 2>&1); then
    no "NO detectan $que: falta una prueba"
  else
    ok "detectan $que"
  fi
  rm -rf "$tmp"
}
romper 'x == 0 || y == 0 || x == w - 1 || y == h - 1' 'x == 0 || y == 0' 'un borde sin pared (reto 1)'
romper '    dejar_la_region_mas_grande(&mut celdas, w, h);' '    // dejar_la_region_mas_grande(&mut celdas, w, h);' 'zonas de suelo sin conectar (reto 2)'
romper 'let mut azar = Azar(semilla);' 'let mut azar = Azar(0);' 'que se ignore la semilla (reto 3)'

if [ -d cuevas_app ]; then
  echo "4. Tu app de Flutter (cuevas_app/)"
  if cmp -s "$ARCHIVO" cuevas_app/rust/src/api/cuevas.rs; then
    ok "cuevas_app usa tu cuevas.rs"
  else
    no "cuevas_app/rust/src/api/cuevas.rs es distinto del tuyo: vuelve a copiarlo y corre flutter_rust_bridge_codegen generate"
  fi
  if [ -f cuevas_app/lib/src/rust/api/cuevas.dart ]; then
    ok "el puente de Dart está generado (lib/src/rust/api/cuevas.dart)"
  else
    no "falta el código de Dart generado: (cd cuevas_app && flutter_rust_bridge_codegen generate)"
  fi
fi

echo
if [ "$mal" -eq 0 ]; then
  echo "Laboratorio completo: $bien de $bien."
  [ -d cuevas_app ] || echo "Para verlo en Flutter: ./tool/crear_app.sh y después cd cuevas_app && flutter run"
  exit 0
fi
echo "Falta$([ "$mal" -eq 1 ] || echo n) $mal punto$([ "$mal" -eq 1 ] || echo s) ($bien bien)."
exit 1
