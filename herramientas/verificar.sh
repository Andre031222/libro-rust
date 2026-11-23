#!/usr/bin/env bash
# Verifica el manuscrito del libro. Uso:
#   herramientas/verificar.sh [capitulo.tex ...]
# Sin argumentos revisa todos los capítulos. Sale con 0 si todo pasa.
set -u
RAIZ="$(cd "$(dirname "$0")/.." && pwd)"
MS="$RAIZ/01-manuscrito"
ANCHO=72
problemas=0
falla() { echo "✗ $*"; problemas=$((problemas + 1)); }

if [ $# -eq 0 ]; then set -- "$MS"/capitulos/*.tex; fi

# 1. Rutas de \codigo y \terminal; 2. ancho de los .rs citados.
for tex in "$@"; do
  grep -oE '\\(codigo|terminal)(\[[^]]*\])?\{[^}]+\}' "$tex" |
    sed -E 's/.*\{([^}]+)\}/\1/' | sort -u |
    while read -r ruta; do
      if [ ! -f "$RAIZ/$ruta" ]; then
        echo "✗ $(basename "$tex"): no existe $ruta"
      elif [[ "$ruta" == *.rs ]]; then
        awk -v n="$ANCHO" -v r="$ruta" \
          'length($0) > n { printf "✗ %s:%d: %d caracteres (máx. %d)\n", r, NR, length($0), n }' \
          "$RAIZ/$ruta"
      fi
    done
done > "$RAIZ/.verificar.tmp"
if [ -s "$RAIZ/.verificar.tmp" ]; then
  cat "$RAIZ/.verificar.tmp"
  problemas=$((problemas + $(wc -l < "$RAIZ/.verificar.tmp")))
fi
rm -f "$RAIZ/.verificar.tmp"

# 3. Registro de LaTeX: referencias sin resolver y desbordes > 5 pt.
LOG="$MS/main.log"
if [ -f "$LOG" ]; then
  grep -E "undefined" "$LOG" | grep -v "There were undefined" |
    while read -r l; do echo "✗ main.log: $l"; done > "$RAIZ/.verificar.tmp"
  grep -oE "Overfull \\\\hbox \(([0-9.]+)pt too wide\).*" "$LOG" |
    awk '{ gsub(/[()pt]/, "", $3); if ($3 + 0 > 5) print "✗ main.log: " $0 }' \
    >> "$RAIZ/.verificar.tmp"
  if [ -s "$RAIZ/.verificar.tmp" ]; then
    cat "$RAIZ/.verificar.tmp"
    problemas=$((problemas + $(wc -l < "$RAIZ/.verificar.tmp")))
  fi
  rm -f "$RAIZ/.verificar.tmp"
else
  falla "no hay main.log: compile antes con latexmk"
fi

# 4. Figuras: fuente TikZ propia o entrada en CREDITOS.md.
FIG="$RAIZ/figuras"
if [ -d "$FIG" ]; then
  for f in "$FIG"/*.{pdf,svg,png,jpg}; do
    [ -e "$f" ] || continue
    base="$(basename "${f%.*}")"
    if [ ! -f "$FIG/src/$base.tex" ] &&
       ! grep -q "$base" "$FIG/CREDITOS.md" 2>/dev/null; then
      falla "figura sin fuente ni crédito: $(basename "$f")"
    fi
  done
fi

if [ "$problemas" -eq 0 ]; then echo "✓ verificación sin problemas"; exit 0; fi
echo "$problemas problema(s)"; exit 1
