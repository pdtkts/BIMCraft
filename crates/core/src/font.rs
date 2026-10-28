//! Our own single-stroke lettering for plan annotation: capitals, digits, a few marks and the
//! accents of Latin languages, Vietnamese's stacked tones among them, drawn on an 8 x 12 grid (y up)
//! with room above for a mark. Lower case is drawn as capitals, as architectural lettering is.

type Glyph = &'static [&'static [(i8, i8)]];

const O: &[(i8, i8)] = &[(2, 0), (0, 2), (0, 10), (2, 12), (6, 12), (8, 10), (8, 2), (6, 0), (2, 0)];
const P: &[(i8, i8)] = &[(0, 0), (0, 12), (6, 12), (8, 10), (8, 8), (6, 6), (0, 6)];
const QUESTION: Glyph = &[&[(0, 10), (2, 12), (6, 12), (8, 10), (8, 8), (4, 5), (4, 3)], &[(4, 1), (4, 0)]];

fn glyph(c: char) -> Glyph {
    match c {
        'A' => &[&[(0, 0), (4, 12), (8, 0)], &[(1, 3), (7, 3)]],
        'B' => &[&[(0, 0), (0, 12), (6, 12), (8, 10), (8, 8), (6, 6), (0, 6)], &[(6, 6), (8, 4), (8, 2), (6, 0), (0, 0)]],
        'C' => &[&[(8, 10), (6, 12), (2, 12), (0, 10), (0, 2), (2, 0), (6, 0), (8, 2)]],
        'D' => &[&[(0, 0), (0, 12), (5, 12), (8, 9), (8, 3), (5, 0), (0, 0)]],
        'E' => &[&[(8, 12), (0, 12), (0, 0), (8, 0)], &[(0, 6), (6, 6)]],
        'F' => &[&[(8, 12), (0, 12), (0, 0)], &[(0, 6), (6, 6)]],
        'G' => &[&[(8, 10), (6, 12), (2, 12), (0, 10), (0, 2), (2, 0), (6, 0), (8, 2), (8, 5), (5, 5)]],
        'H' => &[&[(0, 0), (0, 12)], &[(8, 0), (8, 12)], &[(0, 6), (8, 6)]],
        'I' => &[&[(2, 12), (6, 12)], &[(4, 12), (4, 0)], &[(2, 0), (6, 0)]],
        'J' => &[&[(8, 12), (8, 2), (6, 0), (2, 0), (0, 2)]],
        'K' => &[&[(0, 0), (0, 12)], &[(8, 12), (0, 4)], &[(3, 7), (8, 0)]],
        'L' => &[&[(0, 12), (0, 0), (8, 0)]],
        'M' => &[&[(0, 0), (0, 12), (4, 6), (8, 12), (8, 0)]],
        'N' => &[&[(0, 0), (0, 12), (8, 0), (8, 12)]],
        'O' => &[O],
        'P' => &[P],
        'Q' => &[O, &[(5, 3), (8, 0)]],
        'R' => &[P, &[(4, 6), (8, 0)]],
        'S' => &[&[(8, 10), (6, 12), (2, 12), (0, 10), (0, 8), (2, 6), (6, 6), (8, 4), (8, 2), (6, 0), (2, 0), (0, 2)]],
        'T' => &[&[(0, 12), (8, 12)], &[(4, 12), (4, 0)]],
        'U' => &[&[(0, 12), (0, 2), (2, 0), (6, 0), (8, 2), (8, 12)]],
        'V' => &[&[(0, 12), (4, 0), (8, 12)]],
        'W' => &[&[(0, 12), (2, 0), (4, 6), (6, 0), (8, 12)]],
        'X' => &[&[(0, 0), (8, 12)], &[(0, 12), (8, 0)]],
        'Y' => &[&[(0, 12), (4, 6), (8, 12)], &[(4, 6), (4, 0)]],
        'Z' => &[&[(0, 12), (8, 12), (0, 0), (8, 0)]],
        '0' => &[&[(2, 0), (1, 2), (1, 10), (2, 12), (6, 12), (7, 10), (7, 2), (6, 0), (2, 0)]],
        '1' => &[&[(2, 10), (4, 12), (4, 0)], &[(2, 0), (6, 0)]],
        '2' => &[&[(0, 10), (2, 12), (6, 12), (8, 10), (8, 8), (0, 0), (8, 0)]],
        '3' => &[&[(0, 10), (2, 12), (6, 12), (8, 10), (8, 8), (6, 6), (8, 4), (8, 2), (6, 0), (2, 0), (0, 2)], &[(3, 6), (6, 6)]],
        '4' => &[&[(6, 0), (6, 12), (0, 4), (8, 4)]],
        '5' => &[&[(8, 12), (0, 12), (0, 7), (6, 7), (8, 5), (8, 2), (6, 0), (2, 0), (0, 2)]],
        '6' => &[&[(8, 10), (6, 12), (2, 12), (0, 10), (0, 2), (2, 0), (6, 0), (8, 2), (8, 5), (6, 7), (2, 7), (0, 5)]],
        '7' => &[&[(0, 12), (8, 12), (3, 0)]],
        '8' => &[&[
            (2, 6),
            (0, 8),
            (0, 10),
            (2, 12),
            (6, 12),
            (8, 10),
            (8, 8),
            (6, 6),
            (2, 6),
            (0, 4),
            (0, 2),
            (2, 0),
            (6, 0),
            (8, 2),
            (8, 4),
            (6, 6),
        ]],
        '9' => &[&[(0, 2), (2, 0), (6, 0), (8, 2), (8, 10), (6, 12), (2, 12), (0, 10), (0, 7), (2, 5), (6, 5), (8, 7)]],
        ' ' => &[],
        '.' => &[&[(3, 0), (4, 0), (4, 1), (3, 1), (3, 0)]],
        ',' => &[&[(4, 1), (4, 0), (3, -2)]],
        '-' => &[&[(1, 6), (7, 6)]],
        '+' => &[&[(4, 2), (4, 10)], &[(0, 6), (8, 6)]],
        '=' => &[&[(1, 4), (7, 4)], &[(1, 8), (7, 8)]],
        '_' => &[&[(0, -1), (8, -1)]],
        '/' => &[&[(0, 0), (8, 12)]],
        '(' => &[&[(6, 12), (4, 9), (4, 3), (6, 0)]],
        '{' => &[&[(6, 12), (4, 11), (4, 7), (2, 6), (4, 5), (4, 1), (6, 0)]],
        '}' => &[&[(2, 12), (4, 11), (4, 7), (6, 6), (4, 5), (4, 1), (2, 0)]],
        ')' => &[&[(2, 12), (4, 9), (4, 3), (2, 0)]],
        ':' => &[&[(4, 2), (4, 3)], &[(4, 8), (4, 9)]],
        '\'' => &[&[(4, 12), (4, 9)]],
        '"' => &[&[(3, 12), (3, 9)], &[(5, 12), (5, 9)]],
        '#' => &[&[(2, 0), (3, 12)], &[(5, 0), (6, 12)], &[(0, 4), (8, 4)], &[(0, 8), (8, 8)]],
        '×' => &[&[(1, 2), (7, 10)], &[(1, 10), (7, 2)]],
        '²' => &[&[(2, 11), (3, 12), (5, 12), (6, 11), (6, 10), (2, 7), (6, 7)]],
        '°' => &[&[(3, 12), (5, 12), (6, 11), (6, 9), (5, 8), (3, 8), (2, 9), (2, 11), (3, 12)]],
        '±' => &[&[(4, 5), (4, 11)], &[(1, 8), (7, 8)], &[(1, 2), (7, 2)]],
        _ => QUESTION,
    }
}

