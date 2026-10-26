//! The interface in Vietnamese. No test draws a glyph: a window is told it has a font, or that it has none.

use super::*;
use crate::lang::{self, Lang};
use crate::settings::language_in;

fn settings_file(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("bimcraft-ui-vietnamese-{}-{name}", std::process::id())).join("settings.json")
}

#[test]
fn the_window_speaks_vietnamese_once_it_is_put_in_it_and_keeps_to_it() {
    let file = settings_file("kept");
    let mut h = app(BOX);
    h.state_mut().use_settings(file.clone());
    lang::assume_vietnamese_font(&h.ctx);
    let ctx = h.ctx.clone();
    h.state_mut().set_lang(&ctx, Lang::Vietnamese);
    h.run();
    assert!(h.state().lang == Lang::Vietnamese, "the window is {:?}, status: {:?}", h.state().lang, h.state().status);
    for word in ["Kiến trúc", "Chú thích", "Khung nhìn", "Quản lý", "Sửa"] {
        assert!(h.query_all_by_label(word).next().is_some(), "the ribbon's tab `{word}`");
    }
    assert!(h.query_by_label("Architecture").is_none());
    // New projects are named in the project's language, which Vietnamese is: a drawing is named in it too.
    press(&mut h, "Tệp");
    h.get_by_label_contains("Mới").click();
    h.run();
    let levels: Vec<String> = h.state().session.model.levels().iter().map(|(_, l)| l.name.clone()).collect();
    assert_eq!(levels, ["Cao độ 1", "Cao độ 2"]);
    // The settings file has it, and a window started on the file is in it.
    assert_eq!(language_in(&file), Some(Lang::Vietnamese));
    let mut again = app(BOX);
    again.state_mut().use_settings(file.clone());
    assert_eq!(again.state().lang, Lang::Vietnamese);
    let _ = std::fs::remove_dir_all(file.parent().unwrap());
}

#[test]
fn without_a_font_for_the_marks_the_window_stays_as_it_was_and_says_why() {
    let mut h = app(BOX);
    h.state_mut().font_finder = |_| None;
    let ctx = h.ctx.clone();
    h.state_mut().set_lang(&ctx, Lang::Vietnamese);
    h.run();
    assert_eq!(h.state().lang, Lang::English);
    let status = h.state().status.clone();
    assert!(status.error && status.text.starts_with("No font with Vietnamese characters"), "{status:?}");
    // A settings file that says Vietnamese on a machine with no font: the window comes up in English, not in boxes.
    let mut start = app(BOX);
    start.state_mut().font_finder = |_| None;
    start.state_mut().lang = Lang::Vietnamese;
    start.run();
    assert_eq!(start.state().lang, Lang::English);
    assert!(start.state().status.error);
    // In Portuguese, the reason is said in it.
    let mut pt = app(BOX);
    pt.state_mut().font_finder = |_| None;
    pt.state_mut().lang = Lang::Portuguese;
    pt.run();
    let ctx = pt.ctx.clone();
    pt.state_mut().set_lang(&ctx, Lang::Vietnamese);
    assert!(pt.state().status.text.starts_with("Nenhuma fonte com caracteres vietnamitas"), "{:?}", pt.state().status);
}

#[test]
fn the_language_list_offers_vietnamese_and_picking_it_puts_the_window_in_it() {
    let mut h = app(BOX);
    lang::assume_vietnamese_font(&h.ctx);
    h.key_press_modifiers(Modifiers::COMMAND, Key::Comma);
    h.run();
    let ctx = h.ctx.clone();
    h.state_mut().set_lang(&ctx, Lang::Vietnamese);
    h.run();
    assert_eq!(h.state().lang, Lang::Vietnamese);
    // The window is in it, so Settings offers it as the language it is in.
    assert!(h.get_all_by_role(Role::ComboBox).any(|n| n.value().as_deref() == Some("Tiếng Việt")));
    // The ribbon is drawn in it.
    press(&mut h, "Kiến trúc");
    assert!(h.query_by_label("Kiến trúc").is_some(), "the ribbon's tab, in Vietnamese");
    // A new project takes the window's language, so its drawings print Vietnamese: the lettering is the core's own,
    // drawn in code, so it needs no font of the system's.
    press(&mut h, "Tệp");
    h.get_by_label_contains("Mới").click();
    h.run();
    assert_eq!(h.state().session.model.language, bimcraft_core::language::Language::Vietnamese);
    let levels: Vec<String> = h.state().session.model.levels().iter().map(|(_, l)| l.name.clone()).collect();
    assert_eq!(levels, ["Cao độ 1", "Cao độ 2"], "a new project is named in the window's language");
}

