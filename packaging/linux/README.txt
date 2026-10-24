QR Code Maker for Linux x86_64
=============================

This portable desktop build targets 64-bit Linux systems, including Omarchy
and WSL2 with WSLg. It is a native Linux GUI application; it does not run
inside Windows or require Python.

Quick start
-----------

1. Extract this archive.
2. Run ./install.sh to install the app under ~/.local/opt and add it to your
   desktop application menu, or run ./QRCodeMaker directly from this folder.

The host needs a working X11 or Wayland desktop session and the usual Linux
runtime libraries for those display systems. The Save dialog uses the XDG
Desktop Portal; install and start a portal backend if your desktop does not
already provide one. WSL users need WSL2 with WSLg enabled.

To remove an installed copy, run ./uninstall.sh from this extracted folder.
