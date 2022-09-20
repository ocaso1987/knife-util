use crate::{Ok, Result};

pub trait CollectResultTrait<T>: Sized
where
    Self: Iterator<Item = Result<T>>,
{
    fn collect_into_vec(self) -> Result<Vec<T>>
    where
        Self: Sized;
}

impl<T, I> CollectResultTrait<T> for I
where
    I: Iterator<Item = Result<T>>,
{
    fn collect_into_vec(self) -> Result<Vec<T>>
    where
        Self: Sized,
    {
        let mut res = vec![];
        for x in self {
            if let std::result::Result::Ok(v) = x {
                res.push(v);
            } else {
                return Err(x.err().unwrap());
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::ERR_INTERNAL, iter::collect_result::CollectResultTrait, types::IntegerExt};

    #[test]
    fn test_fold_result() {
        let arr = [1, 2, 3, 4, 5]
            .iter()
            .map(|i| i.cast_to_i64())
            .collect_into_vec();
        assert_eq!(arr.unwrap().len(), 5);

        let arr2 = [1, 2, 3, 4, 5]
            .iter()
            .map(|i| {
                if *i == 1 {
                    Err(ERR_INTERNAL.msg_detail("failure"))
                } else {
                    i.cast_to_i64()
                }
            })
            .collect_into_vec();
        assert!(arr2.is_err());
    }
}
