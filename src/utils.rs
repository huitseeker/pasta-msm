//! # Sparse Matrices
//!
//! This module defines a custom implementation of CSR/CSC sparse matrices.
//! Specifically, we implement sparse matrix / dense vector multiplication
//! to compute the `A z`, `B z`, and `C z` in Nova.

use std::{sync::atomic::{AtomicUsize, Ordering}, mem::transmute, cell::UnsafeCell};

use abomonation_derive::Abomonation;
use abomonation::Abomonation;
use pasta_curves::{group::{ff::{PrimeField, Field}, Curve}, arithmetic::{CurveAffine, CurveExt}, pallas};
use rand::{SeedableRng, RngCore};
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;


/// CSR format sparse matrix, We follow the names used by scipy.
/// Detailed explanation here: <https://stackoverflow.com/questions/52299420/scipy-csr-matrix-understand-indptr>
#[derive(Debug, PartialEq, Eq, Abomonation)]
#[abomonation_bounds(where <F as PrimeField>::Repr: Abomonation)]
pub struct SparseMatrix<F: PrimeField> {
  /// all non-zero values in the matrix
  #[abomonate_with(Vec<F::Repr>)]
  pub data: Vec<F>,
  /// column indices
  pub indices: Vec<usize>,
  /// row information
  pub indptr: Vec<usize>,
  /// number of columns
  pub cols: usize,
}

/// [SparseMatrix]s are often large, and this helps with cloning bottlenecks
impl<F: PrimeField> Clone for SparseMatrix<F> {
  fn clone(&self) -> Self {
    Self {
      data: self.data.par_iter().cloned().collect(),
      indices: self.indices.par_iter().cloned().collect(),
      indptr: self.indptr.par_iter().cloned().collect(),
      cols: self.cols,
    }
  }
}

impl<F: PrimeField> SparseMatrix<F> {
  /// 0x0 empty matrix
  pub fn empty() -> Self {
    SparseMatrix {
      data: vec![],
      indices: vec![],
      indptr: vec![0],
      cols: 0,
    }
  }

  /// Construct from the COO representation; Vec<usize(row), usize(col), F>.
  /// We assume that the rows are sorted during construction.
  pub fn new(matrix: &[(usize, usize, F)], rows: usize, cols: usize) -> Self {
    let mut new_matrix = vec![vec![]; rows];
    for (row, col, val) in matrix {
      new_matrix[*row].push((*col, *val));
    }

    for row in new_matrix.iter() {
      assert!(row.windows(2).all(|w| w[0].0 < w[1].0));
    }

    let mut indptr = vec![0; rows + 1];
    for (i, col) in new_matrix.iter().enumerate() {
      indptr[i + 1] = indptr[i] + col.len();
    }

    let mut indices = vec![];
    let mut data = vec![];
    for col in new_matrix {
      let (idx, val): (Vec<_>, Vec<_>) = col.into_iter().unzip();
      indices.extend(idx);
      data.extend(val);
    }

    SparseMatrix {
      data,
      indices,
      indptr,
      cols,
    }
  }

  /// Retrieves the data for row slice [i..j] from `ptrs`.
  /// We assume that `ptrs` is indexed from `indptrs` and do not check if the
  /// returned slice is actually a valid row.
  pub fn get_row_unchecked(&self, ptrs: &[usize; 2]) -> impl Iterator<Item = (&F, &usize)> {
    self.data[ptrs[0]..ptrs[1]]
      .iter()
      .zip(&self.indices[ptrs[0]..ptrs[1]])
  }

  /// Multiply by a dense vector; uses rayon to parallelize.
  pub fn multiply_vec(&self, vector: &[F]) -> Vec<F> {
    assert_eq!(self.cols, vector.len(), "invalid shape");

    self.multiply_vec_unchecked(vector)
  }

  /// Multiply by a dense vector; uses rayon to parallelize.
  /// This does not check that the shape of the matrix/vector are compatible.
  pub fn multiply_vec_unchecked(&self, vector: &[F]) -> Vec<F> {
    self
      .indptr
      .par_windows(2)
      .map(|ptrs| {
        self
          .get_row_unchecked(ptrs.try_into().unwrap())
          .map(|(val, col_idx)| *val * vector[*col_idx])
          .sum()
      })
      .collect()
  }

