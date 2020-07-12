extern crate proc_macro;
use proc_macro::TokenStream;

/// Maximum order for Taylor mode AD
const N: usize = 10;

#[proc_macro]
pub fn ad_struct_def(_item: TokenStream) -> TokenStream {
    let mut total = "".to_string();
    for i in 1 .. N+1 {
        let mut body = "pub d0: f64,\n".to_string();
        for j in 1 .. i+1 {
            body.push_str(&format!("pub d{}: f64,\n", j));
        }
        let one = format!("#[derive(Debug, Copy, Clone, PartialEq, Default)]
        pub struct AD{} {{
            {}
        }}\n", i, body);
        total.push_str(&one);
    }
    total.parse().unwrap()
}

#[proc_macro]
pub fn ad_display(_item: TokenStream) -> TokenStream {
    let mut total = "".to_string();
    for i in 1 .. N+1 {
        let mut body = "".to_string();
        for j in 1 .. i+1 {
            body.push_str(&format!("s.push_str(&format!(\"  d{}: {{}}\n\", self.d{}));", j, j));
        }
        let one = format!("impl std::fmt::Display for AD{} {{
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{
                let mut s = format!(\"AD\n  d0: {{}}\n\", self.d0);
                {}
                write!(f, \"{{}}\", s)
            }}
        }}\n", i, body);
        total.push_str(&one);
    }
    total.parse().unwrap()
}

#[proc_macro]
pub fn ad_impl(_item: TokenStream) -> TokenStream {
    let mut total = "".to_string();
    for i in 1 .. N+1 {
        let mut arg = "d0: f64, ".to_string();
        let mut body = "d0,\n".to_string();
        for j in 1 .. i {
            arg.push_str(&format!("d{}: f64, ", j));
            body.push_str(&format!("d{},\n", j));
        }
        arg.push_str(&format!("d{}: f64", i));
        body.push_str(&format!("d{}", i));

        let one = format!("impl AD{} {{
            pub fn new({}) -> Self {{
                Self {{
                    {}
                }}
            }}

            pub fn print(&self) {{
                println!(\"{{}}\", self);
            }}

            pub fn iter(&self) -> ADIter{} {{
                self.into_iter()
            }}

            pub fn iter_mut(&mut self) -> ADIterMut{} {{
                self.into_iter()
            }}

            pub fn len(&self) -> usize {{
                {}
            }}
        }}", i, arg, body, i, i, i+1);
        total.push_str(&one);
    }

    total.parse().unwrap()
}

#[proc_macro]
pub fn ad_impl_from(_item: TokenStream) -> TokenStream {
    let mut total = "".to_string();
    for i in 1 .. N+1 {
        for j in 1 .. i {
            let mut body = "high.d0, ".to_string();
            for k in 1 .. j {
                body.push_str(&format!("high.d{}, ", k));
            }
            body.push_str(&format!("high.d{}", j));
            // i -> j (i > j)
            let one = format!("impl From<AD{}> for AD{} {{
                fn from(high: AD{}) -> Self {{
                    Self::new({})
                }}
            }}\n", i, j, i, body);
            total.push_str(&one);
        }
        for j in i+1 .. N+1 {
            let mut body = "low.d0, ".to_string();
            for k in 1 .. i+1 {
                body.push_str(&format!("low.d{}, ", k));
            }
            for _k in i+1 .. j {
                body.push_str("0f64, ");
            }
            body.push_str("0f64");
            // i -> j (i <= j)
            let one = format!("impl From<AD{}> for AD{} {{
                fn from(low: AD{}) -> Self {{
                    Self::new({})
                }}
            }}\n", i, j, i, body);
            total.push_str(&one);
        }
    }
    total.parse().unwrap()
}

#[proc_macro]
pub fn ad_iter_def(_item: TokenStream) -> TokenStream {
    let mut total = "".to_string();
    for i in 1 .. N+1 {
        let one = format!("pub struct ADIntoIter{} {{
            ad: AD{},
            index: usize,
            r_index: usize,
        }}\n", i, i);
        let two = format!("pub struct ADIter{}<'a> {{
            ad: &'a AD{},
            index: usize,
            r_index: usize,
        }}\n", i, i);
        let three = format!("pub struct ADIterMut{}<'a> {{
            ad: &'a mut AD{},
            index: usize,
            r_index: usize,
        }}\n", i, i);
        total.push_str(&one);
        total.push_str(&two);
        total.push_str(&three);
    }
    total.parse().unwrap()
}

