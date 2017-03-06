use pixset::Pix;
use std::collections::HashMap;
use std::default::Default;
use units;

lazy_static! {
    pub static ref CH_TO_PIX: HashMap<char, Pix> = {
        let mut m = HashMap::new();
        m.insert('a', Pix::A);
        m.insert('b', Pix::B);
        m.insert('c', Pix::C);
        m.insert('d', Pix::D);
        m.insert('e', Pix::E);
        m.insert('f', Pix::F);
        m.insert('g', Pix::G);
        m.insert('h', Pix::H);
        m.insert('i', Pix::I);
        m.insert('j', Pix::J);
        m.insert('k', Pix::K);
        m.insert('l', Pix::L);
        m.insert('m', Pix::M);
        m.insert('n', Pix::N);
        m.insert('o', Pix::O);
        m.insert('p', Pix::P);
        m.insert('q', Pix::Q);
        m.insert('r', Pix::R);
        m.insert('s', Pix::S);
        m.insert('t', Pix::T);
        m.insert('u', Pix::U);
        m.insert('v', Pix::V);
        m.insert('w', Pix::W);
        m.insert('x', Pix::X);
        m.insert('y', Pix::Y);
        m.insert('z', Pix::Z);
        m.insert('A', Pix::A);
        m.insert('B', Pix::B);
        m.insert('C', Pix::C);
        m.insert('D', Pix::D);
        m.insert('E', Pix::E);
        m.insert('F', Pix::F);
        m.insert('G', Pix::G);
        m.insert('H', Pix::H);
        m.insert('I', Pix::I);
        m.insert('J', Pix::J);
        m.insert('K', Pix::K);
        m.insert('L', Pix::L);
        m.insert('M', Pix::M);
        m.insert('N', Pix::N);
        m.insert('O', Pix::O);
        m.insert('P', Pix::P);
        m.insert('Q', Pix::Q);
        m.insert('R', Pix::R);
        m.insert('S', Pix::S);
        m.insert('T', Pix::T);
        m.insert('U', Pix::U);
        m.insert('V', Pix::V);
        m.insert('W', Pix::W);
        m.insert('X', Pix::X);
        m.insert('Y', Pix::Y);
        m.insert('Z', Pix::Z);
        m.insert(' ', Pix::Empty);
        m.insert('#', Pix::Hash);
        m.insert('.', Pix::Period);
        m.insert(',', Pix::Comma);
        m.insert('"', Pix::Quotes);
        m.insert('\'', Pix::Apostrophe);
        m.insert(':', Pix::Colon);
        m.insert(';', Pix::SemiColon);
        m
    };
}

pub struct Str<'a>(&'a str);

impl<'a> Str<'a> {
    pub fn iter(&self) -> FontIter {
        FontIter::new(self.0)
    }
}

pub struct FontIter<'a> {
    idx: usize,
    string: &'a str,
}

impl<'a> FontIter<'a> {
    fn new(s: &'a str) -> Self {
        FontIter {
            idx: Default::default(),
            string: s,
        }
    }
}

impl<'a> Iterator for FontIter<'a> {
    type Item = (Pix, units::ScreenPoint2D);

    fn next(&mut self) -> Option<(Pix, units::ScreenPoint2D)> {
        if self.idx == self.string.len() {
            None
        } else {
            // TODO can this be better?
            let ch = self.string.chars().nth(self.idx).expect("no char at that index");
            let pix = CH_TO_PIX.get(&ch).expect(format!("no ch for that pix: {}", ch).as_str());
            let x = self.idx;
            self.idx += 1;
            Some((*pix, units::ScreenPoint2D::new(x as i32, 0)))
        }
    }
}

impl<'a> From<&'a str> for Str<'a> {
    fn from(s: &'a str) -> Self {
        Str(s)
    }
}