/// An accented capital as its base letter and its marks: the shape it wears (a breve, a horn, a
/// circumflex) and the tone above or below it. A Vietnamese letter can wear both, and the tone then
/// sits above the shape, so its marks reach a little higher than one alone.
fn accent(c: char) -> Option<(char, [Glyph; 2])> {
    const ACUTE: Glyph = &[&[(3, 13), (5, 15)]];
    const GRAVE: Glyph = &[&[(3, 15), (5, 13)]];
    const CIRCUMFLEX: Glyph = &[&[(2, 13), (4, 15), (6, 13)]];
    const TILDE: Glyph = &[&[(1, 13), (3, 15), (5, 13), (7, 15)]];
    const DIAERESIS: Glyph = &[&[(2, 14), (2, 15)], &[(6, 14), (6, 15)]];
    const CEDILLA: Glyph = &[&[(4, 0), (5, -1), (3, -3)]];
    // Vietnamese: ă wears a breve, ơ and ư a horn, ả a hook, ạ a dot below, đ a stroke through it. A letter with a
    // shape and a tone takes them from [`shape`], [`tone`] and [`tone_over`], which hold those marks.
    const BREVE: Glyph = &[&[(1, 15), (3, 13), (5, 13), (7, 15)]];
    const HORN: Glyph = &[&[(6, 11), (8, 14)]];
    const HORN_U: Glyph = &[&[(8, 12), (10, 15)]];
    const HOOK: Glyph = &[&[(3, 15), (5, 15)], &[(5, 15), (2, 12)]];
    const DOT_BELOW: Glyph = &[&[(3, -3), (4, -3), (4, -2), (3, -2), (3, -3)]];
    const STROKE: Glyph = &[&[(-1, 9), (4, 9)]];
    const NONE: Glyph = &[];
    Some(match c {
        // Portuguese and Spanish.
        'Á' | 'É' | 'Í' | 'Ó' | 'Ú' | 'Ý' => (base(c)?, [ACUTE, NONE]),
        'Ã' | 'Ẽ' | 'Ĩ' | 'Õ' | 'Ũ' | 'Ỹ' | 'Ñ' => (base(c)?, [TILDE, NONE]),
        'À' | 'È' | 'Ì' | 'Ò' | 'Ù' | 'Ỳ' => (base(c)?, [GRAVE, NONE]),
        'Â' | 'Ê' | 'Î' | 'Ô' | 'Û' => (base(c)?, [CIRCUMFLEX, NONE]),
        'Ä' | 'Ë' | 'Ï' | 'Ö' | 'Ü' => (base(c)?, [DIAERESIS, NONE]),
        'Ç' => ('C', [CEDILLA, NONE]),
        // Vietnamese, with one mark.
        'Ả' | 'Ẻ' | 'Ỉ' | 'Ỏ' | 'Ủ' | 'Ỷ' => (base(c)?, [HOOK, NONE]),
        'Ạ' | 'Ẹ' | 'Ị' | 'Ọ' | 'Ụ' | 'Ỵ' => (base(c)?, [DOT_BELOW, NONE]),
        'Ă' => ('A', [BREVE, NONE]),
        'Ơ' => ('O', [HORN, NONE]),
        'Ư' => ('U', [HORN_U, NONE]),
        'Đ' => ('D', [STROKE, NONE]),
        // Vietnamese, with a shape and a tone. A horn sits clear of a tone, so ớ and ứ need no more
        // room than ơ and ư; a breve or a circumflex does not, so there the tone goes above it.
        'Ớ' | 'Ờ' | 'Ở' | 'Ỡ' | 'Ợ' => ('O', [HORN, tone(c)]),
        'Ứ' | 'Ừ' | 'Ử' | 'Ữ' | 'Ự' => ('U', [HORN_U, tone(c)]),
        'Ắ' | 'Ằ' | 'Ẳ' | 'Ẵ' | 'Ặ' | 'Ấ' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ậ' | 'Ế' | 'Ề' | 'Ể' | 'Ễ' | 'Ệ' | 'Ố' | 'Ồ' | 'Ổ' | 'Ỗ' | 'Ộ' => {
            (base(c)?, [shape(c), tone_over(c)])
        }
        _ => return None,
    })
}

