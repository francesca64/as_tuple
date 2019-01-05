use as_tuple::AsTuple;
use std::borrow::Cow;

#[allow(dead_code)]
#[derive(Debug)]
enum ReductionistStereotype {
    Cutesy,
    Goth,
    Preppy,
    Punk,
    Vintage,
}

#[derive(Debug)]
struct PinkLink;

#[derive(AsTuple, Debug)]
struct CoolGirl {
    name: Cow<'static, str>,
    unique_style: ReductionistStereotype,
    secret_telepathy_all_girls_have: &'static PinkLink,
}

static PINK_LINK: &'static PinkLink = &PinkLink;

#[test]
fn named() {
    let mut fran = CoolGirl {
        name: Cow::Borrowed("Francesca"),
        unique_style: ReductionistStereotype::Cutesy,
        secret_telepathy_all_girls_have: PINK_LINK,
    };
    let _ = fran.as_tuple_mut();
    fran.as_tuple();
}
