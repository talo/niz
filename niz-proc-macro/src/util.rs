use proc_macro2::TokenTree;
use syn::Attribute;

pub(crate) fn has_skip_attr(attrs: &Vec<Attribute>) -> bool {
    has_any_attr(&["skip"], attrs)
}

pub(crate) fn has_json_attr(attrs: &Vec<Attribute>) -> bool {
    has_any_attr(&["json"], attrs)
}

fn has_any_attr(options: &[&str], attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if !attr.path.is_ident("niz") {
            // ignore non-niz attributes
            continue;
        }
        // find any of the listed attributes
        if attr
            .tokens
            .clone()
            .into_iter()
            .filter_map(|token_tree| match token_tree {
                TokenTree::Group(group) => Some(group.stream().into_iter()),
                _ => None,
            })
            .flatten()
            .any(|t| {
                let s = t.to_string();
                options.iter().any(|o| &s == o)
            })
        {
            return true;
        }
    }
    false
}