#[proc_macro]
pub fn ad_impl_into_iter(_item: TokenStream) -> TokenStream {
    let mut total = "".to_string();
    for i in 1 .. N+1 {
        let one = format!("impl IntoIterator for AD{} {{
            type Item = f64;
            type IntoIter = ADIntoIter{};

            fn into_iter(self) -> Self::IntoIter {{
                ADIntoIter{} {{
                    ad: self,
                    index: 0,
                    r_index: 0,
                }}
            }}
        }}\n", i, i, i);
        let two = format!("impl<'a> IntoIterator for &'a AD{} {{
            type Item = f64;
            type IntoIter = ADIter{}<'a>;

            fn into_iter(self) -> Self::IntoIter {{
                ADIter{} {{
                    ad: self,
                    index: 0,
                    r_index: 0,
                }}
            }}
        }}\n", i, i, i);
        let three = format!("impl<'a> IntoIterator for &'a mut AD{} {{
            type Item = f64;
            type IntoIter = ADIterMut{}<'a>;

            fn into_iter(self) -> Self::IntoIter {{
                ADIterMut{} {{
                    ad: self,
                    index: 0,
                    r_index: 0,
                }}
            }}
        }}\n", i, i, i);
        total.push_str(&one);
        total.push_str(&two);
        total.push_str(&three);
    }
    total.parse().unwrap()
}

#[proc_macro]
pub fn ad_impl_iter(_item: TokenStream) -> TokenStream {
    let mut total = "".to_string();
    for i in 1 .. N+1 {
        let mut body = "0 => self.ad.d0,\n".to_string();
        for j in 1 .. i+1 {
            body.push_str(&format!("{} => self.ad.d{},\n", j, j));
        }
        body.push_str("_ => return None,");
        let one = format!("impl Iterator for ADIntoIter{} {{
            type Item = f64;
            fn next(&mut self) -> Option<Self::Item> {{
                if self.index + self.r_index == {} {{
                    return None;
                }}
                let result = match self.index {{
                    {}
                }};
                self.index += 1;
                Some(result)
            }}
        }}\n", i, i, body);
        let two = format!("impl<'a> Iterator for ADIter{}<'a> {{
            type Item = f64;
            fn next(&mut self) -> Option<Self::Item> {{
                if self.index + self.r_index == {} {{
                    return None;
                }}
                let result = match self.index {{
                    {}
                }};
                self.index += 1;
                Some(result)
            }}
        }}\n", i, i, body);
        let three = format!("impl<'a> Iterator for ADIterMut{}<'a> {{
            type Item = f64;
            fn next(&mut self) -> Option<Self::Item> {{
                if self.index + self.r_index == {} {{
                    return None;
                }}
                let result = match self.index {{
                    {}
                }};
                self.index += 1;
                Some(result)
            }}
        }}\n", i, i, body);
        total.push_str(&one);
        total.push_str(&two);
        total.push_str(&three);
    }
    total.parse().unwrap()
}

