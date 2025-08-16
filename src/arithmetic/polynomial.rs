use super::*;

pub type Z = Integer;
pub type QPol = Polynomial<Rational>;

#[derive(Clone, Debug)]
pub struct Polynomial<T: Field>{
    pub vector: Vec<T>,
}

impl<T: Field> Polynomial<T> {
    pub fn as_polynomial(input: &T) -> Self {
        let v: Vec<T> = vec![input.clone()];
        Polynomial::<T>{vector: v}
    }

    pub fn monomial(coefficient: &T, order: usize) -> Self {
        let zero = T::zero();
        let mut vec = vec![zero; order +1];
        vec[order] = coefficient.clone();
        Polynomial::<T>{vector: vec}
    }
} 

impl<T: Field> Polynomial<T>{
    pub fn reduced(&self) -> Self{
        let k = self.norm();
        let vec = &self.vector;
        let zero = T::zero();
        let mut target: Vec<T> = vec![zero; k+1];

        let mut i: usize = 0;
        while i <= k {
            target[i] = vec[i];
            i += 1;
        }
        Polynomial::<T>{vector: target}
    }
    
    // read coefficient at the given term.
    pub fn coefficient(&self, order: usize) -> T {
        let vec = &self.vector;
        vec[order]
    }

    // read coefficient at highest term. if the pol is non-zero, then it's principal coeff
    // is non-zero as well. eg "2x+3" to "2".
    pub fn principal_coefficient(&self) -> T {
        let k = self.norm();
        self.coefficient(k)
    }
}

impl<T: Field> EuclideanRing for Polynomial<T>{
    fn zero() -> Self {
        Polynomial::<T> {
            vector: vec![T::zero()]
        }
    }
    fn one() -> Self {
        Polynomial::<T> {
            vector: vec![T::one()]
        }
    }
    fn norm(&self) -> usize {
        let mut k: usize = 0;
        let mut i: usize = 0;
        let zero= T::zero();
        let v = &self.vector;
        while i < v.len() {
            if !T::equal(&v[i], &zero) {
                k = i;
            }
            i += 1;
        }
        k
    }
    fn add (x: &Self, y:& Self) -> Self {
        let order_x = x.norm(); let order_y = y.norm();
        let mut higher_order =order_x;
        let mut lower_order = order_y;
        let mut longer_vec  = &x.vector;
        let mut shorter_vec = &y.vector;
        if order_x < order_y{
            longer_vec = &y.vector; shorter_vec = &x.vector;
            higher_order = order_y; lower_order = order_x;
        }

        let zero = T::zero();
        let mut target: Vec<T> = vec![zero; higher_order+1];
        let mut i: usize = 0;

        while i <= lower_order {
            target[i] = T::add(&shorter_vec[i], &longer_vec[i]);
            i += 1;
        }
        while i <= higher_order {
            target[i] = longer_vec[i];
            i += 1;
        }

        Polynomial::<T>{vector: target}.reduced()
    }

