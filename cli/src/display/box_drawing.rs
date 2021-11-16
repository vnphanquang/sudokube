#[derive(Debug, Copy, Clone)]
pub struct CornerRelative {
    /// vertical cell exists
    pub vertical: bool,
    /// vertical cell is in same subgrid
    pub vertical_sub_grid: bool,
    /// horizontal cell exists
    pub horizontal: bool,
    /// horizontal cell is in same subgrid
    pub horizontal_sub_grid: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct MiddleRelative {
    /// relative cell exists
    pub relative: bool,
    /// relative cell is in same subgrid
    pub relative_sub_grid: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum CornerPosition {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Debug, Copy, Clone)]
pub enum MiddlePosition {
    Vertical,
    Horizontal,
}

fn build_corner_byte(relative: CornerRelative) -> u8 {
    let CornerRelative {
        horizontal,
        horizontal_sub_grid,
        vertical,
        vertical_sub_grid,
    } = relative;

    let mut byte: u8 = 0b0000_0000;

    if horizontal {
        byte |= 0b0000_0001;
    }

    if horizontal_sub_grid {
        byte |= 0b0000_0010;
    }

    if vertical {
        byte |= 0b0000_0100;
    }

    if vertical_sub_grid {
        byte |= 0b0000_1000;
    }

    byte
}

fn build_middle_byte(relative: MiddleRelative) -> u8 {
    let MiddleRelative {
        relative,
        relative_sub_grid,
    } = relative;

    let mut byte: u8 = 0b0000_0000;

    if relative {
        byte |= 0b0000_0001;
    }

    if relative_sub_grid {
        byte |= 0b0000_0010;
    }

    byte
}

///
/// 0b0000_|b4|b3|b2|b1|
///
/// - b1: next horizontal cell exists
/// - b2: next horizontal cell is in same subgrid
/// - b3: next vertical cell exists
/// - b4: next vertical cell is in same subgrid
///
pub fn build_corner_char(relative: CornerRelative, position: CornerPosition) -> char {
    let byte = build_corner_byte(relative);

    match byte {
        0b0000_0000 => match position {
            CornerPosition::TopLeft => '┏',
            CornerPosition::TopRight => '┓',
            CornerPosition::BottomRight => '┛',
            CornerPosition::BottomLeft => '┗',
        },
        0b0000_0001 => match position {
            CornerPosition::TopLeft | CornerPosition::TopRight => '┳',
            CornerPosition::BottomRight | CornerPosition::BottomLeft => '┻',
        },
        0b0000_0011 => match position {
            CornerPosition::TopLeft | CornerPosition::TopRight => '┯',
            CornerPosition::BottomRight | CornerPosition::BottomLeft => '┷',
        },
        0b0000_0100 => match position {
            CornerPosition::TopLeft | CornerPosition::BottomLeft => '┣',
            CornerPosition::TopRight | CornerPosition::BottomRight => '┫',
        },
        0b0000_1100 => match position {
            CornerPosition::TopLeft | CornerPosition::BottomLeft => '┠',
            CornerPosition::TopRight | CornerPosition::BottomRight => '┨',
        },
        0b0000_0101 => '╋',
        0b0000_0111 => '┿',
        0b0000_1101 => '╂',
        0b0000_1111 => '┼',
        _ => '?',
    }
}

///
/// 0b0000_00|b2|b1|
///
/// - b1: next relative cell exists
/// - b2: next relative cell is in same subgrid
///
pub fn build_middle_char(relative: MiddleRelative, position: MiddlePosition) -> char {
    let byte = build_middle_byte(relative);

    match byte {
        0b0000_0000 | 0b0000_0001 => match position {
            MiddlePosition::Horizontal => '┃',
            MiddlePosition::Vertical => '━',
        },
        0b0000_0011 => match position {
            MiddlePosition::Horizontal => '│',
            MiddlePosition::Vertical => '─',
        },
        _ => '?',
    }
}
