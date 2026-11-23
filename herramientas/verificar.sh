#!/usr/bin/env bash
# Verifica el manuscrito del libro. Uso:
#   herramientas/verificar.sh [capitulo.tex ...]
# Sin argumentos revisa todos los capítulos. Sale con 0 si todo pasa.
set -u
RAIZ="$(cd "$(dirname "$0")/.." && pwd)"
MS="$RAIZ/01-manuscrito"
ANCHO=66          # código: \small, Fira Mono 0.86, 133 mm
ANCHO_SALIDA=80   # salidas de terminal: \footnotesize
ANCHO_ERROR=150   # \compilador: 6,2 pt; >100 se parte dentro del recuadro (salida real de rustc)
problemas=0
falla() { echo "✗ $*"; problemas=$((problemas + 1)); }

if [ $# -eq 0 ]; then set -- "$MS"/capitulos/*.tex; fi

# 1. Rutas de \codigo y \terminal; 2. ancho de los .rs citados.
for tex in "$@"; do
  grep -oE '\\(codigo|terminal|compilador)(\[[^]]*\])?\{[^}]+\}' "$tex" |
    sed -E 's/^\\([a-z]+)(\[[^]]*\])?\{([^}]+)\}$/\1 \3/' | sort -u |
    while read -r macro ruta; do
      if [ ! -f "$RAIZ/$ruta" ]; then
        echo "✗ $(basename "$tex"): no existe $ruta"
        continue
      fi
      case "$macro" in
        codigo)     n="$ANCHO" ;;
        terminal)   n="$ANCHO_SALIDA" ;;
        compilador) n="$ANCHO_ERROR" ;;
      esac
      awk -v n="$n" -v r="$ruta" \
        'length($0) > n { printf "✗ %s:%d: %d caracteres (máx. %d)\n", r, NR, length($0), n }' \
        "$RAIZ/$ruta"
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
    awk '{ gsub(/[()pt]/, "", $3); if ($3 + 0 > 1) print "✗ main.log: " $0 }' \
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
