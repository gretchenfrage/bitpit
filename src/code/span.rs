
/// Source code location, for better error reporting.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Span<'a> {
    None,
    Slice(&'a str),
    AddrRange(usize, usize),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Spanned<'a, T>(pub T, pub Span<'a>);

impl<'a, T> Spanned<'a, T> {
    pub fn map<B, F>(self, function: F) -> Spanned<'a, B>
        where F: Fn(T) -> B
    {
        let Spanned(a, span) = self;
        let b = function(a);
        Spanned(b, span)
    }
}

pub fn until<'a>(a: &'a str, b: &'a str) -> Span<'a> {
    let a_addr: usize = a.as_ptr() as usize;
    let b_addr: usize = b.as_ptr() as usize;

    if a_addr > b_addr {
        return Span::None;
    }

    let delta: usize = b_addr - a_addr;

    if (delta + 1) > a.len() {
        return Span::None;
    }

    let slice: &'a str = a.get(..delta).unwrap();

    Span::Slice(slice)
}

pub fn mapping<A, B, F>(function: F) -> impl Fn(Spanned<A>) -> Spanned<B>
    where F: Fn(A) -> B
{
    move |spanned| spanned.map(|a| function(a))
}

pub fn between<'a>(span0: Span<'a>, span1: Span<'a>) -> Span<'a> {

    fn addr_range(span: Span) -> (Option<usize>, Option<usize>) {
        match span {
            Span::None => (None, None),
            Span::Slice(slice) => (
                Some(slice.as_ptr() as usize),
                Some(slice.as_ptr() as usize + slice.len()),
            ),
            Span::AddrRange(a, b) => (Some(a), Some(b)),
        }
    }

    let (addr0a, addr0b) = addr_range(span0);
    let (addr1a, addr1b) = addr_range(span1);

    fn option_cmp<T>(
        a_option: Option<T>,
        b_option: Option<T>,
        cmp: fn(T, T) -> T
    ) -> Option<T> {
        match (a_option, b_option) {
            (Some(a), Some(b)) => Some(cmp(a, b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    let addr_min = option_cmp(addr0a, addr1a, std::cmp::min);
    let addr_max = option_cmp(addr0b, addr1b, std::cmp::max);

    addr_min.and_then(|min| addr_max.map(move |max| (min, max)))
        .map(|(min, max)| Span::AddrRange(min, max))
        .unwrap_or(Span::None)
}