    fn multiply (x: &Self, y:& Self) -> Self {
        let order_x = x.norm(); 
        let order_y = y.norm();
        let zero = T::zero();
        
        let target_order = order_x + order_y;
        let mut target: Vec<T> = vec![zero; target_order + 1];

        let mut i: usize  = 0;
        while i <= order_x {
            let mut j: usize =0;
            while j <= order_y{
                target[i+j] = T::add(&target[i+j],&T::multiply(&x.vector[i], &y.vector[j])); 
                j += 1;
            }
            i += 1;
        }
        Polynomial::<T>{vector: target}.reduced()
    }
    fn neg(&self) -> Self {
        let order = self.norm();
        let vec = &self.vector;
        let zero = T::zero();
        let mut target = vec![zero; order +1];
        let mut i: usize = 0;
        while i <= order {
            target[i] = T::neg(&vec[i]);
            i += 1;
        }
        Polynomial::<T>{vector: target}.reduced()
    }
    fn equal(x: &Self, y: &Self) -> bool {
        let order_x = x.norm(); 
        let order_y = y.norm();

        if order_x != order_y {return false;}

        let mut i: usize = 0;
        while i <= order_x{
            if !T::equal(&x.vector[i], &y.vector[i]){
                return false;
            }
            i += 1;
        }
        true
    }
    fn subtract (x: &Self, y:& Self) -> Self {
        Self::add(x, &y.neg())
    }
    fn divmod(x: &Self, y:&Self) -> Duo<Self> {
        let zero = Self::zero();

        if *y == zero {
            panic!("cannot divide by zero")
        }
        
        let mut dividend = x.reduced();
        let mut dividend_order = dividend.norm();
        let divisor = y.clone();
        
        let divisor_order = y.norm();
        let divisor_coefficient = divisor.vector[divisor_order];

        if dividend_order < divisor_order || dividend == zero {
            return Duo::<Self>{
                first: Self::zero(),
                second: dividend,
            }
        }

        let mut final_quotient = 
            Self::monomial(&T::zero(), dividend_order - divisor_order);
        
        while dividend_order >= divisor_order && dividend != zero {
            let temp_coeff = T::divide(&dividend.principal_coefficient(), &divisor_coefficient);
            let temp_q = Self::monomial(&temp_coeff, dividend_order - divisor_order);
            dividend = Self::subtract(&dividend, &Self::multiply(&temp_q, &divisor));
            dividend_order = dividend.norm();
            final_quotient = Self::add(&final_quotient, &temp_q);
        }

        Duo::<Self>{
            first: final_quotient,
            second: dividend,
        }
    }

    fn regular(&self) -> Trio<Self> {
        if *self == Self::zero() {
            return Trio::<Self> {
                first: Self::zero(),
                second: Self::zero(),
                third: Self::zero(),
            }
        }

        let pr_coeff = self.principal_coefficient();
        let monicify_coeff = pr_coeff.inv();
        let inv_sign = Self::as_polynomial(&monicify_coeff);

        let sign = Self::as_polynomial(&pr_coeff);
        let reg = Self::multiply(self, &inv_sign);

        Trio::<Self> {
            first: sign,
            second: reg,
            third: inv_sign,
        }
    }
}

impl<T: Field> std::cmp::PartialEq for Polynomial<T> {
    fn eq(&self, other: &Self) -> bool {
        Self::equal(self, other)
    }
}




// special pol type with integer coefficients. It can be converted from QPol if possible.
// only find its use in serving as a demonstrator for a  PrimePol.
// it may be made generic in the future, but I don't see a need for that right now.
#[derive(Clone)]
pub struct ZPol {
    pub vector: Vec<Z>,
}

impl ZPol {
    pub fn as_polynomial(input: &Z) -> Self {
        let v: Vec<Z> = vec![input.clone()];
        ZPol{vector: v}
    }

    pub fn monomial(coefficient: &Z, order: usize) -> Self {
        let zero = Z::zero();
        let mut vec = vec![zero; order +1];
        vec[order] = coefficient.clone();
        ZPol{vector: vec}
    }

    pub fn reduced(&self) -> Self{
        let k = self.norm();
        let vec = &self.vector;
        let zero = Z::zero();
        let mut target: Vec<Z> = vec![zero.clone(); k+1];

        let mut i: usize = 0;
        while i <= k {
            target[i] = vec[i];
            i += 1;
        }
        
        ZPol{vector: target}
    }

    pub fn coefficient(&self, order: usize) -> Z {
        let vec = &self.vector;
        vec[order]
    }

    pub fn principal_coefficient(&self) -> Z {
        let k = self.norm();
        self.coefficient(k)
    }
}

// arithmetic functions. This could be done as a simple re-use of QPols,
// but I opt not because that will induce meaningless gcd calculation.
impl ZPol {
    pub fn zero() -> Self {
        ZPol {
            vector: vec![Z::zero()]
        }
    }

    pub fn one() -> Self {
        ZPol {
            vector: vec![Z::one()]
        }
    }

