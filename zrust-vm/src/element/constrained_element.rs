use bellman::{Variable, ConstraintSystem};
use bellman::pairing::Engine;
use ff::{Field, PrimeField};
use franklin_crypto::bellman::{SynthesisError, Namespace};
use crate::element::{ElementOperator, Element, utils};
use crate::RuntimeError;
use std::marker::PhantomData;
use num_bigint::{BigInt, ToBigInt};
use num_integer::Integer;
use std::fmt::{Debug, Display, Formatter, Error};
use franklin_crypto::circuit::num::AllocatedNum;

/// ConstrainedElement is an implementation of Element
/// that for every operation on elements generates corresponding R1CS constraints.
#[derive(Debug, Clone)]
pub struct ConstrainedElement<E: Engine> {
    value: Option<E::Fr>,
    variable: Variable,
}

impl <E: Engine> Display for ConstrainedElement<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self.value {
            Some(value) => {
                let bigint = utils::fr_to_bigint::<E>(value);
                Display::fmt(&bigint, f)
            },
            None => Display::fmt("none", f)
        }
    }
}

impl <E: Engine> ToBigInt for ConstrainedElement<E> {
    fn to_bigint(&self) -> Option<BigInt> {
        self.value.map(|fr| -> BigInt { utils::fr_to_bigint::<E>(&fr) })
    }
}

impl <EN: Debug + Engine> Element for ConstrainedElement<EN> {}

pub struct ConstrainedElementOperator<E, CS>
where
    E: Engine,
    CS: ConstraintSystem<E>
{
    cs: CS,
    counter: usize,
    pd: PhantomData<E>,
}

impl <E, CS> ConstrainedElementOperator<E, CS>
    where
        E: Engine,
        CS: ConstraintSystem<E>
{
    pub fn new(cs: CS) -> Self {
        Self {
            cs,
            counter: 0,
            pd: PhantomData
        }
    }

    fn cs_namespace(&mut self) -> Namespace<E, CS::Root> {
        let s = format!("{}", self.counter);
        self.counter += 1;
        self.cs.namespace(|| s)
    }

    fn one() -> ConstrainedElement<E> {
        ConstrainedElement { value: Some(E::Fr::one()), variable: CS::one() }
    }
}

