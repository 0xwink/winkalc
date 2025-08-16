use crate::arithmetic::integer::GaussInteger;

use super::*;

pub struct NumPair {
    pub first: int,
    pub second: int,
}

impl NumPair {
    pub fn gcd (&self) -> int {
        if self.first ==0 || self.second ==0 {
            return 0;
        }

        let mut r: int = 1;
        let mut a: int = self.first.abs();
        let mut b: int = self.second.abs();

        if a < b {
            let temp = a;
            a = b;
            b = temp;
        }

        while r != 0 {
            r = a % b;

         
            a = b; b = r;
        }

        a
        
    }

    pub fn reduced_pair (&self) -> Self{
        if self.first == 0 && self.second == 0 {
            return Self::new(0,0);
        }
        if self.second == 0 {
            return Self::new(1,0);
        }
        if self.first == 0 {
            return Self::new(0,1);
        }
        let mut x = self.first;
        let mut y = self.second;

        if y < 0 {
            x = -x; y = -y;
        }

        let a = self.gcd();

        Self {
            first: x / a,
            second: y / a,
        }
    }
}

impl NumPair {
    pub fn new(x: int, y: int) -> Self {
        Self { first: (x), second: (y) }
    }

    pub fn display(&self){
        println!("({}, {})",  self.first, self.second);
    }
    
    pub fn to_rational(&self) -> Rational{
        if self.second ==0 {panic!("Denominator must not be 0.");}
    
        let reduced = self.reduced_pair();
    
        Rational { numerator: reduced.first, denominator: reduced.second }
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Rational{
    pub numerator: int,
    pub denominator: int,
}

impl Rational {
    pub fn to_pair (&self) -> NumPair {
        NumPair::new(self.numerator, self.denominator)
    }
    
    pub fn reduced(&self) -> Self {
        self.to_pair().to_rational()
    }

    pub fn new (num: int, denom: int) -> Rational {
        NumPair::new(num, denom).to_rational()
    }

    pub fn is_integer(&self) -> bool {
        let reduced = self.reduced();
        match reduced.denominator {
            1 => true,
            _ => false,
        }
    }

    pub fn to_integer(&self) -> Option<Integer> {
        let reduced = self.reduced();
        match reduced.denominator {
            1 => Some(Integer::new(reduced.numerator)),
            _ => None,
        }
    }

    pub fn is_positive(&self) -> bool {
        self.reduced().numerator > 0
    }

    pub fn is_negative(&self) -> bool {
        self.reduced().numerator < 0  
    }

    pub fn abs(&self) -> Self {
        match self.is_negative() {
            true => self.neg(),
            false => self.clone()
        }.reduced()
    }

    // give a rational x, it returns the greatest integer n such that n <=x.
    // e.g. 0 -> 0; 1 -> 1; 0.5 -> 0; 1.2 -> 1; -0.2 -> -1; 
    pub fn floor(&self) -> Integer {
        // floor function takes an integer to itself
        if self.is_integer() {
            return self.to_integer().unwrap();
        }

        // The following is non-integer case;
        // For x a negative non-integer, floor(x) = -(floor(abs(x))+1)
        let abs = self.abs();

        let num = abs.numerator; let den = abs.denominator;
        let q = num.div_euclid(den);
        
        match self.is_negative() {
            true => Integer::new(-(q+1)),
            false => Integer::new(q)
        }
    }

    // Calculate the distance from self to its floor, if diff <= 1/2 then return floor.
    // If not, return floor + 1.
    // We always have [n, n+0.5] -> n; (n+0.5, n+1) -> n+1. 
    pub fn nearest_integer(&self) -> Integer {
        let half = Rational::new(1,2);
        let floor = self.floor();
        let diff = *self - floor.to_rational();

        if diff <= half {
            floor
        } else {
            Integer::new(floor.number+1)
        }
    }
}

impl Field for Rational {
    fn zero() -> Self {
        Rational{numerator: 0, denominator: 1}
    }
    
    fn one() -> Self {
        Rational{numerator: 1, denominator: 1}
    }

    fn equal(x: &Self, y: &Self) -> bool {
        x.numerator * y.denominator == x.denominator * y.numerator
    }

    fn add(x: &Self, y: &Self ) -> Self{
        let p = x.numerator * y.denominator + y.numerator * x.denominator;
        let q = x.denominator * y.denominator;

        NumPair::new(p,q).to_rational()
    }

    fn inv(&self) -> Self {
        if *self == Self::zero() {
            panic!("zero is not invertible.");
        }

        NumPair::new(self.denominator, self.numerator).to_rational()
    }

    fn neg(&self) -> Self {
        NumPair::new(- self.numerator, self.denominator).to_rational()
    }

    fn subtract(x: &Self, y: &Self) -> Self{
        Self::add(x, &(y.neg()))
    }

    fn multiply(x: &Self, y: &Self) -> Self{
        let p = x.numerator * y.numerator;
        let q = x.denominator * y.denominator;

        NumPair::new(p,q).to_rational()
    }

    fn divide(x: &Self, y: &Self) -> Self {
        if *y == Self::zero() {
            panic!("denominator cannot be zero.");
        }

        Self::multiply(x, &(y.inv()))
    }
}


#[derive(Copy, Clone, Debug)]
pub struct ComplexRational{
    pub real: Rational,
    pub imag: Rational,
}

impl ComplexRational {
    pub fn real_to_pair(&self) -> NumPair {
        self.real.to_pair()
    }
    pub fn imag_to_pair(&self) -> NumPair {
        self.imag.to_pair()
    }

    pub fn reduced(&self) -> Self {
        let reduced_real = self.real_to_pair().to_rational();
        let reduced_imag = self.imag_to_pair().to_rational();
        ComplexRational { real: reduced_real, imag: reduced_imag }
    }

    pub fn new (real_num: int, real_denom: int, imag_num: int, imag_denom: int) -> Self {
        ComplexRational {
            real: NumPair::new(real_num, real_denom).to_rational(),
            imag: NumPair::new(imag_num, imag_denom).to_rational(),
        }
    }

    pub fn is_gauss_integer(&self) -> bool {
        self.real.is_integer() && self.imag.is_integer()
    }

    pub fn to_gauss_integer(&self) -> Option<GaussInteger> {
        if !self.is_gauss_integer() {
            return None;
        }

        let res = GaussInteger {
            real: self.real.to_integer().unwrap(),
            imag: self.imag.to_integer().unwrap(),
        };

        Some(res)
    }

    pub fn is_real(&self) -> bool {
        self.imag == Rational::zero()
    }

    pub fn to_real(&self) -> Option<Rational> {
        if !self.is_real() {
            return None;
        }

        Some(self.real)
    }

    pub fn is_pure_imag(&self) -> bool {
        self.real == Rational::zero()
    }

    // Note that a complex number can have several nearest gauss integer, 
    // e.g. how to find it for 0.5 + 0.5i? No canonical choice. This depends on the implementation.
    // If x is itself a Gaussian integer, then the function always returns itself.
    pub fn nearest_gauss_integer(&self) -> GaussInteger {
        GaussInteger { real: self.real.nearest_integer(), imag: self.imag.nearest_integer() }
    }
}


impl Field for ComplexRational {
    fn zero() -> Self {
        ComplexRational {
            real: Rational {numerator: 0, denominator: 1},
            imag: Rational {numerator: 0, denominator: 1}
        }
    }

    fn one() -> Self {
        ComplexRational {
            real: Rational {numerator: 1, denominator: 1},
            imag: Rational {numerator: 0, denominator: 1}
        }
    }

    fn add (x: &Self, y:& Self) -> Self {
        let a = x.real; let b = x.imag;
        let c = y.real; let d = y.imag;
        ComplexRational {
            real: Rational::add(&a,&c),
            imag: Rational::add(&b,&d)
        }
    }

    fn subtract (x: &Self, y:& Self) -> Self {
        let a = x.real; let b = x.imag;
        let c = y.real; let d = y.imag;
        ComplexRational {
            real: Rational::subtract(&a,&c),
            imag: Rational::subtract(&b,&d)
        }
    }

    // (a+bi)*(c+di) = (ac-bd) + (bc+ad) i
    fn multiply (x: &Self, y:& Self) -> Self {
        let a = x.real; let b = x.imag;
        let c = y.real; let d = y.imag;

        ComplexRational { real: a*c-b*d, imag:  b*c+a*d}
    }

    fn equal(x: &Self, y: &Self) -> bool {
        (x.real == y.real) && (x.imag == y.imag)
    }

    fn neg(&self) -> Self {
        ComplexRational { real: -self.real, imag: -self.imag}
    }

    // 1/(a+bi) = (a-bi)/(a^2+b^2)
    fn inv(&self) -> Self {
        if self.is_zero() {
            panic!("zero is not invertible.")
        }
        let a = self.real; let b = self.imag;
        let n = a*a+b*b;

        ComplexRational { real: a/n, imag: (-b)/n }
    }

    fn divide (x: &Self, y: &Self) -> Self {
        if y.is_zero() {
            panic!("zero cannot be a denominator.")
        }
        Self::multiply(x, &y.inv())
    }
}


mod chore {
    use super::{ComplexRational, Rational, Field};

    // Chore: overloading arithmetic operators (+ - * / = < >) for Rational 
    impl std::cmp::PartialEq for Rational {
        fn eq(&self, other: &Self) -> bool {
            Rational::equal(self, other)
        }
    }

    impl std::cmp::Eq for Rational {}

    impl std::cmp::Ord for Rational {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            use std::cmp::Ordering;

            let res = Rational::subtract(self, other);
            if res.numerator < 0 {
                return Ordering::Less;
            } else if res.numerator > 0 {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        }
    }

    impl std::cmp::PartialOrd for Rational{
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl std::ops::Add for Rational {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            <Self as Field>::add(&self, &rhs)
        }
    }

    impl std::ops::Sub for Rational {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
            <Self as Field>::subtract(&self, &rhs)
        }
    }

    impl std::ops::Mul for Rational {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            <Self as Field>::multiply(&self, &rhs)
        }
    }

