//! A project's language: the words its drawings print (a stair's UP, a title block's SCALE, a
//! schedule's headings) and the names commands give what they make (levels, views, sheets, rooms).
//! English, Brazilian Portuguese, Spanish or Vietnamese. Names already given stay as they are; the
//! core's messages, which agents read, stay English.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    #[default]
    #[serde(rename = "en")]
    English,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "vi")]
    Vietnamese,
}

impl Language {
    pub const ALL: [Language; 4] = [Language::English, Language::Portuguese, Language::Spanish, Language::Vietnamese];

    /// Its code in commands and files: en, pt, es, vi.
    pub fn code(self) -> &'static str {
        self.of(["en", "pt", "es", "vi"])
    }

    /// Its name, in itself.
    pub fn label(self) -> &'static str {
        self.of(["English", "Português (Brasil)", "Español", "Tiếng Việt"])
    }

    /// Of one thing in each language, English first, this language's.
    pub fn of<T>(self, [english, portuguese, spanish, vietnamese]: [T; 4]) -> T {
        match self {
            Language::English => english,
            Language::Portuguese => portuguese,
            Language::Spanish => spanish,
            Language::Vietnamese => vietnamese,
        }
    }

    /// A language by its code or its name, or from a locale as systems give one: pt-BR, es_419.UTF-8.
    pub fn from_code(code: &str) -> Option<Language> {
        let code = code.trim().to_lowercase();
        Some(match code.split(['-', '_', '.']).next().unwrap_or_default() {
            "en" | "english" => Language::English,
            "pt" | "portuguese" | "português" => Language::Portuguese,
            "es" | "spanish" | "español" => Language::Spanish,
            "vi" | "vietnamese" | "tiếng việt" | "viet" => Language::Vietnamese,
            _ => return None,
        })
    }

    /// `english`, a word or a name the core writes, in this language.
    pub fn word(self, english: &str) -> &str {
        self.translated(english).unwrap_or(english)
    }

    /// `english` in this language, when it isn't English and the core's words have it.
    pub fn translated(self, english: &str) -> Option<&'static str> {
        if self == Language::English {
            return None;
        }
        WORDS.iter().find(|w| w[0] == english).map(|&w| self.of(w))
    }

    /// Which of `options` (English words) `name` is, in English or in this language.
    pub fn find(self, name: &str, options: &[&'static str]) -> Option<&'static str> {
        let name = name.trim();
        options.iter().copied().find(|o| o.eq_ignore_ascii_case(name) || self.word(o).to_lowercase() == name.to_lowercase())
    }
}

/// The core's words in English, Brazilian Portuguese, Spanish and Vietnamese.
pub(crate) const WORDS: &[[&str; 4]] = &[
    // Phases and the phase filters a project starts with.
    ["Existing", "Existente", "Existente", "Hiện hữu"],
    ["New Construction", "Nova construção", "Nueva construcción", "Xây mới"],
    ["Show All", "Mostrar tudo", "Mostrar todo", "Hiển thị tất cả"],
    ["Show Complete", "Mostrar completo", "Mostrar completo", "Hiển thị hoàn chỉnh"],
    ["Show New", "Mostrar novo", "Mostrar nuevo", "Hiển thị mới"],
    ["Show Previous + Demolished", "Mostrar anterior + demolido", "Mostrar anterior + demolido", "Hiển thị trước + đã phá dỡ"],
    ["Show Previous + New", "Mostrar anterior + novo", "Mostrar anterior + nuevo", "Hiển thị trước + mới"],
    // Names commands give.
    ["Project", "Projeto", "Proyecto", "Dự án"],
    ["Level", "Nível", "Nivel", "Cao độ"],
    ["Section", "Corte", "Sección", "Mặt cắt"],
    ["Elevation", "Elevação", "Elevación", "Mặt đứng"],
    ["North", "Norte", "Norte", "Bắc"],
    ["South", "Sul", "Sur", "Nam"],
    ["East", "Leste", "Este", "Đông"],
    ["West", "Oeste", "Oeste", "Tây"],
    ["Unnamed", "Sem nome", "Sin nombre", "Chưa đặt tên"],
    ["Room", "Ambiente", "Habitación", "Phòng"],
    ["Mass", "Massa", "Masa", "Khối"],
    ["Room Schedule", "Tabela de ambientes", "Tabla de habitaciones", "Bảng thống kê phòng"],
    ["Door Schedule", "Tabela de portas", "Tabla de puertas", "Bảng thống kê cửa đi"],
    ["Window Schedule", "Tabela de janelas", "Tabla de ventanas", "Bảng thống kê cửa sổ"],
    ["Wall Schedule", "Tabela de paredes", "Tabla de muros", "Bảng thống kê tường"],
    ["Floor Schedule", "Tabela de pisos", "Tabla de pisos", "Bảng thống kê sàn"],
    ["Level Schedule", "Tabela de níveis", "Tabla de niveles", "Bảng thống kê cao độ"],
    ["Ceiling Plan", "Planta de forro", "Plano de cielo raso", "Mặt bằng trần"],
    ["Copy", "Cópia", "Copia", "Bản sao"],
    ["Dependent", "Dependente", "Dependiente", "Phụ thuộc"],
    // The materials library a new project has.
    ["Brick", "Tijolo", "Ladrillo", "Gạch"],
    ["Insulation", "Isolamento", "Aislamiento", "Cách nhiệt"],
    ["Concrete block", "Bloco de concreto", "Bloque de hormigón", "Gạch bê tông"],
    ["Plaster", "Reboco", "Enlucido", "Vữa trát"],
    ["Concrete", "Concreto", "Hormigón", "Bê tông"],
    ["Screed", "Contrapiso", "Recrecido", "Vữa lót"],
    ["Timber", "Madeira", "Madera", "Gỗ"],
    ["Gypsum board", "Placa de gesso", "Placa de yeso", "Tấm thạch cao"],
    ["Ceramic tile", "Cerâmica", "Baldosa cerámica", "Gạch ceramic"],
    ["Roof tiles", "Telhas", "Tejas", "Ngói"],
    ["Membrane", "Manta impermeável", "Lámina impermeable", "Màng chống thấm"],
    ["Fabric", "Tecido", "Tela", "Vải"],
    ["Porcelain", "Porcelana", "Porcelana", "Sứ"],
    // The families a project starts with, and their parameters.
    ["Table", "Mesa", "Mesa", "Bàn"],
    ["Chair", "Cadeira", "Silla", "Ghế"],
    ["Bed", "Cama", "Cama", "Giường"],
    ["Sofa", "Sofá", "Sofá", "Ghế sofa"],
    ["Desk", "Escrivaninha", "Escritorio", "Bàn làm việc"],
    ["Wardrobe", "Guarda-roupa", "Armario", "Tủ quần áo"],
    ["Base Cabinet", "Armário inferior", "Mueble bajo", "Tủ dưới"],
    ["WC", "Vaso sanitário", "Inodoro", "Bồn cầu"],
    ["Washbasin", "Lavatório", "Lavabo", "Chậu rửa mặt"],
    ["Bathtub", "Banheira", "Bañera", "Bồn tắm"],
    // The fixtures of the discipline tabs.
    ["Kitchen Sink", "Pia de cozinha", "Fregadero de cocina", "Bồn rửa bát"],
    ["Shower", "Chuveiro", "Ducha", "Sen tắm"],
    ["Floor Drain", "Ralo", "Sumidero de piso", "Phễu thu sàn"],
    ["Elbow 90", "Joelho 90", "Codo 90", "Cút 90"],
    ["Tee", "Tê", "Te", "Tê"],
    ["Gate Valve", "Registro de gaveta", "Válvula de compuerta", "Van cửa"],
    ["Diameter", "Diâmetro", "Diámetro", "Đường kính"],
    ["Distribution Board", "Quadro de distribuição", "Cuadro de distribución", "Tủ điện"],
    ["Ceiling Luminaire", "Luminária de teto", "Luminaria de techo", "Đèn trần"],
    ["Recessed Luminaire", "Luminária embutida", "Luminaria empotrada", "Đèn âm trần"],
    ["Outlet", "Tomada", "Tomacorriente", "Ổ cắm"],
    ["Switch", "Interruptor", "Interruptor", "Công tắc"],
    ["Mounting Height", "Altura de instalação", "Altura de montaje", "Cao độ lắp đặt"],
    ["Air Terminal", "Captor", "Terminal aéreo", "Kim thu sét"],
    ["Earthing Electrode", "Haste de aterramento", "Electrodo de puesta a tierra", "Cọc tiếp địa"],
    ["Inspection Box", "Caixa de inspeção", "Caja de inspección", "Hộp kiểm tra"],
    ["Data Outlet", "Ponto de dados", "Punto de datos", "Ổ cắm mạng"],
    ["Voice Outlet", "Ponto de voz", "Punto de voz", "Ổ cắm điện thoại"],
    ["Patch Panel", "Patch panel", "Panel de parcheo", "Bảng đấu dây"],
    ["Rack", "Rack", "Rack", "Tủ rack"],
    ["Single Door", "Porta simples", "Puerta simple", "Cửa đơn"],
    ["Double Door", "Porta dupla", "Puerta doble", "Cửa đôi"],
    ["Fixed Window", "Janela fixa", "Ventana fija", "Cửa sổ cố định"],
    ["Casement Window", "Janela de abrir", "Ventana abatible", "Cửa sổ mở"],
    ["Frame Width", "Largura do batente", "Ancho del marco", "Chiều rộng khung"],
    ["Frame Depth", "Profundidade do batente", "Profundidad del marco", "Chiều sâu khung"],
    ["Frame Material", "Material do batente", "Material del marco", "Vật liệu khung"],
    ["Leaf Material", "Material da folha", "Material de la hoja", "Vật liệu cánh"],
    ["Leaf Thickness", "Espessura da folha", "Espesor de la hoja", "Độ dày cánh"],
    ["Glass", "Vidro", "Vidrio", "Kính"],
    ["Metal", "Metal", "Metal", "Kim loại"],
    ["PVC", "PVC", "PVC", "PVC"],
    ["Copper", "Cobre", "Cobre", "Đồng"],
    ["Depth", "Profundidade", "Profundidad", "Chiều sâu"],
    ["Seat Height", "Altura do assento", "Altura del asiento", "Chiều cao mặt ngồi"],
    ["Leg Width", "Largura da perna", "Ancho de la pata", "Chiều rộng chân"],
    ["Arm Width", "Largura do braço", "Ancho del brazo", "Chiều rộng tay vịn"],
    ["Headboard Height", "Altura da cabeceira", "Altura del cabecero", "Chiều cao đầu giường"],
    ["Worktop", "Bancada", "Encimera", "Mặt bàn"],
    // What drawings print.
    ["UP", "SOBE", "SUBE", "LÊN"],
    ["DN", "DESCE", "BAJA", "XUỐNG"],
    ["NOT ENCLOSED", "NÃO DELIMITADO", "NO DELIMITADA", "CHƯA KHÉP KÍN"],
    ["SCALE", "ESCALA", "ESCALA", "TỶ LỆ"],
    ["SCALE AS INDICATED", "ESCALA CONFORME INDICADO", "ESCALA SEGÚN SE INDICA", "TỶ LỆ NHƯ GHI"],
    // Schedule headings.
    ["Number", "Número", "Número", "Số"],
    ["Name", "Nome", "Nombre", "Tên"],
    ["Mark", "Marca", "Marca", "Ký hiệu"],
    ["Type", "Tipo", "Tipo", "Loại"],
    ["Width", "Largura", "Ancho", "Chiều rộng"],
    ["Height", "Altura", "Altura", "Chiều cao"],
    ["Sill", "Peitoril", "Antepecho", "Ngưỡng"],
    ["Wall", "Parede", "Muro", "Tường"],
    ["Length", "Comprimento", "Longitud", "Chiều dài"],
    ["Thickness", "Espessura", "Espesor", "Độ dày"],
    ["Area", "Área", "Área", "Diện tích"],
    ["Perimeter", "Perímetro", "Perímetro", "Chu vi"],
    ["Volume", "Volume", "Volumen", "Thể tích"],
    ["Gross Floor Area", "Área bruta de piso", "Superficie bruta de planta", "Tổng diện tích sàn"],
    ["Count", "Contagem", "Recuento", "Số lượng"],
    ["Family", "Família", "Familia", "Họ"],
    ["Offset", "Deslocamento", "Desfase", "Độ lệch"],
    ["Slope", "Inclinação", "Pendiente", "Độ dốc"],
    ["Plan Area", "Área em planta", "Área en planta", "Diện tích mặt bằng"],
    ["Risers", "Espelhos", "Contrahuellas", "Số bậc"],
    ["Riser Height", "Altura do espelho", "Altura de contrahuella", "Chiều cao bậc"],
    ["Tread", "Piso", "Huella", "Mặt bậc"],
    ["Total", "Total", "Total", "Tổng"],
    ["Yes", "Sim", "Sí", "Có"],
    ["No", "Não", "No", "Không"],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_come_in_the_projects_language() {
        let pt = Language::Portuguese;
        assert_eq!([pt.word("Level"), pt.word("UP"), pt.word("SCALE AS INDICATED")], ["Nível", "SOBE", "ESCALA CONFORME INDICADO"]);
        assert_eq!(Language::English.word("Level"), "Level");
        assert_eq!(pt.word("Id"), "Id", "a word with no translation stays");
        assert_eq!(pt.find("tabela de ambientes", &["Room Schedule", "Door Schedule"]), Some("Room Schedule"));
        assert_eq!(pt.find("Door Schedule", &["Room Schedule", "Door Schedule"]), Some("Door Schedule"));
        let es = Language::Spanish;
        assert_eq!([es.word("Level"), es.word("UP"), es.word("Room Schedule")], ["Nivel", "SUBE", "Tabla de habitaciones"]);
        assert_eq!(es.find("TABLA DE PUERTAS", &["Room Schedule", "Door Schedule"]), Some("Door Schedule"));
        let vi = Language::Vietnamese;
        assert_eq!([vi.word("Level"), vi.word("UP"), vi.word("Room Schedule")], ["Cao độ", "LÊN", "Bảng thống kê phòng"]);
        assert_eq!(vi.find("BẢNG THỐNG KÊ CỬA ĐI", &["Room Schedule", "Door Schedule"]), Some("Door Schedule"));
        assert_eq!(vi.word("Id"), "Id", "a word with no translation stays");
        // Codes as commands write them, and locales as systems give them.
        assert_eq!(
            ["pt-BR", "pt_BR.UTF-8", "es", "es-419", "Español", "en-US", "en", "de", "C", ""].map(Language::from_code),
            [Some(pt), Some(pt), Some(es), Some(es), Some(es), Some(Language::English), Some(Language::English), None, None, None]
        );
        assert_eq!(
            ["vi", "vi-VN", "vi_VN.UTF-8", "Tiếng Việt", "vietnamese", "viet"].map(Language::from_code),
            [Some(vi), Some(vi), Some(vi), Some(vi), Some(vi), Some(vi)]
        );
        assert_eq!([Language::English.code(), vi.code(), vi.label()], ["en", "vi", "Tiếng Việt"]);
        assert_eq!(Language::ALL.len(), 4);
    }
}
