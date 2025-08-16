use crate::unwrapped_bezout;
use super::{polynomial::ZPol, *};

type Z = Integer;

#[derive(Clone, Copy)]
pub struct Prime {
    pub p: int,
}

pub fn is_prime(input: int) -> bool {
    let k = input;
    if k <= 1 {
        return false;
    }
    let mut i = 2;
    while i < k {
        if (k % i) == 0 {return false;}
        i += 1;
    }
    true
}

impl Prime {
    pub fn try_new(input: int) -> Option<Self>{
        if !is_prime(input) {
            return None;
        }

        Some(Prime {p: input})
    }

    pub fn to_integer(&self) -> Integer{
        Integer::new(self.p)
    }
}

// Prime acts on Integer. Outputs always fall in range [0, p-1].
impl Prime {
    pub fn modulo(&self, n: &Integer) -> Integer {
        let x = n.number.rem_euclid(self.p);
        Integer::new(x)
    }

    pub fn add(&self, x: &Integer, y: &Integer) -> Integer {
        let res = Integer::add(x, y);
        self.modulo(&res)
    }

    pub fn neg(&self, x: &Integer) -> Integer {
        let res = x.neg();
        self.modulo(&res)
    }

    pub fn subtract(&self, x: &Integer, y: &Integer) -> Integer {
        let res = Integer::subtract(x, y);
        self.modulo(&res)
    }

    pub fn multiply(&self, x:&Integer, y: &Integer) -> Integer {
        let res = Integer::multiply(x, y);
        self.modulo(&res)
    }

    pub fn inv(&self, input: &Integer) -> Integer {
        let x = self.modulo(input);
        if x.number == 0 { panic!(); }

        let p = self.to_integer();
        let (u, _, _) = unwrapped_bezout(&x, &p);

        self.modulo(&u)
    }

    pub fn divide(&self, x: &Integer, y: &Integer) -> Integer {
        let inv_y = self.inv(y);
        
        self.multiply(x, &inv_y)
    }

    pub fn equal(&self, x: &Z, y: &Z) -> bool {
        self.modulo(x) == self.modulo(y)
    }
}

// Prime acts on ZPol
impl Prime {
    pub fn modpol(&self, f: &ZPol) -> ZPol {
        let ord = f.norm();
        let vec = f.reduced().vector;
        let mut target = vec.clone();
        
        let mut i: usize = 0;
        while i <= ord {
            target[i] = self.modulo(&vec[i]);
            i += 1;
        }

        ZPol { vector: target }.reduced()
    }

    pub fn addpol(&self, f: &ZPol, g: &ZPol) -> ZPol {
        let res = ZPol::add(f, g);
        self.modpol(&res)
    }

    pub fn subpol(&self, f: &ZPol, g: &ZPol) -> ZPol {
        let res = ZPol::subtract(f, g);
        self.modpol(&res)
    }

    pub fn mulpol(&self, f: &ZPol, g: &ZPol) -> ZPol {
        let res = ZPol::multiply(f, g);
        self.modpol(&res)
    }

    pub fn negpol(&self, f: &ZPol) -> ZPol {
        let res = ZPol::neg(f);
        self.modpol(&res)
    }

    pub fn eqpol(&self, f: &ZPol, g: &ZPol) -> bool {
        self.modpol(f) == self.modpol(g)
    }
    
    pub fn normpol(&self, f: &ZPol) -> usize {
        self.modpol(f).norm()
    }
}

// HARD part. impl for bezout and divmod.
impl Prime {
    pub fn divmodpol(&self, f: &ZPol, g:&ZPol) -> Duo<ZPol> {
        let zero = ZPol::zero();

        if self.eqpol(g, &zero) {
            panic!("cannot divide by zero")
        }
        
        let mut dividend = self.modpol(f);
        let mut dividend_order = dividend.norm();
        let divisor = self.modpol(g);
        
        let divisor_order = divisor.norm();
        let divisor_coefficient = divisor.vector[divisor_order];

        if dividend_order < divisor_order || dividend == zero {
            return Duo::<ZPol>{
                first: ZPol::zero(),
                second: dividend,
            }
        }

        let mut final_quotient = 
            ZPol::monomial(&Z::zero(), dividend_order - divisor_order);
        
        while dividend_order >= divisor_order && dividend != zero {
            let temp_coeff = self.divide(&dividend.principal_coefficient(), &divisor_coefficient);
            let temp_q = ZPol::monomial(&temp_coeff, dividend_order - divisor_order);
            dividend = self.subpol(&dividend, &self.mulpol(&temp_q, &divisor));
            dividend_order = dividend.norm();
            final_quotient = self.addpol(&final_quotient, &temp_q);
        }

        Duo::<ZPol>{
            first: final_quotient,
            second: dividend,
        }
    }

