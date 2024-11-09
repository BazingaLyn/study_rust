use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul};
use std::process::Output;
use crate::vector::Vector;
use anyhow::{anyhow, Error, Result};

const NUM_THREADS: usize = 4;

pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}


pub struct MsgInput<T> {

  idx:usize,
  row:Vector<T>,
  col:Vector<T>,
}

pub struct MsgOutput<T> {
    idx:usize,
    value:T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender:oneshot::Sender<MsgOutput<T>>,
}


pub fn multiply<T>(a:&Matrix<T>, b:&Matrix<T>) -> Result<Matrix<T>>
where T:Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static
{
    if a.cols != b.rows {
        return Err(anyhow!("Matrix dimensions do not match"));
    }


}


