//! The interface's language: English, as the code writes it, Brazilian Portuguese, Spanish, Vietnamese, Czech, French or Simplified Chinese.
//! What
//! the window shows goes through [`t`], English being its own key; the widget helpers translate
//! their labels and tips, so most call sites pass English and read it back translated. What a
//! project holds (names, what its drawings say) follows the project's language instead, and the
//! core's messages, which agents read too, stay English.
//!
//! Chinese is the window's alone ([`Lang`]): a drawing's lettering is drawn in code and has no hanzi, so a project is never
//! in it. Chinese interface translations, including schedule headings, live in `translations/ui/zh.po`, one entry for each of
//! the interface's messages: an entry it lacks shows in English. The window draws hanzi with a system font (`font.rs`).
//!
//! Vietnamese is a project's language as well (`Language::Vietnamese`), so the window and the drawings say the same thing; its
//! interface translations live in `translations/ui/vi.po`. egui's own fonts lack a quarter of its tone marks, so the window
//! looks for a system font that has them all ([`use_vietnamese_font`]), and a project's lettering is drawn by the core's own
//! stroke font, which carries the marks itself.

use bimcraft_i18n::msg;

mod font;
include!(concat!(env!("OUT_DIR"), "/catalogs.rs"));

// English keys retained for compatibility with text selected indirectly or stored by older versions.
const _: &[&str] = &[
    msg!("Backups kept"),
    msg!("start"),
    msg!("end"),
    msg!("Click which way it climbs: its risers take it up to the level above. Esc cancels."),
    msg!("Open a schedule"),
    msg!("Stair: click where it starts, then which way it climbs"),
    msg!("Dimension"),
    msg!("Dimensions"),
    msg!("Tag"),
    msg!("Windows"),
];

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
    Czech,
    French,
    Chinese,
}

impl Lang {
    pub const ALL: [Lang; 7] = [Lang::English, Lang::Portuguese, Lang::Spanish, Lang::Vietnamese, Lang::Czech, Lang::French, Lang::Chinese];

    /// The project language that goes with it, which new projects are named in: Chinese has none, so English.
    pub fn project(self) -> Language {
        match self {
            Lang::English | Lang::Chinese => Language::English,
            Lang::Portuguese => Language::Portuguese,
            Lang::Spanish => Language::Spanish,
            Lang::Vietnamese => Language::Vietnamese,
            Lang::Czech => Language::Czech,
            Lang::French => Language::French,
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
            Language::English | Language::Swedish => Lang::English,
            Language::Portuguese => Lang::Portuguese,
            Language::Spanish => Lang::Spanish,
            Language::Vietnamese => Lang::Vietnamese,
            Language::Czech => Lang::Czech,
            Language::French => Lang::French,
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
    msg!("No font with Chinese characters was found on this system: install one (such as Noto Sans CJK) and choose Chinese again.");

/// Said when the window cannot be put in Vietnamese.
pub const NO_VIETNAMESE_FONT: &str =
    msg!("No font with Vietnamese characters was found on this system: install one (such as Noto Sans) and choose Vietnamese again.");

/// Every letter a font must have for Vietnamese, tone marks and all: the base letters, the shapes (`ă`, `ơ`, `ư`, `đ`),
/// the stacked marks (`ế`, `ắ`, `ộ`) and the loose ones Vietnamese also writes (`á`, `à`, `ã`, `Á`). It is what
/// [`use_vietnamese_font`] looks for, so a window in Vietnamese is never boxes. The test
/// `the_letters_a_vietnamese_font_must_have_are_the_ones_vietnamese_text_needs` keeps it level with the letters the
/// catalog's own words use.
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
        Lang::English | Lang::Portuguese | Lang::Spanish | Lang::Czech | Lang::French => None,
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

/// `english` in `lang`, when a non-English interface catalog has a finished translation.
pub fn translated_in(lang: Lang, english: &str) -> Option<&'static str> {
    match lang {
        Lang::Chinese => bimcraft_i18n::translated(CATALOGS, lang.code(), "", english),
        other => translated(other.project(), english),
    }
}

/// An interface translation for one of the project languages, when the catalog has it.
pub fn translated(language: Language, english: &str) -> Option<&'static str> {
    if language == Language::English {
        return None;
    }
    bimcraft_i18n::translated(CATALOGS, language.code(), "", english)
}

/// A message whose meaning is distinguished by a PO `msgctxt`.
pub fn tc<'a>(context: &str, english: &'a str) -> &'a str {
    bimcraft_i18n::translated(CATALOGS, get().code(), context, english).unwrap_or(english)
}

