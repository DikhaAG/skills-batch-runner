Dokumen Kebutuhan Produk (PRD) ini mendefinisikan spesifikasi fungsional, arsitektur teknis, mitigasi kegagalan, dan struktur implementasi utilitas CLI berbasis Rust untuk menjalankan instalasi batch *skills* ke direktori target.

---

# Product Requirement Document (PRD): Skill Batch Runner (`sbr`)

## 1. Ikhtisar & Tujuan Sistem

Alat bantu baris perintah (*CLI utility*) berbasis Rust yang membaca daftar target instalasi *skills* dari file teks, memvalidasi dependensi lingkungan, dan mengeksekusi subproses instalasi secara terisolasi pada direktori kerja yang ditentukan.

* **Masalah:** Menjalankan perintah `npx skills add ...` secara manual berulang-ulang pada repositori proyek memakan waktu, rentan kesalahan pengetikan, dan sulit direproduksi antar-anggota tim.
* **Solusi:** Eksekutor batch deterministik yang menangani deduplikasi input, validasi sintaksis, penargetan direktori kerja atomik, serta pelaporan status eksekusi secara terstruktur.

---

## 2. Antarmuka Baris Perintah (CLI Interface)

### 2.1 Sintaksis Perintah

```bash
sbr [OPTIONS] --file <PATH> --target <DIR>

```

### 2.2 Argumen & Bendera (Flags)

| Argumen / Flag | Tipe | Wajib | Deskripsi |
| --- | --- | --- | --- |
| `-f, --file` | `PathBuf` | Ya | Jalur ke file `.txt` berisi daftar instruksi atau URL *skill*. |
| `-t, --target` | `PathBuf` | Ya | Jalur direktori proyek tujuan instalasi. |
| `-d, --dry-run` | `bool` | Tidak | Memvalidasi dan menampilkan daftar tugas tanpa mengeksekusi subproses. |
| `-c, --concurrency` | `usize` | Tidak | Batas proses konkuren (default: `1` untuk mencegah konflik berkas kunci). |
| `--continue-on-error` | `bool` | Tidak | Tetap melanjutkan sisa entri jika salah satu instalasi gagal. |
| `-v, --verbose` | `bool` | Tidak | Menampilkan output *stdout* dan *stderr* mentah dari subproses `npx`. |

---

## 3. Spesifikasi Masukan File (`skills.txt`)

Parser mendukung dua variasi format masukan secara fleksibel:

1. **Format Perintah Mentah:**
```text
npx skills add https://github.com/getcargohq/cargo-skills --skill cargo-workspace-management
npx skills add https://github.com/svssdeva/agentic-skills --skill cargo-workspace-scaffold

```


2. **Format Pasangan Repo & Sub-Skill (Opsional/Direkomendasikan):**
```text
# Baris komentar diabaikan
https://github.com/getcargohq/cargo-skills cargo-workspace-management
https://github.com/svssdeva/agentic-skills cargo-workspace-scaffold

```



### Aturan Parsing

* Baris kosong dan baris yang diawali karakter `#` otomatis diabaikan.
* Spasi ganda dan karakter pemisah baris (*newline/CRLF*) dinormalisasi.
* Ekstraksi parameter dilakukan menggunakan *tokenization* langsung tanpa memanggil shell (`sh`/`bash`) demi mencegah celah injeksi perintah.
* **Deduplikasi Otomatis:** Jika entri repo dan nama skill yang sama muncul berulang, sistem hanya mengeksekusinya satu kali.

---

## 4. Alur Kerja & Arsitektur Eksekusi

```
[File .txt] + [--target Path]
         │
         ▼
[1. Pre-Flight Check]
  ├─ Validasi ketersediaan biner `npx` di $PATH sistem
  ├─ Validasi keberadaan direktori target (--target)
  └─ Validasi izin tulis (write permissions) pada direktori target
         │
         ▼
[2. Parser & Sanitisasi]
  ├─ Normalisasi baris
  ├─ Ekstraksi URL & argumen --skill
  └─ Deduplikasi tugas (HashSet/BTreeSet)
         │
         ▼
[3. Mesin Eksekusi Subproses]
  ├─ Set working directory: Command::current_dir(target_dir)
  ├─ Eksekusi terisolasi: std::process::Command::new("npx")
  │     args: ["skills", "add", <REPO>, "--skill", <NAME>, "-y"]
  └─ Penangkapan sinyal stdout, stderr, dan exit status code
         │
         ▼
[4. Pelaporan Status]
  └─ Ringkasan tabel: Sukses / Gagal / Dilewati (Skipped)

```

---

## 5. Analisis Risiko & Mitigasi Teknis

* **Prompt Interaktif (Blocking Execution):**
  * *Masalah:* Perintah `npx skills add` memunculkan prompt konfirmasi interaktif ke terminal yang dapat menghentikan (*hang/block*) proses batch otomatis tanpa input pengguna.
  * *Mitigasi:* Menambahkan flag `-y` (atau `--yes`) secara otomatis di akhir daftar argumen subproses (`["skills", "add", <REPO>, "--skill", <NAME>, "-y"]`) agar semua prompt konfirmasi disetujui secara otomatis (*auto-approve* / non-interaktif).


* **Risiko Race Condition / File Locking:**
* *Masalah:* Menjalankan banyak perintah `npx skills add` secara paralel pada satu folder proyek dapat merusak manifes dependensi atau konfigurasi lokal jika operasi menulis ke file yang sama.
* *Mitigasi:* Nilai *concurrency* default diatur secara sekuensial (`1`). Mode paralel (`--concurrency > 1`) memerlukan konfirmasi eksplisit dari pengguna.


* **Injeksi Perintah (*Arbitrary Command Injection*):**
* *Masalah:* Menggunakan `std::process::Command::new("sh").arg("-c").arg(line)` berbahaya jika file teks memuat perintah sembarang seperti `rm -rf` atau perantaian operator `&&`.
* *Mitigasi:* Parsing baris menjadi struktur data ketat: `SkillTask { repo_url: String, skill_name: String }`. Eksekusi dilakukan langsung ke biner `npx` tanpa layer shell.


* **Ketiadaan Runtime Node.js:**
* *Masalah:* Pengguna menjalankan CLI di sistem tanpa `npx` terpasang.
* *Mitigasi:* Pemeriksaan pra-eksekusi (*pre-flight check*) mencari biner `npx` menggunakan bantuan `which` sebelum membaca file input.



---

## 6. Desain Komponen & Pustaka Rust

* **`clap` (v4, fitur `derive`):** Parser argumen CLI dengan pesan bantuan standar industri.
* **`tokio` / `rayon`:** Pengatur konkurensi (jika opsi paralel diaktifkan) atau eksekusi berbasis *thread pool*.
* **`indicatif`:** Penampil status progres dan indikator pemrosesan terminal.
* **`anyhow` & `thiserror`:** Penanganan galat terstruktur untuk kegagalan parser, I/O, maupun subproses.
* **`url`:** Validasi format tautan repositori Git.

---

## 7. Rencana Struktur Modul Repositori

```text
src/
├── main.rs          # Titik masuk, inisialisasi CLI, dan integrasi komponen
├── cli.rs           # Definisi struktur argumen Clap
├── config.rs        # Validasi pre-flight (npx presence, target dir validation)
├── parser.rs        # Pembaca file teks, tokenisasi, dan deduplikasi tugas
├── runner.rs        # Orkestrator Command::new("npx") dan isolasi direktori
└── reporter.rs      # Format laporan terminal dan statistik hasil akhir

```