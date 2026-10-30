//! Our own single-stroke lettering for plan annotation: capitals, Latin and Cyrillic, digits, a few marks and the
//! accents of Latin and Vietnamese languages (acute, grave, circumflex, tilde, diaeresis, double acute, caron,
//! ring, macron, breve, dot, cedilla, ogonek, horn and the strokes of Đ, Ł and Ø), drawn on an 8 x 12 grid (y up).
//! Lower case is drawn as capitals, as architectural lettering is. What it has no letter for, a text
//! type in it letters from IBM Plex ([`crate::text`]).

type Glyph = &'static [&'static [(i8, i8)]];

const O: &[(i8, i8)] = &[(2, 0), (0, 2), (0, 10), (2, 12), (6, 12), (8, 10), (8, 2), (6, 0), (2, 0)];
const P: &[(i8, i8)] = &[(0, 0), (0, 12), (6, 12), (8, 10), (8, 8), (6, 6), (0, 6)];
const QUESTION: Glyph = &[&[(0, 10), (2, 12), (6, 12), (8, 10), (8, 8), (4, 5), (4, 3)], &[(4, 1), (4, 0)]];

fn glyph(c: char) -> Glyph {
    known(c).unwrap_or(QUESTION)
}

/// Its letter for capital `c`, if it has one.
fn known(c: char) -> Option<Glyph> {
    Some(match c {
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
        // Norwegian and Danish: an A grown into an E (its crossbar meets the stem).
        'Æ' => &[&[(0, 0), (4, 12), (8, 12)], &[(4, 12), (4, 0), (8, 0)], &[(4, 6), (7, 6)], &[(1, 4), (4, 4)]],
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
        // Cyrillic: the capitals the Latin alphabet shares are its strokes.
        'А' => glyph('A'),
        'В' => glyph('B'),
        'Е' => glyph('E'),
        'К' => glyph('K'),
        'М' => glyph('M'),
        'Н' => glyph('H'),
        'О' => glyph('O'),
        'Р' => glyph('P'),
        'С' => glyph('C'),
        'Т' => glyph('T'),
        'Х' => glyph('X'),
        'З' => glyph('3'),
        'Б' => &[&[(8, 12), (0, 12), (0, 0), (6, 0), (8, 2), (8, 5), (6, 7), (0, 7)]],
        'Г' => &[&[(8, 12), (0, 12), (0, 0)]],
        'Д' => &[&[(0, -2), (0, 0), (8, 0), (8, -2)], &[(1, 0), (3, 12), (7, 12), (7, 0)]],
        'Ж' => &[&[(4, 0), (4, 12)], &[(0, 12), (4, 6), (8, 12)], &[(0, 0), (4, 6), (8, 0)]],
        'И' => &[&[(0, 12), (0, 0), (8, 12), (8, 0)]],
        'Л' => &[&[(0, 0), (2, 2), (3, 12), (8, 12), (8, 0)]],
        'П' => &[&[(0, 0), (0, 12), (8, 12), (8, 0)]],
        'У' => &[&[(0, 12), (4, 5)], &[(8, 12), (3, 0), (1, 0)]],
        'Ф' => &[&[(4, 0), (4, 12)], &[(4, 10), (1, 10), (0, 9), (0, 5), (1, 4), (7, 4), (8, 5), (8, 9), (7, 10), (4, 10)]],
        'Ц' => &[&[(0, 12), (0, 0), (7, 0), (7, 12)], &[(7, 0), (8, 0), (8, -2)]],
        'Ч' => &[&[(0, 12), (0, 8), (2, 6), (8, 6)], &[(8, 12), (8, 0)]],
        'Ш' => &[&[(0, 12), (0, 0), (8, 0), (8, 12)], &[(4, 12), (4, 0)]],
        'Щ' => &[&[(0, 12), (0, 0), (6, 0), (6, 12)], &[(3, 12), (3, 0)], &[(6, 0), (8, 0), (8, -2)]],
        'Ъ' => &[&[(0, 12), (2, 12), (2, 0), (6, 0), (8, 2), (8, 5), (6, 7), (2, 7)]],
        'Ы' => &[&[(0, 12), (0, 0), (4, 0), (6, 2), (6, 5), (4, 7), (0, 7)], &[(8, 12), (8, 0)]],
        'Ь' => &[&[(0, 12), (0, 0), (6, 0), (8, 2), (8, 5), (6, 7), (0, 7)]],
        'Э' => &[&[(0, 10), (2, 12), (6, 12), (8, 10), (8, 2), (6, 0), (2, 0), (0, 2)], &[(3, 6), (8, 6)]],
        'Ю' => &[&[(0, 12), (0, 0)], &[(0, 6), (3, 6)], &[(5, 12), (3, 10), (3, 2), (5, 0), (6, 0), (8, 2), (8, 10), (6, 12), (5, 12)]],
        'Я' => &[&[(8, 0), (8, 12), (2, 12), (0, 10), (0, 8), (2, 6), (8, 6)], &[(4, 6), (0, 0)]],
        '!' => &[&[(4, 12), (4, 3)], &[(4, 1), (4, 0)]],
        ';' => &[&[(4, 8), (4, 9)], &[(4, 2), (3, -1)]],
        '*' => &[&[(4, 3), (4, 11)], &[(1, 5), (7, 9)], &[(1, 9), (7, 5)]],
        '%' => &[&[(0, 0), (8, 12)], &[(1, 12), (2, 12), (2, 10), (1, 10), (1, 12)], &[(6, 2), (7, 2), (7, 0), (6, 0), (6, 2)]],
        '&' => &[&[(8, 0), (1, 9), (1, 11), (2, 12), (4, 12), (5, 11), (5, 9), (0, 4), (0, 2), (2, 0), (5, 0), (8, 4)]],
        '[' => &[&[(6, 12), (3, 12), (3, 0), (6, 0)]],
        ']' => &[&[(2, 12), (5, 12), (5, 0), (2, 0)]],
        '<' => &[&[(7, 10), (1, 6), (7, 2)]],
        '>' => &[&[(1, 10), (7, 6), (1, 2)]],
        '@' => &[&[(6, 4), (6, 8), (3, 8), (2, 6), (3, 4), (6, 4), (8, 5), (8, 10), (6, 12), (2, 12), (0, 10), (0, 2), (2, 0), (7, 0)]],
        '?' => QUESTION,
        '\\' => &[&[(0, 12), (8, 0)]],
        '|' => &[&[(4, -2), (4, 13)]],
        '~' => &[&[(1, 6), (3, 8), (5, 6), (7, 8)]],
        '$' => &[&[(8, 10), (6, 11), (2, 11), (0, 9), (0, 8), (2, 6), (6, 6), (8, 4), (8, 3), (6, 1), (2, 1), (0, 3)], &[(4, 13), (4, -1)]],
        '€' => &[&[(8, 10), (6, 12), (3, 12), (1, 10), (1, 2), (3, 0), (6, 0), (8, 2)], &[(0, 7), (5, 7)], &[(0, 5), (5, 5)]],
        '³' => &[&[(2, 12), (6, 12), (4, 10), (6, 9), (5, 7), (2, 7)]],
        '·' => &[&[(4, 6), (4, 7)]],
        '–' | '—' => &[&[(0, 6), (8, 6)]],
        '‘' | '’' => &[&[(4, 12), (4, 9)]],
        '“' | '”' | '„' => &[&[(3, 12), (3, 9)], &[(5, 12), (5, 9)]],
        _ => return None,
    })
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
    const DOUBLE_ACUTE: Glyph = &[&[(1, 13), (3, 15)], &[(5, 13), (7, 15)]];
    const CARON: Glyph = &[&[(2, 15), (4, 13), (6, 15)]];
    const RING: Glyph = &[&[(3, 13), (3, 15), (5, 15), (5, 13), (3, 13)]];
    const MACRON: Glyph = &[&[(2, 14), (6, 14)]];
    const BREVE: Glyph = &[&[(2, 15), (3, 13), (5, 13), (6, 15)]];
    const DOT: Glyph = &[&[(4, 14), (4, 15)]];
    const CEDILLA: Glyph = &[&[(4, 0), (5, -1), (3, -3)]];
    const OGONEK: Glyph = &[&[(6, 0), (5, -2), (7, -3)]];
    const SLASH: Glyph = &[&[(-1, -1), (9, 13)]];
    const BAR: Glyph = &[&[(-1, 4), (3, 8)]];
    // Vietnamese: ă wears a breve, ơ and ư a horn, ả a hook, ạ a dot below, đ a stroke through it. A letter with a
    // shape and a tone takes them from [`shape`], [`tone`] and [`tone_over`], which hold those marks.
    const HORN: Glyph = &[&[(6, 11), (8, 14)]];
    const HORN_U: Glyph = &[&[(8, 12), (10, 15)]];
    const HOOK: Glyph = &[&[(3, 15), (5, 15)], &[(5, 15), (2, 12)]];
    const DOT_BELOW: Glyph = &[&[(3, -3), (4, -3), (4, -2), (3, -2), (3, -3)]];
    const STROKE: Glyph = &[&[(-1, 9), (4, 9)]];
    const BREVE_VI: Glyph = &[&[(1, 15), (3, 13), (5, 13), (7, 15)]];
    const NONE: Glyph = &[];
    let mark = match c {
        'Ё' => return Some(('Е', [DIAERESIS, NONE])),
        'Й' => return Some(('И', [BREVE, NONE])),
        'Á' | 'É' | 'Í' | 'Ó' | 'Ú' | 'Ý' | 'Ć' | 'Ĺ' | 'Ń' | 'Ŕ' | 'Ś' | 'Ź' => [ACUTE, NONE],
        'À' | 'È' | 'Ì' | 'Ò' | 'Ù' | 'Ỳ' => [GRAVE, NONE],
        'Â' | 'Ê' | 'Î' | 'Ô' | 'Û' => [CIRCUMFLEX, NONE],
        'Ã' | 'Ẽ' | 'Ĩ' | 'Õ' | 'Ũ' | 'Ỹ' | 'Ñ' => [TILDE, NONE],
        'Ä' | 'Ë' | 'Ï' | 'Ö' | 'Ü' | 'Ÿ' => [DIAERESIS, NONE],
        'Ő' | 'Ű' => [DOUBLE_ACUTE, NONE],
        'Č' | 'Ď' | 'Ě' | 'Ň' | 'Ř' | 'Š' | 'Ť' | 'Ž' => [CARON, NONE],
        'Å' | 'Ů' => [RING, NONE],
        'Ā' | 'Ē' | 'Ī' | 'Ō' | 'Ū' => [MACRON, NONE],
        'Ğ' => [BREVE, NONE],
        'Ė' | 'Ż' | 'İ' => [DOT, NONE],
        'Ç' | 'Ş' => [CEDILLA, NONE],
        'Ą' | 'Ę' => [OGONEK, NONE],
        'Ø' => [SLASH, NONE],
        'Ł' => [BAR, NONE],
        // Vietnamese, with one mark.
        'Ả' | 'Ẻ' | 'Ỉ' | 'Ỏ' | 'Ủ' | 'Ỷ' => [HOOK, NONE],
        'Ạ' | 'Ẹ' | 'Ị' | 'Ọ' | 'Ụ' | 'Ỵ' => [DOT_BELOW, NONE],
        'Ă' => return Some(('A', [BREVE_VI, NONE])),
        'Ơ' => return Some(('O', [HORN, NONE])),
        'Ư' => return Some(('U', [HORN_U, NONE])),
        'Đ' => return Some(('D', [STROKE, NONE])),
        // Vietnamese, with a shape and a tone. A horn sits clear of a tone, so ớ and ứ need no more
        // room than ơ and ư; a breve or a circumflex does not, so there the tone goes above it.
        'Ớ' | 'Ờ' | 'Ở' | 'Ỡ' | 'Ợ' => return Some(('O', [HORN, tone(c)])),
        'Ứ' | 'Ừ' | 'Ử' | 'Ữ' | 'Ự' => return Some(('U', [HORN_U, tone(c)])),
        'Ắ' | 'Ằ' | 'Ẳ' | 'Ẵ' | 'Ặ' | 'Ấ' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ậ' | 'Ế' | 'Ề' | 'Ể' | 'Ễ' | 'Ệ' | 'Ố' | 'Ồ' | 'Ổ' | 'Ỗ' | 'Ộ' =>
        {
            return Some((base(c)?, [shape(c), tone_over(c)]));
        }
        _ => return None,
    };
    Some((base(c)?, mark))
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
        'Á' | 'À' | 'Â' | 'Ã' | 'Ä' | 'Ả' | 'Ạ' | 'Ă' | 'Ắ' | 'Ằ' | 'Ẳ' | 'Ẵ' | 'Ặ' | 'Ấ' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ậ' | 'Å' | 'Ā' | 'Ą' => {
            'A'
        }
        'Ć' | 'Č' | 'Ç' => 'C',
        'Ď' => 'D',
        'É' | 'È' | 'Ê' | 'Ë' | 'Ẻ' | 'Ẹ' | 'Ẽ' | 'Ế' | 'Ề' | 'Ể' | 'Ễ' | 'Ệ' | 'Ě' | 'Ē' | 'Ė' | 'Ę' => 'E',
        'Ğ' => 'G',
        'Í' | 'Ì' | 'Î' | 'Ï' | 'Ĩ' | 'Ỉ' | 'Ị' | 'Ī' | 'İ' => 'I',
        'Ĺ' | 'Ł' => 'L',
        'Ñ' | 'Ń' | 'Ň' => 'N',
        'Ó' | 'Ò' | 'Ô' | 'Õ' | 'Ö' | 'Ỏ' | 'Ọ' | 'Ơ' | 'Ớ' | 'Ờ' | 'Ở' | 'Ỡ' | 'Ợ' | 'Ố' | 'Ồ' | 'Ổ' | 'Ỗ' | 'Ộ' | 'Ő' | 'Ō' | 'Ø' => {
            'O'
        }
        'Ŕ' | 'Ř' => 'R',
        'Ś' | 'Š' | 'Ş' => 'S',
        'Ť' => 'T',
        'Ú' | 'Ù' | 'Û' | 'Ü' | 'Ủ' | 'Ụ' | 'Ũ' | 'Ư' | 'Ứ' | 'Ừ' | 'Ử' | 'Ữ' | 'Ự' | 'Ű' | 'Ů' | 'Ū' => 'U',
        'Ý' | 'Ỳ' | 'Ỷ' | 'Ỹ' | 'Ỵ' | 'Ÿ' => 'Y',
        'Ź' | 'Ž' | 'Ż' => 'Z',
        _ => return None,
    })
}