/// The shape a Vietnamese letter wears above: a breve on ă, a circumflex on â and ê.
fn shape(c: char) -> Glyph {
    const BREVE: Glyph = &[&[(1, 15), (3, 13), (5, 13), (7, 15)]];
    const CIRCUMFLEX: Glyph = &[&[(2, 13), (4, 15), (6, 13)]];
    const NONE: Glyph = &[];
    match c {
        'Ă' | 'Ắ' | 'Ằ' | 'Ẳ' | 'Ẵ' | 'Ặ' => BREVE,
        'Â' | 'Ấ' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ậ' | 'Ê' | 'Ế' | 'Ề' | 'Ể' | 'Ễ' | 'Ệ' | 'Ô' | 'Ố' | 'Ồ' | 'Ổ' | 'Ỗ' | 'Ộ' => {
            CIRCUMFLEX
        }
        _ => NONE,
    }
}

/// The tone a letter wears clear of a horn: an acute, a grave, a hook, a tilde, or a dot below.
fn tone(c: char) -> Glyph {
    const ACUTE: Glyph = &[&[(3, 13), (5, 15)]];
    const GRAVE: Glyph = &[&[(3, 15), (5, 13)]];
    const HOOK: Glyph = &[&[(3, 15), (5, 15)], &[(5, 15), (2, 12)]];
    const TILDE: Glyph = &[&[(1, 13), (3, 15), (5, 13), (7, 15)]];
    const DOT_BELOW: Glyph = &[&[(3, -3), (4, -3), (4, -2), (3, -2), (3, -3)]];
    const NONE: Glyph = &[];
    match c {
        'Ứ' | 'Ớ' => ACUTE,
        'Ừ' | 'Ờ' => GRAVE,
        'Ử' | 'Ở' => HOOK,
        'Ữ' | 'Ỡ' => TILDE,
        'Ự' | 'Ợ' => DOT_BELOW,
        _ => NONE,
    }
}

