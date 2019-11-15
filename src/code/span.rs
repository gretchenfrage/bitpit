
/// Source code location, for better error reporting.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Span<'a> {
    None,
    Slice(&'a str),
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