use std::fmt::Debug;
use std::ops::Add;

#[salsa::query_group(TypeGenericQueryGroupStorage)]
trait TypeGenericQueryGroup<T>
where
    T: Add<Output = T> + Clone + Default + Debug + Eq + 'static,
{
    #[salsa::input]
    fn input(&self) -> T;

    fn multiplied_input(&self, factor: usize) -> T;
}

fn multiplied_input<T>(db: &dyn TypeGenericQueryGroup<T>, factor: usize) -> T
where
    T: Add<Output = T> + Clone + Default + Debug + Eq + 'static,
{
    std::iter::repeat(db.input())
        .take(factor)
        .fold(Default::default(), Add::add)
}

#[salsa::database(TypeGenericQueryGroupStorage)]
#[derive(Default)]
struct DatabaseStruct<T>
where
    T: Add<Output = T> + Clone + Default + Debug + Eq + 'static,
{
    storage: salsa::Storage<Self>,
}

impl<T> salsa::Database for DatabaseStruct<T> where
    T: Add<Output = T> + Clone + Default + Debug + Eq + 'static
{
}

#[test]
fn type_generic_query_group_input_round_trip() {
    let mut db = DatabaseStruct::default();

    db.set_input(123);
    assert_eq!(123, db.input());
    assert_eq!(123 * 3, db.multiplied_input(3));

    db.set_input(456);
    assert_eq!(456, db.input());
    assert_eq!(456 * 3, db.multiplied_input(3));
}