/// The tone of a letter whose shape above it leaves no room, drawn over it (or, a dot, below).
fn tone_over(c: char) -> Glyph {
    const ACUTE: Glyph = &[&[(3, 16), (5, 18)]];
    const GRAVE: Glyph = &[&[(3, 18), (5, 16)]];
    const HOOK: Glyph = &[&[(3, 18), (5, 18)], &[(5, 18), (2, 15)]];
    const TILDE: Glyph = &[&[(1, 16), (3, 18), (5, 16), (7, 18)]];
    const DOT_BELOW: Glyph = &[&[(3, -3), (4, -3), (4, -2), (3, -2), (3, -3)]];
    const NONE: Glyph = &[];
    match c {
        'Ắ' | 'Ấ' | 'Ế' | 'Ố' => ACUTE,
        'Ằ' | 'Ầ' | 'Ề' | 'Ồ' => GRAVE,
        'Ẳ' | 'Ẩ' | 'Ể' | 'Ổ' => HOOK,
        'Ẵ' | 'Ẫ' | 'Ễ' | 'Ỗ' => TILDE,
        'Ặ' | 'Ậ' | 'Ệ' | 'Ộ' => DOT_BELOW,
        _ => NONE,
    }
}

fn base(c: char) -> Option<char> {
    Some(match c {
        'Á' | 'À' | 'Â' | 'Ã' | 'Ä' | 'Ả' | 'Ạ' | 'Ă' | 'Ắ' | 'Ằ' | 'Ẳ' | 'Ẵ' | 'Ặ' | 'Ấ' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ậ' => {
            'A'
        }
        'É' | 'È' | 'Ê' | 'Ë' | 'Ẻ' | 'Ẹ' | 'Ẽ' | 'Ế' | 'Ề' | 'Ể' | 'Ễ' | 'Ệ' => 'E',
        'Í' | 'Ì' | 'Î' | 'Ï' | 'Ỉ' | 'Ị' | 'Ĩ' => 'I',
        'Ó' | 'Ò' | 'Ô' | 'Õ' | 'Ö' | 'Ỏ' | 'Ọ' | 'Ơ' | 'Ớ' | 'Ờ' | 'Ở' | 'Ỡ' | 'Ợ' | 'Ố' | 'Ồ' | 'Ổ' | 'Ỗ' | 'Ộ' => {
            'O'
        }
        'Ú' | 'Ù' | 'Û' | 'Ü' | 'Ủ' | 'Ụ' | 'Ũ' | 'Ư' | 'Ứ' | 'Ừ' | 'Ử' | 'Ữ' | 'Ự' => 'U',
        'Ý' | 'Ỳ' | 'Ỷ' | 'Ỹ' | 'Ỵ' => 'Y',
        'Ñ' => 'N',
        _ => return None,
    })
}