/// A name says little about a font's marks, the glyphs decide; and every one of them must be there.
#[test]
fn a_font_is_taken_only_once_it_has_every_vietnamese_letter() {
    let letters = lang::VIETNAMESE_LETTERS;
    assert!(letters.contains('ế') && letters.contains('ộ') && letters.contains('ự'), "the marks of the words a schedule prints");
    let unique: std::collections::HashSet<char> = letters.chars().collect();
    assert_eq!(letters.chars().count(), unique.len(), "every letter once");
    assert!(letters.chars().all(|c| !c.is_ascii()), "the set is the letters beyond ASCII, which every font has");
    assert!(
        ["ế", "ộ", "ự", "đ", "ă", "ơ", "ư", "á", "à", "ã", "Á"].iter().all(|c| letters.contains(c)),
        "the shapes, the tones and the loose marks"
    );
    // A font the finder offers really covers them: that is why it offered it.
    if let Some((bytes, index)) = lang::vietnamese_font() {
        assert!(lang::font_covers(bytes, index, letters), "a font the finder offers covers every letter");
    }
    // The fonts egui ships do not: a window given one is told there is none rather than drawing boxes. The bundled
    // Ubuntu-Light lacks 28 of the 71, so even one of them is enough to refuse it.
    let bundled = egui::FontDefinitions::default();
    let ubuntu = bundled.font_data.iter().find(|(name, _)| name.contains("Ubuntu")).map(|(_, f)| f.font.to_vec());
    if let Some(bytes) = ubuntu {
        assert!(!lang::font_covers(bytes, 0, "ệuỹỷ"), "the font egui bundles cannot draw every Vietnamese mark");
    }
    // Bytes that are not a font at all: no panic, no coverage.
    assert!(!lang::font_covers(Vec::new(), 0, "a"));
    assert!(!lang::font_covers(b"not a font".to_vec(), 0, "a"));
    assert!(!lang::font_covers(vec![0u8; 64], 0, "a"));
}

/// A word's marked letters and its unmarked ones must be drawn by one face, or the marked ones look bold: the font goes in
/// front of egui's own, so it owns the whole run. The fault this holds down had the font behind, and a window whose
/// Vietnamese text was half Ubuntu-Light (which has no marks) and half the system's font, letter by letter.
#[test]
fn the_whole_word_is_drawn_by_one_face_and_not_only_its_marks() {
    let mut h = app(BOX);
    let ctx = h.ctx.clone();
    h.state_mut().set_lang(&ctx, Lang::Vietnamese);
    h.run();
    if h.state().lang != Lang::Vietnamese {
        return; // no font with the marks on this machine: the window says so and stays as it was
    }
    let owners = ctx.fonts_mut(|fonts| {
        let mut font = fonts.fonts.font(&egui::FontFamily::Proportional);
        let owner = font.characters().clone();
        // A tone-marked ế among unmarked letters, as nearly every Vietnamese word has.
        "Kiến trúc"
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| owner.get(&c).and_then(|n| n.first()).cloned().unwrap_or_default())
            .collect::<std::collections::BTreeSet<String>>()
    });
    assert_eq!(
        owners.len(),
        1,
        "one face draws the whole word, not one per letter: {owners:?} — a second face draws the marked letters heavier"
    );
}

/// The font the finder offers weighs what egui's own does, so a Vietnamese label is not a shade bolder than an English one.
#[test]
fn the_font_taken_weighs_what_egui_s_own_does() {
    // Where the machine has no font with the marks at all, there is nothing to weigh; the window says so instead.
    let Some((bytes, index)) = lang::vietnamese_font() else { return };
    assert!(lang::font_weighs_like(bytes, index), "the font the finder offers weighs like egui's own, so nothing looks bold");
    // And a face that is a third heavier is refused, which is the fault this holds down: Segoe UI, which covers the marks.
    let heavy = std::fs::read(r"C:\Windows\Fonts\segoeui.ttf").ok();
    if let Some(heavy) = heavy
        && lang::font_covers(heavy.clone(), 0, lang::VIETNAMESE_LETTERS)
    {
        assert!(
            !lang::font_weighs_like(heavy, 0),
            "Segoe UI covers the marks and is a third heavier; taken, every Vietnamese word would look bold"
        );
    }
    // Bytes that are not a font: refused, no panic.
    assert!(!lang::font_weighs_like(Vec::new(), 0));
    assert!(!lang::font_weighs_like(b"not a font".to_vec(), 0));
}

/// A Vietnamese label must be the same size as an English one beside it: the font the finder takes draws its letters at
/// the height egui's own draws them. The fault this holds down had the finder take Calibri Light for its weight alone,
/// and the window's text shrank (its x-height is 6 texels at 13 pt against Ubuntu-Light's 7) whenever it was put in
/// Vietnamese.
#[test]
fn the_font_taken_draws_its_letters_at_the_size_egui_s_own_does() {
    let Some((bytes, index)) = lang::vietnamese_font() else { return };
    assert!(lang::font_draws_the_same_size(bytes, index), "the font the finder offers is the size of egui's own, so nothing shrinks");
    // A face that draws smaller is refused, which is the fault this holds down: Calibri Light, right weight and all.
    let light = std::fs::read(r"C:\Windows\Fonts\calibril.ttf").ok();
    if let Some(light) = light
        && lang::font_covers(light.clone(), 0, lang::VIETNAMESE_LETTERS)
        && lang::font_weighs_like(light.clone(), 0)
    {
        assert!(!lang::font_draws_the_same_size(light, 0), "Calibri Light weighs right and draws smaller; taken, the window would shrink");
    }
    // Bytes that are not a font: refused, no panic.
    assert!(!lang::font_draws_the_same_size(Vec::new(), 0));
    assert!(!lang::font_draws_the_same_size(b"not a font".to_vec(), 0));
}
