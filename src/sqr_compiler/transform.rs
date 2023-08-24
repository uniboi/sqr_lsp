/// cast any arbitrary address to type `t`
macro_rules! cast {
    ($address: expr, $t: ty) => {
        std::mem::transmute::<*const (), $t>($address as _)
    };
}

pub(crate) use cast;