  /// number of non-zero entries
  pub fn len(&self) -> usize {
    *self.indptr.last().unwrap()
  }

  /// empty matrix
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// returns a custom iterator
  pub fn iter(&self) -> Iter<'_, F> {
    let mut row = 0;
    while self.indptr[row + 1] == 0 {
      row += 1;
    }
    Iter {
      matrix: self,
      row,
      i: 0,
      nnz: *self.indptr.last().unwrap(),
    }
  }
}

/// Iterator for sparse matrix
pub struct Iter<'a, F: PrimeField> {
  matrix: &'a SparseMatrix<F>,
  row: usize,
  i: usize,
  nnz: usize,
}

impl<'a, F: PrimeField> Iterator for Iter<'a, F> {
  type Item = (usize, usize, F);

  fn next(&mut self) -> Option<Self::Item> {
    // are we at the end?
    if self.i == self.nnz {
      return None;
    }

    // compute current item
    let curr_item = (
      self.row,
      self.matrix.indices[self.i],
      self.matrix.data[self.i],
    );

    // advance the iterator
    self.i += 1;
    // edge case at the end
    if self.i == self.nnz {
      return Some(curr_item);
    }
    // if `i` has moved to next row
    while self.i >= self.matrix.indptr[self.row + 1] {
      self.row += 1;
    }

    Some(curr_item)
  }
}

/// A type that holds commitment generators
#[derive(Clone, Debug, PartialEq, Eq, Abomonation)]
#[abomonation_omit_bounds]
pub struct CommitmentKey<C: CurveAffine>
{
  #[abomonate_with(Vec<[u64; 8]>)] // this is a hack; we just assume the size of the element.
  pub ck: Vec<C>,
}

pub fn gen_points(npoints: usize) -> Vec<pallas::Affine> {
  let mut ret: Vec<pallas::Affine> = Vec::with_capacity(npoints);
  unsafe { ret.set_len(npoints) };

  let mut rnd: Vec<u8> = Vec::with_capacity(32 * npoints);
  unsafe { rnd.set_len(32 * npoints) };
  ChaCha20Rng::from_entropy().fill_bytes(&mut rnd);

  let n_workers = rayon::current_num_threads();
  let work = AtomicUsize::new(0);
  rayon::scope(|s| {
      for _ in 0..n_workers {
          s.spawn(|_| {
          let hash = pallas::Point::hash_to_curve("foobar");

          let mut stride = 1024;
          let mut tmp: Vec<pallas::Point> = Vec::with_capacity(stride);
          unsafe { tmp.set_len(stride) };

          loop {
              let work = work.fetch_add(stride, Ordering::Relaxed);
              if work >= npoints {
                  break;
              }
              if work + stride > npoints {
                  stride = npoints - work;
                  unsafe { tmp.set_len(stride) };
              }
              for i in 0..stride {
                  let off = (work + i) * 32;
                  tmp[i] = hash(&rnd[off..off + 32]);
              }
              #[allow(mutable_transmutes)]
              pallas::Point::batch_normalize(&tmp, unsafe {
                  transmute::<&[pallas::Affine], &mut [pallas::Affine]>(
                      &ret[work..work + stride],
                  )
              });
          }
      })
      }
  });

  ret
}

fn as_mut<T>(x: &T) -> &mut T {
  unsafe { &mut *UnsafeCell::raw_get(x as *const _ as *const _) }
}

pub fn gen_scalars(npoints: usize) -> Vec<pallas::Scalar> {
  let mut ret: Vec<pallas::Scalar> = Vec::with_capacity(npoints);
  unsafe { ret.set_len(npoints) };

  let n_workers = rayon::current_num_threads();
  let work = AtomicUsize::new(0);

  rayon::scope(|s| {
      for _ in 0..n_workers {
          s.spawn(|_| {
              let mut rng = ChaCha20Rng::from_entropy();
              loop {
                  let work = work.fetch_add(1, Ordering::Relaxed);
                  if work >= npoints {
                      break;
                  }
                  *as_mut(&ret[work]) = pallas::Scalar::random(&mut rng);
              }
          })
      }
  });

  ret
}