/// Grid units from one character to the next.
const ADVANCE: f32 = 11.0;

/// How wide `text` is at `height` px.
pub fn width(text: &str, height: f32) -> f32 {
    let n = text.chars().count() as f32;
    if n == 0.0 { 0.0 } else { (n * ADVANCE - 3.0) * height / 12.0 }
}

/// Polylines drawing `text` with capitals `height` px tall, from `(x, y)` on the baseline, y down.
pub fn strokes(text: &str, x: f32, y: f32, height: f32) -> Vec<Vec<(f32, f32)>> {
    let k = height / 12.0;
    let mut out = Vec::new();
    for (i, c) in text.chars().flat_map(char::to_uppercase).enumerate() {
        let x0 = x + i as f32 * ADVANCE * k;
        let (letter, [shape, tone]): (Glyph, [Glyph; 2]) = match accent(c) {
            Some((b, marks)) => (glyph(b), marks),
            None => (glyph(c), [&[], &[]]),
        };
        for line in letter.iter().chain(shape).chain(tone) {
            out.push(line.iter().map(|&(gx, gy)| (x0 + f32::from(gx) * k, y - f32::from(gy) * k)).collect());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_case_is_drawn_as_capitals_and_unknown_characters_as_a_question_mark() {
        assert_eq!(strokes("living", 0.0, 0.0, 12.0), strokes("LIVING", 0.0, 0.0, 12.0));
        assert_eq!(strokes("€", 0.0, 0.0, 12.0), strokes("?", 0.0, 0.0, 12.0));
        assert!(strokes(" ", 0.0, 0.0, 12.0).is_empty());
    }

    #[test]
    fn accented_letters_are_their_base_letter_plus_an_accent() {
        let plain = strokes("A", 0.0, 0.0, 12.0);
        let acute = strokes("á", 0.0, 0.0, 12.0);
        assert_eq!(acute.len(), plain.len() + 1);
        assert_eq!(acute[..plain.len()], plain[..]);
    }

    #[test]
    fn vietnamese_letters_are_drawn_with_their_marks() {
        // A letter with no mark is unknown to the font, so a mark it does know is the whole test.
        let unknown = strokes("@", 0.0, 0.0, 12.0);
        for (letter, base_letter, marks) in [
            ("đ", "D", 1),
            ("ă", "A", 1),
            ("ư", "U", 1),
            ("ơ", "O", 1),
            ("ả", "A", 2),
            ("ạ", "A", 1),
            ("ế", "E", 2),
            ("ệ", "E", 2),
            ("ắ", "A", 2),
            ("ặ", "A", 2),
            ("ớ", "O", 2),
            ("ợ", "O", 2),
            ("ở", "O", 3),
            ("ự", "U", 2),
            ("ỹ", "Y", 1),
        ] {
            let drawn = strokes(letter, 0.0, 0.0, 12.0);
            let base = strokes(base_letter, 0.0, 0.0, 12.0);
            assert_eq!(drawn.len(), base.len() + marks, "{letter} is {base_letter} plus {marks} mark(s)");
            assert_eq!(drawn[..base.len()], base[..], "{letter} keeps its base letter");
            assert_ne!(drawn, unknown, "{letter} is not a question mark");
        }
        // The marks of a doubled tone reach above a plain accent, and nothing reaches below minus.
        let low = strokes("ạ", 0.0, 0.0, 12.0).iter().flatten().map(|p| p.1).fold(f32::MAX, f32::min);
        assert!(low < 0.0, "a dot below the letter hangs below the baseline");
        let top = |text: &str| strokes(text, 0.0, 0.0, 12.0).iter().flatten().map(|p| p.1).fold(f32::MAX, f32::min);
        assert!(top("ế") < top("é"), "the tone over a circumflex sits above a plain acute");
    }

    /// Every Vietnamese letter the window's table lists as one a font must have: the same set the desktop checks
    /// the system's fonts against ([`bimcraft_ui_egui::lang::VIETNAMESE_LETTERS`]), spelled out here so the core's own
    /// lettering can be swept by it without depending on the desktop. A letter it does not know is a box on a plan.
    const VIETNAMESE: &str = "ăĂâÂđĐêÊôÔơƠưƯảẢạẠắẮằẰẳẲẵẴặẶấẤầẦẩẨẫẪậẬéÉèÈẻẺẽẼẹẸếẾềỀểỂễỄệỆíÍìÌỉỈĩĨịỊ\
                               óÓòÒỏỎõÕọỌốỐồỒổỔỗỖộỘớỚờỜởỞỡỠợỢúÚùÙủỦũŨụỤứỨừỪửỬữỮựỰýÝỳỲỷỶỹỸỵỴ";

    #[test]
    fn every_vietnamese_letter_is_drawn_and_not_a_question_mark() {
        // A drawing's lettering has no font to fall back on, so every letter of the language must be drawn: the
        // shapes (ă, ơ, ư, đ), the tones above and below, and a tone over a shape (ế, ắ, ộ).
        let unknown = strokes("@", 0.0, 0.0, 12.0);
        let missing: Vec<char> = VIETNAMESE.chars().filter(|&c| strokes(&c.to_string(), 0.0, 0.0, 12.0) == unknown).collect();
        assert_eq!(VIETNAMESE.chars().count(), 128, "the letters of Vietnamese, upper and lower");
        assert!(missing.is_empty(), "the lettering draws a question mark for {missing:?}");
        // And every letter the core's own words use is among them, so a new word's letter cannot slip through.
        let unknown = strokes("@", 0.0, 0.0, 12.0);
        let words: Vec<char> = crate::language::CATALOGS
            .iter()
            .filter(|(code, _)| *code != "en")
            .flat_map(|(_, messages)| messages.iter().map(|(_, _, t)| *t))
            .flat_map(str::chars)
            .filter(|c| !c.is_ascii() && c.is_alphabetic())
            .filter(|&c| strokes(&c.to_string(), 0.0, 0.0, 12.0) == unknown)
            .collect();
        assert!(words.is_empty(), "a word of the core's draws a question mark for {words:?}");
    }

    #[test]
    fn text_is_scaled_to_its_height_from_the_baseline() {
        let s = strokes("I", 10.0, 100.0, 24.0);
        let ys: Vec<f32> = s.iter().flatten().map(|p| p.1).collect();
        assert_eq!(ys.iter().copied().fold(f32::MAX, f32::min), 76.0);
        assert_eq!(ys.iter().copied().fold(f32::MIN, f32::max), 100.0);
        assert_eq!(width("II", 12.0), 19.0);
    }
}
