use super::*;

impl<T: std::fmt::Display> std::fmt::Display for Duo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.first, self.second)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Trio<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.first, self.second, self.third)
    }
}


// USUALLY used as polynomial constant term
pub trait SignDisplay {
    fn signed_display(&self) -> String;
    fn semi_signed_display(&self) -> String;
}

impl SignDisplay for Rational {
    fn signed_display(&self) -> String {
        let reduced = self.to_pair().to_rational();
        let num = reduced.numerator;
        let den = reduced.denominator;

        // special treatment to 0
        if num == 0 {
            return format!("0");
        }

        // general 
        if num < 0 {
            if den == 1 {
                return format!("- {}", -num);
            } else {
                return format!("- ({}/{})", -num, den);
            }
        }
        else {
            if den == 1 {
                return format!("+ {}", num);
            } else {
                return format!("+ ({}/{})", num, den);
            }
        }
    }

    fn semi_signed_display(&self) -> String {
        let reduced = self.to_pair().to_rational();
        let num = reduced.numerator;
        let den = reduced.denominator;

        // edge case: 0
        if num == 0 {
            return format!("0");
        }

        // general case
        if num < 0 {
            if den == 1 {
                return format!("- {}", -num);
            } else {
                return format!("- ({}/{})", -num, den);
            }
        }
        else {
            if den == 1 {
                return format!("{}", num);
            } else {
                return format!("({}/{})", num, den);
            }
        }
    }
}




// Used as polynomial coefficient before x. 
// Can also used in quotient of polynomial ring. For example Q[i] = Q[x]/(x^2+1).
// Otherwise it looks very strange: -1 -> "- "; and 0 is mapped to an empty string!
pub trait PolCoeffDisplay {
    // This sort of display explicitly contains the plus or minus sign ("+" / "-").
    // "1" is displayed as "+ "; "-1" is displayed as "- "; "0" as "".
    // This behavior is very weird elsewhere, but reasonable for coefficients in polynomial.
    fn pol_signed_display(&self) -> String;

    // almost identical to pol_signed_display, but no plus ("+");
    // It is used in the leading term.
    // Don't use it outside polynomial
    fn pol_semi_signed_display(&self) -> String;
}

impl PolCoeffDisplay for Rational {
    fn pol_signed_display(&self) -> String {
        let reduced = self.to_pair().to_rational();
        let num = reduced.numerator;
        let den = reduced.denominator;

        // special treatment to 0, 1, -1
        if num == 0 {
            return format!("");
        }
        if num == 1 && den == 1 {
            return format!("+ ");
        }
        if num == -1 && den == 1 {
            return format!("- ")
        }
        
        // general 
        if num < 0 {
            if den == 1 {
                return format!("- {}", -num);
            } else {
                return format!("- ({}/{})", -num, den);
            }
        }
        else {
            if den == 1 {
                return format!("+ {}", num);
            } else {
                return format!("+ ({}/{})", num, den);
            }
        }
    }

    fn pol_semi_signed_display(&self) -> String {
        let reduced = self.to_pair().to_rational();
        let num = reduced.numerator;
        let den = reduced.denominator;

        // edge case: 0,1, -1
        if num == 0 {
            return format!("");
        }
        if num == 1 && den == 1 {
            return format!("");
        }
        if num == -1 && den == 1 {
            return format!("- ")
        }

        // general case
        if num < 0 {
            if den == 1 {
                return format!("- {}", -num);
            } else {
                return format!("- ({}/{})", -num, den);
            }
        }
        else {
            if den == 1 {
                return format!("{}", num);
            } else {
                return format!("({}/{})", num, den);
            }
        }
    }
}





// customised displaying trait; it is the primary output format. 
// it's simpler than fmt::Display because it only produces String as output, no streaming stuff.
//
// Principles: No surrounding parentheses or whitespace; As simple as possible; No ambivalence.
// If a number can be reduced (3/6 -> 1/2), then it should be.
pub trait SimpleDisplay {
    fn simple_display(&self) -> String;
}

impl SimpleDisplay for Integer {
    fn simple_display(&self) -> String {
        format!("{}", self.number)
    }
}

impl SimpleDisplay for Rational {
    fn simple_display(&self) -> String {
        let reduced = self.to_pair().to_rational();

        if reduced.denominator == 1 {
            format!("{}", reduced.numerator)
        } else {
            format!("{}/{}", reduced.numerator, reduced.denominator)
        }
    }
}

