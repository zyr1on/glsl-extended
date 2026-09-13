# GLSL Extended — Zed Editor Extension

OpenGL 4.6 (Core Profile) ve modern shader geliştirme için tasarlanmış kapsamlı ve profesyonel **Zed Editor** eklentisi.

---

## 🌟 Özellikler

- **Gelişmiş Tree-sitter Renklendirme (Syntax Highlighting):**
  - 400+ GLSL veri tipi (`vec2` - `dmat4`, sampler, image, atomic_uint).
  - Tüm yerleşik GLSL matematik ve doku fonksiyonları (`texture`, `normalize`, `mix`, `fma` vb.).
  - Yerleşik OpenGL değişkenleri (`gl_Position`, `gl_FragCoord`, `gl_VertexID` vb.).
  - Node-bağımsız regex eşleme (`#match?`) sayesinde tam kararlılık.

- **Çift LSP (Dual Language Server) Mimarisi:**
  - **LSP 1 (`glsl_analyzer`):** Akıllı kod tamamlama (Autocomplete), fareyle üzerine gelme dökümantasyonu (Hover), tanıma gitme (Goto Definition).
  - **LSP 2 (`glsl_validator`):** `glslangValidator` tabanlı gerçek zamanlı derleme hata tespiti (Diagnostics / kırmızı dalgalı çizgiler).

- **Saf Desktop OpenGL 4.6 Desteği:**
  - SPIR-V zorunluluğu yoktur; `out vec3 Normal;` gibi standart OpenGL değişkenleri hatasız derlenir.
  - Kod yazılırken satır ve sütun bazlı anlık hata bildirimi.

---

## 📂 Desteklenen Dosya Uzantıları

| Aşama (Stage) | Uzantılar |
|---|---|
| **Vertex Shader** | `.vert` |
| **Fragment Shader** | `.frag` |
| **Geometry Shader** | `.geom` |
| **Tessellation** | `.tesc`, `.tese` |
| **Compute Shader** | `.comp` |
| **Mesh / Task Shader** | `.mesh`, `.task` |
| **Ray Tracing** | `.rgen`, `.rint`, `.rahit`, `.rchit`, `.rmiss`, `.rcall` |
| **Genel / Başlık** | `.glsl`, `.glslh` |
| **Başlık Tespiti** | `#version \d+` ile başlayan tüm dosyalar otomatik GLSL tanınır |

---

## ⚙️ Gereksinimler

Sisteminizde (PATH üzerinde) aşağıdaki araçların bulunması önerilir:
1. **`glsl_analyzer.exe`**: [nolanderc/glsl_analyzer Releases](https://github.com/nolanderc/glsl_analyzer/releases) *(PATH'te yoksa eklenti otomatik indirebilir)*
2. **`glslangValidator.exe`**: [Vulkan SDK](https://vulkan.lunarg.com/sdk/home) veya MSYS2 UCRT64 (`pacman -S mingw-w64-ucrt-x86_64-glslang`)
3. **`glsl_validator.exe`**: Projenin `bin/` klasöründe hazır derlenmiş olarak mevcuttur veya `glsl_validator` klasöründen `cargo build --release` ile derlenebilir.

---

## 🚀 Kurulum (Zed Dev Extension)

1. **Rust ve wasm target kurulumu (yalnızca ilk seferde):**
   ```powershell
   rustup target add wasm32-wasip2
   ```

2. **Zed Eklentisini Yükleme:**
   - Zed editörünü açın.
   - `Ctrl + Shift + P` basın.
   - `zed: install dev extension` yazın ve seçin.
   - Proje klasörünü (`d:\glsl_extended`) seçin.

3. **Zed Ayarları (`settings.json`):**
   `%APPDATA%\Zed\settings.json` dosyanızda şu blokların bulunduğundan emin olun:
   ```json
   {
       "languages": {
           "GLSL": {
               "language_servers": ["glsl_analyzer", "glsl_validator"],
               "tab_size": 4,
               "format_on_save": "off"
           }
       }
   }
   ```

---

## 📁 Proje Dizin Yapısı

```
glsl_extended/
├── bin/
│   └── glsl_validator.exe       # Önceden derlenmiş linter LSP sunucusu
├── glsl_validator/              # Linter LSP sunucusunun Rust kaynak kodları
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── languages/
│   └── glsl/
│       ├── brackets.scm         # Parantez eşleştirme
│       ├── config.toml          # Dil yapılandırması & uzantılar
│       ├── highlights.scm       # Tree-sitter sözdizimi renklendirme
│       ├── indents.scm          # Otomatik girintileme
│       └── outline.scm          # Fonksiyon / sembol ağacı
├── src/
│   └── lib.rs                   # Zed Extension WASM ana kodları
├── Cargo.toml                   # Rust WASM paket yapılandırması
├── extension.toml               # Zed Eklenti Manifesti
├── .gitignore                   # Git yoksayma kuralları
└── README.md                    # Dökümantasyon
```

---

## 🔍 Hata Ayıklama & Loglar

- **Zed Logları:** `Ctrl + Shift + P` → `zed: open log`
- **glsl_validator Logları:** `%LOCALAPPDATA%\Temp\glsl_validator.log`