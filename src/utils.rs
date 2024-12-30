pub fn pretty_type_name<T>() -> String {
    format!("{:?}", disqualified::ShortName::of::<T>())
}
