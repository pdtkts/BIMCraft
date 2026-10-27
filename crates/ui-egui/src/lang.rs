//! The interface's language: English, as the code writes it, Brazilian Portuguese, Spanish, Vietnamese or Simplified Chinese.
//! What
//! the window shows goes through [`t`], English being its own key; the widget helpers translate
//! their labels and tips, so most call sites pass English and read it back translated. What a
//! project holds (names, what its drawings say) follows the project's language instead, and the
//! core's messages, which agents read too, stay English.
//!
//! Chinese is the window's alone ([`Lang`]): a drawing's lettering is drawn in code and has no hanzi, so a project is never
//! in it, and the core's words (schedule headings and the like) stay English there. Its words are `zh.rs`, one row for each of
//! this table's: a row it lacks shows in English. The window draws hanzi with a font the system has (`font.rs`).
//!
//! Vietnamese is a project's language as well (`Language::Vietnamese`), so the window and the drawings say the same thing.
//! egui's own fonts lack a quarter of its tone marks, so the window looks for a system font that has them all ([`use_vietnamese_font`]),
//! and a project's lettering is drawn by the core's own stroke font, which carries the marks itself.

mod font;
mod zh;

use std::cell::Cell;

use bimcraft_core::language::Language;
use egui::epaint::text::{FontDefinitions, FontInsert, FontPriority, InsertFontFamily};
use egui::{Context, FontData, FontFamily, Id};

/// The interface's language: one of a project's languages, or Simplified Chinese.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Lang {
    #[default]
    English,
    Portuguese,
    Spanish,
    Vietnamese,
    Chinese,
}

impl Lang {
    pub const ALL: [Lang; 5] = [Lang::English, Lang::Portuguese, Lang::Spanish, Lang::Vietnamese, Lang::Chinese];

    /// The project language that goes with it, which new projects are named in: Chinese has none, so English.
    pub fn project(self) -> Language {
        match self {
            Lang::English | Lang::Chinese => Language::English,
            Lang::Portuguese => Language::Portuguese,
            Lang::Spanish => Language::Spanish,
            Lang::Vietnamese => Language::Vietnamese,
        }
    }

    /// Its code in the settings file: en, pt, es, vi, zh.
    pub fn code(self) -> &'static str {
        match self {
            Lang::Chinese => "zh",
            other => other.project().code(),
        }
    }

    /// Its name, in itself: Chinese in hanzi only once the window is drawn in it, which is when it can draw them.
    pub fn label(self) -> &'static str {
        match self {
            Lang::Chinese if get() == Lang::Chinese => "简体中文",
            Lang::Chinese => "Chinese (Simplified)",
            other => other.project().label(),
        }
    }

    /// A language by its code or name, or from a locale as systems give one: pt-BR, es_419.UTF-8, vi-VN, zh-CN. Only Simplified
    /// Chinese is here: zh-TW, zh-HK and the other Traditional locales are none.
    pub fn from_code(code: &str) -> Option<Lang> {
        let lower = code.trim().to_lowercase();
        let parts: Vec<&str> = lower.split(['-', '_', '.', '@']).collect();
        if matches!(parts[0], "zh" | "chinese") || lower == "简体中文" {
            let traditional = parts.iter().any(|p| matches!(*p, "tw" | "hk" | "mo" | "hant"));
            return (!traditional).then_some(Lang::Chinese);
        }
        Language::from_code(code).map(Lang::from)
    }
}

impl From<Language> for Lang {
    fn from(language: Language) -> Lang {
        match language {
            Language::English => Lang::English,
            Language::Portuguese => Lang::Portuguese,
            Language::Spanish => Lang::Spanish,
            Language::Vietnamese => Lang::Vietnamese,
        }
    }
}

thread_local! {
    /// The language this frame is drawn in: each window draws on its own thread.
    static LANG: Cell<Lang> = const { Cell::new(Lang::English) };
}

pub fn set(language: Lang) {
    LANG.with(|l| l.set(language));
}

pub fn get() -> Lang {
    LANG.with(Cell::get)
}

/// The language the system's settings ask for (on Linux, `LC_ALL`, `LC_MESSAGES` or `LANG`), when
/// it's one there is; else English.
pub fn from_env() -> Lang {
    sys_locale::get_locale().and_then(|l| Lang::from_code(&l)).unwrap_or_default()
}

/// Said when the window cannot be put in Chinese.
pub const NO_CHINESE_FONT: &str =
    "No font with Chinese characters was found on this system: install one (such as Noto Sans CJK) and choose Chinese again.";

/// Said when the window cannot be put in Vietnamese.
pub const NO_VIETNAMESE_FONT: &str =
    "No font with Vietnamese characters was found on this system: install one (such as Noto Sans) and choose Vietnamese again.";

/// Every letter a font must have for Vietnamese, tone marks and all: the base letters, the shapes (`ă`, `ơ`, `ư`, `đ`),
/// the stacked marks (`ế`, `ắ`, `ộ`) and the loose ones Vietnamese also writes (`á`, `à`, `ã`, `Á`). It is what
/// [`use_vietnamese_font`] looks for, so a window in Vietnamese is never boxes. The test
/// `the_letters_a_vietnamese_font_must_have_are_the_ones_vietnamese_text_needs` keeps it level with the letters the
/// table's own words use.
pub const VIETNAMESE_LETTERS: &str =
    "ăĂâÂđĐêÊôÔơƠưƯảẢạẠắẮằẰẳẲẵẴặẶấẤầẦẩẨẫẪậẬéÉèÈẻẺẽẼẹẸếẾềỀểỂễỄệỆíÍìÌỉỈĩĨịỊóÓòÒỏỎõÕọỌốỐồỒổỔỗỖộỘớỚờỜởỞỡỠợỢúÚùÙủỦũŨụỤứỨừỪửỬữỮựỰýÝỳỲỷỶỹỸỵỴáÁàã";

/// Whether `ctx` has a font to draw Vietnamese with: [`use_vietnamese_font`] has put one there.
fn has_vietnamese_font(ctx: &Context) -> bool {
    ctx.data(|d| d.get_temp::<bool>(Id::new("vietnamese-font"))).unwrap_or(false)
}

/// The font of the system's [`use_vietnamese_font`] looks for.
pub fn vietnamese_font() -> Option<(Vec<u8>, u32)> {
    font::find_vietnamese()
}

/// Puts a font that `find` finds, with Vietnamese letters, in front of the window's own, once; false where there is none.
/// egui's own fonts lack a quarter of the tone marks, so a window in Vietnamese without one would show boxes for them.
///
/// In front, not behind ([`FontPriority::Highest`]): behind egui's own the font would draw only the letters egui's own
/// lacks, so the tone-marked letters would come from it and every unmarked letter of the same word from egui's own — two
/// faces, two weights, in one word. Which system font is picked, and why it must weigh what egui's own weighs, is
/// [`font`]'s. The test `the_whole_word_is_drawn_by_one_face_and_not_only_its_marks` holds this.
pub fn use_vietnamese_font(ctx: &Context, find: FontFinder) -> bool {
    if has_vietnamese_font(ctx) {
        return true;
    }
    let Some((bytes, index)) = find(Lang::Vietnamese) else { return false };
    let mut data = FontData::from_owned(bytes);
    data.index = index;
    let family = |family| InsertFontFamily { family, priority: FontPriority::Highest };
    ctx.add_font(FontInsert::new("vietnamese", data, vec![family(FontFamily::Proportional), family(FontFamily::Monospace)]));
    ctx.data_mut(|d| d.insert_temp(Id::new("vietnamese-font"), true));
    true
}

/// Whether `bytes` draws at the size egui's own font does, so text does not shrink when the window is put in
/// Vietnamese. Where `font_weighs_like` asks how heavy a face is, this asks how big: the height of a lowercase letter,
/// which is what the eye compares between two faces.
///
/// It is the other half of the reason a candidate is refused — the two mistakes are opposite: Segoe UI is the right size
/// and a third heavier, Calibri Light is the right weight and draws smaller — so a candidate must pass both.
pub(crate) fn font_draws_the_same_size(bytes: Vec<u8>, index: u32) -> bool {
    if bytes.len() < 4 {
        return false;
    }
    let size_of = |definitions: egui::epaint::text::FontDefinitions| {
        let mut fonts = egui::epaint::text::Fonts::new(egui::epaint::text::TextOptions::default(), definitions);
        x_height(&mut fonts)
    };
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let ours = size_of(single("candidate", bytes, index));
        let theirs = size_of(FontDefinitions::default());
        theirs > 0.0 && (ours - theirs).abs() <= SIZE_SLACK * theirs
    }))
    .unwrap_or(false)
}

/// How much a font's letters may differ in size from egui's own, as a fraction: a twentieth, the difference the eye
/// reads as text that shrank. Calibri Light is an eleventh smaller and is refused for it; Segoe UI Semilight matches.
const SIZE_SLACK: f32 = 0.05;

/// The size the x-height is measured at: large, so that the whole texels a glyph is rasterized into are a hundredth of
/// the letter and two faces that draw the same size cannot differ by one of them.
const MEASURED_AT: f32 = 100.0;

/// The height of `x`'s ink in texels at [`MEASURED_AT`] pt: the height of a lowercase letter, which is what the eye
/// compares between two faces more than the advance or the row height. The ink, not the glyph's box: the box carries the
/// face's own ascender and descender, which differ between faces that draw their letters at the very same size.
fn x_height(fonts: &mut egui::epaint::text::Fonts) -> f32 {
    let job =
        egui::epaint::text::LayoutJob::simple("x".to_owned(), egui::FontId::proportional(MEASURED_AT), egui::Color32::WHITE, f32::INFINITY);
    let galley = fonts.with_pixels_per_point(1.0).layout_job(job);
    let atlas = fonts.image();
    let mut ink = (None, 0usize);
    for row in &galley.rows {
        for glyph in &row.glyphs {
            let (x0, x1) = (glyph.uv_rect.min[0] as usize, glyph.uv_rect.max[0] as usize);
            let (y0, y1) = (glyph.uv_rect.min[1] as usize, glyph.uv_rect.max[1] as usize);
            for y in y0..y1 {
                if (x0..x1).any(|x| atlas.pixels[y * atlas.size[0] + x].a() > 0) {
                    ink.0.get_or_insert(y);
                    ink.1 = y;
                }
            }
        }
    }
    match ink.0 {
        Some(top) => (ink.1 - top + 1) as f32,
        None => 0.0,
    }
}

/// One face as `name`, for asking what it draws like.
fn single(name: &str, bytes: Vec<u8>, index: u32) -> FontDefinitions {
    let mut definitions = FontDefinitions::default();
    definitions.font_data.insert(name.to_owned(), std::sync::Arc::new(font(bytes, index)));
    definitions.families.insert(FontFamily::Proportional, vec![name.to_owned()]);
    definitions.families.insert(FontFamily::Monospace, vec![name.to_owned()]);
    definitions
}

/// Whether egui could draw `text` once `bytes` were added: the font's own character map has every letter of it. What
/// [`vietnamese_font`] uses to tell a font that really has the marks from one whose name only says so.
///
/// The character map, not `Fonts::has_glyphs`: that answers about the whole fallback chain, and reports false for every
/// letter of a face that is its own replacement glyph's face — which a candidate read on its own is.
pub(crate) fn font_covers(bytes: Vec<u8>, index: u32, text: &str) -> bool {
    if bytes.len() < 4 {
        return false;
    }
    // A file that says it is a font and is not: epaint's parser panics on one (it treats a bad file as impossible).
    // A candidate that cannot be read is simply not a font the window can use.
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut fonts = egui::epaint::text::Fonts::new(egui::epaint::text::TextOptions::default(), single("candidate", bytes, index));
        let mut face = fonts.fonts.font(&FontFamily::Proportional);
        let characters = face.characters().clone();
        text.chars().all(|c| characters.contains_key(&c))
    }))
    .unwrap_or(false)
}

/// Whether `bytes` weighs what egui's own font does, so a run of it beside the window's own text is not a shade heavier
/// or lighter. Where `covers` asks what a face has, this asks how it draws: the mean width of a stroke through the middle
/// of `o`, `e`, `a` and the letters like them, which is what the eye reads as bold.
///
/// It is one of the two reasons [`vietnamese_font`] refuses a font, and the one that matters when the font goes
/// *in front* ([`use_vietnamese_font`]): Segoe UI, the marks and all, is a third heavier than the Ubuntu-Light egui draws
/// the rest of the window with, so a Vietnamese word among English ones reads as bold.
pub(crate) fn font_weighs_like(bytes: Vec<u8>, index: u32) -> bool {
    if bytes.len() < 4 {
        return false;
    }
    let weight = |definitions: egui::epaint::text::FontDefinitions| {
        let mut fonts = egui::epaint::text::Fonts::new(egui::epaint::text::TextOptions::default(), definitions);
        stroke_weight(&mut fonts)
    };
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let candidate = single("candidate", bytes, index);
        let reference = FontDefinitions::default();
        let (ours, theirs) = (weight(candidate), weight(reference));
        theirs > 0.0 && (ours - theirs).abs() <= WEIGHT_SLACK * theirs
    }))
    .unwrap_or(false)
}

/// How much a font may differ from egui's own before it is drawn as bold beside it, as a fraction: a tenth, which the
/// letter shapes' own hinting already moves, so that a font that differs only by rendering is not refused for it.
const WEIGHT_SLACK: f32 = 0.10;

/// The letters a stroke is measured across: uprights, rounds and the diagonal of a, at 28 pt.
const STROKE_LETTERS: &str = "o e a n h u i l c t r";

/// The mean width of the strokes of [`STROKE_LETTERS`], in texels: what the eye reads as a face's weight. The widest run
/// of ink on the scanlines through the middle of each glyph, clear of a mark above it or a descender below.
fn stroke_weight(fonts: &mut egui::epaint::text::Fonts) -> f32 {
    let job = egui::epaint::text::LayoutJob::simple(
        STROKE_LETTERS.to_owned(),
        egui::FontId::proportional(28.0),
        egui::Color32::WHITE,
        f32::INFINITY,
    );
    let galley = fonts.with_pixels_per_point(1.0).layout_job(job);
    let atlas = fonts.image();
    let mut runs = Vec::new();
    for row in &galley.rows {
        for glyph in &row.glyphs {
            let uv = glyph.uv_rect;
            if uv.is_nothing() || glyph.chr == ' ' {
                continue;
            }
            let (x0, x1) = (uv.min[0] as usize, uv.max[0] as usize);
            let (y0, y1) = (uv.min[1] as usize, uv.max[1] as usize);
            if x1 <= x0 || y1 <= y0 {
                continue;
            }
            for k in 1..=3 {
                let y = y0 + (y1 - y0) * k / 4;
                let (mut best, mut run) = (0usize, 0usize);
                for x in x0..x1 {
                    if atlas.pixels[y * atlas.size[0] + x].a() > 128 {
                        run += 1;
                        best = best.max(run);
                    } else {
                        run = 0;
                    }
                }
                if best > 0 {
                    runs.push(best as f32);
                }
            }
        }
    }
    if runs.is_empty() { 0.0 } else { runs.iter().sum::<f32>() / runs.len() as f32 }
}

/// `bytes` as the font data the candidates are read through, so `font_covers` and `font_weighs_like` read it the same way.
fn font(bytes: Vec<u8>, index: u32) -> FontData {
    let mut data = FontData::from_owned(bytes);
    data.index = index;
    data
}

/// Tests have no say in which fonts a machine has: a window is told it has one.
#[cfg(test)]
pub(crate) fn assume_vietnamese_font(ctx: &Context) {
    ctx.data_mut(|d| d.insert_temp(Id::new("vietnamese-font"), true));
}

/// Whether `ctx` has a font to draw Chinese with: [`use_chinese_font`] has put one there.
fn has_chinese_font(ctx: &Context) -> bool {
    ctx.data(|d| d.get_temp::<bool>(Id::new("chinese-font"))).unwrap_or(false)
}

/// Looks for a font with the glyphs a language needs: its bytes and the face to use. The app holds one, so that a test can
/// say the system has none.
pub type FontFinder = fn(Lang) -> Option<(Vec<u8>, u32)>;

/// The font of the system's [`use_chinese_font`] and [`use_vietnamese_font`] look for.
pub fn system_font(lang: Lang) -> Option<(Vec<u8>, u32)> {
    match lang {
        Lang::Chinese => chinese_font(),
        Lang::Vietnamese => vietnamese_font(),
        Lang::English | Lang::Portuguese | Lang::Spanish => None,
    }
}

/// The font of the system's [`use_chinese_font`] looks for.
pub fn chinese_font() -> Option<(Vec<u8>, u32)> {
    font::find()
}

/// Puts a font that `find` finds, with Chinese glyphs, behind the window's own, once; false where there is none. egui's fonts have
/// no hanzi, so a window in Chinese without one would be boxes.
pub fn use_chinese_font(ctx: &Context, find: FontFinder) -> bool {
    if has_chinese_font(ctx) {
        return true;
    }
    let Some((bytes, index)) = find(Lang::Chinese) else { return false };
    let mut data = FontData::from_owned(bytes);
    data.index = index;
    let families = crate::style::families().map(|family| InsertFontFamily { family, priority: FontPriority::Lowest });
    ctx.add_font(FontInsert::new("chinese", data, families.into()));
    ctx.data_mut(|d| d.insert_temp(Id::new("chinese-font"), true));
    true
}

/// Tests have no say in which fonts a machine has: a window is told it has one.
#[cfg(test)]
pub(crate) fn assume_chinese_font(ctx: &Context) {
    ctx.data_mut(|d| d.insert_temp(Id::new("chinese-font"), true));
}

/// `english` in the interface's language.
pub fn t(english: &str) -> &str {
    translated_in(get(), english).unwrap_or(english)
}

/// `english` in `lang`, when it isn't English and the interface's words, or for the project's languages the core's, have it.
pub fn translated_in(lang: Lang, english: &str) -> Option<&'static str> {
    match lang {
        Lang::Chinese => zh::WORDS.iter().chain(zh::CORE).find(|w| w[0] == english).map(|w| w[1]),
        other => translated(other.project(), english),
    }
}

/// `english` in `language`, when it isn't English and the interface's words, or the core's, have it.
pub fn translated(language: Language, english: &str) -> Option<&'static str> {
    if language == Language::English {
        return None;
    }
    // ponytail: a linear scan, a few hundred a frame; a map built once if it ever shows in a profile.
    WORDS.iter().find(|w| w[0] == english).map(|&w| language.of(w)).or_else(|| language.translated(english))
}

/// `template` translated, its `{}`s filled with `args` in order.
pub fn tf(template: &str, args: &[&str]) -> String {
    args.iter().fold(t(template).to_string(), |s, a| s.replacen("{}", a, 1))
}

/// An element category's name, of one (`Wall`) or of several (`Walls`).
pub fn category(category: &str, several: bool) -> String {
    if get() == Lang::Chinese {
        return zh::category(category).unwrap_or(category).to_string();
    }
    let (one, many) = match category {
        "wall" => (["Wall", "Parede", "Muro", "Tường"], ["Walls", "Paredes", "Muros", "Tường"]),
        "door" => (["Door", "Porta", "Puerta", "Cửa đi"], ["Doors", "Portas", "Puertas", "Cửa đi"]),
        "window" => (["Window", "Janela", "Ventana", "Cửa sổ"], ["Windows", "Janelas", "Ventanas", "Cửa sổ"]),
        "floor" => (["Floor", "Piso", "Piso", "Sàn"], ["Floors", "Pisos", "Pisos", "Sàn"]),
        "room" => (["Room", "Ambiente", "Habitación", "Phòng"], ["Rooms", "Ambientes", "Habitaciones", "Phòng"]),
        "level" => (["Level", "Nível", "Nivel", "Cao độ"], ["Levels", "Níveis", "Niveles", "Cao độ"]),
        "grid" => (["Grid", "Eixo", "Eje", "Trục"], ["Grids", "Eixos", "Ejes", "Trục"]),
        "dimension" => (["Dimension", "Cota", "Cota", "Kích thước"], ["Dimensions", "Cotas", "Cotas", "Kích thước"]),
        "tag" => (["Tag", "Identificador", "Etiqueta", "Nhãn"], ["Tags", "Identificadores", "Etiquetas", "Nhãn"]),
        "section" => (["Section", "Corte", "Sección", "Mặt cắt"], ["Sections", "Cortes", "Secciones", "Mặt cắt"]),
        "elevation" => (["Elevation", "Elevação", "Elevación", "Mặt đứng"], ["Elevations", "Elevações", "Elevaciones", "Mặt đứng"]),
        "sheet" => (["Sheet", "Folha", "Plano", "Bản vẽ"], ["Sheets", "Folhas", "Planos", "Bản vẽ"]),
        "viewport" => (
            ["Viewport", "Janela de vista", "Vista en plano", "Khung nhìn"],
            ["Viewports", "Janelas de vista", "Vistas en plano", "Khung nhìn"],
        ),
        "column" => (["Column", "Pilar", "Columna", "Cột"], ["Columns", "Pilares", "Columnas", "Cột"]),
        "component" => (["Component", "Componente", "Componente", "Cấu kiện"], ["Components", "Componentes", "Componentes", "Cấu kiện"]),
        "stair" => (["Stair", "Escada", "Escalera", "Cầu thang"], ["Stairs", "Escadas", "Escaleras", "Cầu thang"]),
        "roof" => (["Roof", "Telhado", "Cubierta", "Mái"], ["Roofs", "Telhados", "Cubiertas", "Mái"]),
        "ceiling" => (["Ceiling", "Forro", "Cielo raso", "Trần"], ["Ceilings", "Forros", "Cielos rasos", "Trần"]),
        "shaft" => (
            ["Shaft Opening", "Abertura de shaft", "Abertura de ducto", "Lỗ mở kỹ thuật"],
            ["Shaft Openings", "Aberturas de shaft", "Aberturas de ducto", "Lỗ mở kỹ thuật"],
        ),
        "mass" => (["Mass", "Massa", "Masa", "Khối"], ["Masses", "Massas", "Masas", "Khối"]),
        "massfloor" => {
            (["Mass Floor", "Piso de massa", "Piso de masa", "Sàn khối"], ["Mass Floors", "Pisos de massa", "Pisos de masa", "Sàn khối"])
        }
        "ramp" => (["Ramp", "Rampa", "Rampa", "Dốc"], ["Ramps", "Rampas", "Rampas", "Dốc"]),
        "railing" => (["Railing", "Guarda-corpo", "Barandal", "Lan can"], ["Railings", "Guarda-corpos", "Barandales", "Lan can"]),
        "beam" => (["Beam", "Viga", "Viga", "Dầm"], ["Beams", "Vigas", "Vigas", "Dầm"]),
        "structuralcolumn" => (
            ["Structural Column", "Pilar estrutural", "Columna estructural", "Cột kết cấu"],
            ["Structural Columns", "Pilares estruturais", "Columnas estructurales", "Cột kết cấu"],
        ),
        "footing" => (["Footing", "Sapata", "Zapata", "Móng"], ["Footings", "Sapatas", "Zapatas", "Móng"]),
        "stripfooting" => (
            ["Strip Footing", "Sapata corrida", "Zapata corrida", "Móng băng"],
            ["Strip Footings", "Sapatas corridas", "Zapatas corridas", "Móng băng"],
        ),
        "pipe" => (["Pipe", "Tubulação", "Tubería", "Ống"], ["Pipes", "Tubulações", "Tuberías", "Ống"]),
        "conduit" => {
            (["Conduit", "Eletroduto", "Tubo conduit", "Ống luồn dây"], ["Conduits", "Eletrodutos", "Tubos conduit", "Ống luồn dây"])
        }
        "cabletray" => (
            ["Cable Tray", "Eletrocalha", "Bandeja portacables", "Máng cáp"],
            ["Cable Trays", "Eletrocalhas", "Bandejas portacables", "Máng cáp"],
        ),
        "wire" => (["Wire", "Fio", "Cable", "Dây"], ["Wires", "Fios", "Cables", "Dây"]),
        "cable" => (["Cable", "Cabo", "Cable", "Cáp"], ["Cables", "Cabos", "Cables", "Cáp"]),
        "downconductor" => (
            ["Down Conductor", "Condutor de descida", "Conductor de bajada", "Dây thoát sét"],
            ["Down Conductors", "Condutores de descida", "Conductores de bajada", "Dây thoát sét"],
        ),
        "groundconductor" => (
            ["Earthing Conductor", "Condutor de aterramento", "Conductor de puesta a tierra", "Dây tiếp địa"],
            ["Earthing Conductors", "Condutores de aterramento", "Conductores de puesta a tierra", "Dây tiếp địa"],
        ),
        "text" => {
            (["Text Note", "Nota de texto", "Nota de texto", "Ghi chú"], ["Text Notes", "Notas de texto", "Notas de texto", "Ghi chú"])
        }
        "spot" => (
            ["Spot Elevation", "Elevação de ponto", "Cota de nivel", "Cao độ điểm"],
            ["Spot Elevations", "Elevações de ponto", "Cotas de nivel", "Cao độ điểm"],
        ),
        "schedule" => (["Schedule", "Tabela", "Tabla", "Bảng thống kê"], ["Schedules", "Tabelas", "Tablas", "Bảng thống kê"]),
        "view3d" => (["3D View", "Vista 3D", "Vista 3D", "Khung nhìn 3D"], ["3D Views", "Vistas 3D", "Vistas 3D", "Khung nhìn 3D"]),
        "link" => (
            ["Linked Model", "Modelo vinculado", "Modelo vinculado", "Mô hình liên kết"],
            ["Linked Models", "Modelos vinculados", "Modelos vinculados", "Mô hình liên kết"],
        ),
        "pointcloud" => (
            ["Point Cloud", "Nuvem de pontos", "Nube de puntos", "Đám mây điểm"],
            ["Point Clouds", "Nuvens de pontos", "Nubes de puntos", "Đám mây điểm"],
        ),
        other => return other.to_string(),
    };
    get().project().of(if several { many } else { one }).to_string()
}

