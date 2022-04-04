#![feature(derive_default_enum)]

//! ![Probably::Nothing](https://i.imgur.com/AuDdbOK.png)

#[derive(Clone,Copy,PartialOrd,Ord,PartialEq,Eq,Debug,Hash,Default)]
pub enum Probably<T> {
    #[default]
    Nothing,
    Something(T),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
