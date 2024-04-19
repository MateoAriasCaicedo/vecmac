/// Documentation.
#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };

    ($($element: expr),*) => {{
        let mut vs = Vec::with_capacity($crate::avec![@COUNT; $($element),*]);
        $(vs.push($element);)+
        vs
    }};

    ($($element: expr,)*) => {
        $crate::avec![$($element),*]
    };

    ($element: expr; $count:expr) => {{
        let mut vs = Vec::new();
        vs.resize($count, $element);
        vs
    }};

    (@COUNT; $($element : expr),*) => {
        <[()]>::len(&[$($crate::avec![@SUBST; $element]),*])
    };

    (@SUBST; $_element : expr) => {()};
}

#[cfg(test)]
mod tests {

    #[test]
    fn empty_vec() {
        let x: Vec<u32> = avec![];
        assert!(x.is_empty());
    }

    #[test]
    fn single() {
        let x: Vec<u32> = avec![42];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 1);
        assert_eq!(x[0], 42);
    }

    #[test]
    fn double() {
        let x: Vec<u32> = avec![42, 32];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 32);
    }

    #[test]
    fn trailing() {
        let x: Vec<u32> = avec![1, 2,];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 1);
        assert_eq!(x[1], 2);
    }

    #[test]
    fn clone() {
        let x: Vec<u32> = avec![1;100];
        assert_eq!(x.len(), 100);
        assert_eq!(1, x[0]);
        assert_eq!(1, x[99]);
    }
}