    impl std::ops::Neg for Rational {
        type Output = Self;

        fn neg(self) -> Self {
            <Self as Field>::neg(&self)
        }
    }

    impl std::ops::Div for Rational {
        type Output = Self;

        fn div(self, rhs: Self) -> Self {
            <Self as Field>::divide(&self, &rhs)
        }
    }

    // Chore: overloading arithmetic operators (+ - * / =) for ComplexRational
    impl std::cmp::PartialEq for ComplexRational {
        fn eq(&self, other: &Self) -> bool {
            <ComplexRational as Field>::equal(&self, other)
        }
    }

    impl std::cmp::Eq for ComplexRational {}

    impl std::ops::Add for ComplexRational {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            <Self as Field>::add(&self, &rhs)
        }
    }

    impl std::ops::Sub for ComplexRational {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
            <Self as Field>::subtract(&self, &rhs)
        }
    }

    impl std::ops::Mul for ComplexRational {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            <Self as Field>::multiply(&self, &rhs)
        }
    }

    impl std::ops::Neg for ComplexRational {
        type Output = Self;

        fn neg(self) -> Self {
            <Self as Field>::neg(&self)
        }
    }

    impl std::ops::Div for ComplexRational {
        type Output = Self;

        fn div(self, rhs: Self) -> Self {
            <Self as Field>::divide(&self, &rhs)
        }
    }
}

