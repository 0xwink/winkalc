use super::*;

#[derive(Clone, Copy, Debug)] 
pub struct Integer{
    pub number: int,
}

impl Integer{
    pub fn new(x: int) -> Integer {
        Integer{number: x}
    }

    pub fn to_rational(&self) -> Rational {
        Rational {
            numerator: self.number,
            denominator: 1
        }
    }

    pub fn is_positive(&self) -> bool {
        self.number > 0
    }

    pub fn is_negative(&self) -> bool {
        self.number < 0
    }
}

impl EuclideanRing for Integer{
    fn zero() -> Self {
        Self::new(0)
    }

    fn one() -> Self {
        Self::new(1)
    }

    fn add (x: &Self, y:& Self) -> Self{
        Self::new(x.number + y.number)
    }
    
    fn subtract (x: &Self, y:& Self) -> Self{
        Self::new(x.number - y.number)
    }

    fn multiply (x: &Self, y:& Self) -> Self{
        Self::new(x.number * y.number)
    }

    fn neg(&self) -> Self{
        Self::new(-self.number)
    }

    fn divmod(x: &Self, y: &Self) -> Duo<Self> {
        if Integer::equal(y, &Integer::zero()) {
            panic!("cannot divide by zero")
        }
        Duo::<Self>{
            first: Self::new(x.number.div_euclid(y.number)), 
            second: Self::new(x.number.rem_euclid(y.number)),
        }
    }

    fn norm(&self) -> usize {
        self.number.abs() as usize
    }

    fn equal(x: &Self, y:&Self) -> bool {
        x.number == y.number
    }

    fn regular(&self) -> Trio<Self> {
        if self.number < 0 {
            return Trio::<Self> {
                first: Self::new(-1),
                second: self.neg(),
                third: Self::new(-1),
            }
        }
        else if self.number == 0 {
            return Trio::<Self> {
                first: Self::new(0),
                second: Self::new(0),
                third: Self::new(0),
            }
        }
        else {
            return Trio::<Self> {
                first: Self::new(1),
                second: self.clone(),
                third: Self::new(1),
            }
        }
    }
}


#[derive(Clone, Copy, Debug)] 
pub struct GaussInteger{
    pub real: Integer,
    pub imag: Integer,
}

impl GaussInteger{
    pub fn new(real: int, imag: int) -> Self {
        GaussInteger { real: Integer::new(real), imag: Integer::new(imag) }
    }

    pub fn to_complex_rational(&self) -> ComplexRational {
        ComplexRational {
            real: self.real.to_rational(),
            imag: self.imag.to_rational()
        }
    }

    // 0 is regular; if a+bi is nonzero, then it is regular iff a>0 and b>=0. 
    pub fn is_regular(&self) -> bool {
        if self.is_zero() {
            return true;
        }

        self.real.is_positive() && !self.imag.is_negative()
    }
}


// work in progress
impl EuclideanRing for GaussInteger{
    fn zero() -> Self {
        Self::new(0, 0 )
    }

    fn one() -> Self {
        Self::new(1, 0)
    }

    fn add (x: &Self, y:& Self) -> Self {
        Self::new(x.real.number + y.real.number, x.imag.number + y.imag.number)
    }

    fn subtract (x: &Self, y:& Self) -> Self {
        Self::new(x.real.number - y.real.number, x.imag.number - y.imag.number)
        
    }

    // (a+bi)*(c+di) = (ac-bd) + (bc+ad) i 
    fn multiply (x: &Self, y:& Self) -> Self{
        let a = x.real.number; let b = x.imag.number;
        let c = y.real.number; let d = y.imag.number;

        Self::new(a*c-b*d, b*c+a*d)
    }

    fn neg(&self) -> Self {
        Self::new(-self.real.number, -self.imag.number)
    }

    fn equal(x: &Self, y: &Self) -> bool {
        (x.real.number == y.real.number) && (x.imag.number == y.imag.number)
    }

    fn norm(&self) -> usize {
        (self.real.number * self.real.number + self.imag.number * self.imag.number) as usize
    }
    
    fn divmod(x: &Self, y: &Self) -> Duo<Self> {
        if y.is_zero() {
            panic!("Cannot divide by zero.");
        }
        
        let x_rat = x.to_complex_rational(); let y_rat = y.to_complex_rational();
        let q = (x_rat/y_rat).nearest_gauss_integer();
        let r = *x - (q * *y);

        Duo::<Self> {
            first: q, second: r
        }
    }

    fn regular(&self) -> Trio<Self> {
        let i = Self::new(0,1);
        let one = Self::one();
        let z = self.clone();

        if z.is_regular() {
            return Trio::<Self> {
                first: one, second: z, third: one
            }
        } else if (z * i).is_regular() {
            return Trio::<Self> {
                first: -i, second: z * i, third: i
            }
        } else if (-z).is_regular() {
            return Trio::<Self> {
                first: -one, second: -z, third: -one
            }
        } else {
            return Trio::<Self> {
                first: i, second: -(z * i), third: -i
            }
        }
    }
}

mod chore {
    use super::{Integer, GaussInteger, EuclideanRing};

    impl std::cmp::PartialEq for Integer {
        fn eq(&self, other: &Self) -> bool {
            Self::equal(self, other)
        }
    }

    impl std::cmp::Eq for Integer{}

    impl std::cmp::Ord for Integer {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            use std::cmp::Ordering;

            let res = Integer::subtract(self, other);
            if res.number < 0 {
                return Ordering::Less;
            } else if res.number > 0 {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        }
    }

    impl std::cmp::PartialOrd for Integer{
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl std::ops::Add for Integer {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            <Self as EuclideanRing>::add(&self, &rhs)
        }
    }

    impl std::ops::Sub for Integer {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
            <Self as EuclideanRing>::subtract(&self, &rhs)
        }
    }

    impl std::ops::Mul for Integer {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            <Self as EuclideanRing>::multiply(&self, &rhs)
        }
    }

    impl std::ops::Neg for Integer {
        type Output = Self;

        fn neg(self) -> Self {
            <Self as EuclideanRing>::neg(&self)
        }
    }

    
    impl std::cmp::PartialEq for GaussInteger {
        fn eq(&self, other: &Self) -> bool {
            Self::equal(self, other)
        }
    }

    impl std::cmp::Eq for GaussInteger{}

    impl std::ops::Add for GaussInteger {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            <Self as EuclideanRing>::add(&self, &rhs)
        }
    }

    impl std::ops::Sub for GaussInteger {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
            <Self as EuclideanRing>::subtract(&self, &rhs)
        }
    }

    impl std::ops::Mul for GaussInteger {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            <Self as EuclideanRing>::multiply(&self, &rhs)
        }
    }

        impl std::ops::Neg for GaussInteger {
        type Output = Self;

        fn neg(self) -> Self {
            <Self as EuclideanRing>::neg(&self)
        }
    }
}

