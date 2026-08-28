# HolaMundo

App de escritorio "Hola Mundo" para Windows 11, escrita en Rust con `native-windows-gui`.

## Características
- Ventana nativa con campo de texto y saludo personalizado.
- Detección automática de tema claro/oscuro de Windows 11.
- Ícono personalizado.
- Instalador generado con Inno Setup.
- CI/CD con GitHub Actions: compila y publica el instalador en cada tag `vX.Y.Z`.

## Compilar localmente
```powershell
cargo build --release
```

## Generar instalador
Requiere [Inno Setup](https://jrsoftware.org/isinfo.php) instalado.
```powershell
ISCC.exe instalador.iss
```

## Publicar una versión
```powershell
git tag v1.0.0
git push origin v1.0.0
```
Esto dispara el workflow `.github/workflows/release.yml`, que compila el `.exe`, genera el instalador y crea un Release en GitHub con ambos archivos.
