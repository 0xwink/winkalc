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

impl std::cmp::PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        Self::equal(self, other)
    }
}

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
    
}