#[proc_macro]
pub fn ad_impl_double_ended_iter(__item: TokenStream) -> TokenStream {
    let mut total = "".to_string();
    for i in 1 .. N+1 {
        let mut body = format!("0 => self.ad.d{},\n", i);
        for j in 1 .. i+1 {
            body.push_str(&format!("{} => self.ad.d{},\n", j, i-j));
        }
        body.push_str("_ => return None,");
        let one = format!("impl DoubleEndedIterator for ADIntoIter{} {{
            fn next_back(&mut self) -> Option<Self::Item> {{
                if self.index + self.r_index == {} {{
                    return None;
                }}
                let result = match self.r_index {{
                    {}
                }};
                self.r_index += 1;
                Some(result)
            }}
        }}\n", i, i+1, body);
        let two = format!("impl<'a> DoubleEndedIterator for ADIter{}<'a> {{
            fn next_back(&mut self) -> Option<Self::Item> {{
                if self.index + self.r_index == {} {{
                    return None;
                }}
                let result = match self.r_index {{
                    {}
                }};
                self.r_index += 1;
                Some(result)
            }}
        }}\n", i, i+1, body);
        let three = format!("impl<'a> DoubleEndedIterator for ADIterMut{}<'a> {{
            fn next_back(&mut self) -> Option<Self::Item> {{
                if self.index + self.r_index == {} {{
                    return None;
                }}
                let result = match self.r_index {{
                    {}
                }};
                self.r_index += 1;
                Some(result)
            }}
        }}\n", i, i+1, body);
        total.push_str(&one);
        total.push_str(&two);
        total.push_str(&three);
    }
    total.parse().unwrap()
}

#[proc_macro]
pub fn ad_impl_exact_size_iter(_item: TokenStream) -> TokenStream {
    let mut total = "".to_string();
    for i in 1 .. N+1 {
        let one = format!("impl ExactSizeIterator for ADIntoIter{} {{
            fn len(&self) -> usize {{
                self.ad.len() - (self.index + self.r_index)
            }}
        }}\n", i);
        let two = format!("impl<'a> ExactSizeIterator for ADIter{}<'a> {{
            fn len(&self) -> usize {{
                self.ad.len() - (self.index + self.r_index)
            }}
        }}\n", i);
        let three = format!("impl<'a> ExactSizeIterator for ADIterMut{}<'a> {{
            fn len(&self) -> usize {{
                self.ad.len() - (self.index + self.r_index)
            }}
        }}\n", i);
        total.push_str(&one);
        total.push_str(&two);
        total.push_str(&three);
    }
    total.parse().unwrap()
}

#[proc_macro]
pub fn ad_impl_index(_item: TokenStream) -> TokenStream {
    let mut total = "".to_string();
    for i in 1 .. N+1 {
        let mut body1 = "0 => &self.d0,\n".to_string();
        let mut body2 = "0 => &mut self.d0,\n".to_string();
        for j in 1 .. i+1 {
            body1.push_str(&format!("{} => &self.d{},\n", j, j));
            body2.push_str(&format!("{} => &mut self.d{},\n", j, j));
        }
        let one = format!("impl Index<usize> for AD{} {{
            type Output = f64;

            fn index(&self, n: usize) -> &Self::Output {{
                match n {{
                    {}
                    _ => panic!(\"{{}} exceed order of AD{}\", n),
                }}
            }}
        }}\n", i, body1, i);
        let two = format!("impl IndexMut<usize> for AD{} {{
            fn index_mut(&mut self, n: usize) -> &mut Self::Output {{
                match n {{
                    {}
                    _ => panic!(\"{{}} exceed order of AD{}\", n),
                }}
            }}
        }}\n", i, body2, i);
        total.push_str(&one);
        total.push_str(&two);
    }
    total.parse().unwrap()
}

#[proc_macro]
pub fn ad_impl_neg(_item: TokenStream) -> TokenStream {
    let mut total = "".to_string();
    for i in 1 .. N+1 {
        let mut body = "-self.d0, ".to_string();
        for j in 1 .. i {
            body.push_str(&format!("-self.d{}, ", j));
        }
        body.push_str(&format!("-self.d{}", i));
        let one = format!("impl Neg for AD{} {{
            type Output = Self;
            fn neg(self) -> Self::Output {{
                AD{}::new({})
            }}
        }}\n", i, i, body);
        total.push_str(&one);
    }
    total.parse().unwrap()
}

