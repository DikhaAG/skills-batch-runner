# Task List: Skill Batch Runner (`sbr`)

Dokumen pelacakan tugas (*task tracker*) implementasi utilitas CLI berbasis Rust untuk menjalankan instalasi batch *skills* sesuai spesifikasi pada [PRD.md](file:///Users/a69/Desktop/skills-batch-runner/PRD.md).

---

## Legenda Status
- [ ] Belum dimulai
- [/] Sedang dikerjakan
- [x] Selesai

---

## Fase 1: Inisialisasi Proyek & Konfigurasi Dependensi
- [x] Inisialisasi Cargo binary project (`cargo init . --bin` / `Cargo.toml`).
- [x] Konfigurasi dependensi di `Cargo.toml`:
  - `clap` (v4, fitur `derive`) - CLI arguments parser.
  - `anyhow` & `thiserror` - Penanganan error terstruktur.
  - `indicatif` - Progress bar & status display di terminal.
  - `which` - Pre-flight check keberadaan binary `npx` di `$PATH`.
  - `url` - Validasi format URL repositori git.
  - `tokio` atau `rayon` / `indexmap` & `console` - Deduplikasi dan UI terminal.
- [x] Buat file konfigurasi `.gitignore` untuk build artifact Rust (`/target`).

---

## Fase 2: CLI Parsing & Interface (`src/cli.rs`)
- [x] Definisikan struct `Cli` menggunakan `clap::Parser`:
  - [x] `-f, --file <PATH>`: Jalur ke file list skill (`PathBuf`, wajib).
  - [x] `-t, --target <DIR>`: Direktori tujuan eksekusi (`PathBuf`, wajib).
  - [x] `-d, --dry-run`: Mode simulasi tanpa eksekusi subproses (`bool`, opsional).
  - [x] `-c, --concurrency <usize>`: Batas konkurensi (default: `1`, opsional).
  - [x] `--continue-on-error`: Flag melanjutkan eksekusi jika terjadi error (`bool`, opsional).
  - [x] `-v, --verbose`: Output mentah stdout/stderr subproses (`bool`, opsional).
- [x] Tulis unit test untuk validasi argumen CLI.

---

## Fase 3: Pre-Flight Check & Konfigurasi (`src/config.rs`)
- [x] Implementasikan validasi ketersediaan binary `npx` pada sistem (`which::which("npx")`).
- [x] Implementasikan validasi direktori target:
  - Cek keberadaan direktori (`target_dir.exists()`).
  - Cek tipe path adalah direktori (`target_dir.is_dir()`).
  - Cek izin tulis / write permission pada direktori target.
- [x] Kembalikan galat deskriptif jika salah satu validasi pra-eksekusi gagal.

---

## Fase 4: Parser File Input & Deduplikasi (`src/parser.rs`)
- [x] Definisikan struktur data tugas `SkillTask`:
  ```rust
  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  pub struct SkillTask {
      pub repo_url: String,
      pub skill_name: String,
  }
  ```
- [x] Implementasikan fungsi pembacaan file baris per baris:
  - Abaikan baris kosong dan baris komentar (`#`).
  - Normalisasi whitespace dan pemisah baris (CRLF/LF).
- [x] Dukung Format 1 (Perintah Mentah):
  - Ekstraksi URL repositori dan argumen `--skill <NAME>` dari string `npx skills add ...`.
- [x] Dukung Format 2 (Pasangan Repo & Skill):
  - Parsing string format `<REPO_URL> <SKILL_NAME>`.
- [x] Implementasikan deduplikasi otomatis tugas menggunakan `IndexSet` untuk mempertahankan urutan deterministik.
- [x] Validasi sintaksis URL repo menggunakan pustaka `url`.
- [x] Tulis unit test untuk berbagai skenario input (format campuran, baris komentar, spasi acak, duplikat).

---

## Fase 5: Runner Subproses & Penanganan Prompt Non-Interaktif (`src/runner.rs`)
- [x] Implementasikan fungsi eksekutor `SkillTask`:
  - Set working directory target: `Command::current_dir(target_dir)`.
  - Bangun perintah subproses terisolasi: `Command::new("npx")`.
  - Pasang argumen: `["skills", "add", &task.repo_url, "--skill", &task.skill_name, "-y"]`.
  - Konfigurasi penanganan stdout & stderr (sembunyikan jika tidak `--verbose`, tampilkan jika `--verbose`).
- [x] Implementasikan logika timeout & penangkapan exit status code.
- [x] Implementasikan mode `--dry-run`:
  - Cetak perintah yang akan dijalankan ke terminal tanpa memanggil subproses.
- [x] Implementasikan mekanisme eksekusi berurutan (default: sekuensial `1`).
- [x] Implementasikan penanganan `--continue-on-error`:
  - Jika `false`: Hentikan batch pada kegagalan pertama.
  - Jika `true`: Catat kegagalan dan lanjutkan ke entri berikutnya.

---

## Fase 6: Reporter & Tampilan Terminal (`src/reporter.rs`)
- [x] Integrasikan `indicatif` untuk progress spinner / bar selama eksekusi.
- [x] Definisikan struktur pelaporan hasil:
  ```rust
  pub struct ExecutionReport {
      pub total: usize,
      pub succeeded: Vec<SkillTask>,
      pub failed: Vec<(SkillTask, String)>,
      pub skipped: Vec<SkillTask>,
  }
  ```
- [x] Tampilkan ringkasan hasil eksekusi akhir dalam format tabel/daftar terstruktur:
  - Total tugas, jumlah sukses, jumlah gagal, dan jumlah dilewati.
  - Pesan error ringkas untuk entri yang gagal.

---

## Fase 7: Integrasi Entrypoint (`src/main.rs`) & Pengujian E2E
- [x] Hubungkan seluruh modul di `src/main.rs`:
  - `cli` -> `config (pre-flight)` -> `parser` -> `runner` -> `reporter`.
- [x] Buat contoh berkas `skills.sample.txt` untuk pengujian lokal.
- [x] Jalankan pengujian integrasi dan verifikasi perilaku flag `-y` terhadap prompt interaktif.
- [x] Validasi penanganan error (target folder tidak ada, file tidak ada, format salah).
- [x] Finalisasi dokumentasi `README.md` dengan panduan kompilasi dan pemakaian.