impl SimpleDisplay for Polynomial<Rational> {
    fn simple_display(&self) -> String {
        type T = Rational;
        let zero = T::zero();

        let reduced = self.reduced();
        let order = self.norm();
        let vec = reduced.vector;

        // special case
        if order == 0 {
            return format!("{}", vec[0].semi_signed_display());
        }
        if order == 1 && vec[0] == zero {
            return format!("{}x", vec[1].pol_semi_signed_display());
        }
        if order == 1 && vec[0] != zero {
            return format!(
                "{}x {}",
                vec[1].pol_semi_signed_display(),
                vec[0].signed_display()
            );
        }

        // following: general case
        let mut i = order;
        let mut output = format!("{}x^{}", vec[i].pol_semi_signed_display(), i);
        i -= 1;

        // non-constant term
        while i >0 {
            if vec[i] == zero {
                i -= 1;
                continue;
            }
            if i >= 2 {
                let str = format!("{}x^{}", vec[i].pol_signed_display(), i);
                output = format!("{output} {str}");
            }
            else {
                let str = format!("{}x", vec[i].pol_signed_display());
                output = format!("{output} {str}");
            }
            i -= 1;
        }

        // constant term
        if vec[i] != zero {
            let str = format!("{}", vec[i].signed_display());
            output = format!("{output} {str}");
        }

        output
    }
}

impl SimpleDisplay for ZPol {
    fn simple_display(&self) -> String {
        self.to_qpol().simple_display()
    }
}

// In essence this is just a polynomial displayed in descending order,
// with "x" replaced by "i", and all whitespace dropped.
// e.g. what's the relationship between 2+3i and 3x+2?
// Answer: They are connected via quotient map Q[x] -> Q[x]/(x^2+1) = Q[i].
// This hint allows us to use SignDisplay and PolCoeffDisplay handily.
impl SimpleDisplay for ComplexRational {
    fn simple_display(&self) -> String {
        // self = 1, -2, 0, -3/7, 2/5, for example
        if self.is_real() {
            self.real.simple_display()
        }

        // self = i, 2i, -3i, (5/6)i, -(7/9)i
        else if self.is_pure_imag() {
            let b = self.imag;
            let res = format!("{} i", b.pol_semi_signed_display());
            res.split_ascii_whitespace().collect()
        }

        // self = 2+3i, (3/5)+2i, -2-(5/6)i
        else {
            let res = format!("{} {} i", self.real.semi_signed_display(), self.imag.pol_signed_display());
            res.split_ascii_whitespace().collect()
        }
    }
}

impl SimpleDisplay for GaussInteger {
    fn simple_display(&self) -> String {
        self.to_complex_rational().simple_display()
    }
}


// Chore: Wrapping a fmt::Display around SimpleDisplay
impl<T: Field> std::fmt::Display for Polynomial<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reduced = self.reduced();
        let order = self.norm();
        let vec = reduced.vector;

        if order == 0 {
            return write!(
                f, "{}", vec[0]
            );
        }
        
        if order == 1 && vec[0] == T::zero() {
           return write!(
                f, "{} x", vec[1]
           );
        }

        if order == 1 && vec[0] != T::zero() {
            return write!(
                f, "( {} x + {} )", vec[1], vec[0]
            )
        }

        let mut i = order;
        let mut output = format!("( {} x^{}", vec[i], i);
        i -= 1;

        while i >0 {
            if i >= 2 && vec[i] != T::zero(){
                let str = format!("{} x^{}", vec[i], i);
                output = format!("{output} + {str}");
            }
            if i == 1 && vec[i] != T::zero(){
                let str = format!("{} x", vec[i]);
                output = format!("{output} + {str}");
            }
            i -= 1;
        }
        if vec[i] != T::zero(){
            let str = format!("{}", vec[i]);
            output = format!("{output} + {str}")
        }
    
        output = format!("{output} )");

        write!(f, "{}", output)
    }
}

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.simple_display())
    }
}

impl std::fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.simple_display())
    }
}

impl std::fmt::Display for ComplexRational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.simple_display())
    }
}

impl std::fmt::Display for GaussInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.simple_display())
    }
}