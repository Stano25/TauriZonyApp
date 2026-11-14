# Prostredie pre lokalny build Zony App na NixOS (Linux).
#
# Pouzitie:
#   nix-shell                                            # interaktivny shell
#   nix-shell --run "bun run tauri build --bundles deb"  # binarka + .deb balik pre Ubuntu
#   nix-shell --run "bun run build"                      # len frontend
#
# Vysledok:
#   binarka: src-tauri/target/release/zony-app
#   .deb:    src-tauri/target/release/bundle/deb/zony-app_<verzia>_amd64.deb
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    bun

    # Tauri (Linux) prekladove zavislosti
    pkg-config
    glib            # >= 2.70
    gtk3
    webkitgtk_4_1   # Tauri v2 vyzaduje webkit2gtk 4.1 (nie 4.0)
    libsoup_3
    gdk-pixbuf
    pango
    cairo
    gobject-introspection
  ];
}
