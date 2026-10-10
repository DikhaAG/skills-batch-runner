# 🚀 Skill Batch Runner (`sbr`)

> **Utilitas CLI Modern Berbasis Rust** untuk menginstal daftar *agentic skills* (`npx skills add ...`) secara otomatis, cepat, deterministik, dan tanpa interupsi interaktif (*zero-prompt*).

---

## 📑 Daftar Isi
1. [Fitur Unggulan](#-fitur-unggulan)
2. [Prasyarat Sistem](#-prasyarat-sistem)
3. [Panduan Cepat (Quick Start)](#-panduan-cepat-quick-start)
4. [Format File Masukan (`skills.txt`)](#-format-file-masukan-skillstxt)
5. [Daftar Perintah & Argumen CLI](#-daftar-perintah--argumen-cli)
6. [Skenario Penggunaan Nyata](#-skenario-penggunaan-nyata)
7. [Penanganan Kendala (Troubleshooting)](#-penanganan-kendala-troubleshooting)
8. [Kompilasi Standalone Binary](#-kompilasi-standalone-binary)

---

## ✨ Fitur Unggulan

* ⚡ **Auto-Approve Non-Interaktif (`-y`)**: Setiap subproses secara otomatis menyertakan flag `-y` di akhir perintah. Anda tidak perlu lagi menekan tombol `y` / `Enter` berulang kali di terminal!
* 📁 **Isolasi Direktori Kerja**: Eksekusi instalasi langsung ditargetkan ke direktori proyek pilihan Anda (`--target <DIR>`).
* 🛡️ **Pemeriksaan Pra-Eksekusi (Pre-Flight Checks)**: Memastikan ketersediaan Node.js / `npx`, eksistensi file input, dan hak akses tulis direktori sebelum eksekusi dimulai.
* 🧹 **Deduplikasi Cerdas**: Entri yang sama (meskipun ditulis dengan format berbeda) otomatis disaring sehingga hanya dieksekusi 1 kali tanpa mengacak urutan aslinya.
* 🎨 **Visual Progress & Pelaporan Rapi**: Dilengkapi terminal spinner / progress bar serta ringkasan akhir berstatus warna (Sukses, Gagal, Dilewati).
* 🔒 **Bebas Injeksi Shell**: Pemrosesan argumen dilakukan langsung ke biner `npx` tanpa layer `bash`/`sh` untuk keamanan maksimal.

---

## 💻 Prasyarat Sistem

Sebelum menjalankan program, pastikan sistem Anda memiliki:
1. **Node.js & npm / npx**: Terpasang di `$PATH` (uji dengan perintah: `npx --version`).
2. **Rust & Cargo**: Toolchain Rust terpasang (uji dengan perintah: `cargo --version`).

---

## ⚡ Panduan Cepat (Quick Start)

### 1. Masuk ke Direktori Proyek
```bash
cd /Users/a69/Desktop/skills-batch-runner
```

### 2. Jalankan Mode Simulasi (Dry-Run)
Selalu disarankan melakukan simulasi terlebih dahulu guna memastikan daftar perintah Anda valid:
```bash
cargo run -- -f skills.txt -t /path/ke/proyek/tujuan -d
```
> *(Ganti `/path/ke/proyek/tujuan` dengan path folder proyek tempat Anda ingin menginstal skill)*.

### 3. Eksekusi Penuh
Jika hasil simulasi sudah sesuai, jalankan tanpa `-d`:
```bash
cargo run -- -f skills.txt -t /path/ke/proyek/tujuan
```
Duduk santai dan biarkan `sbr` menginstal seluruh skill secara otomatis! 🎉

---

## 📝 Format File Masukan (`skills.txt`)

Program ini sangat fleksibel dan mendukung 2 format baris masukan. Anda cukup menuliskan **satu perintah per baris** (dipisahkan dengan tombol *Enter*).

### Format 1: Format Mentah `npx` (Seperti yang Anda salin dari dokumentasi)
```text
npx skills add https://github.com/getcargohq/cargo-skills --skill cargo-workspace-management
npx skills add https://github.com/aj-geddes/useful-ai-prompts --skill 'Time Series Analysis'
```
*Mendukung nama skill dengan spasi menggunakan kutip tunggal (`'...'`) atau ganda (`"..."`).*

### Format 2: Format Ringkas (URL Spasi Nama-Skill)
```text
# Baris komentar diawali tanda pagar (#) otomatis diabaikan
https://github.com/getcargohq/cargo-skills cargo-workspace-management
https://github.com/svssdeva/agentic-skills cargo-workspace-scaffold
```

> 💡 **Keleluasaan:**
> - Boleh ada baris kosong di antara baris perintah.
> - Boleh mencampur Format 1 dan Format 2 dalam satu berkas.
> - Duplikasi entri akan otomatis disaring.

---

## 🛠️ Daftar Perintah & Argumen CLI

### Sintaksis:
```bash
cargo run -- [OPTIONS] --file <PATH> --target <DIR>
```

| Bendera Singkat | Bendera Lengkap | Tipe | Wajib? | Penjelasan |
| :--- | :--- | :---: | :---: | :--- |
| `-f` | `--file <PATH>` | `Path` | **Ya** | Jalur ke file `.txt` berisi daftar skill (misal: `skills.txt`). |
| `-t` | `--target <DIR>` | `Path` | **Ya** | Jalur direktori proyek tempat skill akan dipasang. |
| `-d` | `--dry-run` | `bool` | Tidak | **Mode Simulasi**: Menampilkan daftar tugas tanpa menjalankan instalasi. |
| `-v` | `--verbose` | `bool` | Tidak | Menampilkan output mentah *stdout/stderr* dari `npx` ke terminal. |
| | `--continue-on-error` | `bool` | Tidak | Jika satu skill gagal diunduh, sistem tetap melanjutkan sisa skill lainnya. |
| `-c` | `--concurrency <NUM>` | `usize` | Tidak | Batas proses konkuren (default: `1` untuk mencegah konflik lockfile). |

---

## 🎯 Skenario Penggunaan Nyata

### Skenario A: Menginstal ke Direktori Proyek Saat Ini (`.`)
Jika target instalasi adalah folder yang sedang Anda buka:
```bash
cargo run -- -f skills.txt -t .
```

### Skenario B: Menginstal ke Direktori Proyek Lain
Misal proyek Anda ada di folder `~/Desktop/my-web-app`:
```bash
cargo run -- -f skills.txt -t ~/Desktop/my-web-app
```

### Skenario C: Menjalankan dengan Toleransi Error (Tetap Lanjut jika Ada Skill yang Rusak)
Jika ada kemungkinan salah satu repo GitHub skill sedang offline atau dihapus:
```bash
cargo run -- -f skills.txt -t ~/Desktop/my-web-app --continue-on-error
```

### Skenario D: Melakukan Debugging Output
Jika ingin melihat pesan detail instalasi dari `npx` secara langsung:
```bash
cargo run -- -f skills.txt -t ~/Desktop/my-web-app -v
```

---

## 🔍 Penanganan Kendala (Troubleshooting)

| Pesan Error | Penyebab | Solusi |
| :--- | :--- | :--- |
| `Biner npx tidak ditemukan pada $PATH sistem` | Node.js belum terpasang atau belum terbaca di terminal. | Pasang Node.js atau jalankan `npm install -g npx`. |
| `File input tidak ditemukan: ...` | Jalur berkas `-f` salah ketik atau belum ada. | Pastikan berkas `skills.txt` berada di folder yang sesuai. |
| `Direktori target tidak ditemukan: ...` | Folder target pada opsi `-t` belum dibuat. | Buat direktori tujuan terlebih dahulu (`mkdir -p <target>`). |
| `Tidak memiliki izin tulis pada direktori target` | Direktori tujuan dikunci oleh sistem operasi / *read-only*. | Periksa hak akses (`chmod +w <target>`) atau pilih direktori pengguna lain. |

---

## 📦 Kompilasi Standalone Binary

Jika Anda ingin menjadikan `sbr` aplikasi biner portabel tanpa perlu mengetik `cargo run --`:

1. **Jalankan kompilasi release:**
   ```bash
   cargo build --release
   ```
2. **Jalankan binary secara langsung:**
   ```bash
   ./target/release/sbr -f skills.txt -t /path/ke/proyek
   ```
3. **(Opsional) Pasang secara global ke sistem:**
   ```bash
   sudo cp ./target/release/sbr /usr/local/bin/sbr
   ```
   Setelah itu, Anda bisa memanggilnya cukup dengan perintah `sbr` dari terminal mana saja:
   ```bash
   sbr -f skills.txt -t /path/ke/proyek
   ```

---

*Dibuat dengan ❤️ menggunakan Rust untuk produktivitas instalasi tools berbasis AI agent.*