impl <E, CS> ElementOperator<ConstrainedElement<E>> for ConstrainedElementOperator<E, CS>
where
    E: Debug + Engine,
    CS: ConstraintSystem<E>
{
    fn constant_u64(&mut self, value: u64) -> Result<ConstrainedElement<E>, RuntimeError> {
        let val = E::Fr::from_str(&value.to_string()).ok_or(RuntimeError::InternalError)?;

        let mut cs = self.cs_namespace();

        let var = cs.alloc(
            || "constant value",
            || Ok(val))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "constant constraint",
            |lc| lc + CS::one(),
            |lc| lc + (val, CS::one()),
            |lc| lc + var,
        );

        Ok(ConstrainedElement {
            value: Some(val),
            variable: var
        })
    }

    fn constant_bigint(&mut self, value: &BigInt) -> Result<ConstrainedElement<E>, RuntimeError> {
        let value = utils::bigint_to_fr::<E>(value).ok_or(RuntimeError::InternalError)?;

        let mut cs = self.cs_namespace();

        let variable = cs.alloc(
            || "constant value",
            || Ok(value))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "constant equation",
            |lc| lc + CS::one(),
            |lc| lc + (value, CS::one()),
            |lc| lc + variable,
        );

        Ok(ConstrainedElement { value: Some(value), variable })
    }

    fn add(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>)
        -> Result<ConstrainedElement<E>, RuntimeError>
    {
        let sum = match (left.value, right.value) {
            (Some(l), Some(r)) => {
                let mut sum = l;
                sum.add_assign(&r);
                Some(sum)
            }
            _ => None
        };

        let mut cs = self.cs_namespace();

        let sum_var = cs.alloc(
            || "sum variable",
            || sum.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "sum constraint",
            |lc| lc + left.variable + right.variable,
            |lc| lc + CS::one(),
            |lc| lc + sum_var,
        );

        Ok(ConstrainedElement {
            value: sum,
            variable: sum_var,
        })
    }

    fn sub(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let diff = match (left.value, right.value) {
            (Some(l), Some(r)) => {
                let mut diff = l;
                diff.sub_assign(&r);
                Some(diff)
            }
            _ => None
        };

        let mut cs = self.cs_namespace();

        let sum_var = cs.alloc(
            || "diff variable",
            || diff.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "diff constraint",
            |lc| lc + left.variable - right.variable,
            |lc| lc + CS::one(),
            |lc| lc + sum_var,
        );

        Ok(ConstrainedElement {
            value: diff,
            variable: sum_var,
        })
    }

    fn mul(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let prod = match (left.value, right.value) {
            (Some(l), Some(r)) => {
                let mut prod = l;
                prod.mul_assign(&r);
                Some(prod)
            }
            _ => None
        };

        let mut cs = self.cs_namespace();

        let sum_var = cs.alloc(
            || "prod variable",
            || prod.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "prod constraint",
            |lc| lc + left.variable,
            |lc| lc + right.variable,
            |lc| lc + sum_var,
        );

        Ok(ConstrainedElement {
            value: prod,
            variable: sum_var,
        })
    }

    fn div_rem(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>)
        -> Result<(ConstrainedElement<E>, ConstrainedElement<E>), RuntimeError>
    {
        let nominator = left;
        let denominator = right;

        let mut quotient: Option<E::Fr> = None;
        let mut remainder: Option<E::Fr> = None;

        if let (Some(nom), Some(denom)) = (nominator.value, denominator.value) {
            let nom_bi = utils::fr_to_bigint::<E>(&nom);
            let denom_bi = utils::fr_to_bigint::<E>(&denom);

            let (q, r) = nom_bi.div_rem(&denom_bi);

            quotient = utils::bigint_to_fr::<E>(&q);
            remainder = utils::bigint_to_fr::<E>(&r);
        }

        let mut cs = self.cs_namespace();

        let qutioent_var = cs.alloc(
            || "qutioent",
            || quotient.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        let remainder_var = cs.alloc(
            || "remainder",
            || remainder.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + qutioent_var,
            |lc| lc + denominator.variable,
            |lc| lc + nominator.variable - remainder_var
        );

        // TODO: add constraint `rem < denom`

        Ok((
            ConstrainedElement { value: quotient, variable: qutioent_var },
            ConstrainedElement { value: remainder, variable: remainder_var }
        ))
    }

    fn neg(&mut self, element: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let neg_value = match element.value {
            Some(value) => {
                let mut neg = E::Fr::zero();
                neg.sub_assign(&value);
                Some(neg)
            }
            _ => None
        };

        let mut cs = self.cs_namespace();

        let neg_variable = cs.alloc(
            || "neg variable",
            || neg_value.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "neg constraint",
            |lc| lc + element.variable,
            |lc| lc + CS::one(),
            |lc| lc - neg_variable,
        );

        Ok(ConstrainedElement {
            value: neg_value,
            variable: neg_variable,
        })
    }

    fn not(&mut self, element: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let one = Self::one();
        self.sub(one, element)
    }

    fn and(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let value = match (left.value, right.value) {
            (Some(a), Some(b)) => {
                let mut conj = a;
                conj.mul_assign(&b);
                Some(conj)
            }
            _ => None
        };

        let variable = cs.alloc(
            || "and",
            || value.ok_or(SynthesisError::AssignmentMissing)
        ).map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + left.variable,
            |lc| lc + right.variable,
            |lc| lc + variable
        );

        Ok(ConstrainedElement { value, variable })
    }

    fn or(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let value = match (left.value, right.value) {
            (Some(a), Some(b)) => {
                if a.is_zero() && b.is_zero() {
                    Some(E::Fr::zero())
                } else {
                    Some(E::Fr::one())
                }
            }
            _ => None
        };

        let variable = cs.alloc(
            || "or",
            || value.ok_or(SynthesisError::AssignmentMissing)
        ).map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + CS::one() - left.variable,
            |lc| lc + CS::one() - right.variable,
            |lc| lc + CS::one() - variable
        );

        Ok(ConstrainedElement { value, variable })
    }

    fn xor(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let value = match (left.value, right.value) {
            (Some(a), Some(b)) => {
                if a.is_zero() == b.is_zero() {
                    Some(E::Fr::zero())
                } else {
                    Some(E::Fr::one())
                }
            }
            _ => None
        };

        let variable = cs.alloc(
            || "conjunction",
            || value.ok_or(SynthesisError::AssignmentMissing)
        ).map_err(|_| RuntimeError::SynthesisError)?;

        // (a + a) * (b) = (a + b - c)
        cs.enforce(
            || "equality",
            |lc| lc + left.variable + left.variable,
            |lc| lc + right.variable,
            |lc| lc + left.variable + right.variable - variable
        );

        Ok(ConstrainedElement { value, variable })
    }

    fn lt(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let one = Self::one();
        let right_minus_one = self.sub(right, one)?;
        self.le(left, right_minus_one)
    }

    fn le(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let diff = self.sub(right, left)?;

        let mut cs = self.cs_namespace();

        let diff_num = AllocatedNum::alloc(
            cs.namespace(|| "diff_num variable"),
            || diff.value.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "allocated_num equality",
            |lc| lc + diff.variable,
            |lc| lc + CS::one(),
            |lc| lc + diff_num.get_variable(),
        );

        let bits = diff_num.into_bits_le_fixed(cs.namespace(|| "diff_num bits"), 32)
            .map_err(|_| RuntimeError::SynthesisError)?;

        let diff_num_repacked = AllocatedNum::pack_bits_to_element(
            cs.namespace(|| "diff_num_repacked"),
            bits.as_slice())
            .map_err(|_| RuntimeError::SynthesisError)?;

        let lt = AllocatedNum::equals(
            cs.namespace(|| "equals"),
            &diff_num, &diff_num_repacked)
            .map_err(|_| RuntimeError::SynthesisError)?;

        Ok(ConstrainedElement {
            value: lt.get_value_field::<E>(),
            variable: lt.get_variable(),
        })
    }

    fn eq(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let l_num = AllocatedNum::alloc(
            cs.namespace(|| "l_num"),
            || left.value.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        let r_num = AllocatedNum::alloc(
            cs.namespace(|| "r_num"),
            || right.value.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        let eq = AllocatedNum::equals(cs, &l_num, &r_num)
            .map_err(|_| RuntimeError::SynthesisError)?;

        Ok(ConstrainedElement {
            value: eq.get_value_field::<E>(),
            variable: eq.get_variable(),
        })
    }

    fn ne(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let eq = self.eq(left, right)?;
        self.not(eq)
    }

    fn ge(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let not_ge = self.lt(left, right)?;
        self.not(not_ge)
    }

    fn gt(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let not_gt = self.le(left, right)?;
        self.not(not_gt)
    }

    fn conditional_select(&mut self, condition: ConstrainedElement<E>, if_true: ConstrainedElement<E>, if_false: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let value = match condition.value {
            Some(value) => {
                if !value.is_zero() {
                    if_true.value
                } else {
                    if_false.value
                }
            },
            None => None
        };

        let variable = cs.alloc(
            || "variable",
            || value.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        // Selected, Right, Left, Condition
        // s = r + c * (l - r)
        // (l - r) * (c) = (s - r)
        cs.enforce(
            || "constraint",
            |lc| lc + if_true.variable - if_false.variable,
            |lc| lc + condition.variable,
            |lc| lc + variable - if_false.variable,
        );

        Ok(ConstrainedElement { value, variable })
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use franklin_crypto::circuit::test::TestConstraintSystem;
    use bellman::pairing::bn256::Bn256;

    #[test]
    fn test_constrained_element() {
        let cs = TestConstraintSystem::<Bn256>::new();
        let mut operator = ConstrainedElementOperator::new(cs);
        let a = operator.constant_u64(42).unwrap();
        let b = operator.constant_u64(7).unwrap();
        let _ = operator.add(a, b).unwrap();
    }
}
