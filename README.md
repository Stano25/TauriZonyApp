# Zóny App

Desktopová utilita na spracovanie meraní pokrytia GSM/LTE/5G sietí. Z nameraných dát (CSV exporty z meracích zariadení) vygeneruje zóny so štvorcovou mriežkou 100 × 100 m, meracie body a dokáže priamo vyplniť protokol vo formáte Excel (.xlsx).

Cieľová platforma je Windows 11, no vďaka Tauri funguje aj na Linuxe.

## Čo aplikácia vie

- **Zóny** – pre každý štvorec mriežky 100 × 100 m vyberie najsilnejšieho operátora (GSM podľa sily signálu, LTE/5G podľa RSRP). Voliteľne je možné nastaviť minimálny SINR prah – ak ho najlepší operátor nespĺňa, použije sa operátor s najlepším RSRP, ktorý prahu vyhovuje.
- **Body** – vygeneruje meracie body z CSV súborov, s možnosťou filtrovania podľa operátorov a doplnenia chýbajúcich operátorov.
- **Protokoly** – automaticky vyplní Excel protokol (LTE, 5G, prípadne DB variant) hodnotami z vygenerovaných zón a zadanými parametrami merania.
- Nastavenia sa ukladajú do `config.json` v používateľskom adresári (`%APPDATA%\App-zony-100m` na Windows), takže prežijú reštart aplikácie.

## Používanie

V ľavom menu si vyber sekciu (Zóny, Body, Protokoly) a technológiu (GSM/LTE/5G). Zvol vstupné súbory, prípadne priečinok s filtrami operátorov, nastav možnosti a klikni na **Generovať**. Priebeh a prípadné chyby sa vypisujú do terminálu v spodnej časti okna.

## Vývojársky setup

Potrebné je [bun](https://bun.sh), [Rust](https://rustup.rs) a podľa platformy aj systémové knižnice pre Tauri (na Windows stačí Visual Studio Build Tools s workloadom *Desktop development with C++*, na Linuxe GTK/WebKitGTK balíčky podľa [dokumentácie Tauri](https://tauri.app/start/prerequisites/)).

```bash
bun install

# vývoj s hot reloadom
bun run tauri dev

# produkčný build (inštalátory v src-tauri/target/release/bundle/)
bun run tauri build

# kontrola typov
bun run check
```

Ak meníš aplikáciu, pred committom sa oplatí pustiť `bun run check`, lebo Rust kompilátor frontend nekontroluje.

## Štruktúra projektu

```
src/            frontend (SvelteKit) – stránky, komponenty, globálne štýly
src-tauri/      backend (Rust) – Tauri commands, spracovanie CSV/Excel, ikony
static/         statické súbory (favicon)
```

Backend sa delí na manažéry: `data_manager` (hlavná logika generovania), `excel_manager` (práca s .xlsx cez umya-spreadsheet/calamine), `json_manager` (ukládanie nastavení) a `position_manager` (prepočty UTM súradníc).

## Použité knižnice

Bez týchto projektov by to nikdy nevzniklo:

**Frontend**
- [Svelte 5 + SvelteKit](https://kit.svelte.dev/) – UI framework a router
- [Vite](https://vitejs.dev/) – buildovací nástroj a dev server
- [TypeScript](https://www.typescriptlang.org/) – typy nad Svelte komponentmi
- [@tauri-apps/api](https://v2.tauri.app/reference/javascript/api/) a [plugin-dialog](https://v2.tauri.app/plugin/dialog/) – komunikácia s backendom a natívne dialógy

**Rust backend**
- [tauri](https://v2.tauri.app/) – samotný cross-platform framework (WebView2)
- [serde](https://serde.rs/) + `serde_json` – serializácia dát a nastavení
- [csv](https://github.com/BurntSushi/rust-csv) – čítanie a zápis CSV
- [umya-spreadsheet](https://github.com/MathNya/umya-spreadsheet) – zápis do Excel súborov
- [calamine](https://github.com/tafia/calamine) – čítanie Excel súborov
- [geo](https://github.com/georust/geo) – geodetické výpočty vzdialeností
- [regex](https://github.com/rust-lang/regex) – parsovanie hlavičiek a filtrov
- [dirs](https://github.com/dirs-dev/dirs-rs) – systémový config adresár
- [tokio](https://tokio.rs/) – async runtime

## Licencia

Projekt je licencovaný pod [MIT licenciou](LICENSE).
