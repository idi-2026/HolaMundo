fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icono.ico");
        res.compile().expect("Fallo al incrustar el ícono");
    }
}
