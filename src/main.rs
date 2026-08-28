use native_windows_gui as nwg;
use native_windows_derive as nwd;
use nwd::NwgUi;
use std::cell::RefCell;

fn tema_oscuro_activo() -> bool {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let ruta = hkcu.open_subkey(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize"
    );

    match ruta {
        Ok(clave) => {
            let valor: u32 = clave.get_value("AppsUseLightTheme").unwrap_or(1);
            valor == 0 // 0 = modo oscuro, 1 = modo claro
        }
        Err(_) => false,
    }
}

#[derive(Default, NwgUi)]
pub struct AppHolaMundo {
    #[nwg_control(size: (400, 200), position: (300, 300), title: "Hola Mundo", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch] )]
    pub window: nwg::Window,

    #[nwg_control(text: "¡Hola, Mundo!", position: (20, 20), size: (360, 30))]
    pub label_titulo: nwg::Label,

    #[nwg_control(text: "", position: (20, 60), size: (250, 30), placeholder_text: Some("Escribe tu nombre")]
    pub input_nombre: nwg::TextInput,

    #[nwg_control(text: "Saludar", position: (280, 60), size: (100, 30))]
    #[nwg_events( OnButtonClick: [AppHolaMundo::saludar] )]
    pub boton_saludar: nwg::Button,

    #[nwg_control(text: "", position: (20, 110), size: (360, 30))]
    pub label_resultado: nwg::Label,

    pub oscuro: RefCell<bool>,
}

impl AppHolaMundo {
    fn saludar(&self) {
        let nombre = self.input_nombre.text();
        let nombre = if nombre.trim().is_empty() { "Mundo".to_string() } else { nombre };
        self.label_resultado.set_text(&format!("¡Hola, {}! Bienvenido a Rust en Windows 11.", nombre));
    }
}

fn main() {
    nwg::init().expect("Fallo al iniciar Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Fallo al fijar fuente");

    let _oscuro = tema_oscuro_activo();

    let _app = AppHolaMundo::build_ui(Default::default()).expect("Fallo al construir la UI");

    nwg::dispatch_thread_events();
}
