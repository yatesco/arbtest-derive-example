#[cfg(any(feature = "tests", test))]
use arbtest::arbitrary::{self,Unstructured};

fn prop(u: &mut Unstructured<'_>) -> arbitrary::Result<()> {
    let expected = u.arbitrary::<d::AThing>().unwrap();
    dbg!(&expected);
    Ok(())
}

#[test]
fn does_it_work() {
    arbtest::builder().budget_ms(1_0).run(prop)
}
