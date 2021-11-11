// Copyright (C) 2021 Ferran Pujol Camins
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// Copyright (C) 2021 Ferran Pujol Camins
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::marker::PhantomData;

pub struct Getter<S, A, Type: Fn(S) -> A> {
    getter: Type,
    s: PhantomData<S>,
    a: PhantomData<A>,
}

pub struct Lens<S, A, Getter: Fn(S) -> A, Setter: Fn(A, S) -> S> {
    getter: Getter,
    setter: Setter,
    s: PhantomData<S>,
    a: PhantomData<A>,
}

pub struct Prism<S, A, Getter: Fn(S) -> Option<A>, Builder: Fn(A) -> S> {
    getter: Getter,
    builder: Builder,
    s: PhantomData<S>,
    a: PhantomData<A>,
}

pub fn _compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    return move |a| g(f(a));
}

pub fn _composel<S, A, Getter, Setter, C, G>(l: Lens<S, A, Getter, Setter>, g: G) -> impl Fn(S) -> C
where
    Getter: Fn(S) -> A,
    Setter: Fn(A, S) -> S,
    G: Fn(A) -> C,
{
    return _compose(l.getter, g);
}

#[macro_export]
macro_rules! compose {
    ( $f:expr, $( $r:expr ),+ $(,)*) => {
        {
            $crate::_compose(
                $f,
                $crate::compose!(
                    $(
                        $r,
                    )*
                )
            )
        }
    };
    ( $f:expr $(,)* ) => {
        {
            $f
        }
    };

}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    use super::*;

    #[test]
    fn test_function_composition() {
        let f = compose!(|x: Vec<i32>| x.len(), |y: usize| y + 1, Option::Some);
        assert_eq!(f(vec![1, 2, 3]), Option::Some(4));
        assert_eq!(f(vec![]), Option::Some(1));
    }

    // #[test]
    // fn test_lens_composition() {
    //     #[derive(Debug)]
    //     struct S {
    //         field: usize,
    //         other: usize
    //     }
    //     let s = S { field: 2, other: 3 };
    //     let l = Lens { 
    //         getter: |s: S| s.field,
    //         setter: |field: usize, s: S| S { field: field, other: s.other },
    //         s: PhantomData,
    //         a: PhantomData
    //         // TODO: constructor to avoid constructing phantom data explicitly
    //     };
    //     let f = compose2!(l, |y: usize| y + 1, Option::Some);
    //     assert_eq!(f(S {field: 5, other: 1}), Option::Some(6));
    //     assert_eq!(f(S {field: 0, other: 1}), Option::Some(1));
    // }
}