    pub fn regular(&self, f: &ZPol) -> Trio<ZPol> {
        let red = self.modpol(f);

        if red == ZPol::zero() {
            return Trio::<ZPol> {
                first: ZPol::zero(),
                second: ZPol::zero(),
                third: ZPol::zero(),
            }
        }

        let pr_coeff = red.principal_coefficient();

        let monicify_coeff = self.inv(&pr_coeff);
        let sign_inv = ZPol::as_polynomial(&monicify_coeff);

        let sign = ZPol::as_polynomial(&pr_coeff);
        let reg = self.mulpol(&red, &sign_inv);

        Trio::<ZPol> {
            first: sign,
            second: reg,
            third: sign_inv
        }
    }

    pub fn recover(&self, input: &Trio<ZPol>) -> Duo<ZPol>{
        let m = &input.first;
        let n = &input.second;
        let q = &input.third;

        let new_m = self.modpol(n);
        let new_n = self.subpol(m, &self.mulpol(n, q));

        Duo::<ZPol>{
            first: new_m,
            second: new_n,
        }
    }

    pub fn bezoutpol(&self, input: &Duo<ZPol>) -> Trio<ZPol>{
        let mut f = self.modpol(&input.first);
        let mut g = self.modpol(&input.second);

        let zero = ZPol::zero();
        let one = ZPol::one();

        // edge case 1
        if f == zero && g == zero {
            return Trio::<ZPol>{
                first: zero.clone(),
                second: zero.clone(),
                third: zero.clone(),
            }
        }

        // edge case 2
        if f != zero && g == zero {
            let reg = self.regular(&f);

            return Trio::<ZPol> {
                first: reg.third,
                second: zero.clone(),
                third: reg.second,
            }
        }

        // edge case 3
        if f == zero && g != zero {
            let reg = self.regular(&g);

            return Trio::<ZPol> {
                first: zero.clone(),
                second: reg.third,
                third: reg.second,
            }
        }

        let max_step: usize = std::cmp::max(f.norm(), g.norm()) + 1;
        let mut vec_q: Vec<ZPol> = vec![zero.clone(); max_step+1];

        let mut mod_temp = self.divmodpol(&f, &g);
        let mut q = mod_temp.first; let mut r = mod_temp.second;

        // edge case
        if r == zero {
            return Trio::<ZPol> {
                first: zero.clone(),
                second: one.clone(),
                third: g.clone(),
            }
        }

        let mut done: bool = { self.divmodpol(&g, &r).second == zero };

        let mut i: usize = 1;
        vec_q[i] = q;

        while !done {
            f = g.clone();
            g = r.clone();
        
            mod_temp = self.divmodpol(&f, &g);

            q = mod_temp.first;
            r = mod_temp.second;

            i += 1;
            vec_q[i] = q;

            done = self.divmodpol(&g, &r).second == zero;  
        }

        let mut duo = Duo::<ZPol>{first: one.clone(), second: self.negpol(&vec_q[i].clone())};
    
        while i > 1 {
            let trio = Trio::<ZPol>{
                first: duo.first,
                second: duo.second, 
                third: vec_q[i-1].clone(),
            };
        
            duo = self.recover(&trio);
            i -= 1;
        }

        let reg_r = self.regular(&r);
        let monicifier = reg_r.third;

        Trio::<ZPol>{
            first: self.mulpol(&duo.first, &monicifier),
            second: self.mulpol(&duo.second, &monicifier),
            third: reg_r.second,
        }
    }

    pub fn unwrapped_bezoutpol (&self, f: &ZPol, g: &ZPol) -> (ZPol, ZPol, ZPol){
        let duo = Duo::<ZPol> {first: f.clone(), second: g.clone()};
        let trio = self.bezoutpol(&duo);
        (trio.first, trio.second, trio.third)
    }
}