#[proc_macro]
pub fn ad_impl_add(_item: TokenStream) -> TokenStream {
    let mut total = "".to_string();
    for i in 1 .. N+1 {
        for j in 1 .. i {
            let mut body = "".to_string();
            for k in 0 .. j+1 {
                body.push_str(&format!("z.d{} += rhs.d{};\n", k, k));
            }
            let one = format!("impl Add<AD{}> for AD{} {{
                type Output = AD{};
                
                fn add(self, rhs: AD{}) -> Self::Output {{
                    let mut z = self.clone();
                    {}
                    z
                }}
            }}\n", j, i, i, j, body);
            total.push_str(&one);
        }
        for j in i .. N+1 {
            let mut body = "".to_string();
            for k in 0 .. i+1 {
                body.push_str(&format!("z.d{} += self.d{};\n", k, k));
            }
            let one = format!("impl Add<AD{}> for AD{} {{
                type Output = AD{};

                fn add(self, rhs: AD{}) -> Self::Output {{
                    let mut z = rhs.clone();
                    {}
                    z
                }}
            }}\n", j, i, j, j, body);
            total.push_str(&one);
        }
    }
    total.parse().unwrap()
}

#[proc_macro]
pub fn ad_impl_sub(_item: TokenStream) -> TokenStream {
    let mut total = "".to_string();
    for i in 1 .. N+1 {
        for j in 1 .. i {
            let mut body = "".to_string();
            for k in 0 .. j+1 {
                body.push_str(&format!("z.d{} -= rhs.d{};\n", k, k));
            }
            let one = format!("impl Sub<AD{}> for AD{} {{
                type Output = AD{};
                
                fn sub(self, rhs: AD{}) -> Self::Output {{
                    let mut z = self.clone();
                    {}
                    z
                }}
            }}\n", j, i, i, j, body);
            total.push_str(&one);
        }
        for j in i .. N+1 {
            let mut body = "".to_string();
            for k in 0 .. i+1 {
                body.push_str(&format!("z.d{} += self.d{};\n", k, k));
            }
            let one = format!("impl Sub<AD{}> for AD{} {{
                type Output = AD{};

                fn sub(self, rhs: AD{}) -> Self::Output {{
                    let mut z = -rhs.clone();
                    {}
                    z
                }}
            }}\n", j, i, j, j, body);
            total.push_str(&one);
        }
    }
    total.parse().unwrap()
}

#[proc_macro]
pub fn ad_impl_mul(_item: TokenStream) -> TokenStream {
    let mut total = "".to_string();
    for i in 1 .. N+1 {
        for j in 1 .. i+1 {
            let one = format!("impl Mul<AD{}> for AD{} {{
                type Output = AD{};

                fn mul(self, rhs: AD{}) -> Self::Output {{
                    let mut z = self.clone();
                    let y = Self::Output::from(rhs);
                    for t in 0 .. z.len() {{
                        z[t] = self.iter()
                                .take(t + 1)
                                .zip(y.iter().take(t+1).rev())
                                .enumerate()
                                .fold(0f64, |s, (k, (x1, y1))| s + (C(t, k) as f64) * x1 * y1)
                    }}
                    z
                }}
            }}\n", j, i, i, j);
            total.push_str(&one);
        }
        for j in i+1 .. N+1 {
            let one = format!("impl Mul<AD{}> for AD{} {{
                type Output = AD{};

                fn mul(self, rhs: AD{}) -> Self::Output {{
                    let mut z = rhs.clone();
                    let x = Self::Output::from(self);
                    for t in 0 .. z.len() {{
                        z[t] = x.iter()
                                .take(t + 1)
                                .zip(rhs.iter().take(t+1).rev())
                                .enumerate()
                                .fold(0f64, |s, (k, (x1, y1))| s + (C(t, k) as f64) * x1 * y1)
                    }}
                    z
                }}
            }}\n", j, i, j, j);
            total.push_str(&one);
        }
    }
    total.parse().unwrap()
}

//fn factorial(n: usize) -> usize {
//    let mut p = 1usize;
//    for i in 1..(n + 1) {
//        p *= i;
//    }
//    p
//}
//
//#[allow(non_snake_case)]
//fn P(n: usize, r: usize) -> usize {
//    let mut p = 1usize;
//    for i in 0..r {
//        p *= n - i;
//    }
//    p
//}
//
//#[allow(non_snake_case)]
//fn C(n: usize, r: usize) -> usize {
//    if r > n / 2 {
//        return C(n, n - r);
//    }
//
//    P(n, r) / factorial(r)
//}