/// The interface's words in English, Brazilian Portuguese, Spanish and Vietnamese; the core's words
/// (schedule names and headings) besides.
const WORDS: &[[&str; 4]] = &[
    ["Autosave", "Salvamento automático", "Guardado automático", "Tự động lưu"],
    [
        "An agent can work on this project: bimcraft-cli mcp --connect {}",
        "Um agente pode trabalhar neste projeto: bimcraft-cli mcp --connect {}",
        "Un agente puede trabajar en este proyecto: bimcraft-cli mcp --connect {}",
        "Một tác nhân có thể làm việc trên dự án này: bimcraft-cli mcp --connect {}",
    ],
    [
        "No font with Chinese characters was found on this system: install one (such as Noto Sans CJK) and choose Chinese again.",
        "Nenhuma fonte com caracteres chineses foi encontrada neste sistema: instale uma (como a Noto Sans CJK) e escolha o chinês de novo.",
        "No se encontró en este sistema ninguna fuente con caracteres chinos: instale una (como Noto Sans CJK) y elija el chino otra vez.",
        "Không tìm thấy phông chữ Hán nào trên hệ thống này: hãy cài một phông (như Noto Sans CJK) rồi chọn lại tiếng Trung.",
    ],
    [
        "No font with Vietnamese characters was found on this system: install one (such as Noto Sans) and choose Vietnamese again.",
        "Nenhuma fonte com caracteres vietnamitas foi encontrada neste sistema: instale uma (como a Noto Sans) e escolha o vietnamita de novo.",
        "No se encontró en este sistema ninguna fuente con caracteres vietnamitas: instale una (como Noto Sans) y elija el vietnamita otra vez.",
        "Không tìm thấy phông chữ tiếng Việt nào trên hệ thống này: hãy cài một phông (như Noto Sans) rồi chọn lại tiếng Việt.",
    ],
    [
        "Autosave in the background",
        "Salvar automaticamente em segundo plano",
        "Guardar automáticamente en segundo plano",
        "Tự động lưu trong nền",
    ],
    ["Recover Autosave…", "Recuperar salvamento automático…", "Recuperar guardado automático…", "Phục hồi bản tự động lưu…"],
    ["Recovery copy found", "Cópia de recuperação encontrada", "Copia de recuperación encontrada", "Đã tìm thấy bản sao phục hồi"],
    ["Recovery copy: {}", "Cópia de recuperação: {}", "Copia de recuperación: {}", "Bản sao phục hồi: {}"],
    ["Untitled project", "Projeto sem título", "Proyecto sin título", "Dự án chưa đặt tên"],
    ["Recover", "Recuperar", "Recuperar", "Phục hồi"],
    ["Discard", "Descartar", "Descartar", "Bỏ"],
    ["Postpone", "Adiar", "Posponer", "Để sau"],
    [
        "Recover opens the backup as unsaved work. Discard deletes it. Postpone keeps it for later without replacing it.",
        "Recuperar abre a cópia como trabalho não salvo. Descartar a exclui. Adiar a mantém para depois, sem substituí-la.",
        "Recuperar abre la copia como trabajo sin guardar. Descartar la elimina. Posponer la conserva para después, sin reemplazarla.",
        "Phục hồi sẽ mở bản sao như công việc chưa lưu. Bỏ sẽ xóa nó. Để sau giữ nó lại mà không thay thế dự án.",
    ],
    [
        "Recovery copies are kept beside saved projects; untitled projects use the temporary folder.",
        "As cópias de recuperação ficam ao lado dos projetos salvos; projetos sem título usam a pasta temporária.",
        "Las copias de recuperación se guardan junto a los proyectos guardados; los proyectos sin título usan la carpeta temporal.",
        "Bản sao phục hồi được giữ bên cạnh các dự án đã lưu; dự án chưa đặt tên dùng thư mục tạm.",
    ],
    // The ribbon's tabs and the File menu.
    ["File", "Arquivo", "Archivo", "Tệp"],
    ["Architecture", "Arquitetura", "Arquitectura", "Kiến trúc"],
    ["Annotate", "Anotar", "Anotar", "Chú thích"],
    ["View", "Vista", "Vista", "Khung nhìn"],
    ["Manage", "Gerenciar", "Gestionar", "Quản lý"],
    ["Modify", "Modificar", "Modificar", "Sửa"],
    ["New", "Novo", "Nuevo", "Mới"],
    ["Open…", "Abrir…", "Abrir…", "Mở…"],
    ["Open Recent", "Abrir recente", "Abrir reciente", "Mở gần đây"],
    ["Clear Recent Files", "Limpar arquivos recentes", "Borrar archivos recientes", "Xóa danh sách gần đây"],
    ["Save", "Salvar", "Guardar", "Lưu"],
    ["Save As…", "Salvar como…", "Guardar como…", "Lưu thành…"],
    ["Export", "Exportar", "Exportar", "Xuất"],
    ["IFC…", "IFC…", "IFC…", "IFC…"],
    ["View as PNG…", "Vista como PNG…", "Vista como PNG…", "Xem dạng PNG…"],
    ["Plan as SVG…", "Planta como SVG…", "Planta como SVG…", "Mặt bằng dạng SVG…"],
    ["Sheets as PDF…", "Folhas como PDF…", "Planos como PDF…", "Bản vẽ dạng PDF…"],
    ["Exit", "Sair", "Salir", "Thoát"],
    ["BIMCraft project", "Projeto BIMCraft", "Proyecto BIMCraft", "Dự án BIMCraft"],
    ["BIMCraft project or IFC", "Projeto BIMCraft ou IFC", "Proyecto BIMCraft o IFC", "Dự án BIMCraft hoặc IFC"],
    ["PNG image", "Imagem PNG", "Imagen PNG", "Ảnh PNG"],
    ["SVG drawing", "Desenho SVG", "Dibujo SVG", "Bản vẽ SVG"],
    // The Quick Access Toolbar and the navigation bar.
    ["Open", "Abrir", "Abrir", "Mở"],
    ["Undo", "Desfazer", "Deshacer", "Hoàn tác"],
    ["Redo", "Refazer", "Rehacer", "Làm lại"],
    ["Default 3D View", "Vista 3D padrão", "Vista 3D predeterminada", "Khung nhìn 3D mặc định"],
    ["Zoom to Fit", "Zoom para ajustar", "Zoom para ajustar", "Thu vừa khung"],
    ["Zoom In", "Aproximar", "Acercar", "Phóng to"],
    ["Zoom Out", "Afastar", "Alejar", "Thu nhỏ"],
    ["Home View", "Vista inicial", "Vista inicial", "Khung nhìn ban đầu"],
    // Panels.
    ["Select", "Selecionar", "Seleccionar", "Chọn"],
    ["Build", "Construir", "Construir", "Xây dựng"],
    ["Circulation", "Circulação", "Circulación", "Giao thông"],
    ["Room & Area", "Ambiente e área", "Habitación y área", "Phòng & Diện tích"],
    ["Datum", "Referência", "Referencia", "Cao độ chuẩn"],
    ["Dimension", "Cota", "Cota", "Kích thước"],
    ["Text", "Texto", "Texto", "Văn bản"],
    ["Tag", "Identificar", "Etiquetar", "Nhãn"],
    ["Graphics", "Gráficos", "Gráficos", "Đồ họa"],
    ["Create", "Criar", "Crear", "Tạo"],
    ["Sheet Composition", "Composição de folha", "Composición de planos", "Bố cục bản vẽ"],
    ["Windows", "Janelas", "Ventanas", "Cửa sổ"],
    ["Settings", "Configurações", "Configuración", "Cài đặt"],
    ["Properties", "Propriedades", "Propiedades", "Thuộc tính"],
    // Tools and their tips.
    [
        "Modify (MD): select elements",
        "Modificar (MD): selecionar elementos",
        "Modificar (MD): seleccionar elementos",
        "Sửa (MD): chọn các phần tử",
    ],
    [
        "Wall (WA): each click builds a wall; Enter or Esc ends the chain",
        "Parede (WA): cada clique constrói uma parede; Enter ou Esc termina a cadeia",
        "Muro (WA): cada clic construye un muro; Enter o Esc termina la cadena",
        "Tường (WA): mỗi lần bấm dựng một đoạn tường; Enter hoặc Esc kết thúc chuỗi",
    ],
    ["Door", "Porta", "Puerta", "Cửa đi"],
    ["Door (DR): click a wall", "Porta (DR): clique numa parede", "Puerta (DR): haz clic en un muro", "Cửa đi (DR): bấm vào một tường"],
    ["Window", "Janela", "Ventana", "Cửa sổ"],
    ["Window (WN): click a wall", "Janela (WN): clique numa parede", "Ventana (WN): haz clic en un muro", "Cửa sổ (WN): bấm vào một tường"],
    ["Column", "Pilar", "Columna", "Cột"],
    [
        "Column (CL): click where it stands",
        "Pilar (CL): clique onde ele fica",
        "Columna (CL): haz clic donde va",
        "Cột (CL): bấm vào nơi cột đứng",
    ],
    ["Roof", "Telhado", "Cubierta", "Mái"],
    [
        "Roof by Footprint: click inside the walls to roof them, on the level above",
        "Telhado por perímetro: clique dentro das paredes para cobri-las, no nível acima",
        "Cubierta por perímetro: haz clic dentro de los muros para cubrirlos, en el nivel superior",
        "Mái theo chu vi: bấm vào bên trong các tường để lợp mái, ở cao độ phía trên",
    ],
    ["Stair", "Escada", "Escalera", "Cầu thang"],
    [
        "Stair: click where it starts, then which way it climbs",
        "Escada: clique onde ela começa e depois na direção em que sobe",
        "Escalera: haz clic donde empieza y luego hacia dónde sube",
        "Cầu thang: bấm vào nơi nó bắt đầu, rồi vào hướng nó đi lên",
    ],
    [
        "Room (RM): click inside walls",
        "Ambiente (RM): clique dentro das paredes",
        "Habitación (RM): haz clic dentro de los muros",
        "Phòng (RM): bấm vào bên trong các tường",
    ],
    [
        "Level (LL): a new level at an elevation",
        "Nível (LL): um novo nível numa elevação",
        "Nivel (LL): un nivel nuevo a una elevación",
        "Cao độ (LL): một cao độ mới tại một độ cao",
    ],
    ["Grid", "Eixo", "Eje", "Trục"],
    [
        "Grid (GR): click its two ends",
        "Eixo (GR): clique nas duas extremidades",
        "Eje (GR): haz clic en sus dos extremos",
        "Trục (GR): bấm vào hai đầu của nó",
    ],
    ["Aligned", "Alinhada", "Alineada", "Thẳng hàng"],
    [
        "Aligned Dimension (DI): pick parallel wall faces, wall ends, doors, windows or grids, then click where the dimension line goes",
        "Cota alinhada (DI): selecione faces e extremidades de paredes, portas, janelas ou eixos paralelos e clique onde vai a linha de cota",
        "Cota alineada (DI): elige caras o extremos de muros, puertas, ventanas o ejes paralelos y haz clic donde va la línea de cota",
        "Kích thước thẳng hàng (DI): chọn các mặt tường song song, đầu tường, cửa đi, cửa sổ hoặc trục, rồi bấm vào nơi đường kích thước đi qua",
    ],
    ["Angular", "Angular", "Angular", "Kích thước góc"],
    [
        "Angular Dimension: pick two lines that meet, then click where the arc goes",
        "Cota angular: selecione duas linhas que se encontram e clique onde vai o arco",
        "Cota angular: elige dos líneas que se encuentran y haz clic donde va el arco",
        "Kích thước góc: chọn hai đường gặp nhau, rồi bấm vào nơi cung đi qua",
    ],
    ["Radial", "Radial", "Radial", "Kích thước bán kính"],
    [
        "Radial Dimension: pick a round column, then click where the radius goes",
        "Cota radial: selecione um pilar redondo e clique onde vai o raio",
        "Cota radial: elige una columna redonda y haz clic donde va el radio",
        "Kích thước bán kính: chọn một cột tròn, rồi bấm vào nơi bán kính đi qua",
    ],
    ["Spot Elevation", "Elevação de ponto", "Cota de nivel", "Cao độ điểm"],
    [
        "Spot Elevation (EL): click a floor, a roof or a stair; in a section, a point",
        "Elevação de ponto (EL): clique num piso, num telhado ou numa escada; num corte, num ponto",
        "Cota de nivel (EL): haz clic en un piso, una cubierta o una escalera; en una sección, en un punto",
        "Cao độ điểm (EL): bấm vào một sàn, một mái hoặc một cầu thang; trong mặt cắt, bấm vào một điểm",
    ],
    [
        "Text (TX): type the note in the Options Bar, then click where it goes",
        "Texto (TX): digite a nota na Barra de opções e clique onde ela vai",
        "Texto (TX): escribe la nota en la barra de opciones y haz clic donde va",
        "Văn bản (TX): nhập ghi chú vào Thanh tùy chọn, rồi bấm vào nơi nó đi qua",
    ],
    ["Tag by Category", "Identificar por categoria", "Etiquetar por categoría", "Nhãn theo loại"],
    [
        "Tag by Category (TG): click an element to tag it with its type, a door's or window's mark, or the field in Label",
        "Identificar por categoria (TG): clique num elemento para identificá-lo com seu tipo, a marca de uma porta ou janela, ou o campo em Rótulo",
        "Etiquetar por categoría (TG): haz clic en un elemento para etiquetarlo con su tipo, la marca de una puerta o ventana, o el campo en Etiqueta",
        "Nhãn theo loại (TG): bấm vào một phần tử để gắn nhãn theo loại của nó, theo ký hiệu của cửa đi hoặc cửa sổ, hoặc theo trường trong Nhãn",
    ],
    ["Tag All", "Identificar todos", "Etiquetar todo", "Gắn nhãn tất cả"],
    [
        "Tag every door and window in this view that has no tag",
        "Identificar as portas e janelas desta vista que não têm identificador",
        "Etiquetar las puertas y ventanas de esta vista que no tienen etiqueta",
        "Gắn nhãn cho mọi cửa đi và cửa sổ trong khung nhìn này chưa có nhãn",
    ],
    ["Thin Lines", "Linhas finas", "Líneas finas", "Nét mảnh"],
    [
        "Thin Lines (TL): every line one pixel wide",
        "Linhas finas (TL): toda linha com um pixel de largura",
        "Líneas finas (TL): todas las líneas de un píxel de ancho",
        "Nét mảnh (TL): mọi đường rộng một điểm ảnh",
    ],
    ["Dark Mode", "Modo escuro", "Modo oscuro", "Chế độ tối"],
    [
        "Choose a theme, including a dark interface with a light canvas",
        "Escolha um tema, incluindo uma interface escura com uma tela clara",
        "Elija un tema, incluida una interfaz oscura con un lienzo claro",
        "Chọn một chủ đề, kể cả giao diện tối với khung vẽ sáng",
    ],
    ["Crop View", "Recortar vista", "Recortar vista", "Cắt khung nhìn"],
    ["Duplicate View", "Duplicar vista", "Duplicar vista", "Nhân bản khung nhìn"],
    ["Duplicate", "Duplicar", "Duplicar", "Nhân bản"],
    ["Duplicate with Detailing", "Duplicar com detalhamento", "Duplicar con detalles", "Nhân bản kèm chi tiết"],
    ["Duplicate as Dependent", "Duplicar como dependente", "Duplicar como dependiente", "Nhân bản dạng phụ thuộc"],
    [
        "Duplicate View (DP): a new plan of this level with its own annotation; with detailing, it starts with this plan's; as a dependent, it follows this plan's range and graphics",
        "Duplicar vista (DP): uma nova planta deste nível com anotações próprias; com detalhamento, ela começa com as desta planta; como dependente, segue o intervalo e os gráficos desta planta",
        "Duplicar vista (DP): un plano nuevo de este nivel con anotaciones propias; con detalles, empieza con las de este plano; como dependiente, sigue el rango y los gráficos de este plano",
        "Nhân bản khung nhìn (DP): một mặt bằng mới của cao độ này với ghi chú riêng; kèm chi tiết, nó bắt đầu từ ghi chú của mặt bằng này; dạng phụ thuộc, nó theo phạm vi và đồ họa của mặt bằng này",
    ],
    [
        "Duplicate View: open a plan first",
        "Duplicar vista: abra uma planta primeiro",
        "Duplicar vista: abre un plano primero",
        "Nhân bản khung nhìn: hãy mở một mặt bằng trước",
    ],
    ["Parent View", "Vista principal", "Vista principal", "Khung nhìn cha"],
    ["Linked Models", "Modelos vinculados", "Modelos vinculados", "Mô hình liên kết"],
    ["{} (not loaded)", "{} (não carregado)", "{} (no cargado)", "{} (chưa nạp)"],
    ["Reload", "Recarregar", "Recargar", "Nạp lại"],
    ["Links", "Vínculos", "Vínculos", "Liên kết"],
    ["Link Model", "Vincular modelo", "Vincular modelo", "Liên kết mô hình"],
    [
        "Link Model (LK): another project, a BIMCraft file or an IFC, shown in this one as a reference that can't be edited: halftone in the plans, the sections and the 3D view, and snapped to",
        "Vincular modelo (LK): outro projeto, um arquivo BIMCraft ou um IFC, mostrado neste como referência que não se edita: em meio-tom nas plantas, nos cortes e na vista 3D, e com snap nele",
        "Vincular modelo (LK): otro proyecto, un archivo BIMCraft o un IFC, mostrado en este como referencia que no se edita: en medio tono en las plantas, los cortes y la vista 3D, y con snap a él",
        "Liên kết mô hình (LK): một dự án khác, một tệp BIMCraft hoặc một tệp IFC, hiển thị trong dự án này như một tham chiếu không sửa được: bán sắc trong mặt bằng, mặt cắt và khung nhìn 3D, và bắt dính vào",
    ],
    ["Reload All", "Recarregar todos", "Recargar todos", "Nạp lại tất cả"],
    ["Origin", "Origem", "Origen", "Gốc"],
    ["Loaded", "Carregado", "Cargado", "Đã nạp"],
    ["Purge Unused", "Limpar não usados", "Purgar no usados", "Dọn phần không dùng"],
    [
        "Purge Unused (PU): take out the types, families and the rest nothing in the project uses; each category keeps a type",
        "Limpar não usados (PU): tira os tipos, famílias e o mais que nada no projeto usa; cada categoria mantém um tipo",
        "Purgar no usados (PU): quita los tipos, familias y lo demás que nada en el proyecto usa; cada categoría conserva un tipo",
        "Dọn phần không dùng (PU): bỏ ra các loại, cấu kiện và những thứ còn lại mà không gì trong dự án dùng; mỗi loại giữ lại một kiểu",
    ],
    [
        "Takes out what nothing in the project uses. Each category keeps a type; Undo brings it back.",
        "Tira o que nada no projeto usa. Cada categoria mantém um tipo; Desfazer traz de volta.",
        "Quita lo que nada en el proyecto usa. Cada categoría conserva un tipo; Deshacer lo trae de vuelta.",
        "Bỏ ra những gì không gì trong dự án dùng. Mỗi loại giữ lại một kiểu; Hoàn tác sẽ đưa chúng trở lại.",
    ],
    ["View Filters", "Filtros de vista", "Filtros de vista", "Bộ lọc khung nhìn"],
    ["Nothing is unused.", "Nada está sem uso.", "Nada está sin usar.", "Không có gì không được dùng."],
    ["Purge {}", "Limpar {}", "Purgar {}", "Dọn {}"],
    ["Purged {} unused.", "{} não usados foram limpos.", "Se purgaron {} no usados.", "Đã dọn {} phần không dùng."],
    [
        "Crop View: show only the crop region; drag its edges by their grips",
        "Recortar vista: mostrar só a região de recorte; arraste suas bordas pelas alças",
        "Recortar vista: mostrar solo la región de recorte; arrastra sus bordes por los tiradores",
        "Cắt khung nhìn: chỉ hiện vùng cắt; kéo các cạnh của nó bằng các núm",
    ],
    ["3D View", "Vista 3D", "Vista 3D", "Khung nhìn 3D"],
    ["Open the default 3D view", "Abrir a vista 3D padrão", "Abrir la vista 3D predeterminada", "Mở khung nhìn 3D mặc định"],
    [
        "Section (CR): click its start, then its end; it looks to the left of the line",
        "Corte (CR): clique no início e depois no fim; ele olha para a esquerda da linha",
        "Sección (CR): haz clic en su inicio y luego en su final; mira hacia la izquierda de la línea",
        "Mặt cắt (CR): bấm vào điểm đầu, rồi điểm cuối; nó nhìn về bên trái của đường",
    ],
    [
        "Elevation: click beside the building, on the side to look at",
        "Elevação: clique ao lado da construção, no lado a olhar",
        "Elevación: haz clic junto al edificio, en el lado que quieres ver",
        "Mặt đứng: bấm bên cạnh công trình, ở phía cần nhìn",
    ],
    ["Schedules", "Tabelas", "Tablas", "Bảng thống kê"],
    ["Open a schedule", "Abrir uma tabela", "Abrir una tabla", "Mở một bảng thống kê"],
    ["Sheet", "Folha", "Plano", "Bản vẽ"],
    ["A new sheet, with a title block", "Uma nova folha, com carimbo", "Un plano nuevo, con cajetín", "Một bản vẽ mới, kèm khung tên"],
    ["Place View", "Inserir vista", "Colocar vista", "Đặt khung nhìn"],
    [
        "Put a view on this sheet: one not on a sheet yet",
        "Colocar nesta folha uma vista que ainda não está em nenhuma",
        "Poner en este plano una vista que aún no está en ninguno",
        "Đặt một khung nhìn lên bản vẽ này: một khung nhìn chưa nằm trên bản vẽ nào",
    ],
    [
        "Every view is on a sheet.",
        "Todas as vistas estão em folhas.",
        "Todas las vistas están en planos.",
        "Mọi khung nhìn đều đã nằm trên một bản vẽ.",
    ],
    ["Close Inactive", "Fechar inativas", "Cerrar inactivas", "Đóng các khung nhìn không hoạt động"],
    [
        "Close every view but this one",
        "Fechar todas as vistas menos esta",
        "Cerrar todas las vistas menos esta",
        "Đóng mọi khung nhìn trừ khung nhìn này",
    ],
    ["User Interface", "Interface do usuário", "Interfaz de usuario", "Giao diện người dùng"],
    [
        "Show or hide the palettes, and pick the interface's language",
        "Mostrar ou ocultar as paletas, e escolher o idioma da interface",
        "Mostrar u ocultar las paletas y elegir el idioma de la interfaz",
        "Hiện hoặc ẩn các bảng, và chọn ngôn ngữ của giao diện",
    ],
    ["Project Browser", "Navegador de projeto", "Navegador de proyectos", "Trình duyệt dự án"],
    ["Language", "Idioma", "Idioma", "Ngôn ngữ"],
    ["Project Units", "Unidades do projeto", "Unidades del proyecto", "Đơn vị dự án"],
    [
        "Project Units (UN): how lengths show and read; now {}",
        "Unidades do projeto (UN): como os comprimentos aparecem e são lidos; agora {}",
        "Unidades del proyecto (UN): cómo se muestran y se leen las longitudes; ahora {}",
        "Đơn vị dự án (UN): cách độ dài hiển thị và được đọc; hiện là {}",
    ],
    ["Project Language", "Idioma do projeto", "Idioma del proyecto", "Ngôn ngữ dự án"],
    [
        "Project Language: what the drawings say and how new things are named; now {}",
        "Idioma do projeto: o que os desenhos dizem e como as coisas novas são nomeadas; agora {}",
        "Idioma del proyecto: lo que dicen los dibujos y cómo se nombran los elementos nuevos; ahora {}",
        "Ngôn ngữ dự án: điều bản vẽ in ra và cách các thứ mới được đặt tên; hiện là {}",
    ],
    ["Millimetres", "Milímetros", "Milímetros", "Milimét"],
    ["Centimetres", "Centímetros", "Centímetros", "Xentimét"],
    ["Metres", "Metros", "Metros", "Mét"],
    ["Inches", "Polegadas", "Pulgadas", "Inch"],
    ["Feet", "Pés", "Pies", "Bộ"],
    ["Feet and inches", "Pés e polegadas", "Pies y pulgadas", "Feet và inch"],
    [
        "Properties (PP): show or hide the palette",
        "Propriedades (PP): mostrar ou ocultar a paleta",
        "Propiedades (PP): mostrar u ocultar la paleta",
        "Thuộc tính (PP): hiện hoặc ẩn bảng",
    ],
    ["Type Properties", "Propriedades de tipo", "Propiedades de tipo", "Thuộc tính kiểu"],
    ["Edit the type", "Editar o tipo", "Editar el tipo", "Sửa kiểu này"],
    ["Align", "Alinhar", "Alinear", "Căn thẳng"],
    [
        "Align (AL): pick a reference line, then what to line up with it",
        "Alinhar (AL): selecione uma linha de referência e depois o que alinhar com ela",
        "Alinear (AL): elige una línea de referencia y luego lo que se alinea con ella",
        "Căn thẳng (AL): chọn một đường tham chiếu, rồi chọn thứ cần căn theo nó",
    ],
    ["Offset", "Deslocamento", "Desfase", "Offset"],
    [
        "Offset (OF): pick a wall; a copy goes to the cursor's side",
        "Deslocamento (OF): selecione uma parede; uma cópia vai para o lado do cursor",
        "Desfase (OF): elige un muro; una copia va al lado del cursor",
        "Offset (OF): chọn một tường; một bản sao đi sang phía con trỏ",
    ],
    ["Mirror Pick Axis", "Espelhar - selecionar eixo", "Reflejar - elegir eje", "Đối xứng qua trục có sẵn"],
    [
        "Mirror, Pick Axis (MM): pick a wall or grid to mirror across",
        "Espelhar, selecionar eixo (MM): selecione uma parede ou um eixo como espelho",
        "Reflejar, elegir eje (MM): elige un muro o un eje como eje de simetría",
        "Đối xứng, chọn trục (MM): chọn một tường hoặc một trục để lấy đối xứng",
    ],
    ["Mirror Draw Axis", "Espelhar - desenhar eixo", "Reflejar - dibujar eje", "Đối xứng, vẽ trục"],
    [
        "Mirror, Draw Axis (DM): click the axis's two ends",
        "Espelhar, desenhar eixo (DM): clique nas duas extremidades do eixo",
        "Reflejar, dibujar eje (DM): haz clic en los dos extremos del eje",
        "Đối xứng, vẽ trục (DM): bấm vào hai đầu của trục",
    ],
    ["Split Element", "Dividir elemento", "Dividir elemento", "Tách phần tử"],
    [
        "Split Element (SL): click a wall where to split it",
        "Dividir elemento (SL): clique numa parede onde dividi-la",
        "Dividir elemento (SL): haz clic en un muro donde dividirlo",
        "Tách phần tử (SL): bấm vào một tường tại nơi cần tách",
    ],
    ["Move", "Mover", "Mover", "Di chuyển"],
    [
        "Move (MV): click a base point, then where it goes",
        "Mover (MV): clique num ponto base e depois para onde ele vai",
        "Mover (MV): haz clic en un punto base y luego adonde va",
        "Di chuyển (MV): bấm một điểm gốc, rồi bấm nơi nó đi tới",
    ],
    ["Copy", "Copiar", "Copiar", "Sao chép"],
    [
        "Copy (CO): click a base point, then where the copy goes",
        "Copiar (CO): clique num ponto base e depois onde vai a cópia",
        "Copiar (CO): haz clic en un punto base y luego donde va la copia",
        "Sao chép (CO): bấm một điểm gốc, rồi bấm nơi bản sao đi tới",
    ],
    ["Rotate", "Rotacionar", "Rotar", "Xoay"],
    [
        "Rotate (RO): click the angle's start, then its end",
        "Rotacionar (RO): clique no início do ângulo e depois no fim",
        "Rotar (RO): haz clic en el inicio del ángulo y luego en su final",
        "Xoay (RO): bấm vào điểm đầu của góc, rồi điểm cuối",
    ],
    ["Trim/Extend to Corner", "Aparar/estender para canto", "Recortar/extender a esquina", "Cắt/Vươn tới góc"],
    [
        "Trim/Extend to Corner (TR): pick two walls, on the parts to keep",
        "Aparar/estender para canto (TR): selecione duas paredes, nas partes a manter",
        "Recortar/extender a esquina (TR): elige dos muros, en las partes que se conservan",
        "Cắt/Vươn tới góc (TR): chọn hai tường, ở phần cần giữ",
    ],
    ["Delete", "Excluir", "Eliminar", "Xóa"],
    ["Delete (DE) the selection", "Excluir (DE) a seleção", "Eliminar (DE) la selección", "Xóa (DE) phần đang chọn"],
    // The contextual tab.
    ["Place Wall", "Inserir parede", "Colocar muro", "Đặt tường"],
    ["Place Door", "Inserir porta", "Colocar puerta", "Đặt cửa đi"],
    ["Place Window", "Inserir janela", "Colocar ventana", "Đặt cửa sổ"],
    ["Place Room", "Inserir ambiente", "Colocar habitación", "Đặt phòng"],
    ["Place Level", "Inserir nível", "Colocar nivel", "Đặt cao độ"],
    ["Place Grid", "Inserir eixo", "Colocar eje", "Đặt trục"],
    ["Place Dimensions", "Inserir cotas", "Colocar cotas", "Đặt kích thước"],
    ["Place Text", "Inserir texto", "Colocar texto", "Đặt văn bản"],
    ["Place Column", "Inserir pilar", "Colocar columna", "Đặt cột"],
    ["Create Stair", "Criar escada", "Crear escalera", "Tạo cầu thang"],
    ["Create Roof Footprint", "Criar perímetro do telhado", "Crear perímetro de cubierta", "Tạo mái theo chu vi"],
    ["Multi-Select", "Seleção múltipla", "Selección múltiple", "Chọn nhiều"],
    // The Options Bar.
    ["Height:", "Altura:", "Altura:", "Chiều cao:"],
    ["Unconnected", "Desconectada", "No conectada", "Không liên kết"],
    ["Chain", "Cadeia", "Cadena", "Chuỗi"],
    ["Width:", "Largura:", "Ancho:", "Chiều rộng:"],
    ["Slope:", "Inclinação:", "Pendiente:", "Độ dốc:"],
    ["Overhang:", "Beiral:", "Voladizo:", "Đua ra:"],
    ["Gable", "Empena", "Hastial", "Đầu hồi"],
    ["Constrain", "Restringir", "Restringir", "Ràng buộc"],
    ["Multiple", "Múltiplo", "Múltiple", "Nhiều"],
    ["Leader", "Linha de chamada", "Directriz", "Đường dẫn"],
    ["Label:", "Rótulo:", "Etiqueta:", "Nhãn:"],
    [
        "a field, such as Type or Height",
        "um campo, como Type ou Height",
        "un campo, como Type o Height",
        "một trường, như Loại hoặc Chiều cao",
    ],
    ["Text:", "Texto:", "Texto:", "Văn bản:"],
    ["what the note says", "o que a nota diz", "lo que dice la nota", "nội dung ghi chú"],
    ["Offset:", "Deslocamento:", "Desfase:", "Độ lệch:"],
    ["Angle:", "Ângulo:", "Ángulo:", "Góc:"],
    ["Place Centre", "Posicionar centro", "Colocar centro", "Đặt tâm"],
    ["the next free name", "o próximo nome livre", "el siguiente nombre libre", "tên còn trống kế tiếp"],
    // The status bar's prompts.
    [
        "Click to select. Drag to orbit (or Shift and the middle button), the middle button pans, the wheel zooms.",
        "Clique para selecionar. Arraste para orbitar (ou Shift e o botão do meio); o botão do meio move a vista, a roda aproxima.",
        "Haz clic para seleccionar. Arrastra para orbitar (o Shift y el botón central); el botón central desplaza la vista, la rueda hace zoom.",
        "Bấm để chọn. Kéo để xoay vòng (hoặc Shift và nút giữa), nút giữa để dịch, con lăn để thu phóng.",
    ],
    [
        "Click a view on the sheet to select it, drag it to move it, double-click it to open it.",
        "Clique numa vista da folha para selecioná-la, arraste para movê-la, clique duas vezes para abri-la.",
        "Haz clic en una vista del plano para seleccionarla, arrástrala para moverla y haz doble clic para abrirla.",
        "Bấm vào một khung nhìn trên bản vẽ để chọn, kéo để di chuyển, bấm đúp để mở.",
    ],
    [
        "Click to select what the view cuts or sees; the middle button pans, the wheel zooms.",
        "Clique para selecionar o que a vista corta ou vê; o botão do meio move a vista, a roda aproxima.",
        "Haz clic para seleccionar lo que la vista corta o ve; el botón central desplaza la vista, la rueda hace zoom.",
        "Bấm để chọn thứ khung nhìn cắt hoặc thấy; nút giữa để dịch, con lăn để thu phóng.",
    ],
    [
        "Type the new level's elevation in the Options Bar, then Create.",
        "Digite a elevação do novo nível na Barra de opções e clique em Criar.",
        "Escribe la elevación del nivel nuevo en la barra de opciones y luego Crear.",
        "Nhập độ cao của cao độ mới vào Thanh tùy chọn, rồi bấm Tạo.",
    ],
    [
        "Type the note in the Options Bar, then click where it goes.",
        "Digite a nota na Barra de opções e clique onde ela vai.",
        "Escribe la nota en la barra de opciones y haz clic donde va.",
        "Nhập ghi chú vào Thanh tùy chọn, rồi bấm vào nơi nó đi qua.",
    ],
    [
        "Click a point to mark its height.",
        "Clique num ponto para marcar sua altura.",
        "Haz clic en un punto para marcar su altura.",
        "Bấm vào một điểm để đánh dấu chiều cao của nó.",
    ],
    [
        "This tool works in plan views: open one from the Project Browser.",
        "Esta ferramenta funciona em plantas: abra uma pelo Navegador de projeto.",
        "Esta herramienta funciona en plantas: abre una desde el navegador de proyectos.",
        "Công cụ này làm việc trong mặt bằng: hãy mở một mặt bằng từ Trình duyệt dự án.",
    ],
    [
        "Click to select, Tab for what's underneath, Ctrl adds, Shift takes away; drag to box or to move.",
        "Clique para selecionar, Tab para o que está embaixo, Ctrl adiciona, Shift remove; arraste para uma caixa ou para mover.",
        "Haz clic para seleccionar, Tab para lo que hay debajo, Ctrl agrega, Shift quita; arrastra para un recuadro o para mover.",
        "Bấm để chọn, Tab cho thứ bên dưới, Ctrl để thêm, Shift để bỏ; kéo để chọn khung hoặc để di chuyển.",
    ],
    [
        "Click to enter the wall's start point.",
        "Clique para indicar o ponto inicial da parede.",
        "Haz clic para indicar el punto inicial del muro.",
        "Bấm để nhập điểm đầu của tường.",
    ],
    [
        "Click to build the wall to the next point. Enter or Esc ends the chain, Shift keeps it square.",
        "Clique para construir a parede até o próximo ponto. Enter ou Esc termina a cadeia, Shift mantém no esquadro.",
        "Haz clic para construir el muro hasta el siguiente punto. Enter o Esc termina la cadena, Shift lo mantiene a escuadra.",
        "Bấm để dựng tường tới điểm kế tiếp. Enter hoặc Esc kết thúc chuỗi, Shift giữ nó vuông góc.",
    ],
    [
        "Click a wall to place the door.",
        "Clique numa parede para inserir a porta.",
        "Haz clic en un muro para colocar la puerta.",
        "Bấm vào một tường để đặt cửa đi.",
    ],
    [
        "Click a wall to place the window.",
        "Clique numa parede para inserir a janela.",
        "Haz clic en un muro para colocar la ventana.",
        "Bấm vào một tường để đặt cửa sổ.",
    ],
    [
        "Click inside walls to place a room; the outline shows the room you'd get.",
        "Clique dentro das paredes para inserir um ambiente; o contorno mostra o ambiente resultante.",
        "Haz clic dentro de los muros para colocar una habitación; el contorno muestra la habitación que resultaría.",
        "Bấm vào bên trong các tường để đặt một phòng; đường bao cho thấy phòng bạn sẽ có.",
    ],
    [
        "Click to enter the grid's start point.",
        "Clique para indicar o ponto inicial do eixo.",
        "Haz clic para indicar el punto inicial del eje.",
        "Bấm để nhập điểm đầu của trục.",
    ],
    [
        "Click to enter the grid's end point.",
        "Clique para indicar o ponto final do eixo.",
        "Haz clic para indicar el punto final del eje.",
        "Bấm để nhập điểm cuối của trục.",
    ],
    [
        "Click a base point to measure from.",
        "Clique num ponto base de referência.",
        "Haz clic en un punto base desde el que medir.",
        "Bấm một điểm gốc để đo từ đó.",
    ],
    [
        "Click where the base point goes, or type a distance and press Enter. Esc cancels.",
        "Clique onde vai o ponto base, ou digite uma distância e pressione Enter. Esc cancela.",
        "Haz clic donde va el punto base, o escribe una distancia y presiona Enter. Esc cancela.",
        "Bấm vào nơi điểm gốc đi tới, hoặc nhập một khoảng cách và nhấn Enter. Esc để hủy.",
    ],
    [
        "Click where the copy's base point goes, or type a distance and press Enter. Esc cancels.",
        "Clique onde vai o ponto base da cópia, ou digite uma distância e pressione Enter. Esc cancela.",
        "Haz clic donde va el punto base de la copia, o escribe una distancia y presiona Enter. Esc cancela.",
        "Bấm vào nơi điểm gốc của bản sao đi tới, hoặc nhập một khoảng cách và nhấn Enter. Esc để hủy.",
    ],
    ["Click the centre of rotation.", "Clique no centro de rotação.", "Haz clic en el centro de rotación.", "Bấm vào tâm xoay."],
    [
        "Click the start of the angle, or type an angle in the Options Bar.",
        "Clique no início do ângulo, ou digite um ângulo na Barra de opções.",
        "Haz clic en el inicio del ángulo, o escribe un ángulo en la barra de opciones.",
        "Bấm vào điểm đầu của góc, hoặc nhập một góc vào Thanh tùy chọn.",
    ],
    [
        "Click the end of the angle. Esc cancels.",
        "Clique no fim do ângulo. Esc cancela.",
        "Haz clic en el final del ángulo. Esc cancela.",
        "Bấm vào điểm cuối của góc. Esc để hủy.",
    ],
    [
        "Pick a wall or grid to mirror across.",
        "Selecione uma parede ou um eixo como espelho.",
        "Elige un muro o un eje como eje de simetría.",
        "Chọn một tường hoặc một trục để lấy đối xứng.",
    ],
    [
        "Click the start of the mirror axis.",
        "Clique no início do eixo de espelhamento.",
        "Haz clic en el inicio del eje de simetría.",
        "Bấm vào điểm đầu của trục đối xứng.",
    ],
    [
        "Click the end of the mirror axis.",
        "Clique no fim do eixo de espelhamento.",
        "Haz clic en el final del eje de simetría.",
        "Bấm vào điểm cuối của trục đối xứng.",
    ],
    [
        "Pick a reference line: a grid, or a wall's face or centre line.",
        "Selecione uma linha de referência: um eixo, ou a face ou a linha de centro de uma parede.",
        "Elige una línea de referencia: un eje, o la cara o la línea central de un muro.",
        "Chọn một đường tham chiếu: một trục, hoặc mặt hay đường tâm của một tường.",
    ],
    [
        "Pick the wall or grid to line up with the reference.",
        "Selecione a parede ou o eixo a alinhar com a referência.",
        "Elige el muro o el eje que se alinea con la referencia.",
        "Chọn tường hoặc trục để căn thẳng với đường tham chiếu.",
    ],
    [
        "Pick the first wall, on the part to keep.",
        "Selecione a primeira parede, na parte a manter.",
        "Elige el primer muro, en la parte que se conserva.",
        "Chọn tường thứ nhất, ở phần cần giữ.",
    ],
    [
        "Pick the second wall, on the part to keep.",
        "Selecione a segunda parede, na parte a manter.",
        "Elige el segundo muro, en la parte que se conserva.",
        "Chọn tường thứ hai, ở phần cần giữ.",
    ],
    [
        "Click a wall where to split it.",
        "Clique numa parede onde dividi-la.",
        "Haz clic en un muro donde dividirlo.",
        "Bấm vào một tường tại nơi cần tách.",
    ],
    [
        "Point to a wall: the offset goes to the cursor's side. Click to place it.",
        "Aponte para uma parede: o deslocamento vai para o lado do cursor. Clique para inseri-lo.",
        "Señala un muro: el desfase va al lado del cursor. Haz clic para colocarlo.",
        "Chỉ vào một tường: độ lệch đi sang phía con trỏ. Bấm để đặt nó.",
    ],
    [
        "Pick a wall face, centre line or end, a door, a window or a grid to measure from.",
        "Selecione a face, a linha de centro ou a extremidade de uma parede, uma porta, uma janela ou um eixo para medir.",
        "Elige la cara, la línea central o el extremo de un muro, una puerta, una ventana o un eje desde el que medir.",
        "Chọn một mặt tường, đường tâm hoặc đầu tường, một cửa đi, một cửa sổ hoặc một trục để đo từ đó.",
    ],
    [
        "Pick the next reference, parallel to the first.",
        "Selecione a próxima referência, paralela à primeira.",
        "Elige la siguiente referencia, paralela a la primera.",
        "Chọn tham chiếu kế tiếp, song song với tham chiếu thứ nhất.",
    ],
    [
        "Pick more references, or click away from them where the dimension line goes. Esc cancels.",
        "Selecione mais referências, ou clique longe delas onde vai a linha de cota. Esc cancela.",
        "Elige más referencias, o haz clic lejos de ellas donde va la línea de cota. Esc cancela.",
        "Chọn thêm tham chiếu, hoặc bấm ra xa chúng tại nơi đường kích thước đi qua. Esc để hủy.",
    ],
    [
        "Click an element to tag it with its type, a door's or window's with its mark, or with the field in Label.",
        "Clique num elemento para identificá-lo com seu tipo, o de uma porta ou janela com sua marca, ou com o campo em Rótulo.",
        "Haz clic en un elemento para etiquetarlo con su tipo, el de una puerta o ventana con su marca, o con el campo en Etiqueta.",
        "Bấm vào một phần tử để gắn nhãn theo loại của nó, theo ký hiệu của cửa đi hoặc cửa sổ, hoặc theo trường trong Nhãn.",
    ],
    [
        "Pick a wall, a grid or another line; then one that meets it.",
        "Selecione uma parede, um eixo ou outra linha; depois uma que a encontre.",
        "Elige un muro, un eje u otra línea; luego una que se encuentre con ella.",
        "Chọn một tường, một trục hoặc một đường khác; rồi chọn một đường gặp nó.",
    ],
    [
        "Click where the arc goes: the angle on that side. Esc cancels.",
        "Clique onde vai o arco: o ângulo desse lado. Esc cancela.",
        "Haz clic donde va el arco: el ángulo de ese lado. Esc cancela.",
        "Bấm vào nơi cung đi qua: góc ở phía đó. Esc để hủy.",
    ],
    ["Pick a round column.", "Selecione um pilar redondo.", "Elige una columna redonda.", "Chọn một cột tròn."],
    [
        "Click where the radius goes. Esc cancels.",
        "Clique onde vai o raio. Esc cancela.",
        "Haz clic donde va el radio. Esc cancela.",
        "Bấm vào nơi bán kính đi qua. Esc để hủy.",
    ],
    [
        "Type the note in the Options Bar, then click where it goes: its top left corner.",
        "Digite a nota na Barra de opções e clique onde ela vai: seu canto superior esquerdo.",
        "Escribe la nota en la barra de opciones y haz clic donde va: su esquina superior izquierda.",
        "Nhập ghi chú vào Thanh tùy chọn, rồi bấm vào nơi nó đi qua: góc trên bên trái của nó.",
    ],
    [
        "Click a floor, a roof, a stair or a wall: the spot elevation reads the height of its top there.",
        "Clique num piso, num telhado, numa escada ou numa parede: a elevação de ponto lê a altura do topo ali.",
        "Haz clic en un piso, una cubierta, una escalera o un muro: la cota de nivel lee la altura de su parte superior ahí.",
        "Bấm vào một sàn, một mái, một cầu thang hoặc một tường: cao độ điểm đọc chiều cao đỉnh của nó tại đó.",
    ],
    [
        "Click the section's start: it looks to the left of the line you draw.",
        "Clique no início do corte: ele olha para a esquerda da linha desenhada.",
        "Haz clic en el inicio de la sección: mira hacia la izquierda de la línea que dibujes.",
        "Bấm vào điểm đầu của mặt cắt: nó nhìn về bên trái của đường bạn vẽ.",
    ],
    ["Click the section's end.", "Clique no fim do corte.", "Haz clic en el final de la sección.", "Bấm vào điểm cuối của mặt cắt."],
    [
        "Click beside the building: the elevation looks at the side you click by.",
        "Clique ao lado da construção: a elevação olha para o lado onde você clicar.",
        "Haz clic junto al edificio: la elevación mira el lado junto al que hagas clic.",
        "Bấm bên cạnh công trình: mặt đứng nhìn về phía bạn bấm vào.",
    ],
    [
        "Click to place a column, as high as the Options Bar says.",
        "Clique para inserir um pilar, da altura que a Barra de opções indica.",
        "Haz clic para colocar una columna, tan alta como indica la barra de opciones.",
        "Bấm để đặt một cột, cao bằng mức Thanh tùy chọn nói.",
    ],
    [
        "Click where the stair starts: the middle of its first riser.",
        "Clique onde a escada começa: o meio do primeiro espelho.",
        "Haz clic donde empieza la escalera: el centro de su primera contrahuella.",
        "Bấm vào nơi cầu thang bắt đầu: giữa bậc đầu tiên của nó.",
    ],
    [
        "Click which way it climbs: its risers take it up to the level above. Esc cancels.",
        "Clique na direção em que ela sobe: os espelhos a levam ao nível acima. Esc cancela.",
        "Haz clic hacia donde sube: sus contrahuellas la llevan al nivel superior. Esc cancela.",
        "Bấm vào hướng nó đi lên: các bậc đưa nó lên cao độ phía trên. Esc để hủy.",
    ],
    [
        "Click inside the walls to roof them, on the level above: the overhang runs past their faces.",
        "Clique dentro das paredes para cobri-las, no nível acima: o beiral passa das faces.",
        "Haz clic dentro de los muros para cubrirlos, en el nivel superior: el voladizo sobresale de sus caras.",
        "Bấm vào bên trong các tường để lợp mái, ở cao độ phía trên: phần đua ra vượt qua các mặt của chúng.",
    ],
    // Messages the window writes itself.
    [
        "This project has no levels. Add one with the Level tool.",
        "Este projeto não tem níveis. Adicione um com a ferramenta Nível.",
        "Este proyecto no tiene niveles. Agrega uno con la herramienta Nivel.",
        "Dự án này chưa có cao độ nào. Hãy thêm một cao độ bằng công cụ Cao độ.",
    ],
    [
        "select what to move, copy, rotate or mirror first",
        "selecione primeiro o que mover, copiar, rotacionar ou espelhar",
        "selecciona primero lo que quieres mover, copiar, rotar o reflejar",
        "hãy chọn thứ cần di chuyển, sao chép, xoay hoặc lấy đối xứng trước",
    ],
    ["`{}` is not a height", "`{}` não é uma altura", "`{}` no es una altura", "`{}` không phải là một chiều cao"],
    [
        "rotate: `{}` is not an angle in degrees",
        "rotacionar: `{}` não é um ângulo em graus",
        "rotar: `{}` no es un ángulo en grados",
        "xoay: `{}` không phải là một góc tính bằng độ",
    ],
    [
        "level: `{}` is not an elevation",
        "nível: `{}` não é uma elevação",
        "nivel: `{}` no es una elevación",
        "cao độ: `{}` không phải là một độ cao",
    ],
    ["column: {}", "pilar: {}", "columna: {}", "cột: {}"],
    ["wall: {}", "parede: {}", "muro: {}", "tường: {}"],
    [
        "stair: `{}` is not a width",
        "escada: `{}` não é uma largura",
        "escalera: `{}` no es un ancho",
        "cầu thang: `{}` không phải là một chiều rộng",
    ],
    [
        "roof: the slope `{}` and the overhang `{}` must be numbers",
        "telhado: a inclinação `{}` e o beiral `{}` precisam ser números",
        "cubierta: la pendiente `{}` y el voladizo `{}` deben ser números",
        "mái: độ dốc `{}` và phần đua ra `{}` phải là các số",
    ],
    [
        "text: type what the note says in the Options Bar first",
        "texto: digite primeiro na Barra de opções o que a nota diz",
        "texto: escribe primero en la barra de opciones lo que dice la nota",
        "văn bản: hãy nhập nội dung ghi chú vào Thanh tùy chọn trước",
    ],
    [
        "align: the lines aren't parallel",
        "alinhar: as linhas não são paralelas",
        "alinear: las líneas no son paralelas",
        "căn thẳng: các đường không song song",
    ],
    [
        "offset: `{}` is not a distance",
        "deslocamento: `{}` não é uma distância",
        "desfase: `{}` no es una distancia",
        "độ lệch: `{}` không phải là một khoảng cách",
    ],
    ["`{}` is not a length", "`{}` não é um comprimento", "`{}` no es una longitud", "`{}` không phải là một chiều dài"],
    [
        "this tool works in plan views: open one from the Project Browser",
        "esta ferramenta funciona em plantas: abra uma pelo Navegador de projeto",
        "esta herramienta funciona en plantas: abre una desde el navegador de proyectos",
        "công cụ này làm việc trong mặt bằng: hãy mở một mặt bằng từ Trình duyệt dự án",
    ],
    ["`{}` is not a number", "`{}` não é um número", "`{}` no es un número", "`{}` không phải là một số"],
    [
        "`{}` is not a point; type x, y",
        "`{}` não é um ponto; digite x, y",
        "`{}` no es un punto; escribe x, y",
        "`{}` không phải là một điểm; hãy nhập x, y",
    ],
    [
        "`{}` is not a point; type x, y in mm",
        "`{}` não é um ponto; digite x, y em mm",
        "`{}` no es un punto; escribe x, y en mm",
        "`{}` không phải là một điểm; hãy nhập x, y tính bằng mm",
    ],
    ["{}: `{}` is not a length", "{}: `{}` não é um comprimento", "{}: `{}` no es una longitud", "{}: `{}` không phải là một chiều dài"],
    ["Imported {}: {}.", "Importado {}: {}.", "Importado {}: {}.", "Đã nhập {}: {}."],
    [" Left out: {}{}.", " Deixado de fora: {}{}.", " Quedó fuera: {}{}.", " Đã bỏ ra: {}{}."],
    ["; and {} more", "; e mais {}", "; y {} más", "; và {} mục nữa"],
    [
        " Moved {}, {} mm to bring it near the origin.",
        " Movido {}, {} mm para trazê-lo perto da origem.",
        " Movido {}, {} mm para acercarlo al origen.",
        " Đã dịch {}, {} mm để đưa nó lại gần gốc tọa độ.",
    ],
    [
        "Not enclosed: the walls around it don't close.",
        "Não delimitado: as paredes em volta não se fecham.",
        "No delimitada: los muros que la rodean no se cierran.",
        "Chưa khép kín: các tường quanh nó không khép lại.",
    ],
    ["Temporary dimension {}", "Cota temporária {}", "Cota temporal {}", "Kích thước tạm thời {}"],
    [
        "the opening has to stay on its wall",
        "a abertura tem de ficar na parede",
        "la abertura tiene que quedar en su muro",
        "lỗ mở phải nằm trên tường của nó",
    ],
    ["Temporary Hide/Isolate", "Ocultar/Isolar Temporariamente", "Ocultar/Aislar temporalmente", "Ẩn/Cô lập tạm thời"],
    [
        "Temporary Hide/Isolate: hide or isolate what is selected, or its categories, in this view until you reset it",
        "Ocultar/Isolar Temporariamente: oculta ou isola o que está selecionado, ou as suas categorias, nesta vista até você redefinir",
        "Ocultar/Aislar temporalmente: oculta o aísla lo seleccionado, o sus categorías, en esta vista hasta que lo restablezca",
        "Ẩn/Cô lập tạm thời: ẩn hoặc cô lập thứ đang chọn, hoặc các loại của nó, trong khung nhìn này cho tới khi bạn đặt lại",
    ],
    ["Hide Element (HH)", "Ocultar Elemento (HH)", "Ocultar elemento (HH)", "Ẩn phần tử (HH)"],
    ["Hide Category (HC)", "Ocultar Categoria (HC)", "Ocultar categoría (HC)", "Ẩn loại (HC)"],
    ["Isolate Element (HI)", "Isolar Elemento (HI)", "Aislar elemento (HI)", "Cô lập phần tử (HI)"],
    ["Isolate Category (IC)", "Isolar Categoria (IC)", "Aislar categoría (IC)", "Cô lập loại (IC)"],
    [
        "Apply Hide/Isolate to View",
        "Aplicar Ocultar/Isolar à Vista",
        "Aplicar ocultar/aislar a la vista",
        "Áp dụng Ẩn/Cô lập cho khung nhìn",
    ],
    [
        "Reset Temporary Hide/Isolate (HR)",
        "Redefinir Ocultar/Isolar Temporário (HR)",
        "Restablecer ocultar/aislar temporal (HR)",
        "Đặt lại Ẩn/Cô lập tạm thời (HR)",
    ],
    ["Reveal Hidden Elements", "Revelar Elementos Ocultos", "Revelar elementos ocultos", "Hiện các phần tử đang ẩn"],
    [
        "Reveal Hidden Elements (RH): draw what this view hides in magenta; again to hide it",
        "Revelar Elementos Ocultos (RH): desenha em magenta o que esta vista oculta; de novo para ocultar",
        "Revelar elementos ocultos (RH): dibuja en magenta lo que esta vista oculta; otra vez para ocultarlo",
        "Hiện các phần tử đang ẩn (RH): vẽ thứ khung nhìn này ẩn bằng màu hồng cánh sen; bấm lần nữa để ẩn lại",
    ],
    ["Revealing hidden elements", "Revelando elementos ocultos", "Revelando elementos ocultos", "Đang hiện các phần tử ẩn"],
    ["Temporary Hide Element", "Ocultar Elemento Temporariamente", "Ocultar elemento temporalmente", "Ẩn phần tử tạm thời"],
    ["Temporary Hide Category", "Ocultar Categoria Temporariamente", "Ocultar categoría temporalmente", "Ẩn loại tạm thời"],
    ["Temporary Isolate Element", "Isolar Elemento Temporariamente", "Aislar elemento temporalmente", "Cô lập phần tử tạm thời"],
    ["Temporary Isolate Category", "Isolar Categoria Temporariamente", "Aislar categoría temporalmente", "Cô lập loại tạm thời"],
    [
        "Reset Temporary Hide/Isolate",
        "Redefinir Ocultar/Isolar Temporário",
        "Restablecer ocultar/aislar temporal",
        "Đặt lại Ẩn/Cô lập tạm thời",
    ],
    ["{} (HR resets)", "{} (HR redefine)", "{} (HR restablece)", "{} (HR đặt lại)"],
    ["Scale", "Escala", "Escala", "Tỷ lệ"],
    ["Custom", "Personalizada", "Personalizada", "Tùy chỉnh"],
    ["`{}` is not a scale", "`{}` não é uma escala", "`{}` no es una escala", "`{}` không phải là một tỷ lệ"],
    [
        "A view always has a scale",
        "Uma vista sempre tem uma escala",
        "Una vista siempre tiene una escala",
        "Một khung nhìn luôn có một tỷ lệ",
    ],
    [
        "High Contrast Point Clouds",
        "Nuvens de pontos em alto contraste",
        "Nubes de puntos en alto contraste",
        "Đám mây điểm tương phản cao",
    ],
    [
        "High Contrast Point Clouds: draw this view's scans in one strong colour, so what they show reads; again for their own colours. A scan given a colour of its own in Visibility/Graphics keeps it.",
        "Nuvens de pontos em alto contraste: desenha as nuvens desta vista em uma só cor forte, para que se leia o que mostram; de novo para as suas próprias cores. Uma nuvem com cor própria em Visibilidade/Gráficos a mantém.",
        "Nubes de puntos en alto contraste: dibuja las nubes de esta vista en un solo color intenso, para que se lea lo que muestran; otra vez para sus propios colores. Una nube con color propio en Visibilidad/Gráficos lo conserva.",
        "Đám mây điểm tương phản cao: vẽ đám mây điểm của khung nhìn này bằng một màu mạnh, để thấy rõ chúng thể hiện gì; bấm lần nữa để trả lại màu riêng. Đám mây điểm đã có màu riêng trong Hiển thị/Đồ họa thì giữ nguyên.",
    ],
    [
        "Scale: the paper this view stands for, 1 : n of the model: its annotation and its line weights are that paper's",
        "Escala: o papel que esta vista representa, 1 : n do modelo: sua anotação e suas espessuras de linha são as desse papel",
        "Escala: el papel que esta vista representa, 1 : n del modelo: su anotación y sus grosores de línea son los de ese papel",
        "Tỷ lệ: tờ giấy mà khung nhìn này đại diện, 1 : n của mô hình: ghi chú và độ dày nét của nó là của tờ giấy đó",
    ],
    ["Hairline", "Linha capilar", "Línea capilar", "Nét mảnh"],
    ["Line Weights", "Espessuras de linha", "Grosores de línea", "Độ dày nét"],
    [
        "Line Weights: what each pen draws on paper, the model's at each scale and the annotation's at any",
        "Espessuras de linha: o que cada pena desenha no papel, as do modelo em cada escala e as de anotação em qualquer uma",
        "Grosores de línea: lo que cada plumilla dibuja en el papel, las del modelo en cada escala y las de anotación en cualquiera",
        "Độ dày nét: mỗi bút vẽ gì trên giấy, của mô hình ở mỗi tỷ lệ và của ghi chú ở mọi tỷ lệ",
    ],
    ["Model Line Weights", "Espessuras de linha do modelo", "Grosores de línea del modelo", "Độ dày nét mô hình"],
    ["Object Styles", "Estilos de objeto", "Estilos de objeto", "Kiểu đối tượng"],
    [
        "Object Styles: the pen each category draws in, seen and cut, in every view",
        "Estilos de objeto: a pena com que cada categoria é desenhada, em vista e em corte, em todas as vistas",
        "Estilos de objeto: la plumilla con la que se dibuja cada categoría, en proyección y en corte, en todas las vistas",
        "Kiểu đối tượng: bút mà mỗi loại vẽ bằng, khi nhìn và khi cắt, trong mọi khung nhìn",
    ],
    [
        "The pen each category draws its lines in, seen and cut, in every plan and section. A view's Visibility/Graphics overrides them.",
        "A pena com que cada categoria desenha suas linhas, em vista e em corte, em todas as plantas e cortes. A Visibilidade/Gráficos de uma vista as substitui.",
        "La plumilla con la que cada categoría dibuja sus líneas, en proyección y en corte, en todas las plantas y secciones. La Visibilidad/Gráficos de una vista las reemplaza.",
        "Bút mà mỗi loại vẽ các đường của nó, khi nhìn và khi cắt, trong mọi mặt bằng và mặt cắt. Hiển thị/Đồ họa của một khung nhìn sẽ ghi đè chúng.",
    ],
    [
        "Back to the pens a project starts with",
        "Voltar às penas com que um projeto começa",
        "Volver a las plumillas con las que empieza un proyecto",
        "Quay lại các bút mà một dự án khởi đầu",
    ],
    ["Annotation Line Weights", "Espessuras de linha de anotação", "Grosores de línea de anotación", "Độ dày nét ghi chú"],
    [
        "Each pen's width on paper, mm, in views at each scale: a view between two takes the nearer one's. Pen 0 is always a hairline.",
        "A largura de cada pena no papel, em mm, nas vistas em cada escala: uma vista entre duas usa a da mais próxima. A pena 0 é sempre uma linha capilar.",
        "El ancho de cada plumilla en el papel, en mm, en las vistas a cada escala: una vista entre dos toma el de la más cercana. La plumilla 0 es siempre una línea capilar.",
        "Chiều rộng của mỗi bút trên giấy, mm, trong các khung nhìn ở mỗi tỷ lệ: một khung nhìn nằm giữa hai mức sẽ lấy của mức gần hơn. Bút 0 luôn là nét mảnh.",
    ],
    [
        "Each pen's width on paper, mm, for dimensions, tags, text, grids, levels and marks, at any scale. Pen 0 is always a hairline.",
        "A largura de cada pena no papel, em mm, para cotas, etiquetas, textos, eixos, níveis e marcas, em qualquer escala. A pena 0 é sempre uma linha capilar.",
        "El ancho de cada plumilla en el papel, en mm, para cotas, etiquetas, textos, rejillas, niveles y marcas, a cualquier escala. La plumilla 0 es siempre una línea capilar.",
        "Chiều rộng của mỗi bút trên giấy, mm, cho kích thước, nhãn, văn bản, trục, cao độ và ký hiệu, ở mọi tỷ lệ. Bút 0 luôn là nét mảnh.",
    ],
    ["Pen", "Pena", "Plumilla", "Bút"],
    ["Width", "Largura", "Ancho", "Chiều rộng"],
    ["Pen {} at {}", "Pena {} em {}", "Plumilla {} a {}", "Bút {} tại {}"],
    ["Annotation pen {}", "Pena de anotação {}", "Plumilla de anotación {}", "Bút ghi chú {}"],
    ["Delete {}", "Excluir {}", "Eliminar {}", "Xóa {}"],
    ["Add Scale:", "Adicionar escala:", "Añadir escala:", "Thêm tỷ lệ:"],
    [
        "Back to the line weights a project in these units starts with",
        "Voltar às espessuras de linha com que um projeto nestas unidades começa",
        "Volver a los grosores de línea con los que empieza un proyecto en estas unidades",
        "Quay lại các độ dày nét mà một dự án theo các đơn vị này khởi đầu",
    ],
    ["Visual Style", "Estilo visual", "Estilo visual", "Kiểu hiển thị"],
    ["Shaded", "Sombreado", "Sombreado", "Có bóng"],
    ["Hidden Line", "Linhas ocultas", "Líneas ocultas", "Đường khuất"],
    ["Wireframe", "Estrutura de arame", "Estructura alámbrica", "Dạng dây"],
    [
        "Visual Style: how the 3D view draws surfaces: Shaded, Hidden Line (WF, HL and SD set them) or Wireframe",
        "Estilo visual: como a vista 3D desenha as superfícies: Sombreado, Linhas ocultas (WF, HL e SD os definem) ou Estrutura de arame",
        "Estilo visual: cómo dibuja la vista 3D las superficies: Sombreado, Líneas ocultas (WF, HL y SD los fijan) o Estructura alámbrica",
        "Kiểu hiển thị: cách khung nhìn 3D vẽ các bề mặt: Có bóng, Đường khuất (WF, HL và SD đặt chúng) hoặc Dạng dây",
    ],
    [
        "visual styles are for the 3D view: open it from the Project Browser",
        "os estilos visuais são da vista 3D: abra-a pelo Navegador de Projeto",
        "los estilos visuales son de la vista 3D: ábrala desde el Explorador de proyectos",
        "kiểu hiển thị dành cho khung nhìn 3D: hãy mở nó từ Trình duyệt dự án",
    ],
    ["Detail Level", "Nível de detalhe", "Nivel de detalle", "Mức chi tiết"],
    ["View Scale", "Escala da vista", "Escala de la vista", "Tỷ lệ khung nhìn"],
    ["Scale Value 1:", "Valor da escala 1:", "Valor de la escala 1:", "Giá trị tỷ lệ 1:"],
    ["View Scale:", "Escala da vista:", "Escala de la vista:", "Tỷ lệ khung nhìn:"],
    ["Detail Level:", "Nível de detalhe:", "Nivel de detalle:", "Mức chi tiết:"],
    ["Visual Style:", "Estilo visual:", "Estilo visual:", "Kiểu hiển thị:"],
    [
        "Set by view template {}: change the template, or stop following it.",
        "Definido pelo modelo de vista {}: mude o modelo, ou deixe de segui-lo.",
        "Definido por la plantilla de vista {}: cambie la plantilla, o deje de seguirla.",
        "Do mẫu khung nhìn {} đặt: hãy đổi mẫu, hoặc thôi theo nó.",
    ],
    ["Coarse", "Grosseiro", "Bajo", "Thô"],
    ["Medium", "Médio", "Medio", "Trung bình"],
    ["Fine", "Fino", "Alto", "Mịn"],
    [
        "Detail Level: Coarse draws what is cut in one poché, Medium adds the layers and the symbols of openings, Fine the patterns",
        "Nível de detalhe: Grosseiro desenha o que é cortado em um só poché, Médio acrescenta as camadas e os símbolos das aberturas, Fino os padrões",
        "Nivel de detalle: Bajo dibuja lo cortado en un solo relleno, Medio añade las capas y los símbolos de las aberturas, Alto las tramas",
        "Mức chi tiết: Thô vẽ thứ bị cắt trong một mảng đặc, Trung bình thêm các lớp và ký hiệu của lỗ mở, Mịn thêm các hoa văn",
    ],
    [
        "select what to hide or isolate first",
        "selecione primeiro o que ocultar ou isolar",
        "seleccione primero qué ocultar o aislar",
        "hãy chọn thứ cần ẩn hoặc cô lập trước",
    ],
    // The status bar, the warning box, the unsaved-changes dialog.
    ["Unsaved changes", "Alterações não salvas", "Cambios sin guardar", "Thay đổi chưa lưu"],
    ["Error", "Erro", "Error", "Lỗi"],
    ["Note", "Nota", "Nota", "Lưu ý"],
    ["Dismiss", "Dispensar", "Descartar", "Bỏ qua"],
    ["Save changes to {}?", "Salvar as alterações em {}?", "¿Guardar los cambios en {}?", "Lưu thay đổi vào {}?"],
    [
        "If you don't, the changes since it was last saved are lost.",
        "Se não salvar, as alterações desde a última vez em que foi salvo serão perdidas.",
        "Si no lo haces, se pierden los cambios desde la última vez que se guardó.",
        "Nếu không, các thay đổi kể từ lần lưu cuối sẽ mất.",
    ],
    ["Don't Save", "Não salvar", "No guardar", "Không lưu"],
    ["Cancel", "Cancelar", "Cancelar", "Hủy"],
    // Updates: the banner under the Options Bar, the File menu's items, what the status box says.
    [
        "BIMCraft {} is available. You have {}.",
        "O BIMCraft {} está disponível. Você tem a versão {}.",
        "BIMCraft {} está disponible. Tienes la versión {}.",
        "BIMCraft {} đã có. Bạn đang dùng {}.",
    ],
    ["Update now", "Atualizar agora", "Actualizar ahora", "Cập nhật ngay"],
    [
        "Downloads and checks the update; BIMCraft then closes, updates and starts again.",
        "Baixa e confere a atualização; depois o BIMCraft fecha, atualiza e abre de novo.",
        "Descarga y comprueba la actualización; después BIMCraft se cierra, se actualiza y se abre de nuevo.",
        "Tải và kiểm tra bản cập nhật; sau đó BIMCraft đóng lại, cập nhật và mở lại.",
    ],
    ["Download", "Baixar", "Descargar", "Tải về"],
    [
        "Opens the release page on GitHub. This copy can't update itself.",
        "Abre a página da versão no GitHub. Esta cópia não consegue se atualizar sozinha.",
        "Abre la página de la versión en GitHub. Esta copia no puede actualizarse sola.",
        "Mở trang phát hành trên GitHub. Bản này không tự cập nhật được.",
    ],
    ["Later", "Depois", "Más tarde", "Lát nữa"],
    ["Skip this version", "Ignorar esta versão", "Omitir esta versión", "Bỏ qua bản này"],
    ["What's new", "O que há de novo", "Novedades", "Có gì mới"],
    ["Downloading BIMCraft {}…", "Baixando o BIMCraft {}…", "Descargando BIMCraft {}…", "Đang tải BIMCraft {}…"],
    [
        "BIMCraft {} is ready. It will close, update and start again.",
        "O BIMCraft {} está pronto. Ele vai fechar, atualizar e abrir de novo.",
        "BIMCraft {} está listo. Se cerrará, se actualizará y se abrirá de nuevo.",
        "BIMCraft {} đã sẵn sàng. Nó sẽ đóng, cập nhật và mở lại.",
    ],
    ["Restart and install", "Reiniciar e instalar", "Reiniciar e instalar", "Khởi động lại và cài đặt"],
    [
        "Asks to save first if there are unsaved changes, then closes BIMCraft, installs the update and starts it again.",
        "Pergunta se quer salvar, se houver alterações não salvas; depois fecha o BIMCraft, instala a atualização e o abre de novo.",
        "Pregunta si guardar, si hay cambios sin guardar; después cierra BIMCraft, instala la actualización y lo abre de nuevo.",
        "Hỏi lưu trước nếu có thay đổi chưa lưu, rồi đóng BIMCraft, cài bản cập nhật và mở lại.",
    ],
    ["Installing BIMCraft {}…", "Instalando o BIMCraft {}…", "Instalando BIMCraft {}…", "Đang cài BIMCraft {}…"],
    // Revit files.
    [
        "BIMCraft project, IFC or Revit",
        "Projeto BIMCraft, IFC ou Revit",
        "Proyecto BIMCraft, IFC o Revit",
        "Dự án BIMCraft, IFC hoặc Revit",
    ],
    [
        "{}: this build of BIMCraft was made without Revit support (`--no-default-features`); build with the default `rvt` feature to read Revit files",
        "{}: esta versão do BIMCraft foi feita sem suporte ao Revit (`--no-default-features`); compile com o recurso `rvt`, que vem ativado por padrão, para ler arquivos do Revit",
        "{}: esta versión de BIMCraft se hizo sin soporte para Revit (`--no-default-features`); compila con la característica `rvt`, activada por defecto, para leer archivos de Revit",
        "{}: bản BIMCraft này được dựng không có hỗ trợ Revit (`--no-default-features`); hãy dựng với tính năng `rvt` mặc định để đọc tệp Revit",
    ],
    ["Couldn't open {}: {}", "Não foi possível abrir {}: {}", "No se pudo abrir {}: {}", "Không mở được {}: {}"],
    ["Opening a Revit file: {}", "Abrindo um arquivo do Revit: {}", "Abriendo un archivo de Revit: {}", "Đang mở một tệp Revit: {}"],
    [
        "Opened from {}: {}, {} left out.",
        "Aberto do {}: {}, {} sem importar.",
        "Abierto desde {}: {}, {} sin importar.",
        "Đã mở từ {}: {}, {} mục bị bỏ ra.",
    ],
    ["Revit import report", "Relatório de importação do Revit", "Informe de importación de Revit", "Báo cáo nhập Revit"],
    ["Came in: {}", "Importados: {}", "Importados: {}", "Đã nhập: {}"],
    ["Left out: {}", "Deixados de fora: {}", "Dejados fuera: {}", "Đã bỏ ra: {}"],
    ["Nothing was left out.", "Nada foi deixado de fora.", "No se dejó nada fuera.", "Không có gì bị bỏ ra."],
    ["Noted", "Anotações", "Observaciones", "Đã ghi"],
    ["Copy Report", "Copiar relatório", "Copiar informe", "Sao chép báo cáo"],
    ["Copied", "Copiado", "Copiado", "Đã sao chép"],
    ["1 element", "1 elemento", "1 elemento", "1 phần tử"],
    ["{} elements", "{} elementos", "{} elementos", "{} phần tử"],
    [
        "Check for updates at startup",
        "Verificar atualizações ao iniciar",
        "Buscar actualizaciones al iniciar",
        "Kiểm tra cập nhật khi khởi động",
    ],
    ["Check for Updates Now", "Verificar atualizações agora", "Buscar actualizaciones ahora", "Kiểm tra cập nhật ngay"],
    ["BIMCraft {} is up to date.", "O BIMCraft {} está atualizado.", "BIMCraft {} está actualizado.", "BIMCraft {} đã là bản mới nhất."],
    [
        "Couldn't check for updates: {}",
        "Não foi possível verificar as atualizações: {}",
        "No se pudo buscar actualizaciones: {}",
        "Không kiểm tra được cập nhật: {}",
    ],
    [
        "Couldn't download the update: {}",
        "Não foi possível baixar a atualização: {}",
        "No se pudo descargar la actualización: {}",
        "Không tải được bản cập nhật: {}",
    ],
    [
        "Couldn't install the update: {}",
        "Não foi possível instalar a atualização: {}",
        "No se pudo instalar la actualización: {}",
        "Không cài được bản cập nhật: {}",
    ],
    [
        "Couldn't save the setting: {}",
        "Não foi possível salvar a configuração: {}",
        "No se pudo guardar el ajuste: {}",
        "Không lưu được cài đặt: {}",
    ],
    // The message box a start that failed shows (apps/bimcraft/src/startup.rs).
    ["BIMCraft couldn't start", "O BIMCraft não conseguiu iniciar", "BIMCraft no pudo iniciar", "BIMCraft không khởi động được"],
    ["{}\n\nThe details are in {}.", "{}\n\nOs detalhes estão em {}.", "{}\n\nLos detalles están en {}.", "{}\n\nChi tiết nằm trong {}."],
    // Views: their kinds, the Project Browser, the view tabs, Place View.
    ["Floor Plan", "Planta de piso", "Planta", "Mặt bằng"],
    ["Schedule", "Tabela", "Tabla", "Bảng thống kê"],
    ["{}: {}", "{}: {}", "{}: {}", "{}: {}"],
    ["Floor Plan: {}", "Planta de piso: {}", "Planta: {}", "Mặt bằng: {}"],
    ["Section: {}", "Corte: {}", "Sección: {}", "Mặt cắt: {}"],
    ["Elevation: {}", "Elevação: {}", "Elevación: {}", "Mặt đứng: {}"],
    ["3D View: {3D}", "Vista 3D: {3D}", "Vista 3D: {3D}", "Khung nhìn 3D: {3D}"],
    ["Schedule: {}", "Tabela: {}", "Tabla: {}", "Bảng thống kê: {}"],
    ["Project Browser · {}", "Navegador de projeto · {}", "Navegador de proyectos · {}", "Trình duyệt dự án · {}"],
    ["Views (all)", "Vistas (todas)", "Vistas (todas)", "Khung nhìn (tất cả)"],
    ["Floor Plans", "Plantas de piso", "Plantas", "Các mặt bằng"],
    ["3D Views", "Vistas 3D", "Vistas 3D", "Khung nhìn 3D"],
    [
        "Elevations (Building Elevation)",
        "Elevações (elevação da construção)",
        "Elevaciones (elevación del edificio)",
        "Mặt đứng (mặt đứng công trình)",
    ],
    ["Sections (Building Section)", "Cortes (corte da construção)", "Secciones (sección del edificio)", "Mặt cắt (mặt cắt công trình)"],
    ["Schedules/Quantities (all)", "Tabelas/quantitativos (todas)", "Tablas/cantidades (todas)", "Bảng thống kê/Số lượng (tất cả)"],
    ["Sheets (all)", "Folhas (todas)", "Planos (todos)", "Bản vẽ (tất cả)"],
    ["Close {}", "Fechar {}", "Cerrar {}", "Đóng {}"],
    [
        "No rows yet: the {} is empty.",
        "Ainda sem linhas: a {} está vazia.",
        "Aún no hay filas: la {} está vacía.",
        "Chưa có dòng nào: {} trống.",
    ],
    // Properties and Type Properties.
    ["Edit Type", "Editar tipo", "Editar tipo", "Sửa kiểu"],
    ["Apply", "Aplicar", "Aplicar", "Áp dụng"],
    ["Type Selector", "Seletor de tipo", "Selector de tipo", "Bộ chọn kiểu"],
    ["Multiple Categories", "Várias categorias", "Varias categorías", "Nhiều loại"],
    ["Multiple Categories ({})", "Várias categorias ({})", "Varias categorías ({})", "Nhiều loại ({})"],
    ["New {}", "Inserir: {}", "Colocar: {}", "{} mới"],
    ["{} ({})", "{} ({})", "{} ({})", "{} ({})"],
    ["Constraints", "Restrições", "Restricciones", "Các ràng buộc"],
    ["Dimensions", "Dimensões", "Dimensiones", "Kích thước"],
    ["Identity Data", "Dados de identidade", "Datos de identidad", "Dữ liệu nhận dạng"],
    ["Other", "Outros", "Otros", "Khác"],
    ["Extents", "Extensões", "Extensión", "Phạm vi"],
    ["View Range", "Intervalo da vista", "Rango de vista", "Phạm vi khung nhìn"],
    ["Camera", "Câmera", "Cámara", "Máy ảnh"],
    ["Base Constraint", "Restrição da base", "Restricción de base", "Ràng buộc đáy"],
    ["Base Offset", "Deslocamento da base", "Desfase de base", "Độ lệch đáy"],
    ["Top Constraint", "Restrição superior", "Restricción superior", "Ràng buộc đỉnh"],
    ["Unconnected Height", "Altura desconectada", "Altura no conectada", "Chiều cao không liên kết"],
    ["Start", "Início", "Inicio", "Bắt đầu"],
    ["End", "Fim", "Final", "Kết thúc"],
    ["Host Wall", "Parede hospedeira", "Muro anfitrión", "Tường chủ"],
    ["Flip Facing", "Inverter face", "Invertir orientación", "Đổi hướng mặt"],
    ["Flip Hand", "Inverter lado", "Invertir lado", "Đổi tay nắm"],
    ["Sill Height", "Altura do peitoril", "Altura de antepecho", "Chiều cao ngưỡng"],
    ["Area", "Área", "Área", "Diện tích"],
    ["Perimeter", "Perímetro", "Perímetro", "Chu vi"],
    ["Location", "Localização", "Ubicación", "Vị trí"],
    ["Value", "Valor", "Valor", "Giá trị"],
    ["Kind", "Tipo de cota", "Tipo de cota", "Kiểu"],
    ["Line Through", "Linha por", "Línea por", "Đường xuyên qua"],
    ["Text Size (mm)", "Tamanho do texto (mm)", "Tamaño del texto (mm)", "Cỡ chữ (mm)"],
    ["Rotation (°)", "Rotação (°)", "Rotación (°)", "Xoay (°)"],
    ["Host", "Hospedeiro", "Anfitrión", "Chủ"],
    ["View Name", "Nome da vista", "Nombre de la vista", "Tên khung nhìn"],
    ["Category", "Categoria", "Categoría", "Phân loại"],
    ["Azimuth (°)", "Azimute (°)", "Acimut (°)", "Phương vị (°)"],
    ["Elevation (°)", "Elevação (°)", "Elevación (°)", "Độ cao (°)"],
    ["Leader Line", "Linha de chamada", "Directriz", "Đường dẫn nhãn"],
    ["Tagged Element", "Elemento identificado", "Elemento etiquetado", "Phần tử được gắn nhãn"],
    ["Sheet Number", "Número da folha", "Número de plano", "Số bản vẽ"],
    ["Sheet Name", "Nome da folha", "Nombre de plano", "Tên bản vẽ"],
    ["Size", "Tamanho", "Tamaño", "Cỡ"],
    ["View Scale (1 : n)", "Escala da vista (1 : n)", "Escala de la vista (1 : n)", "Tỷ lệ khung nhìn (1 : n)"],
    ["Base Level", "Nível da base", "Nivel de base", "Cao độ đáy"],
    ["Top Level", "Nível superior", "Nivel superior", "Cao độ đỉnh"],
    ["Actual Run Width", "Largura real do lance", "Ancho real del tramo", "Chiều rộng vế thực tế"],
    ["Maximum Riser Height", "Altura máxima do espelho", "Altura máxima de contrahuella", "Chiều cao bậc tối đa"],
    ["Actual Tread Depth", "Profundidade real do piso", "Profundidad real de huella", "Độ sâu mặt bậc thực tế"],
    ["Actual Number of Risers", "Número real de espelhos", "Número real de contrahuellas", "Số bậc thực tế"],
    ["Actual Riser Height", "Altura real do espelho", "Altura real de contrahuella", "Chiều cao bậc thực tế"],
    ["Slope (°)", "Inclinação (°)", "Pendiente (°)", "Độ dốc (°)"],
    ["Max Ridge Height", "Altura máxima da cumeeira", "Altura máxima de cumbrera", "Chiều cao đỉnh mái tối đa"],
    ["Gable Ends", "Empenas", "Hastiales", "Các đầu hồi"],
    ["Far Clip Offset", "Deslocamento do recorte distante", "Desfase del plano lejano", "Độ lệch mặt cắt xa"],
    ["No Clip", "Sem recorte", "Sin recorte", "Không cắt"],
    ["Associated Level", "Nível associado", "Nivel asociado", "Cao độ liên kết"],
    ["Level Elevation", "Elevação do nível", "Elevación del nivel", "Độ cao cao độ"],
    ["Top", "Superior", "Superior", "Đỉnh"],
    ["Cut Plane", "Plano de corte", "Plano de corte", "Mặt phẳng cắt"],
    ["Bottom", "Inferior", "Inferior", "Đáy"],
    ["View Depth", "Profundidade da vista", "Profundidad de vista", "Độ sâu khung nhìn"],
    ["Level Above", "Nível acima", "Nivel de arriba", "Cao độ phía trên"],
    ["Default Thickness", "Espessura padrão", "Espesor predeterminado", "Độ dày mặc định"],
    ["Default Sill Height", "Altura padrão do peitoril", "Altura de antepecho predeterminada", "Chiều cao ngưỡng mặc định"],
    ["Depth", "Profundidade", "Profundidad", "Chiều sâu"],
    ["Yes", "Sim", "Sí", "Có"],
    ["No", "Não", "No", "Không"],
    ["Family:", "Família:", "Familia:", "Cấu kiện:"],
    ["Type:", "Tipo:", "Tipo:", "Loại:"],
    ["Duplicate…", "Duplicar…", "Duplicar…", "Nhân bản…"],
    ["New type", "Novo tipo", "Tipo nuevo", "Kiểu mới"],
    ["Type Parameters", "Parâmetros de tipo", "Parámetros de tipo", "Tham số kiểu"],
    ["Parameter", "Parâmetro", "Parámetro", "Tham số"],
    ["OK", "OK", "Aceptar", "OK"],
    // Walls: how they're drawn, the line they stand on, what they're attached to, their layers.
    ["Draw", "Desenhar", "Dibujar", "Vẽ"],
    ["Line", "Linha", "Línea", "Đường"],
    [
        "Line: a wall along each step between the points clicked",
        "Linha: uma parede em cada trecho entre os pontos clicados",
        "Línea: un muro en cada tramo entre los puntos marcados",
        "Đường: một tường dọc theo mỗi bước giữa các điểm được bấm",
    ],
    ["Start-End-Radius Arc", "Arco início-fim-raio", "Arco inicio-fin-radio", "Cung đầu-cuối-bán kính"],
    [
        "Start-End-Radius Arc: click its start, its end, then a point it passes through",
        "Arco início-fim-raio: clique no início, no fim e depois num ponto por onde ele passa",
        "Arco inicio-fin-radio: haz clic en su inicio, en su final y luego en un punto por el que pasa",
        "Cung đầu-cuối-bán kính: bấm vào điểm đầu, điểm cuối, rồi một điểm nó đi qua",
    ],
    ["Modify Wall", "Modificar parede", "Modificar muro", "Sửa tường"],
    ["Flip", "Inverter", "Invertir", "Lật"],
    [
        "Flip (Space): the walls' exterior sides go to the other side of their location lines",
        "Inverter (Espaço): o lado externo das paredes passa para o outro lado da linha de localização",
        "Invertir (Espacio): el lado exterior de los muros pasa al otro lado de su línea de ubicación",
        "Lật (Space): mặt ngoài của các tường chuyển sang phía bên kia đường định vị của chúng",
    ],
    ["Attach Top/Base", "Anexar topo/base", "Unir parte superior/base", "Gắn đỉnh/đáy"],
    [
        "Attach Top/Base: pick a roof or a floor for the walls' tops, or bases, to follow",
        "Anexar topo/base: selecione um telhado ou um piso para o topo das paredes, ou a base, acompanhar",
        "Unir parte superior/base: elige una cubierta o un piso para que lo siga la parte superior de los muros, o su base",
        "Gắn đỉnh/đáy: chọn một mái hoặc một sàn để đỉnh, hoặc đáy, của các tường đi theo",
    ],
    ["Detach Top/Base", "Desanexar topo/base", "Separar parte superior/base", "Tách đỉnh/đáy"],
    [
        "Detach Top/Base: the walls' tops and bases their own again",
        "Desanexar topo/base: o topo e a base das paredes voltam a ser os seus",
        "Separar parte superior/base: la parte superior y la base de los muros vuelven a ser las suyas",
        "Tách đỉnh/đáy: đỉnh và đáy của các tường trở lại là của chính chúng",
    ],
    ["Location Line:", "Linha de localização:", "Línea de ubicación:", "Đường định vị:"],
    ["Location Line", "Linha de localização", "Línea de ubicación", "Đường định vị"],
    ["Wall Centreline", "Linha de centro da parede", "Línea central del muro", "Đường tâm tường"],
    ["Core Centreline", "Linha de centro do núcleo", "Línea central del núcleo", "Đường tâm lõi"],
    ["Finish Face: Exterior", "Face de acabamento: externa", "Cara de acabado: exterior", "Mặt hoàn thiện: Ngoài"],
    ["Finish Face: Interior", "Face de acabamento: interna", "Cara de acabado: interior", "Mặt hoàn thiện: Trong"],
    ["Core Face: Exterior", "Face do núcleo: externa", "Cara del núcleo: exterior", "Mặt lõi: Ngoài"],
    ["Core Face: Interior", "Face do núcleo: interna", "Cara del núcleo: interior", "Mặt lõi: Trong"],
    ["Attach Wall:", "Anexar parede:", "Unir muro:", "Gắn tường:"],
    ["Base", "Base", "Base", "Chân"],
    [
        "select the walls to attach first",
        "selecione primeiro as paredes a anexar",
        "selecciona primero los muros que quieres unir",
        "hãy chọn các tường cần gắn trước",
    ],
    [
        "offset: it offsets straight walls",
        "deslocamento: só desloca paredes retas",
        "desfase: solo desfasa muros rectos",
        "độ lệch: nó tạo độ lệch cho các tường thẳng",
    ],
    [
        "Click to enter the arc's end point.",
        "Clique para indicar o ponto final do arco.",
        "Haz clic para indicar el punto final del arco.",
        "Bấm để nhập điểm cuối của cung.",
    ],
    [
        "Click a point the arc passes through: it sets the radius. Esc cancels.",
        "Clique num ponto por onde o arco passa: ele define o raio. Esc cancela.",
        "Haz clic en un punto por el que pasa el arco: define el radio. Esc cancela.",
        "Bấm vào một điểm cung đi qua: nó đặt bán kính. Esc để hủy.",
    ],
    [
        "Pick a roof or a floor: the selected walls' bases stand on it.",
        "Selecione um telhado ou um piso: a base das paredes selecionadas apoia-se nele.",
        "Elige una cubierta o un piso: la base de los muros seleccionados se apoya en él.",
        "Chọn một mái hoặc một sàn: đáy của các tường đang chọn đứng trên nó.",
    ],
    [
        "Pick a roof or a floor: the selected walls' tops follow its underside.",
        "Selecione um telhado ou um piso: o topo das paredes selecionadas acompanha a face inferior dele.",
        "Elige una cubierta o un piso: la parte superior de los muros seleccionados sigue su cara inferior.",
        "Chọn một mái hoặc một sàn: đỉnh của các tường đang chọn đi theo mặt dưới của nó.",
    ],
    ["Base is Attached", "Base anexada", "Base unida", "Đáy đã được gắn"],
    ["Top is Attached", "Topo anexado", "Parte superior unida", "Đỉnh đã được gắn"],
    ["Radius", "Raio", "Radio", "Bán kính"],
    ["Layers", "Camadas", "Capas", "Các lớp"],
    ["Function", "Função", "Función", "Chức năng"],
    ["Material", "Material", "Material", "Vật liệu"],
    ["Out", "Para fora", "Hacia afuera", "Ngoài"],
    ["In", "Para dentro", "Hacia adentro", "Trong"],
    ["Insert Layer", "Inserir camada", "Insertar capa", "Chèn lớp"],
    ["Structure", "Estrutura", "Estructura", "Kết cấu"],
    ["Substrate", "Substrato", "Sustrato", "Lớp nền"],
    ["Thermal", "Térmica", "Térmica", "Chịu nhiệt"],
    ["Membrane", "Membrana", "Membrana", "Màng chống thấm"],
    ["Finish", "Acabamento", "Acabado", "Hoàn thiện"],
    // Floors, ceilings and shafts.
    ["Floor", "Piso", "Piso", "Sàn"],
    // The Structure tab: its panels, tools, Options Bars, prompts and Properties.
    ["Structure", "Estrutura", "Estructura", "Kết cấu"],
    ["Foundation", "Fundação", "Cimentación", "Nền móng"],
    ["Beam", "Viga", "Viga", "Dầm"],
    ["Floor: Structural", "Laje estrutural", "Losa estructural", "Sàn: Kết cấu"],
    ["Isolated Footing", "Sapata isolada", "Zapata aislada", "Móng đơn"],
    ["Wall Footing", "Sapata corrida", "Zapata corrida", "Móng băng"],
    ["Structural Column", "Pilar estrutural", "Columna estructural", "Cột kết cấu"],
    [
        "Structural Column (CL): click where it stands",
        "Pilar estrutural (CL): clique onde ele fica",
        "Columna estructural (CL): haz clic donde va",
        "Cột kết cấu (CL): bấm vào nơi cột đứng",
    ],
    [
        "Beam (BM): click its start and its end; a chain of clicks makes a beam for each step",
        "Viga (BM): clique no início e no fim; uma sequência de cliques faz uma viga para cada trecho",
        "Viga (BM): haz clic en el inicio y el final; una cadena de clics hace una viga por tramo",
        "Dầm (BM): bấm vào điểm đầu và điểm cuối; một chuỗi lần bấm tạo một dầm cho mỗi bước",
    ],
    [
        "Floor: Structural (SB): click inside the walls to floor them with a slab that carries load",
        "Laje estrutural (SB): clique dentro das paredes para cobri-las com uma laje que recebe carga",
        "Losa estructural (SB): haz clic dentro de los muros para cubrirlos con una losa que soporta carga",
        "Sàn: Kết cấu (SB): bấm vào bên trong các tường để tạo sàn bằng một tấm chịu lực",
    ],
    [
        "Isolated Footing (FI): click where the footing goes",
        "Sapata isolada (FI): clique onde a sapata fica",
        "Zapata aislada (FI): haz clic donde va la zapata",
        "Móng đơn (FI): bấm vào nơi móng đi",
    ],
    [
        "Wall Footing (FW): click its start and its end, or a chain of clicks, under a wall",
        "Sapata corrida (FW): clique no início e no fim, ou em sequência, sob uma parede",
        "Zapata corrida (FW): haz clic en el inicio y el final, o en cadena, bajo un muro",
        "Móng băng (FW): bấm vào điểm đầu và điểm cuối, hoặc một chuỗi lần bấm, dưới một tường",
    ],
    ["Place Beam", "Inserir viga", "Colocar viga", "Đặt dầm"],
    ["Place Wall Footing", "Inserir sapata corrida", "Colocar zapata corrida", "Đặt móng băng"],
    ["Place Structural Column", "Inserir pilar estrutural", "Colocar columna estructural", "Đặt cột kết cấu"],
    ["Place Isolated Footing", "Inserir sapata isolada", "Colocar zapata aislada", "Đặt móng đơn"],
    // The Plumbing tab (NBR 5626 for cold water, NBR 8160 for drains).
    ["Plumbing & Piping", "Tubulação e peças", "Fontanería y tuberías", "Cấp thoát nước"],
    ["Pipe", "Tubulação", "Tubería", "Ống"],
    ["Plumbing Fixture", "Peça sanitária", "Aparato sanitario", "Thiết bị vệ sinh"],
    ["Pipe Fitting", "Conexão de tubulação", "Conexión de tubería", "Phụ tùng ống"],
    ["Pipe Accessory", "Acessório de tubulação", "Accesorio de tubería", "Phụ kiện ống"],
    ["Place Pipe", "Inserir tubulação", "Colocar tubería", "Đặt ống"],
    ["Place Plumbing Fixture", "Inserir peça sanitária", "Colocar aparato sanitario", "Đặt thiết bị vệ sinh"],
    ["Place Pipe Fitting", "Inserir conexão", "Colocar conexión de tubería", "Đặt phụ tùng ống"],
    ["Place Pipe Accessory", "Inserir acessório", "Colocar accesorio de tubería", "Đặt phụ kiện ống"],
    [
        "Pipe (PI): click its start and its end; a chain of clicks makes a pipe for each step",
        "Tubulação (PI): clique no início e no fim; uma sequência de cliques faz uma tubulação para cada trecho",
        "Tubería (PI): haz clic en el inicio y el final; una cadena de clics hace una tubería por tramo",
        "Ống (PI): bấm vào điểm đầu và điểm cuối; một chuỗi lần bấm tạo một ống cho mỗi bước",
    ],
    [
        "Plumbing Fixture (PX): a basin, a WC, a sink, a shower or a drain, where you click; Space turns it",
        "Peça sanitária (PX): lavatório, vaso, pia, chuveiro ou ralo, onde você clicar; Espaço gira",
        "Aparato sanitario (PX): lavabo, inodoro, fregadero, ducha o sumidero, donde hagas clic; Espacio lo gira",
        "Thiết bị vệ sinh (PX): một chậu rửa, một bồn cầu, một bồn rửa bát, một sen tắm hoặc một phễu thu sàn, tại nơi bạn bấm; Space để xoay",
    ],
    [
        "Pipe Fitting (PF): an elbow or a tee, where you click on a pipe's height; Space turns it",
        "Conexão de tubulação (PF): joelho ou tê, onde você clicar, na altura da tubulação; Espaço gira",
        "Conexión de tubería (PF): codo o te, donde hagas clic, a la altura de la tubería; Espacio la gira",
        "Phụ tùng ống (PF): một cút hoặc một tê, tại nơi bạn bấm trên cao độ của ống; Space để xoay",
    ],
    [
        "Pipe Accessory (PA): a valve, where you click on a pipe's height; Space turns it",
        "Acessório de tubulação (PA): um registro, onde você clicar, na altura da tubulação; Espaço gira",
        "Accesorio de tubería (PA): una válvula, donde hagas clic, a la altura de la tubería; Espacio lo gira",
        "Phụ kiện ống (PA): một van, tại nơi bạn bấm trên cao độ của ống; Space để xoay",
    ],
    // The Electrical tab (NBR 5410).
    ["Electrical", "Elétrico", "Eléctrico", "Điện"],
    ["Conduit & Wire", "Eletrodutos e fios", "Tubos y cables", "Ống luồn dây & Dây dẫn"],
    ["Conduit", "Eletroduto", "Tubo conduit", "Ống luồn dây"],
    ["Cable Tray", "Eletrocalha", "Bandeja portacables", "Máng cáp"],
    ["Wire", "Fio", "Cable", "Dây"],
    ["Electrical Equipment", "Equipamento elétrico", "Equipo eléctrico", "Thiết bị điện"],
    ["Device", "Dispositivo", "Dispositivo", "Thiết bị"],
    ["Lighting Fixture", "Luminária", "Luminaria", "Đèn chiếu sáng"],
    ["Place Conduit", "Inserir eletroduto", "Colocar tubo conduit", "Đặt ống luồn dây"],
    ["Place Cable Tray", "Inserir eletrocalha", "Colocar bandeja portacables", "Đặt máng cáp"],
    ["Place Wire", "Inserir fio", "Colocar cable", "Đặt dây"],
    ["Place Electrical Equipment", "Inserir equipamento elétrico", "Colocar equipo eléctrico", "Đặt thiết bị điện"],
    ["Place Device", "Inserir dispositivo", "Colocar dispositivo", "Đặt thiết bị"],
    ["Place Lighting Fixture", "Inserir luminária", "Colocar luminaria", "Đặt đèn chiếu sáng"],
    [
        "Conduit (CN): click its start and its end; a chain of clicks makes a conduit for each step",
        "Eletroduto (CN): clique no início e no fim; uma sequência de cliques faz um eletroduto para cada trecho",
        "Tubo conduit (CN): haz clic en el inicio y el final; una cadena de clics hace un tubo por tramo",
        "Ống luồn dây (CN): bấm vào điểm đầu và điểm cuối; một chuỗi lần bấm tạo một ống cho mỗi bước",
    ],
    [
        "Cable Tray (CT): click its start and its end; a chain of clicks makes a tray for each step",
        "Eletrocalha (CT): clique no início e no fim; uma sequência de cliques faz uma eletrocalha para cada trecho",
        "Bandeja portacables (CT): haz clic en el inicio y el final; una cadena de clics hace una bandeja por tramo",
        "Máng cáp (CT): bấm vào điểm đầu và điểm cuối; một chuỗi lần bấm tạo một máng cho mỗi bước",
    ],
    [
        "Wire (EW): click its start and its end; a chain of clicks makes a wire for each step",
        "Fio (EW): clique no início e no fim; uma sequência de cliques faz um fio para cada trecho",
        "Cable (EW): haz clic en el inicio y el final; una cadena de clics hace un cable por tramo",
        "Dây (EW): bấm vào điểm đầu và điểm cuối; một chuỗi lần bấm tạo một dây cho mỗi bước",
    ],
    [
        "Electrical Equipment (EE): a distribution board, where you click on the wall; Space turns it",
        "Equipamento elétrico (EE): um quadro de distribuição, onde você clicar na parede; Espaço gira",
        "Equipo eléctrico (EE): un cuadro de distribución, donde hagas clic en el muro; Espacio lo gira",
        "Thiết bị điện (EE): một tủ phân phối, tại nơi bạn bấm trên tường; Space để xoay",
    ],
    [
        "Device (DV): an outlet or a switch, where you click on the wall; Space turns it",
        "Dispositivo (DV): uma tomada ou um interruptor, onde você clicar na parede; Espaço gira",
        "Dispositivo (DV): un tomacorriente o un interruptor, donde hagas clic en el muro; Espacio lo gira",
        "Thiết bị (DV): một ổ cắm hoặc một công tắc, tại nơi bạn bấm trên tường; Space để xoay",
    ],
    [
        "Lighting Fixture (LF): a luminaire, where you click, hung from the slab; Space turns it",
        "Luminária (LF): uma luminária, onde você clicar, presa à laje; Espaço gira",
        "Luminaria (LF): una luminaria, donde hagas clic, colgada de la losa; Espacio la gira",
        "Đèn chiếu sáng (LF): một đèn, tại nơi bạn bấm, treo trên sàn; Space để xoay",
    ],
    // The Lightning Protection tab (SPDA, NBR 5419).
    ["Lightning Protection", "SPDA", "SPDA", "Chống sét"],
    ["Capture & Descent", "Captação e descida", "Captación y bajada", "Kim thu & Dây xuống"],
    ["Earthing", "Aterramento", "Puesta a tierra", "Tiếp địa"],
    ["Down Conductor", "Condutor de descida", "Conductor de bajada", "Dây xuống"],
    ["Earthing Conductor", "Condutor de aterramento", "Conductor de puesta a tierra", "Dây tiếp địa"],
    ["Place Air Terminal", "Inserir captor", "Colocar terminal aéreo", "Đặt kim thu sét"],
    ["Place Down Conductor", "Inserir condutor de descida", "Colocar conductor de bajada", "Đặt dây xuống"],
    ["Place Earthing Electrode", "Inserir haste de aterramento", "Colocar electrodo de puesta a tierra", "Đặt cọc tiếp địa"],
    ["Place Inspection Box", "Inserir caixa de inspeção", "Colocar caja de inspección", "Đặt hộp kiểm tra"],
    ["Place Earthing Conductor", "Inserir condutor de aterramento", "Colocar conductor de puesta a tierra", "Đặt dây tiếp địa"],
    [
        "Air Terminal (AT): a captor, where you click on the roof; Space turns it",
        "Captor (AT): um captor, onde você clicar na cobertura; Espaço gira",
        "Terminal aéreo (AT): un captor, donde hagas clic en la cubierta; Espacio lo gira",
        "Kim thu sét (AT): một kim thu, tại nơi bạn bấm trên mái; Space để xoay",
    ],
    [
        "Down Conductor (DC): click its start and its end; the same point twice and two heights stand it up a wall",
        "Condutor de descida (DC): clique no início e no fim; o mesmo ponto duas vezes e duas alturas o põem de pé numa parede",
        "Conductor de bajada (DC): haz clic en el inicio y el final; el mismo punto dos veces y dos alturas lo ponen de pie en un muro",
        "Dây xuống (DC): bấm vào điểm đầu và điểm cuối; cùng một điểm hai lần và hai cao độ sẽ dựng nó đứng trên tường",
    ],
    [
        "Earthing Electrode (EG): a rod driven down from the level, where you click; Space turns it",
        "Haste de aterramento (EG): uma haste cravada abaixo do nível, onde você clicar; Espaço gira",
        "Electrodo de puesta a tierra (EG): una pica clavada bajo el nivel, donde hagas clic; Espacio lo gira",
        "Cọc tiếp địa (EG): một cọc đóng xuống từ cao độ, tại nơi bạn bấm; Space để xoay",
    ],
    [
        "Inspection Box (IB): over an electrode, flush with the level, where you click; Space turns it",
        "Caixa de inspeção (IB): sobre uma haste, rente ao nível, onde você clicar; Espaço gira",
        "Caja de inspección (IB): sobre un electrodo, a ras del nivel, donde hagas clic; Espacio la gira",
        "Hộp kiểm tra (IB): trên một cọc tiếp địa, ngang bằng với cao độ, tại nơi bạn bấm; Space để xoay",
    ],
    [
        "Earthing Conductor (GC): click its start and its end; a chain of clicks makes the ring or the mesh, under the level",
        "Condutor de aterramento (GC): clique no início e no fim; uma sequência de cliques faz o anel ou a malha, sob o nível",
        "Conductor de puesta a tierra (GC): haz clic en el inicio y el final; una cadena de clics hace el anillo o la malla, bajo el nivel",
        "Dây tiếp địa (GC): bấm vào điểm đầu và điểm cuối; một chuỗi lần bấm tạo vòng hoặc lưới, dưới cao độ",
    ],
    // The Data tab (lógica, dados e voz; NBR 14565).
    ["Data", "Lógica, Dados e Voz", "Datos y Voz", "Mạng"],
    ["Outlets & Equipment", "Pontos e equipamentos", "Puntos y equipos", "Ổ cắm & Thiết bị"],
    ["Cabling", "Cabeamento", "Cableado", "Hệ thống cáp"],
    ["Place Data Outlet", "Inserir ponto de dados", "Colocar punto de datos", "Đặt ổ cắm mạng"],
    ["Place Voice Outlet", "Inserir ponto de voz", "Colocar punto de voz", "Đặt ổ cắm điện thoại"],
    ["Place Rack", "Inserir rack", "Colocar rack", "Đặt tủ rack"],
    ["Place Patch Panel", "Inserir patch panel", "Colocar panel de parcheo", "Đặt bảng đấu dây"],
    ["Place Cabling", "Inserir cabeamento", "Colocar cableado", "Đặt cáp"],
    [
        "Data Outlet (DT): a data point, where you click on the wall; Space turns it",
        "Ponto de dados (DT): um ponto de dados, onde você clicar na parede; Espaço gira",
        "Punto de datos (DT): un punto de datos, donde hagas clic en el muro; Espacio lo gira",
        "Ổ cắm mạng (DT): một điểm mạng, tại nơi bạn bấm trên tường; Space để xoay",
    ],
    [
        "Voice Outlet (VO): a telephone point, where you click on the wall; Space turns it",
        "Ponto de voz (VO): um ponto de telefone, onde você clicar na parede; Espaço gira",
        "Punto de voz (VO): un punto de teléfono, donde hagas clic en el muro; Espacio lo gira",
        "Ổ cắm điện thoại (VO): một điểm điện thoại, tại nơi bạn bấm trên tường; Space để xoay",
    ],
    [
        "Rack (RK): a floor rack, where you click; Space turns it",
        "Rack (RK): um rack de piso, onde você clicar; Espaço gira",
        "Rack (RK): un rack de piso, donde hagas clic; Espacio lo gira",
        "Tủ rack (RK): một tủ rack đặt sàn, tại nơi bạn bấm; Space để xoay",
    ],
    [
        "Patch Panel (PN): a panel on the wall, where you click; Space turns it",
        "Patch panel (PN): um painel na parede, onde você clicar; Espaço gira",
        "Panel de parcheo (PN): un panel en el muro, donde hagas clic; Espacio lo gira",
        "Bảng đấu dây (PN): một bảng trên tường, tại nơi bạn bấm; Space để xoay",
    ],
    [
        "Cabling (CB): click its start and its end; a chain of clicks makes a cable for each step",
        "Cabeamento (CB): clique no início e no fim; uma sequência de cliques faz um cabo para cada trecho",
        "Cableado (CB): haz clic en el inicio y el final; una cadena de clics hace un cable por tramo",
        "Cáp (CB): bấm vào điểm đầu và điểm cuối; một chuỗi lần bấm tạo một sợi cáp cho mỗi bước",
    ],
    ["Data Outlets", "Pontos de dados", "Puntos de datos", "Ổ cắm mạng"],
    ["Voice Outlets", "Pontos de voz", "Puntos de voz", "Ổ cắm điện thoại"],
    ["Patch Panels", "Patch panels", "Paneles de parcheo", "Bảng đấu dây"],
    ["Racks", "Racks", "Racks", "Tủ rack"],
    ["Air Terminals", "Captores", "Terminales aéreos", "Kim thu sét"],
    ["Earthing Electrodes", "Hastes de aterramento", "Electrodos de puesta a tierra", "Cọc tiếp địa"],
    ["Inspection Boxes", "Caixas de inspeção", "Cajas de inspección", "Hộp kiểm tra"],
    ["Outlets", "Tomadas", "Tomacorrientes", "Ổ cắm"],
    ["Switches", "Interruptores", "Interruptores", "Công tắc"],
    ["Round", "Redonda", "Redonda", "Tròn"],
    ["Justification:", "Alinhamento:", "Alineación:", "Căn chỉnh:"],
    ["Top Offset:", "Deslocamento do topo:", "Desfase superior:", "Độ lệch đỉnh:"],
    ["Thickness:", "Espessura:", "Espesor:", "Độ dày:"],
    [
        "Click the start of the run: a chain of clicks makes one for each step, Esc ends it.",
        "Clique no início do trecho: uma sequência de cliques faz um para cada passo; Esc encerra.",
        "Haz clic en el inicio del tramo: una cadena de clics hace uno por paso; Esc termina.",
        "Bấm vào điểm đầu của đoạn: một chuỗi lần bấm tạo một đoạn cho mỗi bước, Esc kết thúc.",
    ],
    [
        "Click the end of the run. Esc stops the chain.",
        "Clique no fim do trecho. Esc interrompe a sequência.",
        "Haz clic en el final del tramo. Esc detiene la cadena.",
        "Bấm vào điểm cuối của đoạn. Esc dừng chuỗi.",
    ],
    [
        "Click to place a structural column, as high as the Options Bar says.",
        "Clique para inserir um pilar estrutural, com a altura da barra de opções.",
        "Haz clic para colocar una columna estructural, con la altura de la barra de opciones.",
        "Bấm để đặt một cột kết cấu, cao bằng mức Thanh tùy chọn nói.",
    ],
    [
        "Click to place an isolated footing: its top at the Options Bar's offset, as thick as it says.",
        "Clique para inserir uma sapata isolada: o topo no deslocamento da barra de opções, com a espessura indicada.",
        "Haz clic para colocar una zapata aislada: la cara superior en el desfase de la barra de opciones, con el espesor indicado.",
        "Bấm để đặt một móng đơn: đỉnh của nó ở độ lệch của Thanh tùy chọn, dày bằng mức nó nói.",
    ],
    ["{}: `{}` is not a height", "{}: `{}` não é uma altura", "{}: `{}` no es una altura", "{}: `{}` không phải là một chiều cao"],
    ["Reference Level", "Nível de referência", "Nivel de referencia", "Cao độ tham chiếu"],
    ["Start Level Offset", "Deslocamento inicial do nível", "Desfase inicial de nivel", "Độ lệch cao độ đầu"],
    ["End Level Offset", "Deslocamento final do nível", "Desfase final de nivel", "Độ lệch cao độ cuối"],
    ["Same as start", "Igual ao início", "Igual al inicio", "Như điểm đầu"],
    ["Z Justification", "Alinhamento em Z", "Alineación en Z", "Căn chỉnh theo Z"],
    ["System", "Sistema", "Sistema", "Hệ thống"],
    ["Structural", "Estrutural", "Estructural", "Chịu lực"],
    [
        "Floor: click inside the walls to floor them",
        "Piso: clique dentro das paredes para criar o piso sob elas",
        "Piso: haz clic dentro de los muros para crear el piso bajo ellos",
        "Sàn: bấm vào bên trong các tường để tạo sàn",
    ],
    ["Ceiling", "Forro", "Cielo raso", "Trần"],
    [
        "Ceiling: click inside a room for a ceiling out to its walls",
        "Forro: clique dentro de um ambiente para um forro até as paredes",
        "Cielo raso: haz clic dentro de una habitación para un cielo raso hasta sus muros",
        "Trần: bấm vào bên trong một phòng để tạo trần tới các tường của nó",
    ],
    ["Opening", "Abertura", "Abertura", "Lỗ mở"],
    ["Shaft", "Shaft", "Ducto", "Hộp gen"],
    [
        "Shaft Opening: click two opposite corners; it cuts the floors, ceilings and roofs it passes through",
        "Abertura de shaft: clique em dois cantos opostos; ela corta os pisos, forros e telhados por onde passa",
        "Abertura de ducto: haz clic en dos esquinas opuestas; corta los pisos, cielos rasos y cubiertas por donde pasa",
        "Lỗ mở kỹ thuật: bấm vào hai góc đối nhau; nó cắt các sàn, trần và mái mà nó xuyên qua",
    ],
    ["Shape Editing", "Edição de forma", "Edición de forma", "Sửa hình dạng"],
    ["Slope Arrow", "Seta de inclinação", "Flecha de pendiente", "Mũi tên dốc"],
    [
        "Slope Arrow: click its tail, then its head; the floor rises toward the head at the Options Bar's slope",
        "Seta de inclinação: clique na cauda e depois na ponta; o piso sobe em direção à ponta com a inclinação da barra de opções",
        "Flecha de pendiente: haz clic en la cola y luego en la punta; el piso sube hacia la punta con la pendiente de la barra de opciones",
        "Mũi tên dốc: bấm vào đuôi, rồi vào đầu; sàn dốc lên về phía đầu theo độ dốc của Thanh tùy chọn",
    ],
    ["Create Floor", "Criar piso", "Crear piso", "Tạo sàn"],
    ["Place Ceiling", "Posicionar forro", "Colocar cielo raso", "Đặt trần"],
    ["Create Shaft Opening", "Criar abertura de shaft", "Crear abertura de ducto", "Tạo lỗ mở kỹ thuật"],
    ["Ceiling Plans", "Plantas de forro", "Planos de cielo raso", "Mặt bằng trần"],
    ["Height Offset:", "Deslocamento de altura:", "Desfase de altura:", "Độ lệch chiều cao:"],
    ["Height Offset From Level", "Deslocamento de altura do nível", "Desfase de altura desde nivel", "Độ lệch chiều cao so với cao độ"],
    ["Tile Size", "Tamanho da placa", "Tamaño de placa", "Cỡ ô"],
    [
        "Click inside the walls to floor them: the slab runs under their outline, its top at the level.",
        "Clique dentro das paredes para criar o piso: a laje passa sob o contorno delas, com o topo no nível.",
        "Haz clic dentro de los muros para crear el piso: la losa pasa bajo su contorno, con la cara superior en el nivel.",
        "Bấm vào bên trong các tường để tạo sàn: tấm sàn chạy dưới đường bao của chúng, đỉnh ở cao độ.",
    ],
    [
        "Click inside a room: a ceiling out to its walls' faces, as high as the Options Bar says.",
        "Clique dentro de um ambiente: um forro até as faces das paredes, na altura que a barra de opções indica.",
        "Haz clic dentro de una habitación: un cielo raso hasta las caras de sus muros, a la altura que indica la barra de opciones.",
        "Bấm vào bên trong một phòng: một trần tới các mặt tường của nó, cao bằng mức Thanh tùy chọn nói.",
    ],
    [
        "Click a corner of the shaft opening.",
        "Clique em um canto da abertura de shaft.",
        "Haz clic en una esquina de la abertura de ducto.",
        "Bấm vào một góc của lỗ mở kỹ thuật.",
    ],
    [
        "Click the opposite corner: the shaft cuts the floors, ceilings and roofs up to its top. Esc cancels.",
        "Clique no canto oposto: o shaft corta os pisos, forros e telhados até o topo dele. Esc cancela.",
        "Haz clic en la esquina opuesta: el ducto corta los pisos, cielos rasos y cubiertas hasta su parte superior. Esc cancela.",
        "Bấm vào góc đối nhau: lỗ mở cắt các sàn, trần và mái tới đỉnh của nó. Esc để hủy.",
    ],
    [
        "Click the slope arrow's tail: where the floor's top stays at its height offset.",
        "Clique na cauda da seta de inclinação: onde o topo do piso fica no deslocamento de altura.",
        "Haz clic en la cola de la flecha de pendiente: donde la cara superior del piso queda en su desfase de altura.",
        "Bấm vào đuôi của mũi tên dốc: nơi đỉnh sàn giữ ở độ lệch chiều cao của nó.",
    ],
    [
        "Click its head: the floor rises toward it at the Options Bar's slope. Esc cancels.",
        "Clique na ponta: o piso sobe em direção a ela com a inclinação da barra de opções. Esc cancela.",
        "Haz clic en la punta: el piso sube hacia ella con la pendiente de la barra de opciones. Esc cancela.",
        "Bấm vào đầu của nó: sàn dốc lên về phía đó theo độ dốc của Thanh tùy chọn. Esc để hủy.",
    ],
    [
        "select the floor to slope first",
        "selecione primeiro o piso a inclinar",
        "selecciona primero el piso que quieres inclinar",
        "hãy chọn sàn cần tạo dốc trước",
    ],
    [
        "ceiling: `{}` is not a height",
        "forro: `{}` não é uma altura",
        "cielo raso: `{}` no es una altura",
        "trần: `{}` không phải là một chiều cao",
    ],
    ["shaft: {}", "shaft: {}", "ducto: {}", "lỗ mở kỹ thuật: {}"],
    [
        "slope arrow: `{}` is not a slope",
        "seta de inclinação: `{}` não é uma inclinação",
        "flecha de pendiente: `{}` no es una pendiente",
        "mũi tên dốc: `{}` không phải là một độ dốc",
    ],
    // Masses.
    ["Massing", "Volumetria", "Volumetría", "Khối đặc"],
    ["Conceptual Mass", "Massa conceitual", "Masa conceptual", "Khối ý niệm"],
    ["Model by Face", "Modelo por face", "Modelo por cara", "Mô hình theo mặt"],
    ["Top Outline", "Contorno superior", "Contorno superior", "Đường bao đỉnh"],
    ["Show Mass", "Mostrar massa", "Mostrar masa", "Hiện khối"],
    [
        "Show Mass (MH): hides the masses in this view, or shows them again",
        "Mostrar massa (MH): oculta as massas nesta vista, ou as mostra de novo",
        "Mostrar masa (MH): oculta las masas en esta vista, o las muestra otra vez",
        "Hiện khối (MH): ẩn các khối trong khung nhìn này, hoặc hiện chúng lại",
    ],
    ["Create Mass", "Criar massa", "Crear masa", "Tạo khối"],
    [
        "Mass (MS): click the corners of the footprint; Enter, or the first corner again, finishes; Esc cancels",
        "Massa (MS): clique nos cantos da projeção; Enter, ou o primeiro canto de novo, termina; Esc cancela",
        "Masa (MS): haz clic en las esquinas de la huella; Enter, o la primera esquina otra vez, termina; Esc cancela",
        "Khối (MS): bấm vào các góc của đường bao; Enter, hoặc bấm lại góc đầu, để kết thúc; Esc để hủy",
    ],
    ["Mass Floors", "Pisos de massa", "Pisos de masa", "Sàn khối"],
    [
        "Mass Floors (MF): a floor at each level the selected masses cross",
        "Pisos de massa (MF): um piso em cada nível que as massas selecionadas atravessam",
        "Pisos de masa (MF): un piso en cada nivel que cruzan las masas seleccionadas",
        "Sàn khối (MF): một sàn tại mỗi cao độ mà các khối đang chọn đi qua",
    ],
    ["Wall by Face", "Parede por face", "Muro por cara", "Tường theo mặt"],
    [
        "Wall by Face (WB): a wall on each edge of the selected masses' footprints",
        "Parede por face (WB): uma parede em cada aresta da projeção das massas selecionadas",
        "Muro por cara (WB): un muro en cada arista de la huella de las masas seleccionadas",
        "Tường theo mặt (WB): một tường trên mỗi cạnh đường bao của các khối đang chọn",
    ],
    ["Floor by Face", "Piso por face", "Piso por cara", "Sàn theo mặt"],
    [
        "Floor by Face (FB): a floor on the selected masses' top faces",
        "Piso por face (FB): um piso na face superior das massas selecionadas",
        "Piso por cara (FB): un piso en la cara superior de las masas seleccionadas",
        "Sàn theo mặt (FB): một sàn trên các mặt đỉnh của các khối đang chọn",
    ],
    ["Roof by Face", "Telhado por face", "Cubierta por cara", "Mái theo mặt"],
    [
        "Roof by Face (RB): a roof on the selected masses' top faces",
        "Telhado por face (RB): um telhado na face superior das massas selecionadas",
        "Cubierta por cara (RB): una cubierta en la cara superior de las masas seleccionadas",
        "Mái theo mặt (RB): một mái trên các mặt đỉnh của các khối đang chọn",
    ],
    ["Update to Face", "Atualizar para a face", "Actualizar a la cara", "Cập nhật theo mặt"],
    [
        "Update to Face (UF): the walls, floors and roofs made by face from the selected masses follow them again",
        "Atualizar para a face (UF): as paredes, pisos e telhados feitos por face a partir das massas selecionadas voltam a segui-las",
        "Actualizar a la cara (UF): los muros, pisos y cubiertas hechos por cara a partir de las masas seleccionadas vuelven a seguirlas",
        "Cập nhật theo mặt (UF): các tường, sàn và mái được tạo theo mặt từ các khối đang chọn sẽ theo chúng lần nữa",
    ],
    ["Select a mass first.", "Selecione uma massa primeiro.", "Selecciona una masa primero.", "Hãy chọn một khối trước."],
    ["mass: {}", "massa: {}", "masa: {}", "khối: {}"],
    ["Volume (m³)", "Volume (m³)", "Volumen (m³)", "Thể tích (m³)"],
    [
        "Click the first corner of the mass's footprint.",
        "Clique no primeiro canto da projeção da massa.",
        "Haz clic en la primera esquina de la huella de la masa.",
        "Bấm vào góc đầu của đường bao khối.",
    ],
    [
        "Click the footprint's next corner. Esc cancels.",
        "Clique no próximo canto da projeção. Esc cancela.",
        "Haz clic en la siguiente esquina de la huella. Esc cancela.",
        "Bấm vào góc kế tiếp của đường bao. Esc để hủy.",
    ],
    [
        "Click the next corner; Enter, or the first corner again, finishes the mass. Esc cancels.",
        "Clique no próximo canto; Enter, ou o primeiro canto de novo, termina a massa. Esc cancela.",
        "Haz clic en la siguiente esquina; Enter, o la primera esquina otra vez, termina la masa. Esc cancela.",
        "Bấm vào góc kế tiếp; Enter, hoặc bấm lại góc đầu, để kết thúc khối. Esc để hủy.",
    ],
    // Stairs, ramps and railings.
    ["Ramp", "Rampa", "Rampa", "Dốc"],
    ["Railing", "Guarda-corpo", "Barandal", "Lan can"],
    ["Railings", "Guarda-corpos", "Barandales", "Lan can"],
    [
        "Stair: click where it starts, then where its run ends; a landing turns it",
        "Escada: clique onde ela começa e depois onde o lance termina; um patamar a faz virar",
        "Escalera: haz clic donde empieza y luego donde termina su tramo; un descanso la hace girar",
        "Cầu thang: bấm vào nơi nó bắt đầu, rồi vào nơi vế của nó kết thúc; một chiếu nghỉ sẽ đổi hướng",
    ],
    [
        "Ramp: click where it starts, then where its run ends; a landing turns it",
        "Rampa: clique onde ela começa e depois onde o lance termina; um patamar a faz virar",
        "Rampa: haz clic donde empieza y luego donde termina su tramo; un descanso la hace girar",
        "Dốc: bấm vào nơi nó bắt đầu, rồi vào nơi vế của nó kết thúc; một chiếu nghỉ sẽ đổi hướng",
    ],
    [
        "Railing: click a stair or a ramp for one down each side, or click a path and press Enter",
        "Guarda-corpo: clique numa escada ou rampa para um em cada lado, ou clique um caminho e tecle Enter",
        "Barandal: haz clic en una escalera o una rampa para uno a cada lado, o haz clic en un recorrido y pulsa Enter",
        "Lan can: bấm vào một cầu thang hoặc một dốc để có một lan can mỗi bên, hoặc bấm một đường đi rồi nhấn Enter",
    ],
    ["Create Ramp", "Criar rampa", "Crear rampa", "Tạo dốc"],
    ["Create Railing Path", "Criar caminho do guarda-corpo", "Crear recorrido del barandal", "Tạo đường lan can"],
    [
        "Click where its run ends: past its last riser, one run to the level above; short of it, a landing. Esc cancels.",
        "Clique onde o lance termina: além do último espelho, um lance até o nível acima; antes dele, um patamar. Esc cancela.",
        "Haz clic donde termina el tramo: más allá de su última contrahuella, un tramo hasta el nivel superior; antes, un descanso. Esc cancela.",
        "Bấm vào nơi vế của nó kết thúc: qua bậc cuối, một vế tới cao độ phía trên; chưa tới thì là một chiếu nghỉ. Esc để hủy.",
    ],
    [
        "Click where the ramp starts: the middle of its foot.",
        "Clique onde a rampa começa: o meio do seu pé.",
        "Haz clic donde empieza la rampa: el centro de su arranque.",
        "Bấm vào nơi dốc bắt đầu: giữa chân của nó.",
    ],
    [
        "Click where its run ends: past its top, one run, as high as the Options Bar says; short of it, a landing. Esc cancels.",
        "Clique onde o lance termina: além do topo, um lance, na altura que a barra de opções indica; antes dele, um patamar. Esc cancela.",
        "Haz clic donde termina el tramo: más allá de su parte superior, un tramo, a la altura que indica la barra de opciones; antes, un descanso. Esc cancela.",
        "Bấm vào nơi vế của nó kết thúc: qua đỉnh, một vế, cao bằng mức Thanh tùy chọn nói; chưa tới thì là một chiếu nghỉ. Esc để hủy.",
    ],
    [
        "Click which way the next run goes from the landing: on, left, right, or back beside it.",
        "Clique para onde o próximo lance segue a partir do patamar: em frente, à esquerda, à direita ou de volta ao lado.",
        "Haz clic hacia donde sigue el siguiente tramo desde el descanso: de frente, a la izquierda, a la derecha o de vuelta a su lado.",
        "Bấm vào hướng vế kế tiếp đi từ chiếu nghỉ: thẳng, trái, phải, hoặc quay lại bên cạnh nó.",
    ],
    [
        "Click a stair or a ramp for a railing down each side, or the start of a path.",
        "Clique numa escada ou rampa para um guarda-corpo em cada lado, ou no início de um caminho.",
        "Haz clic en una escalera o una rampa para un barandal a cada lado, o en el inicio de un recorrido.",
        "Bấm vào một cầu thang hoặc một dốc để có một lan can mỗi bên, hoặc vào điểm đầu của một đường đi.",
    ],
    [
        "Click the path's next point. Enter finishes, Esc cancels.",
        "Clique no próximo ponto do caminho. Enter termina, Esc cancela.",
        "Haz clic en el siguiente punto del recorrido. Enter termina, Esc cancela.",
        "Bấm vào điểm kế tiếp của đường đi. Enter để kết thúc, Esc để hủy.",
    ],
    ["click where its run ends", "clique onde o lance termina", "haz clic donde termina el tramo", "bấm vào nơi vế của nó kết thúc"],
    [
        "ramp: `{}` is not a slope",
        "rampa: `{}` não é uma inclinação",
        "rampa: `{}` no es una pendiente",
        "dốc: `{}` không phải là một độ dốc",
    ],
    ["ramp: `{}` is not a width", "rampa: `{}` não é uma largura", "rampa: `{}` no es un ancho", "dốc: `{}` không phải là một chiều rộng"],
    ["ramp: {}", "rampa: {}", "rampa: {}", "dốc: {}"],
    ["{} of {} risers", "{} de {} espelhos", "{} de {} contrahuellas", "{} trong {} bậc"],
    ["{} up", "{} de subida", "{} de subida", "{} lên"],
    ["Max Slope (%)", "Inclinação máxima (%)", "Pendiente máxima (%)", "Độ dốc tối đa (%)"],
    ["Host", "Hospedeiro", "Anfitrión", "Chủ"],
    ["Side", "Lado", "Lado", "Bên"],
    ["Baluster Spacing", "Espaçamento dos balaústres", "Separación de balaustres", "Khoảng cách lan can"],
    // Roofs by extrusion, dormers and gable edges.
    ["Roof by Extrusion", "Telhado por extrusão", "Cubierta por extrusión", "Mái theo trục"],
    [
        "Roof by Extrusion: click a line's start and end, then how far across it's swept; its profile rises from the line",
        "Telhado por extrusão: clique no início e no fim de uma linha e depois até onde ele se estende; o perfil sobe a partir da linha",
        "Cubierta por extrusión: haz clic en el inicio y el final de una línea y luego hasta dónde se extiende; su perfil sube desde la línea",
        "Mái theo trục: bấm vào điểm đầu và điểm cuối của một đường, rồi vào khoảng quét ngang; biên dạng của nó dựng lên từ đường đó",
    ],
    ["Dormer", "Água-furtada", "Buhardilla", "Cửa sổ mái"],
    [
        "Dormer: click a roof's slope where its front wall goes; the roof opens under it",
        "Água-furtada: clique na água do telhado onde vai a parede da frente; o telhado se abre embaixo dela",
        "Buhardilla: haz clic en el faldón donde va su muro frontal; la cubierta se abre debajo",
        "Cửa sổ mái: bấm vào một mặt dốc của mái tại nơi tường trước của nó đi; mái mở ra bên dưới nó",
    ],
    ["Gable Edge", "Borda de empena", "Borde de hastial", "Cạnh đầu hồi"],
    [
        "Gable Edge: click an edge of a roof's footprint to stand it upright, a gable end; click again to slope it",
        "Borda de empena: clique numa borda do perímetro de um telhado para deixá-la em pé, uma empena; clique de novo para inclinar",
        "Borde de hastial: haz clic en un borde del perímetro de una cubierta para dejarlo vertical, un hastial; otra vez para inclinarlo",
        "Cạnh đầu hồi: bấm vào một cạnh đường bao của mái để dựng nó đứng, thành đầu hồi; bấm lần nữa để tạo dốc",
    ],
    ["Shed", "Meia-água", "A un agua", "Mái một dốc"],
    ["Create Extrusion Roof", "Criar telhado por extrusão", "Crear cubierta por extrusión", "Tạo mái theo trục"],
    [
        "Click the start of the line the roof's profile runs along.",
        "Clique no início da linha ao longo da qual corre o perfil do telhado.",
        "Haz clic en el inicio de la línea a lo largo de la cual va el perfil de la cubierta.",
        "Bấm vào điểm đầu của đường mà biên dạng mái chạy dọc theo.",
    ],
    [
        "Click the line's end: the profile rises from it at the Options Bar's slope.",
        "Clique no fim da linha: o perfil sobe a partir dela com a inclinação da barra de opções.",
        "Haz clic en el final de la línea: el perfil sube desde ella con la pendiente de la barra de opciones.",
        "Bấm vào điểm cuối của đường: biên dạng dựng lên từ đó theo độ dốc của Thanh tùy chọn.",
    ],
    [
        "Click how far across the roof is swept, to either side of the line. Esc cancels.",
        "Clique até onde o telhado se estende, de qualquer lado da linha. Esc cancela.",
        "Haz clic hasta dónde se extiende la cubierta, a cualquier lado de la línea. Esc cancela.",
        "Bấm vào khoảng quét ngang của mái, sang hai bên đường. Esc để hủy.",
    ],
    [
        "Click a roof's slope where the dormer's front wall goes, across the slope.",
        "Clique na água do telhado onde vai a parede da frente da água-furtada, atravessando a inclinação.",
        "Haz clic en el faldón donde va el muro frontal de la buhardilla, a lo ancho de la pendiente.",
        "Bấm vào một mặt dốc của mái tại nơi tường trước của cửa sổ mái đi, ngang qua mặt dốc.",
    ],
    [
        "Click an edge of a roof's footprint: it stands upright, a gable end; click it again to slope it.",
        "Clique numa borda do perímetro de um telhado: ela fica em pé, uma empena; clique de novo para incliná-la.",
        "Haz clic en un borde del perímetro de una cubierta: queda vertical, un hastial; haz clic otra vez para inclinarlo.",
        "Bấm vào một cạnh đường bao của mái: nó dựng đứng, thành đầu hồi; bấm lần nữa để tạo dốc.",
    ],
    ["click its line's end", "clique no fim da linha", "haz clic en el final de la línea", "bấm vào điểm cuối đường của nó"],
    [
        "roof: `{}` is not a slope",
        "telhado: `{}` não é uma inclinação",
        "cubierta: `{}` no es una pendiente",
        "mái: `{}` không phải là một độ dốc",
    ],
    [
        "dormer: the width `{}`, the height `{}` and the slope `{}` must be numbers",
        "água-furtada: a largura `{}`, a altura `{}` e a inclinação `{}` precisam ser números",
        "buhardilla: el ancho `{}`, la altura `{}` y la pendiente `{}` tienen que ser números",
        "cửa sổ mái: chiều rộng `{}`, chiều cao `{}` và độ dốc `{}` phải là các số",
    ],
    ["Extrusion Start", "Início da extrusão", "Inicio de extrusión", "Điểm đầu trục"],
    ["Extrusion End", "Fim da extrusão", "Fin de extrusión", "Điểm cuối trục"],
    // The materials library, and layered floors, roofs and ceilings.
    ["Materials", "Materiais", "Materiales", "Các vật liệu"],
    ["Component", "Componente", "Componente", "Cấu kiện"],
    [
        "Component (CM): a family's type, where you click; Space turns it",
        "Componente (CM): um tipo de família, onde você clicar; Espaço o gira",
        "Componente (CM): un tipo de familia, donde haga clic; Espacio lo gira",
        "Cấu kiện (CM): một kiểu của cấu kiện, tại nơi bạn bấm; Space để xoay",
    ],
    ["Place Component", "Inserir componente", "Colocar componente", "Đặt cấu kiện"],
    [
        "Click where the component goes; Space turns it a quarter turn.",
        "Clique onde vai o componente; Espaço o gira um quarto de volta.",
        "Haga clic donde va el componente; Espacio lo gira un cuarto de vuelta.",
        "Bấm vào nơi cấu kiện đi; Space để xoay nó một phần tư vòng.",
    ],
    ["Mode", "Modo", "Modo", "Chế độ"],
    ["Load Family", "Carregar família", "Cargar familia", "Nạp cấu kiện"],
    [
        "Load Family: a family saved as a .json file, BIMCraft's own",
        "Carregar família: uma família salva como arquivo .json, do próprio BIMCraft",
        "Cargar familia: una familia guardada como archivo .json, propia de BIMCraft",
        "Nạp cấu kiện: một cấu kiện được lưu thành tệp .json, của riêng BIMCraft",
    ],
    ["BIMCraft family", "Família do BIMCraft", "Familia de BIMCraft", "Cấu kiện BIMCraft"],
    ["Families", "Famílias", "Familias", "Cấu kiện mẫu"],
    ["Materials and Finishes", "Materiais e acabamentos", "Materiales y acabados", "Vật liệu và lớp hoàn thiện"],
    ["Material:", "Material:", "Material:", "Vật liệu:"],
    [
        "Materials: the project's library, each material's colour in 3D and the patterns its cut and its surfaces show",
        "Materiais: a biblioteca do projeto, a cor de cada material em 3D e os padrões do seu corte e das suas superfícies",
        "Materiales: la biblioteca del proyecto, el color de cada material en 3D y los patrones de su corte y sus superficies",
        "Vật liệu: thư viện của dự án, màu của mỗi vật liệu trong 3D và các hoa văn mà mặt cắt và các bề mặt của nó thể hiện",
    ],
    ["Name", "Nome", "Nombre", "Tên"],
    ["Color", "Cor", "Color", "Màu"],
    ["Cut Pattern", "Padrão de corte", "Patrón de corte", "Hoa văn cắt"],
    ["Surface Pattern", "Padrão de superfície", "Patrón de superficie", "Hoa văn bề mặt"],
    ["Close", "Fechar", "Cerrar", "Đóng"],
    ["(none)", "(nenhum)", "(ninguno)", "(không có)"],
    ["Up", "Para cima", "Arriba", "Lên"],
    ["Down", "Para baixo", "Abajo", "Xuống"],
    ["None", "Nenhum", "Ninguno", "Không có"],
    ["Solid Fill", "Preenchimento sólido", "Relleno sólido", "Tô đặc"],
    ["Diagonal", "Diagonal", "Diagonal", "Chéo"],
    ["Crosshatch", "Hachura cruzada", "Rayado cruzado", "Lưới chéo"],
    ["Horizontal", "Horizontal", "Horizontal", "Ngang"],
    ["Vertical", "Vertical", "Vertical", "Dọc"],
    ["Square Grid", "Quadriculado", "Cuadrícula", "Lưới vuông"],
    ["Brick", "Tijolo", "Ladrillo", "Gạch"],
    ["Concrete", "Concreto", "Hormigón", "Bê tông"],
    ["Insulation", "Isolamento", "Aislamiento", "Vật liệu cách nhiệt"],
    // Project parameters and schedules.
    ["Schedule/Quantities…", "Tabela/quantitativos…", "Tabla/cantidades…", "Bảng thống kê/Số lượng…"],
    [
        "Make a schedule, or open one",
        "Criar uma tabela ou abrir uma",
        "Crear una tabla o abrir una",
        "Tạo một bảng thống kê, hoặc mở một bảng có sẵn",
    ],
    ["Project Parameters", "Parâmetros do projeto", "Parámetros de proyecto", "Tham số dự án"],
    [
        "Project Parameters: values of your own each element of their categories holds, for schedules and IFC",
        "Parâmetros do projeto: valores próprios que cada elemento das suas categorias guarda, para tabelas e IFC",
        "Parámetros de proyecto: valores propios que guarda cada elemento de sus categorías, para tablas e IFC",
        "Tham số dự án: các giá trị của riêng bạn mà mỗi phần tử thuộc loại của chúng mang, cho bảng thống kê và IFC",
    ],
    ["Edit…", "Editar…", "Editar…", "Sửa…"],
    ["Fields", "Campos", "Campos", "Trường"],
    ["Filter", "Filtro", "Filtro", "Lọc"],
    ["Sorting/Grouping", "Classificação/agrupamento", "Clasificación/agrupación", "Sắp xếp/Nhóm"],
    ["Yes/No", "Sim/Não", "Sí/No", "Có/Không"],
    ["Parameter:", "Parâmetro:", "Parámetro:", "Tham số:"],
    ["Type of Parameter", "Tipo de parâmetro", "Tipo de parámetro", "Kiểu tham số"],
    ["Categories", "Categorias", "Categorías", "Các loại"],
    [
        "No project parameters yet: New makes one.",
        "Ainda não há parâmetros do projeto: Novo cria um.",
        "Aún no hay parámetros de proyecto: Nuevo crea uno.",
        "Chưa có tham số dự án nào: bấm Mới để tạo một tham số.",
    ],
    ["New Schedule", "Nova tabela", "Nueva tabla", "Bảng thống kê mới"],
    ["Category:", "Categoria:", "Categoría:", "Phân loại:"],
    ["{} Schedule", "Tabela de {}", "Tabla de {}", "Bảng thống kê {}"],
    ["Name:", "Nome:", "Nombre:", "Tên:"],
    ["Schedule Properties: {}", "Propriedades da tabela: {}", "Propiedades de tabla: {}", "Thuộc tính bảng thống kê: {}"],
    ["Available fields:", "Campos disponíveis:", "Campos disponibles:", "Các trường có sẵn:"],
    [
        "Scheduled fields (in order):",
        "Campos da tabela (em ordem):",
        "Campos de la tabla (en orden):",
        "Các trường trong bảng thống kê (theo thứ tự):",
    ],
    ["Calculated Value…", "Valor calculado…", "Valor calculado…", "Giá trị tính toán…"],
    ["Calculated Value", "Valor calculado", "Valor calculado", "Giá trị tính toán"],
    ["Move Up", "Mover para cima", "Subir", "Chuyển lên"],
    ["Move Down", "Mover para baixo", "Bajar", "Chuyển xuống"],
    ["Heading", "Cabeçalho", "Encabezado", "Tiêu đề"],
    ["Calculate totals", "Calcular totais", "Calcular totales", "Tính tổng"],
    ["Formula", "Fórmula", "Fórmula", "Công thức"],
    ["Add", "Adicionar", "Añadir", "Thêm"],
    ["Filter by:", "Filtrar por:", "Filtrar por:", "Lọc theo:"],
    ["Remove", "Remover", "Quitar", "Gỡ"],
    ["Add Filter", "Adicionar filtro", "Añadir filtro", "Thêm bộ lọc"],
    ["Sort by:", "Classificar por:", "Ordenar por:", "Sắp xếp theo:"],
    ["Then by:", "Depois por:", "Luego por:", "Rồi theo:"],
    ["Ascending", "Crescente", "Ascendente", "Tăng dần"],
    ["Descending", "Decrescente", "Descendente", "Giảm dần"],
    ["Add Sort", "Adicionar classificação", "Añadir orden", "Thêm sắp xếp"],
    ["Itemize every instance", "Detalhar cada instância", "Detallar cada ejemplar", "Liệt kê từng trường hợp"],
    ["equals", "é igual a", "es igual a", "bằng"],
    ["does not equal", "não é igual a", "no es igual a", "không bằng"],
    ["is greater than", "é maior que", "es mayor que", "lớn hơn"],
    ["is greater than or equal to", "é maior ou igual a", "es mayor o igual que", "lớn hơn hoặc bằng"],
    ["is less than", "é menor que", "es menor que", "nhỏ hơn"],
    ["is less than or equal to", "é menor ou igual a", "es menor o igual que", "nhỏ hơn hoặc bằng"],
    ["contains", "contém", "contiene", "chứa"],
    ["does not contain", "não contém", "no contiene", "không chứa"],
    ["begins with", "começa com", "empieza por", "bắt đầu bằng"],
    ["does not begin with", "não começa com", "no empieza por", "không bắt đầu bằng"],
    ["ends with", "termina com", "termina en", "kết thúc bằng"],
    ["does not end with", "não termina com", "no termina en", "không kết thúc bằng"],
    ["has a value", "tem um valor", "tiene un valor", "có giá trị"],
    ["has no value", "não tem valor", "no tiene valor", "không có giá trị"],
    ["Edit {}", "Editar {}", "Editar {}", "Sửa {}"],
    ["And:", "E:", "Y:", "Và:"],
    ["Comparison", "Comparação", "Comparación", "So sánh"],
    ["Order", "Ordem", "Orden", "Thứ tự"],
    // Revit's shortcuts: Keyboard Shortcuts (KS), the names it gives those no ribbon button has, and what Create
    // Similar and Select All Instances say when nothing fits.
    ["Keyboard Shortcuts", "Teclas de Atalho", "Atajos de teclado", "Phím tắt"],
    ["Shortcuts", "Teclas de Atalho", "Atajos de teclado", "Phím tắt"],
    [
        "Keyboard Shortcuts (KS): customize, edit, and load shortcut keys",
        "Teclas de Atalho (KS): personalize, edite e carregue teclas de atalho",
        "Atajos de teclado (KS): personalizar, editar y cargar teclas de atajo",
        "Phím tắt (KS): tùy chỉnh, sửa và tải các phím tắt",
    ],
    ["Load File...", "Carregar arquivo...", "Cargar archivo...", "Tải tệp..."],
    ["Add Shortcut", "Adicionar tecla de atalho", "Añadir atajo", "Thêm phím tắt"],
    ["Restore Defaults", "Restaurar padrões", "Restaurar valores predeterminados", "Khôi phục mặc định"],
    ["Shortcut Keys:", "Teclas de atalho:", "Teclas de método abreviado:", "Phím tắt:"],
    ["Command / Tool:", "Comando / Ferramenta:", "Comando / Herramienta:", "Lệnh / Công cụ:"],
    ["Edit", "Editar", "Editar", "Sửa"],
    [
        "The shortcut '{}' already exists for '{}'.",
        "O atalho '{}' já existe para '{}'.",
        "El atajo '{}' ya existe para '{}'.",
        "Phím tắt '{}' đã tồn tại cho '{}'.",
    ],
    [
        "Shortcut '{}' added for '{}'.",
        "Atalho '{}' adicionado para '{}'.",
        "Atajo '{}' añadido para '{}'.",
        "Đã thêm phím tắt '{}' cho '{}'.",
    ],
    ["Shortcut '{}' updated.", "Atalho '{}' atualizado.", "Atajo '{}' actualizado.", "Đã cập nhật phím tắt '{}'."],
    [
        "Loaded {} shortcuts from file.",
        "Carregados {} atalhos do arquivo.",
        "Cargados {} atajos del archivo.",
        "Đã tải {} phím tắt từ tệp.",
    ],
    [
        "That is too big to be a shortcuts file.",
        "Isso é grande demais para ser um arquivo de atalhos.",
        "Eso es demasiado grande para ser un archivo de atajos.",
        "Tệp này quá lớn để là tệp phím tắt.",
    ],
    ["{} Skipped {}.", "{} Ignorados: {}.", "{} Omitidos: {}.", "{} Đã bỏ qua {}."],
    [
        "Shortcut keys are two characters, letters or numbers.",
        "As teclas de atalho têm dois caracteres, letras ou números.",
        "Las teclas de atajo son dos caracteres, letras o números.",
        "Phím tắt gồm hai ký tự, chữ cái hoặc chữ số.",
    ],
    [
        "Select an element first to match its properties",
        "Selecione um elemento primeiro para igualar suas propriedades",
        "Seleccione un elemento primero para igualar sus propiedades",
        "Hãy chọn một phần tử trước để khớp thuộc tính của nó",
    ],
    [
        "The selected element has no type to match",
        "O elemento selecionado não possui tipo para igualar",
        "El elemento seleccionado no tiene tipo para igualar",
        "Phần tử đã chọn không có kiểu để khớp",
    ],
    [
        "Match Properties: source type '{}' selected. Select target elements and press MA again.",
        "Igualar propriedades: tipo de origem '{}' selecionado. Selecione os elementos de destino e pressione MA novamente.",
        "Igualar propiedades: tipo de origen '{}' seleccionado. Seleccione los elementos de destino y presione MA nuevamente.",
        "Khớp thuộc tính: đã chọn kiểu nguồn '{}'. Hãy chọn các phần tử đích và nhấn MA lần nữa.",
    ],
    [
        "Matched properties: applied type '{}' to {} element(s)",
        "Propriedades igualadas: tipo '{}' aplicado a {} elemento(s)",
        "Propiedades igualadas: tipo '{}' aplicado a {} elemento(s)",
        "Đã khớp thuộc tính: áp dụng kiểu '{}' cho {} phần tử",
    ],
    [
        "Select an action to assign.",
        "Selecione uma ação para atribuir.",
        "Seleccione una acción para asignar.",
        "Hãy chọn một hành động để gán.",
    ],
    ["Failed to read file: {}", "Falha ao ler arquivo: {}", "Error al leer el archivo: {}", "Không đọc được tệp: {}"],
    ["Match Properties", "Igualar propriedades", "Igualar propiedades", "Khớp thuộc tính"],
    ["Create Similar", "Criar similar", "Crear similar", "Tạo tương tự"],
    ["Select All Instances", "Selecionar todas as instâncias", "Seleccionar todos los ejemplares", "Chọn mọi trường hợp"],
    ["Aligned Dimension", "Cota alinhada", "Cota alineada", "Kích thước thẳng hàng"],
    ["Split", "Dividir", "Dividir", "Tách"],
    [
        "select one element to create one like it first",
        "selecione primeiro um elemento para criar outro igual",
        "selecciona primero un elemento para crear otro igual",
        "hãy chọn một phần tử để tạo một phần tử giống nó trước",
    ],
    [
        "select an element that has a type first",
        "selecione primeiro um elemento que tenha um tipo",
        "selecciona primero un elemento que tenga un tipo",
        "hãy chọn một phần tử có kiểu trước",
    ],
    // Insert > Point Cloud.
    ["Insert", "Inserir", "Insertar", "Chèn"],
    ["Link", "Vincular", "Vincular", "Liên kết"],
    [
        "Point Cloud: laser scans (PTS, XYZ, PLY, LAS or E57) brought into the project, one file or a survey's stations as one scan, drawn in the 3D view, plans and sections and snapped to",
        "Nuvem de pontos: escaneamentos a laser (PTS, XYZ, PLY, LAS ou E57) trazidos para o projeto, um arquivo ou as estações de um levantamento como um só escaneamento, desenhados na vista 3D, nas plantas e nos cortes e usados como snap",
        "Nube de puntos: escaneos láser (PTS, XYZ, PLY, LAS o E57) traídos al proyecto, un archivo o las estaciones de un levantamiento como un solo escaneo, dibujados en la vista 3D, las plantas y los cortes y usados como referencia",
        "Đám mây điểm: bản quét laser (PTS, XYZ, PLY, LAS hoặc E57) mang vào dự án, một tệp hay các trạm đo của một khảo sát thành một bản quét, được vẽ trong khung nhìn 3D, mặt bằng và mặt cắt và được bắt dính",
    ],
    ["Point clouds", "Nuvens de pontos", "Nubes de puntos", "Đám mây điểm"],
    [
        "{} files, brought in as one scan's stations",
        "{} arquivos, trazidos como as estações de um só escaneamento",
        "{} archivos, traídos como las estaciones de un solo escaneo",
        "{} tệp, mang vào như các trạm đo của một bản quét",
    ],
    ["Units", "Unidades", "Unidades", "Đơn vị"],
    ["Spacing (mm)", "Espaçamento (mm)", "Espaciado (mm)", "Khoảng cách (mm)"],
    ["all the points", "todos os pontos", "todos los puntos", "tất cả các điểm"],
    ["Positioning", "Posicionamento", "Posicionamiento", "Định vị"],
    ["Auto - Center to Center", "Auto - Centro a centro", "Auto - Centro a centro", "Tự động - Tâm đến tâm"],
    ["Auto - Origin to Origin", "Auto - Origem a origem", "Auto - Origen a origen", "Tự động - Gốc đến gốc"],
    [
        "Its middle on the project's origin and its foot on the level",
        "O meio dele na origem do projeto e a base dele no nível",
        "Su centro en el origen del proyecto y su base en el nivel",
        "Tâm của nó tại gốc dự án và chân của nó trên cao độ",
    ],
    [
        "Where its file's coordinates put it, as the survey placed it",
        "Onde as coordenadas do arquivo o colocam, como o levantamento o posicionou",
        "Donde lo ponen las coordenadas de su archivo, como lo situó el levantamiento",
        "Nơi tọa độ trong tệp của nó đặt nó, đúng như khảo sát đã đặt",
    ],
    ["Scan", "Escaneamento", "Escaneo", "Bản quét"],
    // Snaps and the ruler.
    ["Measure", "Medir", "Medir", "Đo"],
    [
        "Measure (ME): click two points; their length and angle read beside the ruler between them",
        "Medir (ME): clique em dois pontos; o comprimento e o ângulo aparecem ao lado da régua entre eles",
        "Medir (ME): haga clic en dos puntos; su longitud y su ángulo se leen junto a la regla entre ellos",
        "Đo (ME): bấm vào hai điểm; chiều dài và góc của chúng hiện bên cạnh thước giữa chúng",
    ],
    ["Snaps", "Snaps", "Referencias", "Bắt dính"],
    ["Snap to Endpoints", "Snap em extremidades", "Referencia a extremos", "Bắt dính vào điểm cuối"],
    ["Snap to Midpoints", "Snap em pontos médios", "Referencia a puntos medios", "Bắt dính vào điểm giữa"],
    ["Snap to Intersections", "Snap em interseções", "Referencia a intersecciones", "Bắt dính vào giao điểm"],
    ["Snap to Centres", "Snap em centros", "Referencia a centros", "Bắt dính vào tâm"],
    ["Snap to Perpendiculars", "Snap em perpendiculares", "Referencia a perpendiculares", "Bắt dính vào điểm vuông góc"],
    ["Snap to Nearest", "Snap no ponto mais próximo", "Referencia al punto más cercano", "Bắt dính vào điểm gần nhất"],
    ["Snaps Off", "Desligar snaps", "Desactivar referencias", "Tắt bắt dính"],
    ["Turn Snap Override Off", "Cancelar a exceção de snap", "Cancelar la excepción de referencia", "Tắt ghi đè bắt dính"],
    ["Snap: {} (next point)", "Snap: {} (próximo ponto)", "Referencia: {} (próximo punto)", "Bắt dính: {} (điểm kế tiếp)"],
    [
        "Snaps off (next point)",
        "Snaps desligados (próximo ponto)",
        "Referencias desactivadas (próximo punto)",
        "Đã tắt bắt dính (điểm kế tiếp)",
    ],
    [
        "Snaps: what a picked point sticks to (ends, middles, crossings, centres, perpendiculars), and the angles it steps by; SE, SM, SI, SC, SP and SN take one alone for the next point, SO none",
        "Snaps: a que um ponto clicado se prende (extremidades, meios, cruzamentos, centros, perpendiculares) e os ângulos em que ele avança; SE, SM, SI, SC, SP e SN tomam um só para o próximo ponto, SO nenhum",
        "Referencias: a qué se pega un punto señalado (extremos, medios, cruces, centros, perpendiculares) y los ángulos de sus pasos; SE, SM, SI, SC, SP y SN toman una sola para el próximo punto, SO ninguna",
        "Bắt dính: điểm được chọn sẽ dính vào gì (đầu, giữa, giao, tâm, vuông góc), và các góc mà nó bước theo; SE, SM, SI, SC, SP và SN chỉ lấy một kiểu cho điểm kế tiếp, SO thì không lấy gì",
    ],
    ["Object Snaps", "Snaps de objeto", "Referencias a objetos", "Bắt dính đối tượng"],
    ["Angle steps", "Passos de ângulo", "Pasos de ángulo", "Bước góc"],
    ["Off", "Desligado", "Desactivado", "Tắt"],
    ["Endpoint", "Extremidade", "Extremo", "Điểm cuối"],
    ["Midpoint", "Ponto médio", "Punto medio", "Điểm giữa"],
    ["Intersection", "Interseção", "Intersección", "Giao điểm"],
    ["Centre", "Centro", "Centro", "Tâm"],
    ["Perpendicular", "Perpendicular", "Perpendicular", "Vuông góc"],
    ["Nearest", "Mais próximo", "Más cercano", "Gần nhất"],
    ["Point Cloud", "Nuvem de pontos", "Nube de puntos", "Đám mây điểm"],
    [
        "A scan's points where the plan's cut plane cuts it, to trace what was measured",
        "Os pontos de um escaneamento onde o plano de corte da planta o corta, para seguir o que foi medido",
        "Los puntos de un escaneo donde el plano de corte de la planta lo corta, para seguir lo que se midió",
        "Điểm của một bản quét nơi mặt phẳng cắt của mặt bằng cắt qua nó, để dò lại những gì đã đo",
    ],
    [
        "The ends of walls, grids and what else is drawn, and the points picked so far",
        "As extremidades de paredes, eixos e do que mais estiver desenhado, e os pontos já clicados",
        "Los extremos de muros, ejes y lo demás dibujado, y los puntos ya señalados",
        "Đầu của các tường, trục và những gì khác được vẽ, và các điểm đã chọn cho tới giờ",
    ],
    [
        "The middle of a wall, along its arc",
        "O meio de uma parede, ao longo do seu arco",
        "El medio de un muro, a lo largo de su arco",
        "Giữa của một tường, dọc theo cung của nó",
    ],
    [
        "Where two walls or grids cross",
        "Onde duas paredes ou eixos se cruzam",
        "Donde se cruzan dos muros o ejes",
        "Nơi hai tường hoặc hai trục giao nhau",
    ],
    [
        "The centre of a curved wall, a column or a component",
        "O centro de uma parede curva, de um pilar ou de um componente",
        "El centro de un muro curvo, de una columna o de un componente",
        "Tâm của một tường cong, một cột hoặc một cấu kiện",
    ],
    [
        "The foot of the perpendicular from the last point to a wall or grid",
        "O pé da perpendicular do último ponto até uma parede ou eixo",
        "El pie de la perpendicular desde el último punto a un muro o eje",
        "Chân đường vuông góc từ điểm cuối cùng tới một tường hoặc một trục",
    ],
    [
        "A wall's centre line, so a wall drawn on it meets it",
        "O eixo central de uma parede, para que uma parede desenhada nele se encontre com ela",
        "La línea central de un muro, para que un muro dibujado sobre ella se encuentre con él",
        "Đường tâm của một tường, để một tường vẽ trên đó sẽ gặp nó",
    ],
    [
        "A direction from the last point sticks to multiples of this angle, from east and from the line before; a dashed guide shows it",
        "Uma direção a partir do último ponto se prende a múltiplos deste ângulo, a partir do leste e da linha anterior; uma guia tracejada a mostra",
        "Una dirección desde el último punto se pega a múltiplos de este ángulo, desde el este y desde la línea anterior; una guía discontinua la muestra",
        "Một hướng từ điểm cuối cùng sẽ dính vào các bội của góc này, tính từ hướng đông và từ đường trước đó; một đường dẫn nét đứt cho thấy nó",
    ],
    [
        "Click the ruler's start: points snap as they do drawing.",
        "Clique no início da régua: os pontos se prendem como ao desenhar.",
        "Haga clic en el inicio de la regla: los puntos se pegan como al dibujar.",
        "Bấm vào điểm đầu của thước: các điểm bắt dính như khi vẽ.",
    ],
    [
        "Click its end: its length and angle read beside it. Esc cancels.",
        "Clique no fim: o comprimento e o ângulo aparecem ao lado. Esc cancela.",
        "Haga clic en el final: su longitud y su ángulo se leen al lado. Esc cancela.",
        "Bấm vào điểm cuối: chiều dài và góc của nó hiện bên cạnh. Esc để hủy.",
    ],
    [
        "Turn the wall about its {}: {}",
        "Girar a parede em torno do seu {}: {}",
        "Girar el muro alrededor de su {}: {}",
        "Xoay tường quanh {} của nó: {}",
    ],
    ["start", "início", "inicio", "điểm đầu"],
    ["end", "fim", "fin", "điểm cuối"],
    ["`{}` is not an angle", "`{}` não é um ângulo", "`{}` no es un ángulo", "`{}` không phải là một góc"],
    ["<None>", "<Nenhum>", "<Ninguno>", "<Không có>"],
    ["<Pick>", "<Escolher>", "<Elegir>", "<Chọn>"],
    [
        "A filter is made in View > Filters",
        "Um filtro é criado em Vista > Filtros",
        "Un filtro se crea en Vista > Filtros",
        "Một bộ lọc được tạo trong Khung nhìn > Bộ lọc",
    ],
    ["Add Selected", "Adicionar seleção", "Añadir selección", "Thêm phần đang chọn"],
    ["Add filter:", "Adicionar filtro:", "Añadir filtro:", "Thêm bộ lọc:"],
    ["Annotation Categories", "Categorias de anotação", "Categorías de anotación", "Các loại ghi chú"],
    ["Model Categories", "Categorias do modelo", "Categorías del modelo", "Các loại mô hình"],
    ["Create from This View", "Criar a partir desta vista", "Crear desde esta vista", "Tạo từ khung nhìn này"],
    ["Cut", "Corte", "Corte", "Cắt"],
    ["Projection", "Projeção", "Proyección", "Chiếu"],
    ["Solid", "Contínua", "Continua", "Đặc"],
    ["Dashed", "Tracejada", "Discontinua", "Nét đứt"],
    ["Dotted", "Pontilhada", "Punteada", "Chấm"],
    ["Elements", "Elementos", "Elementos", "Các phần tử"],
    ["Fill color", "Cor do preenchimento", "Color de relleno", "Màu tô"],
    ["Line color", "Cor da linha", "Color de línea", "Màu đường"],
    ["Line dash", "Tracejado da linha", "Guiones de línea", "Kiểu nét"],
    ["Line weight", "Espessura da linha", "Grosor de línea", "Độ dày nét"],
    ["Filter Rules", "Regras do filtro", "Reglas del filtro", "Quy tắc lọc"],
    ["Filter:", "Filtro:", "Filtro:", "Bộ lọc:"],
    ["Filters", "Filtros", "Filtros", "Bộ lọc"],
    [
        "Filters (FT): rules that pick out elements for a view to hide or draw another way",
        "Filtros (FT): regras que escolhem elementos para uma vista ocultar ou desenhar de outro modo",
        "Filtros (FT): reglas que eligen elementos para que una vista los oculte o los dibuje de otro modo",
        "Bộ lọc (FT): các quy tắc chọn ra phần tử để một khung nhìn ẩn đi hoặc vẽ cách khác",
    ],
    ["Follow in This View", "Seguir nesta vista", "Seguir en esta vista", "Theo trong khung nhìn này"],
    ["Stop Following in This View", "Deixar de seguir nesta vista", "Dejar de seguir en esta vista", "Thôi theo trong khung nhìn này"],
    ["Halftone", "Meio-tom", "Medio tono", "Bán sắc"],
    ["Hide Category", "Ocultar categoria", "Ocultar categoría", "Ẩn loại"],
    [
        "Hide Category (VH): leave every category the selection is of out of this view",
        "Ocultar categoria (VH): deixa fora desta vista todas as categorias da seleção",
        "Ocultar categoría (VH): deja fuera de esta vista todas las categorías de la selección",
        "Ẩn loại (VH): bỏ mọi loại mà phần đang chọn thuộc về ra khỏi khung nhìn này",
    ],
    ["Hide Element", "Ocultar elemento", "Ocultar elemento", "Ẩn phần tử"],
    [
        "Hide Element (EH): leave the selected elements out of this view",
        "Ocultar elemento (EH): deixa os elementos selecionados fora desta vista",
        "Ocultar elemento (EH): deja los elementos seleccionados fuera de esta vista",
        "Ẩn phần tử (EH): bỏ các phần tử đang chọn ra khỏi khung nhìn này",
    ],
    ["New template:", "Novo modelo de vista:", "Nueva plantilla de vista:", "Mẫu mới:"],
    [
        "No element is hidden or overridden here: select some and Add Selected.",
        "Nenhum elemento está oculto ou substituído aqui: selecione alguns e use Adicionar seleção.",
        "Ningún elemento está oculto ni reemplazado aquí: seleccione algunos y use Añadir selección.",
        "Không phần tử nào bị ẩn hoặc bị ghi đè ở đây: hãy chọn một số rồi bấm Thêm phần đang chọn.",
    ],
    ["No filters applied.", "Nenhum filtro aplicado.", "Ningún filtro aplicado.", "Không có bộ lọc nào được áp dụng."],
    [
        "No filters yet: New makes one.",
        "Ainda não há filtros: Novo cria um.",
        "Aún no hay filtros: Nuevo crea uno.",
        "Chưa có bộ lọc nào: bấm Mới để tạo một bộ lọc.",
    ],
    [
        "No view templates yet: Create from This View makes one.",
        "Ainda não há modelos de vista: Criar a partir desta vista cria um.",
        "Aún no hay plantillas de vista: Crear desde esta vista crea una.",
        "Chưa có mẫu khung nhìn nào: bấm Tạo từ khung nhìn này để tạo một mẫu.",
    ],
    ["Overridden", "Com substituição", "Con reemplazo", "Bị ghi đè"],
    ["Override Graphics", "Substituir gráficos", "Reemplazar gráficos", "Ghi đè đồ họa"],
    ["Override Graphics by Element", "Substituir gráficos por elemento", "Reemplazar gráficos por elemento", "Ghi đè đồ họa theo phần tử"],
    [
        "Override Graphics by Element (EO): draw the selected elements their own way in this view",
        "Substituir gráficos por elemento (EO): desenha os elementos selecionados a seu modo nesta vista",
        "Reemplazar gráficos por elemento (EO): dibuja los elementos seleccionados a su manera en esta vista",
        "Ghi đè đồ họa theo phần tử (EO): vẽ các phần tử đang chọn theo cách riêng của chúng trong khung nhìn này",
    ],
    ["Visibility", "Visibilidade", "Visibilidad", "Hiển thị"],
    ["Projection/Surface", "Projeção/Superfície", "Proyección/Superficie", "Chiếu/Bề mặt"],
    ["Lines", "Linhas", "Líneas", "Các đường"],
    ["Patterns", "Padrões", "Patrones", "Hoa văn"],
    ["Projection Lines", "Linhas de projeção", "Líneas de proyección", "Đường chiếu"],
    ["Projection Patterns", "Padrões de projeção", "Patrones de proyección", "Hoa văn chiếu"],
    ["Cut Lines", "Linhas de corte", "Líneas de corte", "Đường cắt"],
    ["Cut Patterns", "Padrões de corte", "Patrones de corte", "Các hoa văn cắt"],
    ["Line Graphics", "Gráficos de linha", "Gráficos de línea", "Đồ họa đường"],
    ["Pattern Graphics", "Gráficos de padrão", "Gráficos de patrón", "Đồ họa hoa văn"],
    ["Surface Transparency", "Transparência da superfície", "Transparencia de superficie", "Độ trong suốt bề mặt"],
    ["Clear Overrides", "Limpar substituições", "Borrar reemplazos", "Xóa ghi đè"],
    ["By View", "Pela vista", "Por vista", "Theo khung nhìn"],
    ["Invert", "Inverter", "Invertir", "Đảo ngược"],
    ["Category name search:", "Buscar categoria:", "Buscar categoría:", "Tìm tên loại:"],
    ["No category has that name.", "Nenhuma categoria tem esse nome.", "Ninguna categoría tiene ese nombre.", "Không loại nào có tên đó."],
    [
        "Categories not overridden are drawn as Object Styles sets them.",
        "As categorias sem substituição são desenhadas como os Estilos de objeto definem.",
        "Las categorías sin reemplazo se dibujan como las definen los Estilos de objeto.",
        "Các loại không bị ghi đè được vẽ như Kiểu đối tượng đặt.",
    ],
    ["Object Styles…", "Estilos de objeto…", "Estilos de objeto…", "Kiểu đối tượng…"],
    ["Override…", "Substituir…", "Reemplazar…", "Ghi đè…"],
    ["Reset", "Redefinir", "Restablecer", "Đặt lại"],
    ["Template:", "Modelo de vista:", "Plantilla de vista:", "Mẫu:"],
    [
        "The views following it get from it:",
        "O que as vistas que o seguem recebem dele:",
        "Lo que reciben de ella las vistas que la siguen:",
        "Các khung nhìn theo nó nhận được từ nó:",
    ],
    [
        "This view has no graphics to override.",
        "Esta vista não tem gráficos para substituir.",
        "Esta vista no tiene gráficos que reemplazar.",
        "Khung nhìn này không có đồ họa để ghi đè.",
    ],
    ["Transparency", "Transparência", "Transparencia", "Độ trong suốt"],
    ["Unhide Elements", "Reexibir elementos", "Mostrar elementos", "Bỏ ẩn các phần tử"],
    ["Unhide Element", "Reexibir elemento", "Mostrar elemento", "Bỏ ẩn phần tử"],
    ["Unhide Category", "Reexibir categoria", "Mostrar categoría", "Bỏ ẩn loại"],
    [
        "Unhide Element (EU): bring back the selected elements this view hides",
        "Reexibir elemento (EU): traz de volta os elementos selecionados que esta vista oculta",
        "Mostrar elemento (EU): devuelve los elementos seleccionados que esta vista oculta",
        "Bỏ ẩn phần tử (EU): đưa trở lại các phần tử đang chọn mà khung nhìn này ẩn",
    ],
    [
        "Unhide Category (VU): bring back every category the selection is of that this view hides",
        "Reexibir categoria (VU): traz de volta cada categoria da seleção que esta vista oculta",
        "Mostrar categoría (VU): devuelve cada categoría de la selección que esta vista oculta",
        "Bỏ ẩn loại (VU): đưa trở lại mọi loại mà phần đang chọn thuộc về và khung nhìn này ẩn",
    ],
    [
        "Nothing selected is hidden in this view. Reveal Hidden Elements (RH) draws what is, to select it.",
        "Nada do que está selecionado está oculto nesta vista. Revelar elementos ocultos (RH) desenha o que está, para selecioná-lo.",
        "Nada de lo seleccionado está oculto en esta vista. Revelar elementos ocultos (RH) dibuja lo que lo está, para seleccionarlo.",
        "Không thứ gì đang chọn bị ẩn trong khung nhìn này. Hiện các phần tử đang ẩn (RH) sẽ vẽ những gì bị ẩn để chọn chúng.",
    ],
    ["View Template", "Modelo de vista", "Plantilla de vista", "Mẫu khung nhìn"],
    ["View Templates", "Modelos de vista", "Plantillas de vista", "Các mẫu khung nhìn"],
    [
        "View Templates (VT): a view's settings under a name, for other views to follow",
        "Modelos de vista (VT): os ajustes de uma vista sob um nome, para outras vistas seguirem",
        "Plantillas de vista (VT): los ajustes de una vista bajo un nombre, para que otras vistas los sigan",
        "Mẫu khung nhìn (VT): các cài đặt của một khung nhìn dưới một tên, cho các khung nhìn khác theo",
    ],
    ["Visibility/Graphics", "Visibilidade/Gráficos", "Visibilidad/Gráficos", "Hiển thị/Đồ họa"],
    [
        "Visibility/Graphics (VV): what this view hides, and how it draws the rest, by category, filter and element",
        "Visibilidade/Gráficos (VV): o que esta vista oculta e como desenha o resto, por categoria, filtro e elemento",
        "Visibilidad/Gráficos (VV): lo que esta vista oculta y cómo dibuja el resto, por categoría, filtro y elemento",
        "Hiển thị/Đồ họa (VV): điều khung nhìn này ẩn, và cách nó vẽ phần còn lại, theo loại, bộ lọc và phần tử",
    ],
    [
        "Visibility/Graphics Overrides",
        "Substituições de visibilidade/gráficos",
        "Reemplazos de visibilidad/gráficos",
        "Ghi đè Hiển thị/Đồ họa",
    ],
    [
        "Visibility/Graphics Overrides for {}",
        "Substituições de visibilidade/gráficos para {}",
        "Reemplazos de visibilidad/gráficos para {}",
        "Ghi đè Hiển thị/Đồ họa cho {}",
    ],
    [
        "{} follow view template {}: change them there.",
        "{} seguem o modelo de vista {}: altere-os lá.",
        "{} siguen la plantilla de vista {}: cámbielos allí.",
        "{} theo mẫu khung nhìn {}: hãy đổi chúng ở đó.",
    ],
    ["Phasing", "Fases", "Fases", "Các pha"],
    [
        "Phasing (PH): the project's phases in order, the phase filters that show them, and how a filter draws each",
        "Fases (PH): as fases do projeto em ordem, os filtros de fase que as mostram e como um filtro desenha cada uma",
        "Fases (PH): las fases del proyecto en orden, los filtros de fase que las muestran y cómo los dibuja cada filtro",
        "Pha (PH): các pha của dự án theo thứ tự, các bộ lọc pha thể hiện chúng, và cách một bộ lọc vẽ mỗi pha",
    ],
    ["Project Phasing", "Fases do projeto", "Fases del proyecto", "Pha của dự án"],
    ["Phase Filters", "Filtros de fase", "Filtros de fase", "Các bộ lọc pha"],
    ["Graphic Overrides", "Substituições gráficas", "Reemplazos gráficos", "Các ghi đè đồ họa"],
    ["Apply Overrides", "Aplicar substituições", "Aplicar reemplazos", "Áp dụng ghi đè"],
    ["Phases, in order:", "Fases, em ordem:", "Fases, en orden:", "Các pha, theo thứ tự:"],
    ["Add at End", "Adicionar no fim", "Añadir al final", "Thêm vào cuối"],
    ["Insert Before", "Inserir antes", "Insertar antes", "Chèn trước"],
    ["Insert After", "Inserir depois", "Insertar después", "Chèn sau"],
    ["Rename", "Renomear", "Renombrar", "Đổi tên"],
    ["Combine with Previous", "Combinar com a anterior", "Combinar con la anterior", "Gộp với pha trước"],
    ["Combine with Next", "Combinar com a seguinte", "Combinar con la siguiente", "Gộp với pha sau"],
    [
        "Combine: what was made, taken down and shown in this phase goes to the neighbouring one, and this one goes",
        "Combinar: o que foi feito, demolido e mostrado nesta fase passa para a vizinha, e esta sai",
        "Combinar: lo que se hizo, se demolió y se muestra en esta fase pasa a la vecina, y esta sale",
        "Gộp: những gì được làm, bị dỡ và được hiện trong pha này sẽ chuyển sang pha lân cận, và pha này biến mất",
    ],
    ["Phase filter:", "Filtro de fase:", "Filtro de fase:", "Bộ lọc pha:"],
    ["New filter:", "Novo filtro:", "Nuevo filtro:", "Bộ lọc mới:"],
    ["New Filter", "Novo filtro", "Nuevo filtro", "Bộ lọc mới"],
    [
        "How it shows what is, in the view's phase:",
        "Como mostra o que há, na fase da vista:",
        "Cómo muestra lo que hay, en la fase de la vista:",
        "Cách nó thể hiện những gì đang có, trong pha của khung nhìn:",
    ],
    ["Demolished", "Demolido", "Demolido", "Đã phá dỡ"],
    ["Temporary", "Temporário", "Temporal", "Tạm thời"],
    ["By Category", "Por categoria", "Por categoría", "Theo loại"],
    ["Not Displayed", "Não exibido", "No mostrado", "Không hiển thị"],
    [
        "A phase filter draws a status this way where it says Overridden.",
        "Um filtro de fase desenha um estado deste modo onde diz Com substituição.",
        "Un filtro de fase dibuja un estado de este modo donde dice Con reemplazo.",
        "Một bộ lọc pha vẽ một trạng thái theo cách này tại nơi nó ghi là Bị ghi đè.",
    ],
    ["Phase and Phase Filter", "Fase e filtro de fase", "Fase y filtro de fase", "Pha và bộ lọc pha"],
    ["Phase Filter", "Filtro de fase", "Filtro de fase", "Bộ lọc pha"],
    ["Phase", "Fase", "Fase", "Pha"],
    ["Phase Created", "Fase de criação", "Fase de creación", "Pha tạo"],
    ["Phase Demolished", "Fase de demolição", "Fase de demolición", "Pha phá dỡ"],
    ["Design Option", "Opção de projeto", "Opción de diseño", "Phương án thiết kế"],
    ["Design Options", "Opções de projeto", "Opciones de diseño", "Các phương án thiết kế"],
    [
        "Design Options (DO): sets of options for a part of the design, and the one new elements are made in",
        "Opções de projeto (DO): conjuntos de opções para uma parte do projeto, e aquela em que os novos elementos são feitos",
        "Opciones de diseño (DO): conjuntos de opciones para una parte del diseño, y aquella en la que se hacen los elementos nuevos",
        "Phương án thiết kế (DO): các tập phương án cho một phần của thiết kế, và phương án mà các phần tử mới được tạo trong đó",
    ],
    ["Option set:", "Conjunto de opções:", "Conjunto de opciones:", "Tập phương án:"],
    ["Options:", "Opções:", "Opciones:", "Các phương án:"],
    ["Option", "Opção", "Opción", "Phương án"],
    ["{} (primary)", "{} (principal)", "{} (principal)", "{} (chính)"],
    ["Primary ({})", "Principal ({})", "Principal ({})", "Chính ({})"],
    ["New Set", "Novo conjunto", "Nuevo conjunto", "Tập mới"],
    ["Rename Set", "Renomear conjunto", "Renombrar conjunto", "Đổi tên tập"],
    ["Delete Set", "Excluir conjunto", "Eliminar conjunto", "Xóa tập"],
    [
        "Delete Set: the set goes, and every element in its options",
        "Excluir conjunto: o conjunto sai, com todos os elementos de suas opções",
        "Eliminar conjunto: el conjunto sale, con todos los elementos de sus opciones",
        "Xóa tập: tập biến mất, cùng mọi phần tử trong các phương án của nó",
    ],
    ["Accept Primary", "Aceitar a principal", "Aceptar la principal", "Nhận phương án chính"],
    [
        "Accept Primary: the primary option's elements stay in the model; the others' elements and the set go",
        "Aceitar a principal: os elementos da opção principal ficam no modelo; os das outras e o conjunto saem",
        "Aceptar la principal: los elementos de la opción principal quedan en el modelo; los de las otras y el conjunto salen",
        "Nhận phương án chính: các phần tử của phương án chính ở lại trong mô hình; phần tử của các phương án khác và tập đều biến mất",
    ],
    ["Add Option", "Adicionar opção", "Añadir opción", "Thêm phương án"],
    ["Rename Option", "Renomear opção", "Renombrar opción", "Đổi tên phương án"],
    ["Make Primary", "Tornar principal", "Hacer principal", "Đặt làm chính"],
    ["Delete Option", "Excluir opção", "Eliminar opción", "Xóa phương án"],
    ["Edit Option", "Editar opção", "Editar opción", "Sửa phương án"],
    [
        "Edit Option: new elements are made in this option, and the view on screen shows it",
        "Editar opção: os novos elementos são feitos nesta opção, e a vista na tela a mostra",
        "Editar opción: los elementos nuevos se hacen en esta opción, y la vista en pantalla la muestra",
        "Sửa phương án: các phần tử mới được tạo trong phương án này, và khung nhìn trên màn hình thể hiện nó",
    ],
    [
        "No design option sets yet: New Set makes one.",
        "Ainda não há conjuntos de opções: Novo conjunto cria um.",
        "Aún no hay conjuntos de opciones: Nuevo conjunto crea uno.",
        "Chưa có tập phương án thiết kế nào: bấm Tập mới để tạo một tập.",
    ],
    [
        "The project has no design options: Manage > Design Options makes them.",
        "O projeto não tem opções de projeto: Gerenciar > Opções de projeto as cria.",
        "El proyecto no tiene opciones de diseño: Gestionar > Opciones de diseño las crea.",
        "Dự án chưa có phương án thiết kế: Quản lý > Phương án thiết kế sẽ tạo chúng.",
    ],
    ["Active design option:", "Opção de projeto ativa:", "Opción de diseño activa:", "Phương án thiết kế đang dùng:"],
    ["Main Model", "Modelo principal", "Modelo principal", "Mô hình chính"],
    [
        "This view shows another option of that set: pick it again to show it here",
        "Esta vista mostra outra opção desse conjunto: escolha-a de novo para mostrá-la aqui",
        "Esta vista muestra otra opción de ese conjunto: elíjala de nuevo para mostrarla aquí",
        "Khung nhìn này thể hiện một phương án khác của tập đó: hãy chọn lại nó để thể hiện ở đây",
    ],
    [
        "New elements are made in this option; the view on screen shows it",
        "Os novos elementos são feitos nesta opção; a vista na tela a mostra",
        "Los elementos nuevos se hacen en esta opción; la vista en pantalla la muestra",
        "Các phần tử mới được tạo trong phương án này; khung nhìn trên màn hình thể hiện nó",
    ],
    ["Demolish", "Demolir", "Demoler", "Phá dỡ"],
    [
        "Demolish (DW): click an element to take it down in the view's phase; click it again to take it back",
        "Demolir (DW): clique num elemento para demoli-lo na fase da vista; clique de novo para desfazer",
        "Demoler (DW): haga clic en un elemento para demolerlo en la fase de la vista; haga clic otra vez para deshacerlo",
        "Phá dỡ (DW): bấm vào một phần tử để phá dỡ nó ở pha của khung nhìn; bấm lần nữa để hoàn tác",
    ],
    [
        "Click an element to take it down in the view's phase; click it again to take it back.",
        "Clique num elemento para demoli-lo na fase da vista; clique de novo para desfazer.",
        "Haga clic en un elemento para demolerlo en la fase de la vista; haga clic otra vez para deshacerlo.",
        "Bấm vào một phần tử để phá dỡ nó ở pha của khung nhìn; bấm lần nữa để hoàn tác.",
    ],
    [
        "{} holds no parameters, so it has no phase to demolish",
        "{} não tem parâmetros, então não tem fase a demolir",
        "{} no tiene parámetros, así que no tiene fase que demoler",
        "{} không có tham số, nên không có pha để phá dỡ",
    ],
    [
        "{} follows its wall, which comes down in {}: take the wall back instead",
        "{} acompanha a parede, que é demolida em {}: desfaça a demolição da parede",
        "{} acompaña al muro, que se demuele en {}: deshaga la demolición del muro",
        "{} đi theo tường của nó, mà tường bị phá dỡ ở {}: hãy hoàn tác việc phá dỡ tường",
    ],
    ["Paint", "Pintar", "Pintar", "Sơn"],
    ["Remove Paint", "Remover pintura", "Quitar pintura", "Bỏ sơn"],
    ["Geometry", "Geometria", "Geometría", "Hình học"],
    [
        "Paint (PT): lay a material, and its texture, over the face you click in the 3D view",
        "Pintar (PT): aplica um material, e sua textura, na face clicada na vista 3D",
        "Pintar (PT): aplica un material, y su textura, a la cara en la que se hace clic en la vista 3D",
        "Sơn (PT): phủ một vật liệu, và họa tiết của nó, lên mặt bạn bấm trong khung nhìn 3D",
    ],
    [
        "Remove Paint: take the paint off the face you click in the 3D view",
        "Remover pintura: tira a pintura da face clicada na vista 3D",
        "Quitar pintura: quita la pintura de la cara en la que se hace clic en la vista 3D",
        "Bỏ sơn: bỏ lớp sơn khỏi mặt bạn bấm trong khung nhìn 3D",
    ],
    [
        "Click a face to paint it with the material in the Options Bar; the 3D view orbits as ever.",
        "Clique numa face para pintá-la com o material da Barra de Opções; a vista 3D orbita como sempre.",
        "Haga clic en una cara para pintarla con el material de la Barra de opciones; la vista 3D gira como siempre.",
        "Bấm vào một mặt để sơn nó bằng vật liệu trong Thanh tùy chọn; khung nhìn 3D vẫn xoay vòng như thường.",
    ],
    [
        "Click a painted face to take its paint off.",
        "Clique numa face pintada para tirar a pintura.",
        "Haga clic en una cara pintada para quitarle la pintura.",
        "Bấm vào một mặt đã sơn để bỏ lớp sơn của nó.",
    ],
    ["there's no face to paint there", "não há face para pintar ali", "no hay cara que pintar ahí", "ở đó không có mặt nào để sơn"],
    ["Favorites", "Favoritos", "Favoritos", "Mục yêu thích"],
    [
        "Favorites: the materials and families you keep for every project; add one to this project with a click",
        "Favoritos: os materiais e as famílias que você guarda para todos os projetos; adicione um a este projeto com um clique",
        "Favoritos: los materiales y las familias que guarda para todos los proyectos; añada uno a este proyecto con un clic",
        "Mục yêu thích: các vật liệu và cấu kiện bạn giữ cho mọi dự án; thêm một mục vào dự án này bằng một cú bấm",
    ],
    ["Favorites…", "Favoritos…", "Favoritos…", "Mục yêu thích…"],
    [
        "Kept for every project: add one to this project with a click.",
        "Guardados para todos os projetos: adicione um a este projeto com um clique.",
        "Guardados para todos los proyectos: añada uno a este proyecto con un clic.",
        "Được giữ cho mọi dự án: thêm một mục vào dự án này bằng một cú bấm.",
    ],
    [
        "No favorite materials yet: save one from Manage > Materials.",
        "Ainda não há materiais favoritos: guarde um em Gerenciar > Materiais.",
        "Todavía no hay materiales favoritos: guarde uno desde Administrar > Materiales.",
        "Chưa có vật liệu yêu thích nào: hãy lưu một vật liệu từ Quản lý > Vật liệu.",
    ],
    [
        "No favorite families yet: save one from the Family Editor.",
        "Ainda não há famílias favoritas: guarde uma no Editor de Famílias.",
        "Todavía no hay familias favoritas: guarde una desde el Editor de familias.",
        "Chưa có cấu kiện yêu thích nào: hãy lưu một cấu kiện từ Trình soạn cấu kiện.",
    ],
    ["No texture", "Sem textura", "Sin textura", "Không có họa tiết"],
    ["Add to Project", "Adicionar ao projeto", "Añadir al proyecto", "Thêm vào dự án"],
    ["Added {} to the project", "{} adicionado ao projeto", "{} añadido al proyecto", "Đã thêm {} vào dự án"],
    ["Save to Favorites", "Guardar nos Favoritos", "Guardar en Favoritos", "Lưu vào Mục yêu thích"],
    ["Saved {} to your Favorites", "{} guardado nos seus Favoritos", "{} guardado en sus Favoritos", "Đã lưu {} vào Mục yêu thích của bạn"],
    ["Add to Favorites", "Adicionar aos Favoritos", "Añadir a Favoritos", "Thêm vào Mục yêu thích"],
    [
        "there's no folder to keep favorites in here",
        "não há pasta para guardar favoritos aqui",
        "no hay carpeta donde guardar favoritos aquí",
        "ở đây không có thư mục nào để giữ mục yêu thích",
    ],
    ["Texture", "Textura", "Textura", "Họa tiết"],
    ["Picture…", "Imagem…", "Imagen…", "Ảnh…"],
    ["Picture", "Imagem", "Imagen", "Ảnh"],
    ["Pictures", "Imagens", "Imágenes", "Các ảnh"],
    ["Block", "Bloco", "Bloque", "Khối ký hiệu"],
    ["Boards", "Tábuas", "Tablas", "Ván"],
    ["Tile", "Ladrilho", "Baldosa", "Ô"],
    ["Tile Width", "Largura do ladrilho", "Ancho de la baldosa", "Chiều rộng ô"],
    ["Tile Height", "Altura do ladrilho", "Alto de la baldosa", "Chiều cao ô"],
    ["Texture Angle", "Ângulo da textura", "Ángulo de la textura", "Góc họa tiết"],
    ["Texture Offset Across", "Deslocamento horizontal da textura", "Desplazamiento horizontal de la textura", "Độ lệch họa tiết ngang"],
    ["Texture Offset Up", "Deslocamento vertical da textura", "Desplazamiento vertical de la textura", "Độ lệch họa tiết lên"],
    [
        "`{}` is not a size, an angle or an offset",
        "`{}` não é um tamanho, um ângulo nem um deslocamento",
        "`{}` no es un tamaño, un ángulo ni un desplazamiento",
        "`{}` không phải là một cỡ, một góc hay một độ lệch",
    ],
    ["Family Editor", "Editor de Famílias", "Editor de familias", "Trình soạn cấu kiện"],
    ["New Family", "Nova família", "Nueva familia", "Cấu kiện mới"],
    ["Edit Family", "Editar família", "Editar familia", "Sửa cấu kiện"],
    [
        "New Family: start a family of your own, of a category, to edit its parameters, types and forms",
        "Nova família: comece uma família sua, de uma categoria, para editar seus parâmetros, tipos e formas",
        "Nueva familia: empiece una familia propia, de una categoría, para editar sus parámetros, tipos y formas",
        "Cấu kiện mới: bắt đầu một cấu kiện của riêng bạn, thuộc một loại, để sửa tham số, kiểu và hình khối của nó",
    ],
    [
        "Edit Family: the family of what is selected, or of the component being placed",
        "Editar família: a família do que está selecionado, ou do componente que se coloca",
        "Editar familia: la familia de lo seleccionado, o del componente que se coloca",
        "Sửa cấu kiện: cấu kiện của thứ đang chọn, hoặc của cấu kiện đang được đặt",
    ],
    ["Family name", "Nome da família", "Nombre de la familia", "Tên cấu kiện"],
    ["Parameters", "Parâmetros", "Parámetros", "Các tham số"],
    ["Types", "Tipos", "Tipos", "Các kiểu"],
    ["Forms", "Formas", "Formas", "Hình khối"],
    ["Symbols", "Símbolos", "Símbolos", "Biểu tượng"],
    ["Furniture", "Mobiliário", "Mobiliario", "Nội thất"],
    ["Casework", "Marcenaria", "Carpintería", "Tủ kệ"],
    ["Plumbing", "Hidrossanitário", "Fontanería", "Cấp thoát nước"],
    ["Plumbing Fixtures", "Peças sanitárias", "Sanitarios", "Các thiết bị vệ sinh"],
    ["Lighting", "Iluminação", "Iluminación", "Chiếu sáng"],
    ["Generic", "Genérico", "Genérico", "Chung"],
    ["Parameter Name", "Nome do parâmetro", "Nombre del parámetro", "Tên tham số"],
    ["Parameter Instance", "Instância do parâmetro", "Instancia del parámetro", "Tham số trường hợp"],
    ["Parameter Value", "Valor do parâmetro", "Valor del parámetro", "Giá trị tham số"],
    ["Parameter Formula", "Fórmula do parâmetro", "Fórmula del parámetro", "Công thức tham số"],
    ["Instance", "Instância", "Instancia", "Trường hợp"],
    ["Angle", "Ângulo", "Ángulo", "Góc"],
    ["Add Parameter", "Adicionar parâmetro", "Añadir parámetro", "Thêm tham số"],
    ["Add Type", "Adicionar tipo", "Añadir tipo", "Thêm kiểu"],
    ["Duplicate Type", "Duplicar tipo", "Duplicar tipo", "Nhân bản kiểu"],
    ["Delete Type", "Excluir tipo", "Eliminar tipo", "Xóa kiểu"],
    ["Type Name", "Nome do tipo", "Nombre del tipo", "Tên kiểu"],
    ["Type Value", "Valor do tipo", "Valor del tipo", "Giá trị kiểu"],
    [
        "A blank value is the family's own, as the Parameters tab has it.",
        "Um valor em branco é o da própria família, como a aba Parâmetros o tem.",
        "Un valor en blanco es el de la propia familia, tal como lo tiene la pestaña Parámetros.",
        "Giá trị để trống là giá trị của chính cấu kiện, như thẻ Tham số có nó.",
    ],
    [
        "A door or a window needs lengths named Width and Height; its formulas can read Wall Thickness, the wall's.",
        "Uma porta ou janela precisa de comprimentos chamados Width e Height; suas fórmulas podem ler Wall Thickness, a espessura da parede.",
        "Una puerta o ventana necesita longitudes llamadas Width y Height; sus fórmulas pueden leer Wall Thickness, el espesor del muro.",
        "Một cửa đi hoặc cửa sổ cần các chiều dài tên là Chiều rộng và Chiều cao; công thức của nó có thể đọc Độ dày tường, của tường.",
    ],
    ["Plane", "Plano", "Plano", "Mặt phẳng"],
    ["Plane XY", "Plano XY", "Plano XY", "Mặt phẳng XY"],
    ["Plane XZ", "Plano XZ", "Plano XZ", "Mặt phẳng XZ"],
    ["Plane YZ", "Plano YZ", "Plano YZ", "Mặt phẳng YZ"],
    ["Shape", "Contorno", "Contorno", "Hình dạng"],
    ["Profile", "Perfil", "Perfil", "Biên dạng"],
    ["Circle", "Círculo", "Círculo", "Hình tròn"],
    ["Corners", "Vértices", "Vértices", "Các góc"],
    ["Form Corners", "Vértices da forma", "Vértices de la forma", "Các góc hình khối"],
    ["Circle centre and radius", "Centro e raio do círculo", "Centro y radio del círculo", "Tâm và bán kính hình tròn"],
    ["Form Circle X", "Centro X da forma", "Centro X de la forma", "Tâm hình tròn X"],
    ["Form Circle Y", "Centro Y da forma", "Centro Y de la forma", "Tâm hình tròn Y"],
    ["Form Circle Radius", "Raio da forma", "Radio de la forma", "Bán kính hình tròn"],
    ["Revolve", "Revolução", "Revolución", "Xoay quanh trục"],
    ["turned about an axis", "girada em torno de um eixo", "girada alrededor de un eje", "xoay quanh một trục"],
    ["Axis", "Eixo", "Eje", "Trục tọa độ"],
    ["Form Axis X1", "Eixo da forma X1", "Eje de la forma X1", "Trục hình khối X1"],
    ["Form Axis Y1", "Eixo da forma Y1", "Eje de la forma Y1", "Trục hình khối Y1"],
    ["Form Axis X2", "Eixo da forma X2", "Eje de la forma X2", "Trục hình khối X2"],
    ["Form Axis Y2", "Eixo da forma Y2", "Eje de la forma Y2", "Trục hình khối Y2"],
    ["From", "De", "Desde", "Từ"],
    ["To", "Até", "Hasta", "Đến"],
    ["From (degrees)", "De (graus)", "Desde (grados)", "Từ (độ)"],
    ["To (degrees)", "Até (graus)", "Hasta (grados)", "Đến (độ)"],
    ["Form From", "Início da forma", "Inicio de la forma", "Hình khối từ"],
    ["Form To", "Fim da forma", "Fin de la forma", "Hình khối đến"],
    ["Form Material", "Material da forma", "Material de la forma", "Vật liệu hình khối"],
    ["Visible when", "Visível quando", "Visible cuando", "Hiện khi"],
    ["Form Visible", "Visibilidade da forma", "Visibilidad de la forma", "Hình khối hiển thị"],
    ["Void", "Vazio", "Vacío", "Rỗng"],
    ["void", "vazio", "vacío", "rỗng"],
    [
        "cuts the forms listed above it",
        "corta as formas listadas acima dele",
        "corta las formas listadas encima",
        "cắt các hình khối được liệt kê phía trên nó",
    ],
    ["Form Void", "Vazio da forma", "Vacío de la forma", "Hình khối rỗng"],
    ["Mesh", "Malha", "Malla", "Lưới tam giác"],
    ["{} triangles, from a file", "{} triângulos, de um arquivo", "{} triángulos, de un archivo", "{} tam giác, từ một tệp"],
    ["Extrusion", "Extrusão", "Extrusión", "Đùn"],
    ["Cylinder", "Cilindro", "Cilindro", "Trụ"],
    ["Revolved Circle", "Círculo revolvido", "Círculo de revolución", "Hình tròn xoay"],
    ["Add Box", "Adicionar bloco", "Añadir bloque", "Thêm khối hộp"],
    ["Add Cylinder", "Adicionar cilindro", "Añadir cilindro", "Thêm trụ"],
    ["Add Revolve", "Adicionar revolução", "Añadir revolución", "Thêm khối xoay"],
    ["Add Void", "Adicionar vazio", "Añadir vacío", "Thêm khối rỗng"],
    ["Add Line", "Adicionar linha", "Añadir línea", "Thêm đường"],
    ["Add Arc", "Adicionar arco", "Añadir arco", "Thêm cung"],
    ["Arc", "Arco", "Arco", "Cung"],
    ["Points", "Pontos", "Puntos", "Các điểm"],
    ["Symbol Centre X", "Centro X do símbolo", "Centro X del símbolo", "Tâm ký hiệu X"],
    ["Symbol Centre Y", "Centro Y do símbolo", "Centro Y del símbolo", "Tâm ký hiệu Y"],
    ["Symbol Radius", "Raio do símbolo", "Radio del símbolo", "Bán kính ký hiệu"],
    ["Symbol From", "Início do símbolo", "Inicio del símbolo", "Ký hiệu từ"],
    ["Symbol To", "Fim do símbolo", "Fin del símbolo", "Ký hiệu đến"],
    ["Symbol Points", "Pontos do símbolo", "Puntos del símbolo", "Các điểm ký hiệu"],
    ["Symbol Visible", "Visibilidade do símbolo", "Visibilidad del símbolo", "Ký hiệu hiển thị"],
    [
        "Symbols are the lines a door's or a window's plan draws; this family isn't one.",
        "Símbolos são as linhas que a planta de uma porta ou janela desenha; esta família não é uma.",
        "Los símbolos son las líneas que dibuja la planta de una puerta o ventana; esta familia no es una.",
        "Ký hiệu là các đường mà mặt bằng của một cửa đi hoặc cửa sổ vẽ; cấu kiện này không phải vậy.",
    ],
    ["Preview", "Pré-visualização", "Vista previa", "Xem trước"],
    ["Previewing", "Pré-visualizando", "Previsualizando", "Đang xem trước"],
    ["Preview Type", "Tipo na pré-visualização", "Tipo en la vista previa", "Xem trước kiểu"],
    ["Drag the picture to turn it.", "Arraste a imagem para girá-la.", "Arrastre la imagen para girarla.", "Kéo hình để xoay nó."],
    ["Load Family…", "Carregar família…", "Cargar familia…", "Nạp họ…"],
    ["Save Family…", "Salvar família…", "Guardar familia…", "Lưu họ…"],
    ["a family needs a name", "uma família precisa de um nome", "una familia necesita un nombre", "một cấu kiện cần có tên"],
    ["a family needs a type", "uma família precisa de um tipo", "una familia necesita un tipo", "một cấu kiện cần có một kiểu"],
    [
        "corner {}: each corner is `x, y`",
        "vértice {}: cada vértice é `x, y`",
        "vértice {}: cada vértice es `x, y`",
        "góc {}: mỗi góc là `x, y`",
    ],
    ["two types are named `{}`", "dois tipos se chamam `{}`", "dos tipos se llaman `{}`", "hai kiểu cùng tên `{}`"],
    ["{}: `{}` isn't a number", "{}: `{}` não é um número", "{}: `{}` no es un número", "{}: `{}` không phải là một số"],
    // Manage > Inquiry: Select by ID and IDs of Selection.
    ["Inquiry", "Consulta", "Consulta", "Tra cứu"],
    ["Select by ID", "Selecionar por ID", "Seleccionar por ID", "Chọn theo ID"],
    ["IDs of Selection", "IDs da seleção", "ID de la selección", "ID của phần đang chọn"],
    [
        "Select by ID: select elements by their IDs, GUIDs or IFC GlobalIds, as many as you paste, and show them",
        "Selecionar por ID: seleciona elementos por seus IDs, GUIDs ou GlobalIds IFC, quantos você colar, e os mostra",
        "Seleccionar por ID: selecciona elementos por sus ID, GUID o GlobalId IFC, tantos como pegue, y los muestra",
        "Chọn theo ID: chọn các phần tử theo ID, GUID hoặc GlobalId IFC của chúng, bao nhiêu tùy bạn dán, và hiện chúng",
    ],
    [
        "IDs of Selection: select elements first",
        "IDs da seleção: selecione elementos antes",
        "ID de la selección: seleccione elementos primero",
        "ID của phần đang chọn: hãy chọn các phần tử trước",
    ],
    [
        "IDs of Selection: the ID, GUID and IFC GlobalId of {}",
        "IDs da seleção: o ID, o GUID e o GlobalId IFC de {}",
        "ID de la selección: el ID, el GUID y el GlobalId IFC de {}",
        "ID của phần đang chọn: ID, GUID và GlobalId IFC của {}",
    ],
    [
        "IDs of Selection: the IDs, GUIDs and IFC GlobalIds of the {} selected",
        "IDs da seleção: os IDs, GUIDs e GlobalIds IFC dos {} selecionados",
        "ID de la selección: los ID, GUID y GlobalId IFC de los {} seleccionados",
        "ID của phần đang chọn: ID, GUID và GlobalId IFC của {} phần tử đang chọn",
    ],
    [
        "IDs, GUIDs or IFC GlobalIds, as many as you like:",
        "IDs, GUIDs ou GlobalIds IFC, quantos quiser:",
        "ID, GUID o GlobalId IFC, tantos como quiera:",
        "ID, GUID hoặc GlobalId IFC, bao nhiêu cũng được:",
    ],
    ["Finds {}.", "Encontra {}.", "Encuentra {}.", "Tìm thấy {}."],
    [
        "Finds {}; nothing has these: {}",
        "Encontra {}; nada tem estes: {}",
        "Encuentra {}; nada tiene estos: {}",
        "Tìm thấy {}; không gì có những cái này: {}",
    ],
    ["Add to the selection", "Adicionar à seleção", "Añadir a la selección", "Thêm vào phần đang chọn"],
    [
        "Show: open the plan they are in, zoomed to them",
        "Mostrar: abrir a planta em que estão, com zoom neles",
        "Mostrar: abrir la planta en que están, con zoom en ellos",
        "Hiện: mở mặt bằng chứa chúng, thu phóng tới chúng",
    ],
    [
        "Type or paste an ID, a GUID or an IFC GlobalId.",
        "Digite ou cole um ID, um GUID ou um GlobalId IFC.",
        "Escriba o pegue un ID, un GUID o un GlobalId IFC.",
        "Nhập hoặc dán một ID, một GUID hoặc một GlobalId IFC.",
    ],
    ["Nothing has that ID or GUID: {}", "Nada tem esse ID ou GUID: {}", "Nada tiene ese ID o GUID: {}", "Không gì có ID hoặc GUID đó: {}"],
    ["Selected {}.", "{} selecionados.", "{} seleccionados.", "Đã chọn {}."],
    [
        "Selected {}; nothing has these: {}",
        "{} selecionados; nada tem estes: {}",
        "{} seleccionados; nada tiene estos: {}",
        "Đã chọn {}; không gì có những cái này: {}",
    ],
    ["ID", "ID", "ID", "ID"],
    ["GUID", "GUID", "GUID", "GUID"],
    ["IFC GlobalId", "GlobalId IFC", "GlobalId IFC", "GlobalId IFC"],
    [
        "Copy its ID, GUID and IFC GlobalId",
        "Copiar seu ID, GUID e GlobalId IFC",
        "Copiar su ID, GUID y GlobalId IFC",
        "Sao chép ID, GUID và GlobalId IFC của nó",
    ],
    [
        "Copied the names of {} {}.",
        "Copiados os identificadores de {} {}.",
        "Copiados los identificadores de {} {}.",
        "Đã sao chép tên của {} {}.",
    ],
    ["{} selected.", "{} selecionados.", "{} seleccionados.", "{} phần tử đang chọn."],
    ["Copy IDs", "Copiar IDs", "Copiar ID", "Sao chép ID"],
    ["Copy GUIDs", "Copiar GUIDs", "Copiar GUID", "Sao chép GUID"],
    ["Copy IFC GlobalIds", "Copiar GlobalIds IFC", "Copiar GlobalId IFC", "Sao chép GlobalId IFC"],
    ["Copy All", "Copiar tudo", "Copiar todo", "Sao chép tất cả"],
    ["Copied {} IDs.", "{} IDs copiados.", "{} ID copiados.", "Đã sao chép {} ID."],
    ["Copied {} GUIDs.", "{} GUIDs copiados.", "{} GUID copiados.", "Đã sao chép {} GUID."],
    ["Copied {} IFC GlobalIds.", "{} GlobalIds IFC copiados.", "{} GlobalId IFC copiados.", "Đã sao chép {} GlobalId IFC."],
    ["material", "material", "material", "vật liệu"],
    ["picture", "imagem", "imagen", "ảnh"],
    ["family", "família", "familia", "cấu kiện"],
    ["parameter", "parâmetro", "parámetro", "tham số"],
    ["view filter", "filtro de vista", "filtro de vista", "bộ lọc khung nhìn"],
    ["view template", "modelo de vista", "plantilla de vista", "mẫu khung nhìn"],
    ["phase", "fase", "fase", "pha"],
    ["phase filter", "filtro de fase", "filtro de fase", "bộ lọc pha"],
    ["design option set", "conjunto de opções de projeto", "conjunto de opciones de diseño", "tập phương án thiết kế"],
    ["design option", "opção de projeto", "opción de diseño", "phương án thiết kế"],
    [
        "In the library, not elements to select: {}",
        "Na biblioteca, não elementos a selecionar: {}",
        "En la biblioteca, no elementos que seleccionar: {}",
        "Nằm trong thư viện, không phải phần tử để chọn: {}",
    ],
    [
        "Finds {} in the library, not elements.",
        "Encontra {} na biblioteca, não elementos.",
        "Encuentra {} en la biblioteca, no elementos.",
        "Tìm thấy {} trong thư viện, không phải phần tử.",
    ],
    [
        "Copied {} rows, tab-separated.",
        "{} linhas copiadas, separadas por tabulação.",
        "{} filas copiadas, separadas por tabulación.",
        "Đã sao chép {} dòng, phân tách bằng tab.",
    ],
    // Palettes that dock and float, views that float, the ribbon folded, File > Settings.
    ["Dock Left", "Encaixar à esquerda", "Acoplar a la izquierda", "Gắn trái"],
    ["Dock Right", "Encaixar à direita", "Acoplar a la derecha", "Gắn phải"],
    ["Dock Bottom", "Encaixar embaixo", "Acoplar abajo", "Gắn dưới"],
    ["Float", "Flutuar", "Flotar", "Nổi"],
    ["Dock {}", "Encaixar {}", "Acoplar {}", "Gắn {}"],
    ["Tab with {}", "Aba com {}", "Pestaña con {}", "Ghép thẻ với {}"],
    ["Dock in the Views", "Encaixar nas vistas", "Acoplar en las vistas", "Gắn vào Khung nhìn"],
    ["Expand the Ribbon", "Expandir a faixa de opções", "Expandir la cinta de opciones", "Mở rộng dải lệnh"],
    ["Minimize the Ribbon", "Minimizar a faixa de opções", "Minimizar la cinta de opciones", "Thu gọn dải lệnh"],
    ["Settings…", "Configurações…", "Configuración…", "Cài đặt…"],
    ["Performance", "Desempenho", "Rendimiento", "Hiệu năng"],
    ["Files", "Arquivos", "Archivos", "Tệp tin"],
    ["Updates", "Atualizações", "Actualizaciones", "Cập nhật"],
    ["Theme", "Tema", "Tema", "Chủ đề"],
    ["As the System", "Como o sistema", "Como el sistema", "Theo hệ thống"],
    ["Light", "Claro", "Claro", "Sáng"],
    ["Dark", "Escuro", "Oscuro", "Tối"],
    ["Dark UI, Light Canvas", "Interface escura, tela clara", "Interfaz oscura, lienzo claro", "Giao diện tối, khung vẽ sáng"],
    ["Layout", "Disposição", "Disposición", "Bố cục"],
    ["Floating", "Flutuante", "Flotante", "Nổi"],
    ["Compact", "Compacta", "Compacta", "Gọn"],
    [
        "The ribbon, the palettes and the bars as cards floating over the view",
        "A faixa de opções, as paletas e as barras como cartões flutuando sobre a vista",
        "La cinta de opciones, las paletas y las barras como tarjetas flotando sobre la vista",
        "Dải lệnh, các bảng và các thanh là thẻ nổi trên khung nhìn",
    ],
    [
        "The ribbon, the palettes and the bars flush to the window's edges",
        "A faixa de opções, as paletas e as barras rentes às bordas da janela",
        "La cinta de opciones, las paletas y las barras pegadas a los bordes de la ventana",
        "Dải lệnh, các bảng và các thanh sát mép cửa sổ",
    ],
    ["Ribbon", "Faixa de opções", "Cinta de opciones", "Dải lệnh"],
    [
        "Minimize the ribbon to its tabs",
        "Minimizar a faixa de opções às abas",
        "Minimizar la cinta de opciones a sus pestañas",
        "Thu gọn dải lệnh về các thẻ của nó",
    ],
    [
        "Double-clicking a tab's name does the same; a click on a tab then shows its tools until one is picked",
        "Um clique duplo no nome de uma aba faz o mesmo; um clique numa aba mostra então suas ferramentas até que uma seja escolhida",
        "Un doble clic en el nombre de una pestaña hace lo mismo; un clic en una pestaña muestra entonces sus herramientas hasta que se elige una",
        "Bấm đúp vào tên một thẻ cũng vậy; sau đó bấm vào một thẻ sẽ hiện các công cụ của nó cho tới khi một công cụ được chọn",
    ],
    ["Palettes", "Paletas", "Paletas", "Các bảng"],
    ["Minimize", "Minimizar", "Minimizar", "Thu nhỏ"],
    ["Maximize", "Maximizar", "Maximizar", "Phóng to"],
    ["Restore Down", "Restaurar tamanho", "Restaurar", "Khôi phục xuống"],
    ["Title bar", "Barra de título", "Barra de título", "Thanh tiêu đề"],
    [
        "Use the system's title bar",
        "Usar a barra de título do sistema",
        "Usar la barra de título del sistema",
        "Dùng thanh tiêu đề của hệ thống",
    ],
    [
        "The system's own title bar and frame, rather than the window's: the app bar then sits under it",
        "A barra de título e a moldura do próprio sistema, em vez das da janela: a barra do aplicativo fica então abaixo dela",
        "La barra de título y el marco del propio sistema, en lugar de los de la ventana: la barra de la aplicación queda debajo",
        "Thanh tiêu đề và khung của chính hệ thống, thay vì của cửa sổ: thanh ứng dụng nằm dưới nó",
    ],
    ["Reset Palette Layout", "Redefinir a disposição das paletas", "Restablecer la disposición de las paletas", "Đặt lại bố cục bảng"],
    [
        "Properties over the Project Browser, on the left, as at first",
        "Propriedades sobre o Navegador de projeto, à esquerda, como no início",
        "Propiedades sobre el Navegador de proyectos, a la izquierda, como al principio",
        "Thuộc tính trên Trình duyệt dự án, ở bên trái, như lúc đầu",
    ],
    ["Coming in a later version", "Em uma versão futura", "En una versión futura", "Sẽ có trong một phiên bản sau"],
    ["Interface scale", "Escala da interface", "Escala de la interfaz", "Tỷ lệ giao diện"],
    [
        "Show shortcut keys in tooltips",
        "Mostrar os atalhos nas dicas",
        "Mostrar los atajos en las sugerencias",
        "Hiện phím tắt trong chú thích",
    ],
    [
        "Open the last project at startup",
        "Abrir o último projeto ao iniciar",
        "Abrir el último proyecto al iniciar",
        "Mở dự án cuối cùng khi khởi động",
    ],
    [
        "Reopen the views that were open",
        "Reabrir as vistas que estavam abertas",
        "Volver a abrir las vistas que estaban abiertas",
        "Mở lại các khung nhìn đã mở",
    ],
    [
        "This copy doesn't look for updates.",
        "Esta cópia não procura atualizações.",
        "Esta copia no busca actualizaciones.",
        "Bản này không tìm bản cập nhật.",
    ],
    ["Undo steps kept", "Passos de desfazer guardados", "Pasos de deshacer guardados", "Số bước hoàn tác được giữ"],
    ["3D view resolution", "Resolução da vista 3D", "Resolución de la vista 3D", "Độ phân giải khung nhìn 3D"],
    ["Full", "Total", "Completa", "Đầy đủ"],
    ["Half", "Metade", "Mitad", "Một nửa"],
    ["Smooth lines (anti-aliasing)", "Linhas suaves (anti-aliasing)", "Líneas suaves (antialiasing)", "Làm mượt đường (khử răng cưa)"],
    [
        "Redraw views in the background",
        "Redesenhar as vistas em segundo plano",
        "Redibujar las vistas en segundo plano",
        "Vẽ lại các khung nhìn trong nền",
    ],
    [
        "Draw the 3D view on the graphics card",
        "Desenhar a vista 3D na placa de vídeo",
        "Dibujar la vista 3D en la tarjeta gráfica",
        "Vẽ khung nhìn 3D trên card đồ họa",
    ],
    ["Graphics card", "Placa de vídeo", "Tarjeta gráfica", "Card đồ họa"],
    [
        "Faster for large models. Hidden line and wireframe are still drawn on the processor, and so is everything if the card fails",
        "Mais rápido para modelos grandes. Linhas ocultas e arame ainda são desenhados no processador, e tudo também se a placa falhar",
        "Más rápido para modelos grandes. La línea oculta y el alambre se siguen dibujando en el procesador, y todo si la tarjeta falla",
        "Nhanh hơn với mô hình lớn. Đường khuất và dạng dây vẫn được vẽ trên vi xử lý, và mọi thứ cũng vậy nếu card lỗi",
    ],
    [
        "This copy has no graphics card it can draw the 3D view on",
        "Esta cópia não tem placa de vídeo em que possa desenhar a vista 3D",
        "Esta copia no tiene una tarjeta gráfica en la que dibujar la vista 3D",
        "Bản này không có card đồ họa nào để vẽ khung nhìn 3D",
    ],
    ["Card: {}", "Placa: {}", "Tarjeta: {}", "Card đồ hoạ: {}"],
    [
        "The 3D view is drawn on the processor: {}",
        "A vista 3D é desenhada no processador: {}",
        "La vista 3D se dibuja en el procesador: {}",
        "Khung nhìn 3D được vẽ trên vi xử lý: {}",
    ],
    [
        "The graphics card couldn't draw the 3D view ({}): it is drawn on the processor",
        "A placa de vídeo não conseguiu desenhar a vista 3D ({}): ela é desenhada no processador",
        "La tarjeta gráfica no pudo dibujar la vista 3D ({}): se dibuja en el procesador",
        "Card đồ họa không vẽ được khung nhìn 3D ({}): nó được vẽ trên vi xử lý",
    ],
    ["Selection colour", "Cor da seleção", "Color de la selección", "Màu phần đang chọn"],
    ["High contrast colour", "Cor de alto contraste", "Color de alto contraste", "Màu tương phản cao"],
    ["Reset Colour", "Redefinir cor", "Restablecer color", "Đặt lại màu"],
    [
        "What High Contrast Point Clouds, on the view control bar, draws a view's scans in",
        "A cor em que Nuvens de pontos em alto contraste, na barra de controle da vista, desenha as nuvens de uma vista",
        "El color en que Nubes de puntos en alto contraste, en la barra de control de vista, dibuja las nubes de una vista",
        "Màu mà Đám mây điểm tương phản cao, trên thanh điều khiển khung nhìn, dùng để vẽ đám mây điểm của một khung nhìn",
    ],
    [
        "Highlight what's under the cursor",
        "Destacar o que está sob o cursor",
        "Resaltar lo que está bajo el cursor",
        "Làm nổi thứ dưới con trỏ",
    ],
    ["Background of the 3D view", "Fundo da vista 3D", "Fondo de la vista 3D", "Nền của khung nhìn 3D"],
    ["Plain", "Liso", "Liso", "Trơn"],
    ["Sky", "Céu", "Cielo", "Trời"],
    [
        "Save a recovery copy every",
        "Salvar uma cópia de recuperação a cada",
        "Guardar una copia de recuperación cada",
        "Lưu một bản sao phục hồi mỗi",
    ],
    ["Backups kept", "Cópias de segurança guardadas", "Copias de seguridad guardadas", "Số bản sao được giữ"],
    ["Folder for new projects", "Pasta para novos projetos", "Carpeta para proyectos nuevos", "Thư mục cho dự án mới"],
    ["All", "Tudo", "Todo", "Tất cả"],
    ["List {}", "Listar {}", "Listar {}", "Liệt kê {}"],
    ["Views", "Vistas", "Vistas", "Các khung nhìn"],
    ["Sheets", "Folhas", "Planos", "Bản vẽ"],
    ["Legends", "Legendas", "Leyendas", "Chú thích"],
    ["Search…", "Pesquisar…", "Buscar…", "Tìm…"],
    [
        "Search the Project Browser",
        "Pesquisar no Navegador de projeto",
        "Buscar en el Navegador de proyectos",
        "Tìm trong Trình duyệt dự án",
    ],
    ["Clear the Search", "Limpar a pesquisa", "Borrar la búsqueda", "Xóa ô tìm"],
    ["Nothing here yet.", "Nada aqui ainda.", "Nada aquí todavía.", "Chưa có gì ở đây."],
    ["Nothing matches the search.", "Nada corresponde à pesquisa.", "Nada coincide con la búsqueda.", "Không gì khớp với ô tìm."],
    [
        "{} opened without {} fields this version doesn't know; saving it leaves them out.",
        "{} foi aberto sem {} campos que esta versão não conhece; ao salvar, eles ficam de fora.",
        "{} se abrió sin {} campos que esta versión no conoce; al guardarlo, quedan fuera.",
        "{} được mở mà thiếu {} trường mà phiên bản này không biết; khi lưu sẽ bỏ chúng ra.",
    ],
    // The start page.
    ["Just now", "Agora mesmo", "Justo ahora", "Vừa xong"],
    ["{} min ago", "há {} min", "hace {} min", "{} phút trước"],
    ["{} h ago", "há {} h", "hace {} h", "{} giờ trước"],
    ["Yesterday", "Ontem", "Ayer", "Hôm qua"],
    ["{} days ago", "há {} dias", "hace {} días", "{} ngày trước"],
    ["{} months ago", "há {} meses", "hace {} meses", "{} tháng trước"],
    ["{} years ago", "há {} anos", "hace {} años", "{} năm trước"],
    ["New Project", "Novo projeto", "Nuevo proyecto", "Dự án mới"],
    [
        "An empty project to model in.",
        "Um projeto vazio para modelar.",
        "Un proyecto vacío para modelar.",
        "Một dự án trống để dựng mô hình.",
    ],
    [
        "A BIMCraft project, an IFC or a Revit file.",
        "Um projeto BIMCraft, um arquivo IFC ou do Revit.",
        "Un proyecto de BIMCraft, un archivo IFC o de Revit.",
        "Một dự án BIMCraft, một tệp IFC hoặc một tệp Revit.",
    ],
    [
        "A BIMCraft project or an IFC file.",
        "Um projeto BIMCraft ou um arquivo IFC.",
        "Un proyecto de BIMCraft o un archivo IFC.",
        "Một dự án BIMCraft hoặc một tệp IFC.",
    ],
    ["Back to {}", "Voltar a {}", "Volver a {}", "Quay lại {}"],
    ["Back", "Voltar", "Volver", "Quay lại"],
    ["Back to the project ({}D)", "Voltar ao projeto ({}D)", "Volver al proyecto ({}D)", "Quay lại dự án ({}D)"],
    ["Undo History", "Histórico de desfazer", "Historial de deshacer", "Lịch sử hoàn tác"],
    ["Redo History", "Histórico de refazer", "Historial de rehacer", "Lịch sử làm lại"],
    ["Create {}", "Criar {}", "Crear {}", "Tạo {}"],
    ["Undo {}", "Desfazer {}", "Deshacer {}", "Hoàn tác {}"],
    ["Redo {}", "Refazer {}", "Rehacer {}", "Làm lại {}"],
    ["Undo {} steps", "Desfazer {} passos", "Deshacer {} pasos", "Hoàn tác {} bước"],
    ["Redo {} steps", "Refazer {} passos", "Rehacer {} pasos", "Làm lại {} bước"],
    [
        "The project stays as you left it.",
        "O projeto fica como você o deixou.",
        "El proyecto queda como lo dejaste.",
        "Dự án vẫn như bạn để lại.",
    ],
    ["Recent", "Recentes", "Recientes", "Gần đây"],
    ["No recent projects yet", "Nenhum projeto recente ainda", "Aún no hay proyectos recientes", "Chưa có dự án gần đây nào"],
    [
        "The projects you open or save show here.",
        "Os projetos que você abrir ou salvar aparecem aqui.",
        "Los proyectos que abras o guardes aparecen aquí.",
        "Các dự án bạn mở hoặc lưu hiện ở đây.",
    ],
    ["Remove from List", "Remover da lista", "Quitar de la lista", "Bỏ khỏi danh sách"],
    ["Not found", "Não encontrado", "No encontrado", "Không tìm thấy"],
    ["Drawing…", "Desenhando…", "Dibujando…", "Đang vẽ…"],
    ["Source code on GitHub", "Código-fonte no GitHub", "Código fuente en GitHub", "Mã nguồn trên GitHub"],
    ["Version {}", "Versão {}", "Versión {}", "Phiên bản {}"],
    ["BIMCraft {} is available", "O BIMCraft {} está disponível", "BIMCraft {} está disponible", "BIMCraft {} đã có"],
    [
        "BIMCraft {} is ready to install",
        "O BIMCraft {} está pronto para instalar",
        "BIMCraft {} está listo para instalar",
        "BIMCraft {} đã sẵn sàng để cài",
    ],
    ["Checking for updates…", "Procurando atualizações…", "Buscando actualizaciones…", "Đang kiểm tra cập nhật…"],
    ["BIMCraft {} is up to date", "O BIMCraft {} está atualizado", "BIMCraft {} está actualizado", "BIMCraft {} đã là bản mới nhất"],
    ["Check for updates", "Procurar atualizações", "Buscar actualizaciones", "Kiểm tra cập nhật"],
    [
        "Building information modelling, open and free.",
        "Modelagem da informação da construção, aberta e livre.",
        "Modelado de información de construcción, abierto y libre.",
        "Mô hình thông tin công trình, mở và miễn phí.",
    ],
    ["Home", "Início", "Inicio", "Trang chủ"],
    [
        "Legends are coming in a later version.",
        "As legendas virão em uma versão futura.",
        "Las leyendas llegarán en una versión futura.",
        "Chú thích sẽ có trong một phiên bản sau.",
    ],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_is_its_own_key_and_the_others_fall_back_to_the_cores_words() {
        set(Lang::English);
        assert_eq!((t("Wall Schedule"), t("Wall")), ("Wall Schedule", "Wall"));
        set(Lang::Portuguese);
        assert_eq!((t("Wall Schedule"), t("Arquitetura"), t("Architecture")), ("Tabela de paredes", "Arquitetura", "Arquitetura"));
        assert_eq!(
            tf("roof: the slope `{}` and the overhang `{}` must be numbers", &["x", "y"]),
            "telhado: a inclinação `x` e o beiral `y` precisam ser números"
        );
        assert_eq!((category("wall", true), category("spot", false)), ("Paredes".to_string(), "Elevação de ponto".to_string()));
        set(Lang::Spanish);
        assert_eq!((t("Wall Schedule"), t("Save changes to {}?")), ("Tabla de muros", "¿Guardar los cambios en {}?"));
        assert_eq!((category("room", true), category("viewport", false)), ("Habitaciones".to_string(), "Vista en plano".to_string()));
        set(Lang::English);
    }

    #[test]
    fn chinese_has_its_own_words_and_what_it_lacks_is_english() {
        set(Lang::Chinese);
        assert_eq!(
            (t("Architecture"), t("File"), t("Wall Schedule"), t("Copy")),
            ("建筑", "文件", "墙明细表", "复制"),
            "the window's words first, then the core's"
        );
        assert_eq!(tf("Save changes to {}?", &["house"]), "是否将更改保存到 house？");
        assert_eq!((category("wall", true), category("wall", false), category("spot", false)), ("墙".into(), "墙".into(), "高程点".into()));
        assert_eq!(category("airterminal", true), "airterminal", "a category without a name is its key, as in the other languages");
        assert_eq!((Lang::Chinese.label(), Lang::Portuguese.label()), ("简体中文", "Português (Brasil)"));
        set(Lang::English);
        assert_eq!(
            (t("Architecture"), Lang::Chinese.label()),
            ("Architecture", "Chinese (Simplified)"),
            "hanzi only once the window can draw them"
        );
    }

    #[test]
    fn every_chinese_row_is_a_row_of_the_table_with_its_placeholders_and_the_table_is_nearly_all_covered() {
        for [english, chinese] in zh::WORDS {
            assert!(WORDS.iter().any(|w| w[0] == *english), "`{english}` is not a row of the table");
            assert_eq!(english.matches("{}").count(), chinese.matches("{}").count(), "`{english}`: the `{{}}`s of `{chinese}`");
            assert!(!chinese.trim().is_empty(), "`{english}` is blank");
        }
        // A row without Chinese shows in English, so a new label need not wait for one; but a table that lets most go has lost it.
        for [english, chinese] in zh::CORE {
            assert!(Language::Portuguese.translated(english).is_some(), "`{english}` is not one of the core's words");
            assert!(!chinese.trim().is_empty(), "`{english}` is blank");
        }
        let missing: Vec<&str> = WORDS.iter().map(|w| w[0]).filter(|e| !zh::WORDS.iter().any(|z| z[0] == *e)).collect();
        assert!(
            missing.len() * 20 <= WORDS.len(),
            "{} rows have no Chinese, such as {:?}",
            missing.len(),
            &missing[..missing.len().min(5)]
        );
    }

    #[test]
    fn the_codes_and_locales_that_mean_simplified_chinese_and_the_ones_that_do_not() {
        for code in ["zh", "zh-CN", "zh_CN.UTF-8", "zh-Hans", "zh_Hans_CN", "zh-SG", "ZH", "chinese", "简体中文"] {
            assert_eq!(Lang::from_code(code), Some(Lang::Chinese), "{code}");
        }
        for code in ["zh-TW", "zh_HK.UTF-8", "zh-Hant", "zh-Hant-TW", "zh_MO"] {
            assert_eq!(Lang::from_code(code), None, "{code}: Traditional is not what is here");
        }
        assert_eq!(
            ["pt-BR", "es_419.UTF-8", "en-US", "de", ""].map(Lang::from_code),
            [Some(Lang::Portuguese), Some(Lang::Spanish), Some(Lang::English), None, None]
        );
        assert_eq!((Lang::Chinese.code(), Lang::Chinese.project()), ("zh", Language::English), "a project is never in Chinese");
        assert_eq!(Lang::ALL.map(|l| Lang::from_code(l.code())), Lang::ALL.map(Some));
        assert_eq!([Lang::Vietnamese.code(), Lang::Vietnamese.label()], ["vi", "Tiếng Việt"]);
        assert_eq!(Lang::Vietnamese.project(), Language::Vietnamese, "a project is in Vietnamese, so its drawings print it too");
    }

    #[test]
    fn every_row_of_the_table_says_the_same_thing_in_every_language() {
        for &[english, portuguese, spanish, vietnamese] in WORDS {
            for (language, cell) in [("pt", portuguese), ("es", spanish), ("vi", vietnamese)] {
                assert!(!cell.trim().is_empty(), "`{english}` is blank in {language}");
                // A key with a leading space (` Left out: {}{}.`) keeps it in the cell, as the joined text needs it:
                // what is wrong is a space the key has not.
                if cell != cell.trim() {
                    // Only a key with a space of its own (` Left out: {}{}.`) may have one, and only where it has it.
                    assert_eq!(
                        (cell.starts_with(' '), cell.ends_with(' ')),
                        (english.starts_with(' '), english.ends_with(' ')),
                        "`{english}` in {language} has a space it did not come with: `{cell}`"
                    );
                }
                assert_eq!(english.matches("{}").count(), cell.matches("{}").count(), "`{english}` in {language}: the `{{}}`s of `{cell}`");
            }
            // Where the core has the word too, Vietnamese there must be Vietnamese and not the English left in by
            // mistake: a row the window's table lacks falls back to the core's, so a blank one would show English.
            if let Some(core) = Language::Vietnamese.translated(english) {
                assert!(!core.trim().is_empty(), "`{english}` is blank in the core's table");
            }
        }
        assert_eq!(Language::English.translated("Level"), None, "English is its own key");
    }

    #[test]
    fn the_letters_a_vietnamese_font_must_have_are_the_ones_vietnamese_text_needs() {
        // Every letter of every Vietnamese cell must be one the finder demands, or a font that covers the set
        // would still draw a box for a word the window shows.
        set(Lang::Vietnamese);
        // Every letter the window can show in Vietnamese: the table's own cells, and the category names, which are
        // not rows of it.
        let needed: std::collections::BTreeSet<char> =
            WORDS.iter().flat_map(|w| w[3].chars()).chain(category_letters()).filter(|c| c.is_alphabetic()).collect();
        set(Lang::English);
        let covered: std::collections::BTreeSet<char> = VIETNAMESE_LETTERS.chars().collect();
        // ASCII is in every font, so what matters is every letter beyond it: a word the window shows must be
        // drawable by a font that covers the set the finder demands.
        let missing: Vec<char> = needed.difference(&covered).filter(|c| !c.is_ascii()).copied().collect();
        assert!(missing.is_empty(), "a font covering VIETNAMESE_LETTERS would still show boxes for {missing:?}");
        assert!(covered.iter().all(|c| !c.is_ascii()), "the set is the letters beyond ASCII");
        assert!(covered.len() >= 100, "the set holds the base letters, the shapes and the tones");
        // What egui's own fonts are missing, spelled out: the count the desktop's write-up gives, and the reason the
        // finder exists at all. A future egui that gains them shows up here, where the write-up can be corrected.
        use egui::epaint::text::{FontDefinitions, Fonts, TextOptions};
        let id = egui::FontId::proportional(12.0);
        let mut fonts = Fonts::new(TextOptions::default(), FontDefinitions::default());
        let egui_lacks: String = covered.iter().filter(|&&c| !fonts.has_glyphs(&id, &c.to_string())).collect();
        assert_eq!(
            egui_lacks.chars().count(),
            88,
            "egui's own fonts lack 88 letters: {egui_lacks:?} — update the desktop's write-up if this changes"
        );
    }

    /// The letters the category names use, upper and lower.
    fn category_letters() -> Vec<char> {
        let mut out = Vec::new();
        for key in ["wall", "door", "window", "level", "room", "sheet", "elevation", "schedule", "pipe", "footing", "shaft"] {
            out.extend(category(key, false).chars());
            out.extend(category(key, true).chars());
        }
        out
    }

    #[test]
    fn vietnamese_has_its_own_words_and_the_cores_where_the_window_lacks_them() {
        set(Lang::Vietnamese);
        assert_eq!(
            (t("Architecture"), t("File"), t("Wall Schedule"), t("Gross Floor Area")),
            ("Kiến trúc", "Tệp", "Bảng thống kê tường", "Tổng diện tích sàn"),
            "the window's words first, then the core's"
        );
        assert_eq!(t("Copy"), "Sao chép", "a word the window's table has is the window's, not the core's");
        assert_eq!(tf("Save changes to {}?", &["house"]), "Lưu thay đổi vào house?", "the placeholders are filled in, in order");
        assert_eq!(
            (category("wall", true), category("wall", false), category("spot", false)),
            ("Tường".into(), "Tường".into(), "Cao độ điểm".into())
        );
        set(Lang::English);
    }
}