/// Whether it has a letter for `c` (in either case): else it draws a question mark.
pub fn has(c: char) -> bool {
    let mut upper = c.to_uppercase();
    match (upper.next(), upper.next()) {
        (Some(u), None) => accent(u).is_some() || known(u).is_some(),
        // ß is SS.
        (Some(_), Some(_)) => c.to_uppercase().all(has),
        _ => false,
    }
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
        assert_eq!(strokes("ω", 0.0, 0.0, 12.0), strokes("?", 0.0, 0.0, 12.0));
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
    fn cyrillic_capitals_are_drawn_each_its_own_and_lower_case_as_them() {
        let unknown = strokes("?", 0.0, 0.0, 12.0);
        let all = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ";
        let drawn: Vec<_> = all.chars().map(|c| strokes(&c.to_string(), 0.0, 0.0, 12.0)).collect();
        for (c, s) in all.chars().zip(&drawn) {
            assert!(!s.is_empty() && *s != unknown, "{c} is not drawn");
        }
        for (i, a) in drawn.iter().enumerate() {
            assert!(drawn.iter().skip(i + 1).all(|b| b != a), "{} looks like another", all.chars().nth(i).unwrap_or(' '));
        }
        // The letters the Latin alphabet shares are the same strokes.
        assert_eq!(strokes("АВЕКМНОРСТХ", 0.0, 0.0, 12.0), strokes("ABEKMHOPCTX", 0.0, 0.0, 12.0));
        assert_eq!(strokes("шлейф ёж", 0.0, 0.0, 12.0), strokes("ШЛЕЙФ ЁЖ", 0.0, 0.0, 12.0));
        // Й and Ё are И and Е with their marks over them.
        let (i, short) = (strokes("И", 0.0, 0.0, 12.0), strokes("Й", 0.0, 0.0, 12.0));
        assert_eq!(short[..i.len()], i[..]);
        let (e, yo) = (strokes("Е", 0.0, 0.0, 12.0), strokes("Ё", 0.0, 0.0, 12.0));
        assert_eq!(yo[..e.len()], e[..]);
    }

    #[test]
    fn it_says_which_letters_it_has_so_a_text_type_can_letter_the_rest_from_plex() {
        for c in "aÁőŰčŘåŁøßé€!?Жй".chars() {
            assert!(has(c), "{c}");
        }
        for c in "ωΣ漢".chars() {
            assert!(!has(c), "{c}");
        }
        assert_eq!(strokes("Ő", 0.0, 0.0, 12.0).len(), strokes("O", 0.0, 0.0, 12.0).len() + 2, "a double acute");
    }

    #[test]
    fn the_ae_ligature_is_drawn_not_questioned() {
        let ae = strokes("Æ", 0.0, 0.0, 12.0);
        assert_ne!(ae, strokes("?", 0.0, 0.0, 12.0));
        assert_eq!(strokes("æ", 0.0, 0.0, 12.0), ae);
        assert!(ae.len() > strokes("E", 0.0, 0.0, 12.0).len(), "an A grown into an E has more strokes than an E");
        assert!(has('Æ'));
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
