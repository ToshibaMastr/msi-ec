#!/bin/bash

INSTALL_DIR="/usr/bin"
SERVICE_DIR="/usr/lib/systemd/system"
DBUS_DIR="/usr/share/dbus-1/system.d"

SERVICE_NAME="MsiEc.service"
DBUS_CONF="by.toshibam.MsiEc.conf"
BINARY_NAME="msi-ec"

systemctl disable --now "$SERVICE_NAME"

rm -f "$SERVICE_DIR/$SERVICE_NAME" "$DBUS_DIR/$DBUS_CONF" "$INSTALL_DIR/$BINARY_NAME"

systemctl daemon-reload && systemctl reload dbus