    pub fn norm(&self) -> usize {
        let mut k: usize = 0;
        let mut i: usize = 0;
        let zero= Z::zero();
        let v = &self.vector;
        while i < v.len() {
            if v[i] != zero {
                k = i;
            }
            i += 1;
        }
        k
    }

    pub fn add (x: &Self, y:& Self) -> Self {
        let order_x = x.norm(); let order_y = y.norm();
        let mut higher_order =order_x;
        let mut lower_order = order_y;
        let mut longer_vec  = &x.vector;
        let mut shorter_vec = &y.vector;
        if order_x < order_y{
            longer_vec = &y.vector; shorter_vec = &x.vector;
            higher_order = order_y; lower_order = order_x;
        }

        let zero = Z::zero();
        let mut target: Vec<Z> = vec![zero; higher_order+1];
        let mut i: usize = 0;

        while i <= lower_order {
            target[i] = Z::add(&shorter_vec[i], &longer_vec[i]);
            i += 1;
        }
        while i <= higher_order {
            target[i] = longer_vec[i];
            i += 1;
        }

        ZPol{vector: target}.reduced()
    }

    pub fn multiply (x: &Self, y:& Self) -> Self {
        let order_x = x.norm(); 
        let order_y = y.norm();
        let zero = Z::zero();
        
        let target_order = order_x + order_y;
        let mut target: Vec<Z> = vec![zero; target_order + 1];

        let mut i: usize  = 0;
        while i <= order_x {
            let mut j: usize =0;
            while j <= order_y{
                target[i+j] = Z::add(&target[i+j],&Z::multiply(&x.vector[i], &y.vector[j])); 
                j += 1;
            }
            i += 1;
        }
        ZPol{vector: target}.reduced()
    }

    pub fn neg(&self) -> Self {
        let order = self.norm();
        let vec = &self.vector;
        let zero = Z::zero();
        let mut target = vec![zero; order +1];
        
        let mut i: usize = 0;
        while i <= order {
            target[i] = Z::neg(&vec[i]);
            i += 1;
        }

        ZPol{vector: target}
    }

    pub fn equal(x: &Self, y: &Self) -> bool {
        let order_x = x.norm(); 
        let order_y = y.norm();

        if order_x != order_y {return false;}

        let mut i: usize = 0;
        while i <= order_x{
            if x.vector[i] != y.vector[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    pub fn subtract (x: &Self, y:& Self) -> Self {
        Self::add(x, &y.neg())
    }


}

impl std::cmp::PartialEq for ZPol {
    fn eq(&self, other: &Self) -> bool {
        Self::equal(self, other)
    }
}


// Important: QPol -> ZPol conversion, for re-use of qpol parser
impl QPol {
    pub fn is_zpol(&self) -> bool {
        let vec = self.reduced().vector;
        let ord = self.norm();

        let mut i: usize = 0;
        while i <= ord {
            if !vec[i].is_integer() {
                return false;
            } 
            i += 1;
        }

        true
    }

    pub fn to_zpol(&self) -> Option<ZPol> {
        let vec = self.reduced().vector;
        let ord = self.norm();
        let zero = Z::zero();

        let mut target: Vec<Z> = vec![zero.clone(); ord+1];

        let mut i: usize = 0;
        while i <= ord {
            let Some(int) = vec[i].to_integer() else {
                return None;
            };

            target[i] = int;

            i += 1;
        }

        Some(
            ZPol {vector: target}
        )
    }
}

// important: ZPol -> QPol conversion, for re-use of qpol diplayer
impl ZPol {
    pub fn to_qpol(&self) -> QPol {
        let vec = &self.vector;
        let ord = self.norm();
        let qzero = Rational::zero();
        let mut target = vec![qzero.clone(); ord+1];

        let mut i: usize = 0;
        while i <= ord {
            target[i] = vec[i].to_rational();
            i += 1;
        }

        QPol {
            vector: target,
        }
    }
}