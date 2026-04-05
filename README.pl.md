<h1 align="center">shrimp-checker</h1>

<p align="center">
  <a href="https://github.com/wielorzeczownik/shrimp-checker/actions/workflows/release.yml"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/shrimp-checker/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/shrimp-checker/release.yml?branch=main&style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/shrimp-checker/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950" alt="release"/></picture></a> <a href="https://github.com/wielorzeczownik/shrimp-checker/releases/latest"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/shrimp-checker?style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/shrimp-checker?style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/v/release/wielorzeczownik/shrimp-checker?style=flat-square&labelColor=2d333b&color=3fb950" alt="Latest Release"/></picture></a> <a href="https://github.com/wielorzeczownik/shrimp-checker/blob/main/LICENSE"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/License-MIT-3fb950?style=flat-square&labelColor=2d333b"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/badge/License-MIT-2ea043?style=flat-square"/><img src="https://img.shields.io/badge/License-MIT-3fb950?style=flat-square&labelColor=2d333b" alt="License: MIT"/></picture></a>
  <br/>
  <img src="https://img.shields.io/badge/Rust-B7410E?style=flat-square&logo=rust&logoColor=white" alt="Rust"/>
  <img src="https://img.shields.io/badge/Iced-4D9DE0?style=flat-square&logo=iced&logoColor=white" alt="Iced"/>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/wielorzeczownik/shrimp-checker/main/assets/logo.png" alt="shrimp-checker logo" width="200" />
</p>

<p align="center">🇵🇱 Polski | 🇬🇧 <a href="README.md">English</a></p>

Lekka aplikacja desktopowa z GUI, która sprawdza, czy jesteś krewetką — napisana w Rust z użyciem [Iced](https://github.com/iced-rs/iced). Inspirowana memową aplikacją, którą kiedyś widziałem na Twitterze i nie mogłem jej znaleźć, więc zbudowałem własną wersję.

> Źródło emoji krewetki: <a href="https://emojipedia.org/joypixels/6.0/shrimp">JoyPixels 6.0</a>.

## Uruchomienie z gotowych plików binarnych

Każde wydanie zawiera gotowe archiwa dla systemu Linux, macOS i Windows.
Najnowsze wydanie: [GitHub Releases](https://github.com/wielorzeczownik/shrimp-checker/releases/latest)

1. Pobierz archiwum dla swojej platformy ze strony wydania.
2. Rozpakuj je.
3. Uruchom program.

Przykład (Linux/macOS):

```bash
./shrimp-checker
```

Przykład (Windows PowerShell):

```powershell
.\shrimp-checker.exe
```

Pobierz najnowsze archiwum dla swojej platformy:

**Linux (glibc — wymaga glibc 2.35+):**
- [shrimp-checker-x86_64-unknown-linux-gnu.tar.gz](https://github.com/wielorzeczownik/shrimp-checker/releases/latest/download/shrimp-checker-x86_64-unknown-linux-gnu.tar.gz) – Linux (Intel/AMD 64-bit)
- [shrimp-checker-aarch64-unknown-linux-gnu.tar.gz](https://github.com/wielorzeczownik/shrimp-checker/releases/latest/download/shrimp-checker-aarch64-unknown-linux-gnu.tar.gz) – Linux (ARM64, np. Raspberry Pi 64-bit)

**macOS:**
- [shrimp-checker-x86_64-apple-darwin.tar.gz](https://github.com/wielorzeczownik/shrimp-checker/releases/latest/download/shrimp-checker-x86_64-apple-darwin.tar.gz) – macOS na Intel
- [shrimp-checker-aarch64-apple-darwin.tar.gz](https://github.com/wielorzeczownik/shrimp-checker/releases/latest/download/shrimp-checker-aarch64-apple-darwin.tar.gz) – macOS na Apple Silicon (M1/M2/M3/M4)

**Windows:**
- [shrimp-checker-x86_64-pc-windows-msvc.zip](https://github.com/wielorzeczownik/shrimp-checker/releases/latest/download/shrimp-checker-x86_64-pc-windows-msvc.zip) – Windows 64-bit (x86_64)
- [shrimp-checker-aarch64-pc-windows-msvc.zip](https://github.com/wielorzeczownik/shrimp-checker/releases/latest/download/shrimp-checker-aarch64-pc-windows-msvc.zip) – Windows ARM64
- [shrimp-checker-i686-pc-windows-msvc.zip](https://github.com/wielorzeczownik/shrimp-checker/releases/latest/download/shrimp-checker-i686-pc-windows-msvc.zip) – Windows 32-bit (x86)

## Budowanie ze źródeł

Wymaga [Rust](https://rustup.rs/) w wersji stable oraz, na Linuksie, nagłówków ALSA:

```bash
# Tylko Linux
sudo apt-get install -y libasound2-dev pkg-config
```

```bash
cargo build --release
./target/release/shrimp-checker
```

## Obsługiwane platformy

- **Linux** — testowany i traktowany jako główna platforma.
- **macOS** — powinien działać; binaria publikowane dla Intel i Apple Silicon.
- **Windows** — binaria publikowane dla x86_64, x86 i ARM64; środowisko uruchomieniowe nie jest regularnie weryfikowane na dedykowanym systemie Windows.
