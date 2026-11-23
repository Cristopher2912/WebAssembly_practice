# WebAssembly Practice

Ejemplo mínimo de WebAssembly: un módulo escrito en Rust, compilado a `.wasm`, que entrega una lista de elementos a una página HTML.

## Archivos

- `src/lib.rs`: código fuente en Rust (expone la lista como texto).
- `Cargo.toml`: configuración del proyecto.
- `hola_wasm.wasm`: módulo compilado.
- `index.html`: página que carga el módulo y muestra la lista.

## Compilar (opcional)

    rustup target add wasm32-unknown-unknown
    cargo build --release --target wasm32-unknown-unknown
    cp target/wasm32-unknown-unknown/release/hola_wasm.wasm .

## Ejecutar localmente

El navegador exige un servidor HTTP (no funciona abriendo el archivo directamente):

    python -m http.server 8000

Abrir http://localhost:8000

## Publicar

GitHub Pages: Settings → Pages → Branch `main` / carpeta `/ (root)`.
