use molmap::{self, Element, Map, atomic::AtomMap0, entities::Atom, view::All};

#[test]
fn views_usage() {
    let mut mm = AtomMap0::new();
    let h1 = mm.add_atom(Element::H);
    let _atom_views = mm.views::<Atom>(All).unwrap();
    let _atom_views = mm.views([h1]).unwrap();
}
