#!/bin/sh
# Despliega `mediciones` en un servidor Linux con systemd.
# Uso: ./desplegar.sh usuario@servidor [objetivo]
# Requiere SSH con clave y permiso de sudo en el servidor.
# Para compilar con zig: SUBCOMANDO=zigbuild ./desplegar.sh ...
set -eu

DESTINO=${1:?"uso: desplegar.sh usuario@servidor [objetivo]"}
OBJETIVO=${2:-x86_64-unknown-linux-gnu}
SUBCOMANDO=${SUBCOMANDO:-build}
RAIZ=$(cd "$(dirname "$0")/../../.." && pwd)
BINARIO="$RAIZ/target/$OBJETIVO/release/mediciones"
UNIDAD="$RAIZ/caps/cap18-proyecto/despliegue/mediciones.service"

# 1. Compilar para Linux.
cargo "$SUBCOMANDO" --release -p cap18-proyecto \
    --bin mediciones --target "$OBJETIVO"

# 2. Copiar el binario y la unidad al directorio personal.
scp "$BINARIO" "$UNIDAD" "$DESTINO:"

# 3. Instalar, recargar systemd y reiniciar el servicio.
ssh "$DESTINO" '
  set -e
  sudo install -m 755 mediciones /usr/local/bin/mediciones
  sudo install -m 644 mediciones.service /etc/systemd/system/
  rm mediciones mediciones.service
  sudo systemctl daemon-reload
  sudo systemctl enable mediciones.service
  sudo systemctl restart mediciones.service
  systemctl --no-pager status mediciones.service
'