/// `template` translated, its `{}`s filled with `args` in order.
pub fn tf(template: &str, args: &[&str]) -> String {
    args.iter().fold(t(template).to_string(), |s, a| s.replacen("{}", a, 1))
}

/// An element category's name, of one (`Wall`) or of several (`Walls`).
pub fn category(category: &str, several: bool) -> String {
    let (one, many) = match category {
        "wall" => (msg!("category", "Wall"), msg!("category-plural", "Walls")),
        "door" => (msg!("category", "Door"), msg!("category-plural", "Doors")),
        "window" => (msg!("category", "Window"), msg!("category-plural", "Windows")),
        "floor" => (msg!("category", "Floor"), msg!("category-plural", "Floors")),
        "room" => (msg!("category", "Room"), msg!("category-plural", "Rooms")),
        "level" => (msg!("category", "Level"), msg!("category-plural", "Levels")),
        "grid" => (msg!("category", "Grid"), msg!("category-plural", "Grids")),
        "dimension" => (msg!("category", "Dimension"), msg!("category-plural", "Dimensions")),
        "tag" => (msg!("category", "Tag"), msg!("category-plural", "Tags")),
        "section" => (msg!("category", "Section"), msg!("category-plural", "Sections")),
        "elevation" => (msg!("category", "Elevation"), msg!("category-plural", "Elevations")),
        "sheet" => (msg!("category", "Sheet"), msg!("category-plural", "Sheets")),
        "viewport" => (msg!("category", "Viewport"), msg!("category-plural", "Viewports")),
        "column" => (msg!("category", "Column"), msg!("category-plural", "Columns")),
        "component" => (msg!("category", "Component"), msg!("category-plural", "Components")),
        "stair" => (msg!("category", "Stair"), msg!("category-plural", "Stairs")),
        "roof" => (msg!("category", "Roof"), msg!("category-plural", "Roofs")),
        "ceiling" => (msg!("category", "Ceiling"), msg!("category-plural", "Ceilings")),
        "shaft" => (msg!("category", "Shaft Opening"), msg!("category-plural", "Shaft Openings")),
        "mass" => (msg!("category", "Mass"), msg!("category-plural", "Masses")),
        "region" => (msg!("category", "Filled Region"), msg!("category-plural", "Filled Regions")),
        "massfloor" => (msg!("category", "Mass Floor"), msg!("category-plural", "Mass Floors")),
        "ramp" => (msg!("category", "Ramp"), msg!("category-plural", "Ramps")),
        "railing" => (msg!("category", "Railing"), msg!("category-plural", "Railings")),
        "roomseparator" => (msg!("category", "Room Separator"), msg!("category-plural", "Room Separators")),
        "beam" => (msg!("category", "Beam"), msg!("category-plural", "Beams")),
        "structuralcolumn" => (msg!("category", "Structural Column"), msg!("category-plural", "Structural Columns")),
        "footing" => (msg!("category", "Footing"), msg!("category-plural", "Footings")),
        "stripfooting" => (msg!("category", "Strip Footing"), msg!("category-plural", "Strip Footings")),
        "pipe" => (msg!("category", "Pipe"), msg!("category-plural", "Pipes")),
        "conduit" => (msg!("category", "Conduit"), msg!("category-plural", "Conduits")),
        "cabletray" => (msg!("category", "Cable Tray"), msg!("category-plural", "Cable Trays")),
        "wire" => (msg!("category", "Wire"), msg!("category-plural", "Wires")),
        "cable" => (msg!("category", "Cable"), msg!("category-plural", "Cables")),
        "downconductor" => (msg!("category", "Down Conductor"), msg!("category-plural", "Down Conductors")),
        "groundconductor" => (msg!("category", "Earthing Conductor"), msg!("category-plural", "Earthing Conductors")),
        "text" => (msg!("category", "Text Note"), msg!("category-plural", "Text Notes")),
        "line" => (msg!("category", "Detail Line"), msg!("category-plural", "Lines")),
        "spot" => (msg!("category", "Spot Elevation"), msg!("category-plural", "Spot Elevations")),
        "schedule" => (msg!("category", "Schedule"), msg!("category-plural", "Schedules")),
        "view3d" => (msg!("category", "3D View"), msg!("category-plural", "3D Views")),
        "link" => (msg!("category", "Linked Model"), msg!("category-plural", "Linked Models")),
        "pointcloud" => (msg!("category", "Point Cloud"), msg!("category-plural", "Point Clouds")),
        "cadimport" => (msg!("category", "CAD Import"), msg!("category-plural", "CAD Imports")),
        other => return other.to_string(),
    };
    tc(if several { "category-plural" } else { "category" }, if several { many } else { one }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_is_its_own_key_and_project_terms_have_interface_translations() {
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
    fn missing_catalog_entries_fall_back_and_categories_use_their_own_contexts() {
        for language in Lang::ALL {
            set(language);
            assert_eq!(t("A new untranslated label"), "A new untranslated label");
            assert_eq!(tf("Untranslated {}", &["value"]), "Untranslated value");
            assert_eq!(tc("missing-context", "Copy"), "Copy");
        }
        set(Lang::Chinese);
        assert_eq!((t("Copy"), tc("missing-context", "Copy")), ("复制", "Copy"));
        assert_eq!(category("link", true), "Linked Models");
        set(Lang::English);
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
        assert_eq!(Lang::from(Language::Swedish), Lang::English, "Swedish is a project language until its interface catalog is added");
        assert_eq!(Lang::ALL.map(|l| Lang::from_code(l.code())), Lang::ALL.map(Some));
        assert_eq!([Lang::Vietnamese.code(), Lang::Vietnamese.label()], ["vi", "Tiếng Việt"]);
        assert_eq!(Lang::Vietnamese.project(), Language::Vietnamese, "a project is in Vietnamese, so its drawings print it too");
    }

    /// Every translation of one language in the compiled interface catalogs.
    fn catalog(language: &str) -> Vec<(&'static str, &'static str, &'static str)> {
        CATALOGS.iter().find(|(code, _)| *code == language).map(|(_, m)| m.to_vec()).unwrap_or_default()
    }

    #[test]
    fn every_catalog_entry_keeps_its_placeholders_and_is_never_blank() {
        // The compiler refuses a blank or placeholder-mismatched *finished* entry, so this holds the invariant over
        // what actually shipped: a key with a leading space (` Left out: {}{}.`) is the one the joined text needs.
        for (language, _) in CATALOGS {
            for (_, english, translation) in catalog(language) {
                assert!(!translation.trim().is_empty(), "`{english}` is blank in {language}");
                if translation != translation.trim() {
                    assert_eq!(
                        (translation.starts_with(' '), translation.ends_with(' ')),
                        (english.starts_with(' '), english.ends_with(' ')),
                        "`{english}` in {language} has a space it did not come with: `{translation}`"
                    );
                }
                assert_eq!(
                    english.matches("{}").count(),
                    translation.matches("{}").count(),
                    "`{english}` in {language}: the `{{}}`s of `{translation}`"
                );
            }
            assert!(!catalog(language).is_empty(), "{language} has no interface translations");
        }
        assert_eq!(Language::English.translated("Level"), None, "English is its own key");
    }

    #[test]
    fn the_letters_a_vietnamese_font_must_have_are_the_ones_vietnamese_text_needs() {
        // Every letter of every Vietnamese translation must be one the finder demands, or a font that covers the set
        // would still draw a box for a word the window shows.
        set(Lang::Vietnamese);
        // Every letter the window can show in Vietnamese: the catalog's own translations, and the category names,
        // which are entries of it too.
        let needed: std::collections::BTreeSet<char> =
            catalog("vi").iter().flat_map(|(_, _, t)| t.chars()).chain(category_letters()).filter(|c| c.is_alphabetic()).collect();
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
        assert_eq!(t("Copy"), "Sao chép", "a word the window's catalog has is the window's, not the core's");
        assert_eq!(tf("Save changes to {}?", &["house"]), "Lưu thay đổi vào house?", "the placeholders are filled in, in order");
        assert_eq!(
            (category("wall", true), category("wall", false), category("spot", false)),
            ("Tường".into(), "Tường".into(), "Cao độ điểm".into())
        );
        set(Lang::English);
    }
}
