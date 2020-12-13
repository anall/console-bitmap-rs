#![warn( clippy::all, clippy::pedantic )]
use std::cmp;

pub trait ConsoleBitmap {
    const COLS_PER_CHARACTER : usize;
    const ROWS_PER_CHARACTER : usize;

    #[must_use]
    #[inline]
    fn translate_character(v : usize) -> usize { v }
    fn character(v : usize) -> char;
}

#[must_use]
pub fn draw_from_vec<S : ConsoleBitmap>(v : &[Vec<bool>]) -> Vec<String> {
    assert!( S::COLS_PER_CHARACTER > 0 );
    assert!( S::ROWS_PER_CHARACTER > 0 );

    let mut out : Vec<String> = Vec::new();
    for row in v.chunks( S::ROWS_PER_CHARACTER ) {
        let mut str = String::new();
        for j in ( 0 .. row[0].len() ).step_by( S::COLS_PER_CHARACTER ) {
            let mut character : usize = 0;
            let mut shift : usize = 0;
            for line in row {
                let v = &line[j .. cmp::min(line.len(), j+S::COLS_PER_CHARACTER)].iter().enumerate()
                    .fold(0,|s,(i,&v)| if v { s  | 1 << i } else { s });
                character |= v << shift;
                shift += S::COLS_PER_CHARACTER;
            }
            str.push( S::character( S::translate_character( character ) ) );
        }
        out.push(str);
    }
    out
}
pub fn draw<S : ConsoleBitmap, T : IntoIterator>(v : T) -> Vec<String>
    where <T as IntoIterator>::Item: IntoIterator, <<T as IntoIterator>::Item as IntoIterator>::Item: Into<bool> {
    draw_from_vec::<S>( &v.into_iter().map(|r| r.into_iter().map(Into::into).collect() ).collect::<Vec<_>>() )
}

pub struct BlockElements;
const BLOCK_CHARACTER_MAP : [char; 16] = [
    ' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█'
];
impl ConsoleBitmap for BlockElements {
    const COLS_PER_CHARACTER: usize = 2;
    const ROWS_PER_CHARACTER: usize = 2;

    #[inline]
    fn character(v : usize) -> char {
        BLOCK_CHARACTER_MAP[v]
    }
}

pub struct BraillePatterns;
const BRAILLE_MAP_BITS : [usize;8] = [0,3,1,4,2,5,6,7];

impl ConsoleBitmap for BraillePatterns {
    const COLS_PER_CHARACTER: usize = 2;
    const ROWS_PER_CHARACTER: usize = 4;

    #[inline]
    fn translate_character(v : usize) -> usize {
        (0 .. 8).filter(|&bit| (v & 1<<bit) != 0 ).fold(0,|out,bit| out | 1<<BRAILLE_MAP_BITS[bit])
    }

    #[inline]
    fn character(v : usize) -> char {
        #[allow(clippy::cast_possible_truncation)]
        std::char::from_u32((0x2800 + v) as u32 ).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::BlockElements;

    fn convert(c : char) -> bool {
        c == '#'
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        for v in crate::draw::<BlockElements,_>(vec![
            ( "######   ###   ".chars().map(convert ) ).collect::<Vec<_>>(),
            ( "# ## ## # # # ##".chars().map( convert ) ).collect::<Vec<_>>(),
            ( "######   ###   #".chars().map(convert ) ).collect::<Vec<_>>(),
        ]) {
            println!("{}",v);
        }
    }
}
