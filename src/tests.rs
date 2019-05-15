use crate::Value;
use proptest::prelude::*;

proptest! {
    #[test]
    fn parse_of_to_string(v: Value) {
        let s = dbg!(v.to_string());
        let r = s.parse::<Value>();
        prop_assert!(r.is_ok());
        let v2 = r.unwrap();
        prop_assert_eq!(v, v2);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValueArbitraryParams {
    pub depth: u32,
    pub max_size: u32,
    pub max_collection_size: u32,
}

impl Default for ValueArbitraryParams {
    fn default() -> ValueArbitraryParams {
        ValueArbitraryParams {
            depth: 8,
            max_size: 256,
            max_collection_size: 10,
        }
    }
}

impl Arbitrary for Value {
    type Parameters = ValueArbitraryParams;
    type Strategy = BoxedStrategy<Value>;

    fn arbitrary_with(params: ValueArbitraryParams) -> Self::Strategy {
        let max_collection_size = params.max_collection_size as usize;
        any::<String>()
            .prop_map(Value::Sym)
            .prop_recursive(
                params.depth,
                params.max_size,
                params.max_collection_size,
                move |inner| {
                    prop::collection::vec(inner, 0..max_collection_size).prop_map(Value::List)
                },
            )
            .boxed()
    }
}
