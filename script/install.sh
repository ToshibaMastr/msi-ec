#!/bin/bash

INSTALL_DIR="/usr/bin"
SERVICE_DIR="/usr/lib/systemd/system"
DBUS_DIR="/usr/share/dbus-1/system.d"

SERVICE_NAME="MsiEc.service"
DBUS_CONF="by.toshibam.MsiEc.conf"
BINARY_NAME="msi-ec"

cp "target/release/$BINARY_NAME" "$INSTALL_DIR/"
cp "dbus/systemd/$SERVICE_NAME" "$SERVICE_DIR/"
cp "dbus/system.d/$DBUS_CONF" "$DBUS_DIR/"

chmod 755 "$INSTALL_DIR/$BINARY_NAME"
chmod 644 "$SERVICE_DIR/$SERVICE_NAME" "$DBUS_DIR/$DBUS_CONF"

systemctl daemon-reload && systemctl reload dbus

systemctl enable --now "$SERVICE_NAME"
