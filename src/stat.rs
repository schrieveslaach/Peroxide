pub use matrix::*;
pub use vector_macro::*;

pub trait Statistics {
    type Array;
    type Value;

    fn mean(&self) -> Self::Value;
    fn var(&self) -> Self::Value;
    fn sd(&self) -> Self::Value;
    fn cov(&self) -> Self::Array;
}

impl Statistics for Vector {
    type Array = Vector;
    type Value = f64;

    /// Mean
    ///
    /// # Examples
    /// ```
    /// extern crate peroxide;
    /// use peroxide::*;
    ///
    /// let a = c!(1,2,3,4,5);
    /// assert_eq!(a.mean(), 3.0);
    /// ```
    fn mean(&self) -> f64 {
        self.reduce(0f64, |x, y| x + y) / (self.len() as f64)
    }

    /// Variance
    ///
    /// # Examples
    /// ```
    /// extern crate peroxide;
    /// use peroxide::*;
    ///
    /// let a = c!(1,2,3,4,5);
    /// assert_eq!(a.var(), 2.5);
    /// ```
    fn var(&self) -> f64 {
        let mut ss = 0f64;
        let mut s = 0f64;
        let mut l = 0f64;

        for x in self.into_iter() {
            ss += x.powf(2f64);
            s += x;
            l += 1f64;
        }
        assert_ne!(l, 1f64);
        (ss / l - (s / l).powf(2f64)) * l / (l - 1f64)
    }

    /// Standard Deviation
    ///
    /// # Examples
    /// ```
    /// extern crate peroxide;
    /// use peroxide::*;
    ///
    /// let a = c!(1,2,3);
    /// assert!(nearly_eq(a.sd(), 1f64)); // Floating Number Error
    /// ```
    fn sd(&self) -> f64 {
        self.var().sqrt()
    }

    fn cov(&self) -> Vector {
        unimplemented!()
    }
}

impl Statistics for Matrix {
    type Array = Matrix;
    type Value = Vector;

    /// Column Mean
    ///
    /// # Examples
    /// ```
    /// extern crate peroxide;
    /// use peroxide::*;
    ///
    /// let m = matrix(c!(1,3,3,1), 2, 2, Col);
    /// assert_eq!(m.mean(), c!(2,2));
    /// ```
    fn mean(&self) -> Vector {
        let mut container: Vector = Vec::new();
        let c = self.col;

        for i in 0 .. c {
            container.push(self.col(i).mean());
        }
        container
    }

    fn var(&self) -> Vector {
        let mut container: Vector = Vec::new();
        let c = self.col;

        for i in 0 .. c {
            container.push(self.col(i).var());
        }
        container
    }

    fn sd(&self) -> Vector {
        let mut container: Vector = Vec::new();
        let c = self.col;

        for i in 0 .. c {
            container.push(self.col(i).sd());
        }
        container
    }

    fn cov(&self) -> Matrix {
        let c = self.col;

        let mut m: Vector = Vec::new();

        for i in 0 .. c {
            let m1 = self.col(i);
            for j in 0 .. c {
                let m2 = self.col(j);
                m.push(cov(m1.clone(), m2));
            }
        }
        matrix(m, c, c, Row)
    }
}

/// Covariance (to Value)
///
/// # Examples
/// ```
/// extern crate peroxide;
/// use peroxide::*;
///
/// let v1 = c!(1,2,3);
/// let v2 = c!(3,2,1);
/// assert!(nearly_eq(cov(v1,v2), -1f64));
/// ```
pub fn cov(v1: Vector, v2: Vector) -> f64 {
    let mut ss = 0f64;
    let mut sx = 0f64;
    let mut sy = 0f64;
    let mut l = 0f64;

    for (x, y) in v1.into_iter().zip(&v2) {
        ss += x * y;
        sx += x;
        sy += y;
        l += 1f64;
    }
    assert_ne!(l, 1f64);
    (ss / l - (sx * sy) / (l.powf(2f64))) * l / (l - 1f64)
}
