#[macro_export]
macro_rules! impl_new {
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn new() -> $T {
                $T($U::default())
            }
        }
    };
}
#[macro_export]
macro_rules! impl_new_x2 {
    ($T:ident,$U:ident) => {
        impl $T {
            pub fn new() -> $T {
                $T($U::default(), $U::default())
            }
        }
    };
}
#[macro_export]
macro_rules! impl_default {
    ($T:ident) => {
        impl $T {
            pub fn default() -> $T {
                $T(0)
            }
        }
        impl Default for $T {
            fn default() -> $T {
                Self::default()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_op {
    ($T:ident,$U:ident,$V:tt,$func:ident,$op:tt) => {
        impl $T {
            pub fn $func(self, other: $T) -> $T {
                $T(self.0 $op other.0)
            }
        }
        impl $V<$T> for $T {
            type Output = $T;
            fn $func(self, other: $T) -> $T {
                $T(self.0 $op other.0)
            }
        }
        impl $V<$U> for $T {
            type Output = $T;
            fn $func(self, other: $U) -> $T {
                $T(self.0 $op other)
            }
        }
        impl $V<$T> for $U {
            type Output = $T;
            fn $func(self, other: $T) -> $T {
                $T(self $op other.0)
            }
        }
    };
    ($T:ident,$U:ident,$V:ident,$W:tt, $func:ident, $op:tt, $flag:expr) => {
        impl $T {
            pub fn $func(self, other: $T) -> $T {
                match $flag {
                    0 => {
                        $T(self.0 $op other.0, self.1 $op other.1)
                    }
                    1 => {
                        $T(self.0 $op other.0, 0)
                    }
                    2 => {
                        $T(0, self.1 $op other.1)
                    }
                    3 => {
                        $T(self.0 $op other.0, self.1)
                    }
                    _ => {
                    $T(self.0, self.1 $op other.1)
                    }
                }
            }
        }
        impl $W<$T> for $T {
            type Output = $T;
            fn $func(self, other: $T) -> $T {
                match $flag {
                    0 => {
                        $T(self.0 $op other.0, self.1 $op other.1)
                    }
                    1 => {
                        $T(self.0 $op other.0, 0)
                    }
                    2 => {
                        $T(0, self.1 $op other.1)
                    }
                    3 => {
                        $T(self.0 $op other.0, self.1)
                    }
                    _ => {
                        $T(self.0, self.1 $op other.1)
                    }
                }
            }
        }
        impl $W<$U> for $T {
            type Output = $T;
            fn $func(self, other: $U) -> $T {
                match $flag {
                    0 => {
                        $T(self.0 $op other, self.1 $op other as $V)
                    }
                    1 => {
                        $T(self.0 $op other, 0)
                    }
                    2 => {
                        $T(0, self.1 $op other as $V)
                    }
                    3 => {
                        $T(self.0 $op other, self.1)
                    }
                    _ => {
                        $T(self.0, self.1 $op other as $V)
                    }
                }
            }
        }

        impl $W<$V> for $T {
            type Output = $T;
            fn $func(self, other: $V) -> $T {
                match $flag {
                    0 => {
                        $T(self.0 $op other as $U, self.1 $op other)
                    }
                    1 => {
                        $T(self.0 $op other as $U, 0)
                    }
                    2 => {
                        $T(0, self.1 $op other)
                    }
                    3 => {
                        $T(self.0 $op other as $U, self.1)
                    }
                    _ => {
                        $T(self.0, self.1 $op other)
                    }
                }
            }
        }
        impl $W<$T> for $U {
            type Output = $T;
            fn $func(self, other: $T) -> $T {
                match $flag {
                    0 => {
                        $T(self $op other.0, self as $V $op other.1)
                    }
                    1 => {
                        $T(self $op other.0, 0)
                    }
                    2 => {
                        $T(0, self as $V $op other.1)
                    }
                    3 => {
                        $T(self $op other.0, self as $V)
                    }
                    _ => {
                        $T(self, self as $V $op other.1)
                    }
                }
            }
        }
        impl $W<$T> for $V {
            type Output = $T;
            fn $func(self, other: $T) -> $T {
                match $flag {
                    0 => {
                        $T(self as $U $op other.0, self $op other.1)
                    }
                    1 => {
                        $T(self as $U $op other.0, 0)
                    }
                    2 => {
                        $T(0, self $op other.1)
                    }
                    3 => {
                        $T(self as $U $op other.0, self)
                    }
                    _ => {
                        $T(self as $U, self $op other.1)
                    }
                }
            }
        }
    };
}
#[macro_export]
macro_rules! impl_op_assign {
    ($T:ident,$U:ident,$V:tt,$func:ident, $op:tt) => {
        impl $T {
            pub fn $func(mut self, other: $T) {
                let _ = self.0 $op other.0;
                let _ = self.1 $op other.1;
            }
        }
        impl $V<$T> for $T {
            fn $func(&mut self, other: $T) {
                let _ = self.0 $op other.0;
                let _ = self.1 $op other.1;
            }
        }
        impl $V<$U> for $T {
            fn $func(&mut self, other: $U) {
                let _ = self.0 $op other;
                let _ = self.1 $op other;
            }
        }
        impl $V<$T> for $U {
            fn $func(&mut self, other: $T) {
                let _ = self $op other.0;
                let _ = self $op other.1;
            }
        }
    };
    ($T:ident,$U:ident,$V:ident,$W:tt,$func:ident, $op:tt, $flag:expr) => {
        impl $T {
            pub fn $func(mut self, other: $T) {
                match $flag {
                    0 => {
                        let _ = self.0 $op other.0;
                        let _ = self.1 $op other.1;
                    }
                    1 => {
                        let _ = self.0 $op other.0;
                        self.1 = 0;
                    }
                    2 => {
                        self.0 = 0;
                        let _ = self.1 $op other.1;
                    }
                    3 => {
                        let _ = self.0 $op other.0;
                    }
                    _ => {
                        let _ = self.1 $op other.1;
                    }
                }
            }
        }
        impl $W<$T> for $T {
            fn $func(&mut self, other: $T) {
                match $flag {
                    0 => {
                        let _ = self.0 $op other.0;
                        let _ = self.1 $op other.1;
                    }
                    1 => {
                        let _ = self.0 $op other.0;
                        self.1 = 0;
                    }
                    2 => {
                        self.0 = 0;
                        let _ = self.1 $op other.1;
                    }
                    3 => {
                        let _ = self.0 $op other.0;
                    }
                    _ => {
                        let _ = self.1 $op other.1;
                    }
                }
            }
        }
        impl $W<$U> for $T {
            fn $func(&mut self, other: $U) {
                match $flag {
                    0 => {
                        let _ = self.0 $op other;
                        let _ = self.1 $op other as $V;
                    }
                    1 => {
                        let _ = self.0 $op other;
                        self.1 = 0;
                    }
                    2 => {
                        self.0 = 0;
                        let _ = self.1 $op other as $V;
                    }
                    3 => {
                        let _ = self.0 $op other;
                    }
                    _ => {
                        let _ = self.1 $op other as $V;
                    }
                }
            }
        }
        impl $W<$V> for $T {
            fn $func(&mut self, other: $V) {
                match $flag {
                    0 => {
                        let _ = self.0 $op other as $U;
                        let _ = self.1 $op other;
                    }
                    1 => {
                        let _ = self.0 $op other as $U;
                        self.1 = 0;
                    }
                    2 => {
                        self.0 = 0;
                        let _ = self.1 $op other;
                    }
                    3 => {
                        let _ = self.0 $op other as $U;
                    }
                    _ => {
                        let _ = self.1 $op other;
                    }
                }
            }
        }
        impl $W<$T> for $U {
            fn $func(&mut self, other: $T) {
                match $flag {
                    0 => {
                        let _ = *self $op other.0;
                        let _ = *self $op other.1 as $U;
                    }
                    1 => {
                        let _ = *self $op other.0;
                        *self = 0;
                    }
                    2 => {
                        *self = 0;
                        let _ = *self $op other.1 as $U;
                    }
                    3 => {
                        let _ = *self $op other.0;
                    }
                    _ => {
                        let _ = *self $op other.1 as $U;
                    }
                }
            }
        }
        impl $W<$T> for $V {
            fn $func(&mut self, other: $T) {
                match $flag {
                    0 => {
                        let _ = *self $op other.0 as $V;
                        let _ = *self $op other.1;
                    }
                    1 => {
                        let _ = *self $op other.0 as $V;
                        *self = 0;
                    }
                    2 => {
                        *self = 0;
                        let _ = *self $op other.1;
                    }
                    3 => {
                        let _ = *self $op other.0 as $V;
                    }
                    _ => {
                        let _ = *self $op other.1;
                    }
                }
            }
        }
    };
}
#[macro_export]
macro_rules! impl_any {
    ($T:ident) => {
        impl Any for $T {
            fn is<T: ?Sized>(&self, type_id: TypeId) -> bool {
                type_id == TypeId::of::<T>()
            }
            fn type_id(&self) -> TypeId {
                TypeId::of::<T>()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_bitand {
    ($T:ident,$U:ident) => {
        impl $T {
            pub fn bitand(self, other: $T) -> $T {
                $T(self.0 & other.0)
            }
            pub fn bitand_assign(&mut self, other: $T) {
                self.0 &= other.0;
            }
        }
        impl BitAnd for $T {
            type Output = $T;
            fn bitand(self, other: $T) -> $T {
                Self::bitand(self, other)
            }
        }
        impl BitAndAssign for $T {
            fn bitand_assign(&mut self, other: $T) {
                Self::bitand_assign(self, other)
            }
        }
        impl BitAnd<$U> for $T {
            type Output = $T;
            fn bitand(self, other: $U) -> $T {
                $T(self.0 & other)
            }
        }
        impl BitAndAssign<$U> for $T {
            fn bitand_assign(&mut self, other: $U) {
                self.0 &= other
            }
        }
        impl BitAnd<$T> for $U {
            type Output = $T;
            fn bitand(self, other: $T) -> $T {
                $T(self & other.0)
            }
        }
        impl BitAndAssign<$T> for $U {
            fn bitand_assign(&mut self, other: $T) {
                *self &= other.0
            }
        }
    };
    ($T:ident, $U:ident, $v:literal) => {
        impl $T {
            pub fn bitand(self, other: $T) -> $T {
                $T(self.$v & other.$v)
            }
            pub fn bitand_assign(&mut self, other: $T) {
                self.$v &= other.$v;
            }
        }
        impl BitAnd<$T> for $T {
            type Output = $T;
            fn bitand(self, other: $T) -> $T {
                $T(self.$v & other.$v)
            }
        }
        impl BitAndAssign<$T> for $T {
            fn bitand_assign(&mut self, other: $T) {
                self.$v &= other.$v;
            }
        }
        impl BitAnd<$U> for $T {
            type Output = $T;
            fn bitand(self, other: $U) -> $T {
                $T(self.$v & other)
            }
        }
        impl BitAndAssign<$U> for $T {
            fn bitand_assign(&mut self, other: $U) {
                self.$v &= other;
            }
        }
        impl BitAnd<$T> for $U {
            type Output = $T;
            fn bitand(self, other: $T) -> $T {
                Self::bitand(self, other.$v)
            }
        }
        impl BitAndAssign<$T> for $U {
            fn bitand_assign(&mut self, other: $T) {
                Self::bitand_assign(self, other.$v)
            }
        }
    };
    ($T:ident, $U:ident, $v1:literal,$v2:literal) => {
        impl $T {
            pub fn bitand(self, other: $T) -> $T {
                $T(self.$v1 & other.$v1)
            }
            pub fn bitand_assign(&mut self, other: $T) {
                self.$v1 &= other.$v1;
            }
        }
        impl BitAnd<$T> for $T {
            type Output = $T;
            fn bitand(self, other: $T) -> $T {
                $T(self.$v1 & other.$v1)
            }
        }
        impl BitAndAssign<$T> for $T {
            fn bitand_assign(&mut self, other: $T) {
                self.$v1 &= other.$v1;
            }
        }
        impl BitAnd<$U> for $T {
            type Output = $T;
            fn bitand(self, other: $U) -> $T {
                $T(self.$v1 & other.$v2)
            }
        }
        impl BitAndAssign<$U> for $T {
            fn bitand_assign(&mut self, other: $U) {
                self.$v1 &= other.$v2;
            }
        }
        impl BitAnd<$T> for $U {
            type Output = $T;
            fn bitand(self, other: $T) -> $T {
                $T(self.$v2 & other.$v1)
            }
        }
        impl BitAndAssign<$T> for $U {
            fn bitand_assign(&mut self, other: $T) {
                self.$v2 &= other.$v1;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_bitor {
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn bitor(self, other: $T) -> $T {
                $T(self.0 | other.0)
            }
            pub fn bitor_assign(&mut self, other: $T) {
                self.0 |= other.0;
            }
        }
        impl BitOr<$T> for $T {
            type Output = $T;
            fn bitor(self, other: $T) -> $T {
                $T(self.0 | other.0)
            }
        }
        impl BitOrAssign<$T> for $T {
            fn bitor_assign(&mut self, other: $T) {
                self.0 |= other.0;
            }
        }
        impl BitOr<$U> for $T {
            type Output = $T;
            fn bitor(self, other: $U) -> $T {
                $T(self.0 | other)
            }
        }
        impl BitOrAssign<$U> for $T {
            fn bitor_assign(&mut self, other: $U) {
                self.0 |= other;
            }
        }
        impl BitOr<$T> for $U {
            type Output = $T;
            fn bitor(self, other: $T) -> $T {
                $T(self | other.0)
            }
        }
        impl BitOrAssign<$T> for $U {
            fn bitor_assign(&mut self, other: $T) {
                *self |= other.0;
            }
        }
    };
    ($T:ident,$U:ident,$v:literal) => {
        impl $T {
            pub fn bitor(self, other: $T) -> $T {
                $T(self.$v | other.$v)
            }
            pub fn bitor_assign(&mut self, other: $T) {
                self.$v |= other.$v;
            }
        }
        impl BitOr<$T> for $T {
            type Output = $T;
            fn bitor(self, other: $T) -> $T {
                $T(self.$v | other.$v)
            }
        }
        impl BitOrAssign<$T> for $T {
            fn bitor_assign(&mut self, other: $T) {
                self.$v |= other.$v;
            }
        }
        impl BitOr<$U> for $T {
            type Output = $T;
            fn bitor(self, other: $U) -> $T {
                $T(self.$v | other)
            }
        }
        impl BitOrAssign<$U> for $T {
            fn bitor_assign(&mut self, other: $U) {
                self.$v |= other;
            }
        }
        impl BitOr<$T> for $U {
            type Output = $T;
            fn bitor(self, other: $T) -> $T {
                $T(self | other.$v)
            }
        }
        impl BitOrAssign<$T> for $U {
            fn bitor_assign(&mut self, other: $T) {
                *self |= other.$v;
            }
        }
    };
    ($T:ident,$U:ident,$v1:literal,$v2:literal) => {
        impl $T {
            pub fn bitor(self, other: $T) -> $T {
                $T(self.$v1 | other.$v1)
            }
            pub fn bitor_assign(&mut self, other: $T) {
                self.$v1 |= other.$v1;
            }
        }
        impl BitOr<$T> for $T {
            type Output = $T;
            fn bitor(self, other: $T) -> $T {
                self.$v1 | other.$v1
            }
        }
        impl BitOrAssign<$T> for $T {
            fn bitor_assign(&mut self, other: $T) {
                self.$v1 |= other.$v1;
            }
        }
        impl BitOr<$U> for $T {
            type Output = $T;
            fn bitor(self, other: $U) -> $T {
                $T(self.$v1 | other.$v2)
            }
        }
        impl BitOrAssign<$U> for $T {
            fn bitor_assign(&mut self, other: $U) {
                self.$v1 |= other.$v2;
            }
        }
        impl BitOr<$T> for $U {
            type Output = $T;
            fn bitor(self, other: $T) -> $T {
                $T(self.$v2 | other.$v1)
            }
        }
        impl BitOrAssign<$T> for $U {
            fn bitor_assign(&mut self, other: $T) {
                self.$v2 |= other.$v1;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_bitxor {
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn bitxor(self, other: $T) -> $T {
                $T(self.0 ^ other.0)
            }
            pub fn bitxor_assign(&mut self, other: $T) {
                self.0 ^= other.0;
            }
        }
        impl BitXor<$T> for $T {
            type Output = $T;
            fn bitxor(self, other: $T) -> $T {
                $T(self.0 ^ other.0)
            }
        }
        impl BitXorAssign<$T> for $T {
            fn bitxor_assign(&mut self, other: $T) {
                self.0 ^= other.0;
            }
        }
        impl BitXor<$U> for $T {
            type Output = $T;
            fn bitxor(self, other: $U) -> $T {
                $T(self.0 ^ other)
            }
        }
        impl BitXorAssign<$U> for $T {
            fn bitxor_assign(&mut self, other: $U) {
                self.0 ^= other;
            }
        }
        impl BitXor<$T> for $U {
            type Output = $T;
            fn bitxor(self, other: $T) -> $T {
                $T(self ^ other.0)
            }
        }
        impl BitXorAssign<$T> for $U {
            fn bitxor_assign(&mut self, other: $T) {
                *self ^= other.0;
            }
        }
    };
    ($T:ident,$U:ident,$v:literal) => {
        impl $T {
            pub fn bitxor(self, other: $T) -> $T {
                $T(self.$v ^ other.$v)
            }
            pub fn bitxor_assign(&mut self, other: $T) {
                self.$v ^= other.$v;
            }
        }
        impl BitXor<$T> for $T {
            type Output = $T;
            fn bitxor(self, other: $T) -> $T {
                $T(self.$v ^ other.$v)
            }
        }
        impl BitXorAssign<$T> for $T {
            fn bitxor_assign(&mut self, other: $T) {
                self.$v ^= other.$v;
            }
        }
        impl BitXor<$U> for $T {
            type Output = $T;
            fn bitxor(self, other: $U) -> $T {
                $T(self.$v ^ other)
            }
        }
        impl BitXorAssign<$U> for $T {
            fn bitxor_assign(&mut self, other: $U) {
                self.$v ^= other;
            }
        }
        impl BitXor<$T> for $U {
            type Output = $T;
            fn bitxor(self, other: $T) -> $T {
                $T(self ^ other.$v)
            }
        }
        impl BitXorAssign<$T> for $U {
            fn bitxor_assign(&mut self, other: $T) {
                *self ^= other.$v;
            }
        }
    };
    ($T:ident,$U:ident,$v1:literal,$v2:literal) => {
        impl $T {
            pub fn bitxor(self, other: $T) -> $T {
                $T(self.$v1 ^ other.$v1)
            }
            pub fn bitxor_assign(&mut self, other: $T) {
                self.$v1 ^= other.$v1;
            }
        }
        impl BitXor<$T> for $T {
            type Output = $T;
            fn bitxor(self, other: $T) -> $T {
                $T(self.$v1 ^ other.$v1)
            }
        }
        impl BitXorAssign<$T> for $T {
            fn bitxor_assign(&mut self, other: $T) {
                self.$v1 ^= other.$v1;
            }
        }
        impl BitXor<$U> for $T {
            type Output = $T;
            fn bitxor(self, other: $U) -> $T {
                $T(self.$v1 ^ other.$v2)
            }
        }
        impl BitXorAssign<$U> for $T {
            fn bitxor_assign(&mut self, other: $U) {
                self.$v1 ^= other.$v2;
            }
        }
        impl BitXor<$T> for $U {
            type Output = $T;
            fn bitxor(self, other: $T) -> $T {
                $T(self.$v2 ^ other.$v1)
            }
        }
        impl BitXorAssign<$T> for $U {
            fn bitxor_assign(&mut self, other: $T) {
                self.$v2 ^= other.$v1;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_not {
    ($T:ident) => {
        impl Not for $T {
            type Output = $T;
            fn not(self) -> $T {
                $T(!self.0)
            }
        }
    };
    ($T:ident,$v:literal) => {
        impl $T {
            pub fn not(self) -> $T {
                $T(!self.$v)
            }
        }
        impl Not for $T {
            type Output = $T;
            fn not(self) -> $T {
                $T(!self.$v)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_add {
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn add(self, other: $T) -> $T {
                $T(self.0 + other.0)
            }
            pub fn add_assign(&mut self, other: $T) {
                self.0 += other.0;
            }
        }
        impl Add<$T> for $T {
            type Output = $T;
            fn add(self, other: $T) -> $T {
                $T(self.0 + other.0)
            }
        }
        impl AddAssign<$T> for $T {
            fn add_assign(&mut self, other: $T) {
                self.0 += other.0;
            }
        }
        impl Add<$U> for $T {
            type Output = $T;
            fn add(self, other: $U) -> $T {
                $T(self.0 + other)
            }
        }
        impl AddAssign<$U> for $T {
            fn add_assign(&mut self, other: $U) {
                self.0 += other;
            }
        }
        impl Add<$T> for $U {
            type Output = $T;
            fn add(self, other: $T) -> $T {
                $T(self + other.0)
            }
        }
        impl AddAssign<$T> for $U {
            fn add_assign(&mut self, other: $T) {
                *self += other.0;
            }
        }
    };
    ($T:ident,$U:ident,$v:literal) => {
        impl $T {
            pub fn add(self, other: $T) -> $T {
                $T(self.$v + other.$v)
            }
            pub fn add_assign(&mut self, other: $T) {
                self.$v += other.$v;
            }
        }
        impl Add<$T> for $T {
            type Output = $T;
            fn add(self, other: $T) -> $T {
                $T(self.$v + other.$v)
            }
        }
        impl AddAssign<$T> for $T {
            fn add_assign(&mut self, other: $T) {
                self.$v += other.$v;
            }
        }
        impl Add<$U> for $T {
            type Output = $T;
            fn add(self, other: $U) -> $T {
                $T(self.$v + other)
            }
        }
        impl AddAssign<$U> for $T {
            fn add_assign(&mut self, other: $U) {
                self.$v += other;
            }
        }
        impl Add<$T> for $U {
            type Output = $T;
            fn add(self, other: $T) -> $T {
                $T(self + other.$v)
            }
        }
        impl AddAssign<$T> for $U {
            fn add_assign(&mut self, other: $T) {
                *self += other.$v;
            }
        }
    };
    ($T:ident,$U:ident,$v1:literal,$v2:literal) => {
        impl $T {
            pub fn add(self, other: $T) -> $T {
                $T(self.$v1 + other.$v1)
            }
            pub fn add_assign(&mut self, other: $T) {
                self.$v1 += other.$v1;
            }
        }
        impl Add<$U> for $T {
            type Output = $T;
            fn add(self, other: $U) -> $T {
                $T(self.$v1 + other.$v2)
            }
        }
        impl AddAssign<$U> for $T {
            fn add_assign(&mut self, other: $U) {
                self.$v1 += other.$v2;
            }
        }
        impl Add<$T> for $U {
            type Output = $T;
            fn add(self, other: $T) -> $T {
                $T(self.$v2 + other.$v1)
            }
        }
        impl AddAssign<$T> for $U {
            fn add_assign(&mut self, other: $T) {
                self.$v2 += other.$v1;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_sub {
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn sub(self, other: $T) -> $T {
                $T(self.0 - other.0)
            }
            pub fn sub_assign(&mut self, other: $T) {
                self.0 -= other.0;
            }
        }
        impl Sub<$T> for $T {
            type Output = $T;
            fn sub(self, other: $T) -> $T {
                $T(self.0 - other.0)
            }
        }
        impl SubAssign<$T> for $T {
            fn sub_assign(&mut self, other: $T) {
                self.0 -= other.0;
            }
        }
        impl Sub<$U> for $T {
            type Output = $T;
            fn sub(self, other: $U) -> $T {
                $T(self.0 - other)
            }
        }
        impl SubAssign<$U> for $T {
            fn sub_assign(&mut self, other: $U) {
                self.0 -= other;
            }
        }
        impl Sub<$T> for $U {
            type Output = $T;
            fn sub(self, other: $T) -> $T {
                $T(self - other.0)
            }
        }
        impl SubAssign<$T> for $U {
            fn sub_assign(&mut self, other: $T) {
                *self -= other.0;
            }
        }
    };
    ($T:ident,$U:ident,$v:literal) => {
        impl $T {
            pub fn sub(self, other: $T) -> $T {
                $T(self.$v - other.$v)
            }
            pub fn sub_assign(&mut self, other: $T) {
                self.$v -= other.$v;
            }
        }
        impl Sub<$T> for $T {
            type Output = $T;
            fn sub(self, other: $T) -> $T {
                $T(self.$v - other.$v)
            }
        }
        impl SubAssign<$T> for $T {
            fn sub_assign(&mut self, other: $T) {
                self.$v -= other.$v;
            }
        }
        impl Sub<$U> for $T {
            type Output = $T;
            fn sub(self, other: $U) -> $T {
                $T(self.$v - other)
            }
        }
        impl SubAssign<$U> for $T {
            fn sub_assign(&mut self, other: $U) {
                self.$v -= other;
            }
        }
        impl Sub<$T> for $U {
            type Output = $T;
            fn sub(self, other: $T) -> $T {
                $T(self - other.$v)
            }
        }
        impl SubAssign<$T> for $U {
            fn sub_assign(&mut self, other: $T) {
                *self -= other.$v;
            }
        }
    };
    ($T:ident,$U:ident,$v1:literal,$v2:literal) => {
        impl $T {
            pub fn sub(self, other: $T) -> $T {
                $T(self.$v1 - other.$v1)
            }
            pub fn sub_assign(&mut self, other: $T) {
                self.$v1 -= other.$v1;
            }
        }
        impl Sub<$U> for $T {
            type Output = $T;
            fn sub(self, other: $U) -> $T {
                $T(self.$v1 - other.$v2)
            }
        }
        impl SubAssign<$U> for $T {
            fn sub_assign(&mut self, other: $U) {
                self.$v1 -= other.$v2;
            }
        }
        impl Sub<$T> for $U {
            type Output = $T;
            fn sub(self, other: $T) -> $T {
                $T(self.$v2 - other.$v1)
            }
        }
        impl SubAssign<$T> for $U {
            fn sub_assign(&mut self, other: $T) {
                self.$v2 -= other.$v1;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_mul {
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn mul(self, other: $T) -> $T {
                $T(self.0 * other.0)
            }
            pub fn mul_assign(&mut self, other: $T) {
                self.0 *= other.0;
            }
        }
        impl Mul<$T> for $T {
            type Output = $T;
            fn mul(self, other: $T) -> $T {
                $T(self.0 * other.0)
            }
        }
        impl MulAssign<$T> for $T {
            fn mul_assign(&mut self, other: $T) {
                self.0 *= other.0;
            }
        }
        impl Mul<$U> for $T {
            type Output = $T;
            fn mul(self, other: $U) -> $T {
                $T(self.0 * other)
            }
        }
        impl MulAssign<$U> for $T {
            fn mul_assign(&mut self, other: $U) {
                self.0 *= other;
            }
        }
        impl Mul<$T> for $U {
            type Output = $T;
            fn mul(self, other: $T) -> $T {
                $T(self * other.0)
            }
        }
        impl MulAssign<$T> for $U {
            fn mul_assign(&mut self, other: $T) {
                *self *= other.0;
            }
        }
    };
    ($T:ident,$U:ident,$v:literal) => {
        impl $T {
            pub fn mul(self, other: $T) -> $T {
                $T(self.$v * other.$v)
            }
            pub fn mul_assign(&mut self, other: $T) {
                self.$v *= other.$v;
            }
        }
        impl Mul<$T> for $T {
            type Output = $T;
            fn mul(self, other: $T) -> $T {
                $T(self.$v * other.$v)
            }
        }
        impl MulAssign<$T> for $T {
            fn mul_assign(&mut self, other: $T) {
                self.$v *= other.$v;
            }
        }
        impl Mul<$U> for $T {
            type Output = $T;
            fn mul(self, other: $U) -> $T {
                $T(self.$v * other)
            }
        }
        impl MulAssign<$U> for $T {
            fn mul_assign(&mut self, other: $U) {
                self.$v *= other;
            }
        }
        impl Mul<$T> for $U {
            type Output = $T;
            fn mul(self, other: $T) -> $T {
                $T(self * other.$v)
            }
        }
        impl MulAssign<$T> for $U {
            fn mul_assign(&mut self, other: $T) {
                *self *= other.$v;
            }
        }
    };
    ($T:ident,$U:ident,$v1:literal,$v2:literal) => {
        impl $T {
            pub fn mul(self, other: $T) -> $T {
                $T(self.$v1 * other.$v1, self.$v2 * other.$v2)
            }
            pub fn mul_assign(&mut self, other: $T) {
                self.$v1 *= other.$v1;
                self.$v2 *= other.$v2;
            }
        }
        impl Mul<$U> for $T {
            type Output = $T;
            fn mul(self, other: $U) -> $T {
                $T(self.$v1 * other.$v1, self.$v2 * other.$v2)
            }
        }
        impl MulAssign<$U> for $T {
            fn mul_assign(&mut self, other: $U) {
                self.$v1 *= other.$v1;
                self.$v2 *= other.$v2;
            }
        }
        impl Mul<$T> for $U {
            type Output = $T;
            fn mul(self, other: $T) -> $T {
                $T(self.$v1 * other.$v1, self.$v2 * other.$v2)
            }
        }
        impl MulAssign<$T> for $U {
            fn mul_assign(&mut self, other: $T) {
                self.$v1 *= other.$v1;
                self.$v2 *= other.$v2;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_div {
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn div(self, other: $T) -> $T {
                $T(self.0 / other.0)
            }
            pub fn div_assign(&mut self, other: $T) {
                self.0 /= other.0;
            }
        }
        impl Div<$T> for $T {
            type Output = $T;
            fn div(self, other: $T) -> $T {
                $T(self.0 / other.0)
            }
        }
        impl DivAssign<$T> for $T {
            fn div_assign(&mut self, other: $T) {
                self.0 /= other.0;
            }
        }
        impl Div<$U> for $T {
            type Output = $T;
            fn div(self, other: $U) -> $T {
                $T(self.0 / other)
            }
        }
        impl DivAssign<$U> for $T {
            fn div_assign(&mut self, other: $U) {
                self.0 /= other;
            }
        }
        impl Div<$T> for $U {
            type Output = $T;
            fn div(self, other: $T) -> $T {
                $T(self / other.0)
            }
        }
        impl DivAssign<$T> for $U {
            fn div_assign(&mut self, other: $T) {
                *self /= other.0;
            }
        }
    };
    ($T:ident,$U:ident,$v:literal) => {
        impl $T {
            pub fn div(self, other: $T) -> $T {
                $T(self.$v / other.$v)
            }
            pub fn div_assign(&mut self, other: $T) {
                self.$v /= other.$v;
            }
        }
        impl Div<$T> for $T {
            type Output = $T;
            fn div(self, other: $T) -> $T {
                $T(self.$v / other.$v)
            }
        }
        impl DivAssign<$T> for $T {
            fn div_assign(&mut self, other: $T) {
                self.$v /= other.$v;
            }
        }
        impl Div<$U> for $T {
            type Output = $T;
            fn div(self, other: $U) -> $T {
                $T(self.$v / other)
            }
        }
        impl DivAssign<$U> for $T {
            fn div_assign(&mut self, other: $U) {
                self.$v /= other;
            }
        }
        impl Div<$T> for $U {
            type Output = $T;
            fn div(self, other: $T) -> $T {
                $T(self / other.$v)
            }
        }
        impl DivAssign<$T> for $U {
            fn div_assign(&mut self, other: $T) {
                *self /= other.$v;
            }
        }
    };
    ($T:ident,$U:ident,$v1:literal,$v2:literal) => {
        impl $T {
            pub fn div(self, other: $T) -> $T {
                $T(self.$v1 / other.$v1)
            }
            pub fn div_assign(&mut self, other: $T) {
                self.$v1 /= other.$v1;
            }
        }
        impl Div<$T> for $T {
            type Output = $T;
            fn div(self, other: $T) -> $T {
                $T(self.$v1 / other.$v1)
            }
        }
        impl DivAssign<$T> for $T {
            fn div_assign(&mut self, other: $T) {
                self.$v1 /= other.$v1;
            }
        }
        impl Div<$U> for $T {
            type Output = $T;
            fn div(self, other: $U) -> $T {
                $T(self.$v1 / other.$v2)
            }
        }
        impl DivAssign<$U> for $T {
            fn div_assign(&mut self, other: $U) {
                self.$v1 /= other.$v2;
            }
        }
        impl Div<$T> for $U {
            type Output = $T;
            fn div(self, other: $T) -> $T {
                $T(self.$v2 / other.$v1)
            }
        }
        impl DivAssign<$T> for $U {
            fn div_assign(&mut self, other: $T) {
                self.$v2 /= other.$v1;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_rem {
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn rem(self, other: $T) -> $T {
                $T(self.0 % other.0)
            }
            pub fn rem_assign(&mut self, other: $T) {
                self.0 %= other.0;
            }
        }
        impl Rem<$T> for $T {
            type Output = $T;
            fn rem(self, other: $T) -> $T {
                $T(self.0 % other.0)
            }
        }
        impl RemAssign<$T> for $T {
            fn rem_assign(&mut self, other: $T) {
                self.0 %= other.0;
            }
        }
        impl Rem<$U> for $T {
            type Output = $T;
            fn rem(self, other: $U) -> $T {
                $T(self.0 % other)
            }
        }
        impl RemAssign<$U> for $T {
            fn rem_assign(&mut self, other: $U) {
                self.0 %= other;
            }
        }
        impl Rem<$T> for $U {
            type Output = $T;
            fn rem(self, other: $T) -> $T {
                $T(self % other.0)
            }
        }
        impl RemAssign<$T> for $U {
            fn rem_assign(&mut self, other: $T) {
                *self %= other.0;
            }
        }
    };
    ($T:ident,$U:ident,$v:literal) => {
        impl $T {
            pub fn rem(self, other: $T) -> $T {
                $T(self.$v % other.$v)
            }
            pub fn rem_assign(&mut self, other: $T) {
                self.$v %= other.$v;
            }
        }
        impl Rem<$T> for $T {
            type Output = $T;
            fn rem(self, other: $T) -> $T {
                $T(self.$v % other.$v)
            }
        }
        impl RemAssign<$T> for $T {
            fn rem_assign(&mut self, other: $T) {
                self.$v %= other.$v;
            }
        }
        impl Rem<$U> for $T {
            type Output = $T;
            fn rem(self, other: $U) -> $T {
                $T(self.$v % other)
            }
        }
        impl RemAssign<$U> for $T {
            fn rem_assign(&mut self, other: $U) {
                self.$v %= other;
            }
        }
        impl Rem<$T> for $U {
            type Output = $T;
            fn rem(self, other: $T) -> $T {
                $T(self % other.$v)
            }
        }
        impl RemAssign<$T> for $U {
            fn rem_assign(&mut self, other: $T) {
                *self %= other.$v;
            }
        }
    };
    ($T:ident,$U:ident,$v1:literal,$v2:literal) => {
        impl $T {
            pub fn rem(self, other: $T) -> $T {
                $T(self.$v1 % other.$v1)
            }
            pub fn rem_assign(&mut self, other: $T) {
                self.$v1 %= other.$v1;
            }
        }
        impl Rem<$T> for $T {
            type Output = $T;
            fn rem(self, other: $T) -> $T {
                $T(self.$v1 % other.$v1)
            }
        }
        impl RemAssign<$T> for $T {
            fn rem_assign(&mut self, other: $T) {
                self.$v1 %= other.$v1;
            }
        }
        impl Rem<$U> for $T {
            type Output = $T;
            fn rem(self, other: $U) -> $T {
                $T(self.$v1 % other.$v2)
            }
        }
        impl RemAssign<$U> for $T {
            fn rem_assign(&mut self, other: $U) {
                self.$v1 %= other.$v2;
            }
        }
        impl Rem<$T> for $U {
            type Output = $T;
            fn rem(self, other: $T) -> $T {
                $T(self.$v2 % other.$v1)
            }
        }
        impl RemAssign<$T> for $U {
            fn rem_assign(&mut self, other: $T) {
                self.$v2 %= other.$v1;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_shl {
    ($T:ident) => {
        impl $T {
            pub fn shl(self, other: $T) -> $T {
                $T(self.0 << other.0)
            }
            pub fn shl_assign(&mut self, other: $T) {
                self.0 <<= other.0;
            }
        }
        impl Shl<$T> for $T {
            type Output = $T;
            fn shl(self, other: $T) -> $T {
                $T(self.0 << other.0)
            }
        }
        impl ShlAssign<$T> for $T {
            fn shl_assign(&mut self, other: $T) {
                self.0 <<= other.0;
            }
        }
    };
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn shl(self, other: $T) -> $T {
                $T(self.0 << other.0)
            }
            pub fn shl_assign(&mut self, other: $T) {
                self.0 <<= other.0;
            }
        }
        impl Shl<$T> for $T {
            type Output = $T;
            fn shl(self, other: $T) -> $T {
                $T(self.0 << other.0)
            }
        }
        impl ShlAssign<$T> for $T {
            fn shl_assign(&mut self, other: $T) {
                self.0 <<= other.0;
            }
        }
        impl Shl<$U> for $T {
            type Output = $T;
            fn shl(self, other: $U) -> $T {
                $T(self.0 << other)
            }
        }
        impl ShlAssign<$U> for $T {
            fn shl_assign(&mut self, other: $U) {
                self.0 <<= other;
            }
        }
        impl Shl<$T> for $U {
            type Output = $T;
            fn shl(self, other: $T) -> $T {
                $T(self << other.0)
            }
        }
        impl ShlAssign<$T> for $U {
            fn shl_assign(&mut self, other: $T) {
                *self <<= other.0;
            }
        }
    };
    ($T:ident,$U:ident, $v:literal) => {
        impl $T {
            pub fn shl(self, other: $T) -> $T {
                $T(self.$v << other.$v)
            }
            pub fn shl_assign(&mut self, other: $T) {
                self.$v <<= other.$v;
            }
        }
        impl Shl<$T> for $T {
            type Output = $T;
            fn shl(self, other: $T) -> $T {
                $T(self.$v << other.$v)
            }
        }
        impl ShlAssign<$T> for $T {
            fn shl_assign(&mut self, other: $T) {
                self.$v <<= other.$v;
            }
        }
        impl Shl<$U> for $T {
            type Output = $T;
            fn shl(self, other: $U) -> $T {
                $T(self.$v << other)
            }
        }
        impl ShlAssign<$U> for $T {
            fn shl_assign(&mut self, other: $U) {
                self.$v <<= other;
            }
        }
        impl Shl<$T> for $U {
            type Output = $U;
            fn shl(self, other: $T) -> $U {
                self << other.$v
            }
        }
        impl ShlAssign<$T> for $U {
            fn shl_assign(&mut self, other: $T) {
                self <<= other.$v;
            }
        }
    };
    ($T:ident,$U:ident,$v1:literal,$v2:literal) => {
        impl $T {
            pub fn shl(self, other: $T) -> $T {
                $T(self.$v1 << other.$v1)
            }
            pub fn shl_assign(&mut self, other: $T) {
                self.$v1 <<= other.$v1;
            }
        }
        impl Shl<$T> for $T {
            type Output = $T;
            fn shl(self, other: $T) -> $T {
                $T(self.$v1 << other.$v1)
            }
        }
        impl ShlAssign<$T> for $T {
            fn shl_assign(&mut self, other: $T) {
                self.$v1 <<= other.$v1;
            }
        }
        impl Shl<$U> for $T {
            type Output = $T;
            fn shl(self, other: $U) -> $T {
                $T(self.$v1 << other.$v2)
            }
        }
        impl ShlAssign<$U> for $T {
            fn shl_assign(&mut self, other: $U) {
                self.$v1 <<= other.$v2;
            }
        }
        impl Shl<$T> for $U {
            type Output = $U;
            fn shl(self, other: $T) -> $U {
                self.$v2 << other.$v1
            }
        }
        impl ShlAssign<$T> for $U {
            fn shl_assign(&mut self, other: $T) {
                self.$v2 <<= other.$v1;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_shr {
    ($T:ident) => {
        impl $T {
            pub fn shr(self, other: $T) -> $T {
                $T(self.0 >> other.0)
            }
            pub fn shr_assign(&mut self, other: $T) {
                self.0 >>= other.0;
            }
        }
        impl Shr<$T> for $T {
            type Output = $T;
            fn shr(self, other: $T) -> $T {
                $T(self.0 >> other.0)
            }
        }
        impl ShrAssign<$T> for $T {
            fn shr_assign(&mut self, other: $T) {
                self.0 >>= other.0;
            }
        }
    };
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn shr(self, other: $T) -> $T {
                $T(self.0 >> other.0)
            }
            pub fn shr_assign(&mut self, other: $T) {
                self.0 >>= other.0;
            }
        }
        impl Shr<$T> for $T {
            type Output = $T;
            fn shr(self, other: $T) -> $T {
                $T(self.0 >> other.0)
            }
        }
        impl ShrAssign<$T> for $T {
            fn shr_assign(&mut self, other: $T) {
                self.0 >>= other.0;
            }
        }
        impl Shr<$U> for $T {
            type Output = $T;
            fn shr(self, other: $U) -> $T {
                $T(self.0 >> other)
            }
        }
        impl ShrAssign<$U> for $T {
            fn shr_assign(&mut self, other: $U) {
                self.0 >>= other;
            }
        }
        impl Shr<$T> for $U {
            type Output = $U;
            fn shr(self, other: $T) -> $U {
                self >> other.0
            }
        }
        impl ShrAssign<$T> for $U {
            fn shr_assign(&mut self, other: $T) {
                *self >>= other.0;
            }
        }
    };
    ($T:ident,$U:ident, $v:literal) => {
        impl $T {
            pub fn shr(self, other: $T) -> $T {
                $T(self.$v >> other.$v)
            }
            pub fn shr_assign(&mut self, other: $T) {
                self.$v >>= other.$v;
            }
        }
        impl Shr<$U> for $T {
            type Output = $T;
            fn shr(self, other: $U) -> $T {
                $T(self.$v >> other)
            }
        }
        impl ShrAssign<$U> for $T {
            fn shr_assign(&mut self, other: $U) {
                self.$v >>= other;
            }
        }
        impl Shr<$T> for $U {
            type Output = $U;
            fn shr(self, other: $T) -> $U {
                $U(self >> other.$v)
            }
        }
        impl ShrAssign<$T> for $U {
            fn shr_assign(&mut self, other: $T) {
                self >>= other.$v;
            }
        }
    };
    ($T:ident,$U:ident,$v1:literal,$v2:literal) => {
        impl $T {
            pub fn shr(self, other: $T) -> $T {
                $T(self.$v1 >> other.$v1)
            }
            pub fn shr_assign(&mut self, other: $T) {
                self.$v1 >>= other.$v1;
            }
        }
        impl Shr<$T> for $T {
            type Output = $T;
            fn shr(self, other: $T) -> $T {
                $T(self.$v1 >> other.$v1)
            }
        }
        impl ShrAssign<$T> for $T {
            fn shr_assign(&mut self, other: $T) {
                self.$v1 >>= other.$v1;
            }
        }
        impl Shr<$U> for $T {
            type Output = $T;
            fn shr(self, other: $U) -> $T {
                $T(self.$v1 >> other.$v2)
            }
        }
        impl ShrAssign<$U> for $T {
            fn shr_assign(&mut self, other: $U) {
                self.$v1 >>= other.$v2;
            }
        }
        impl Shr<$T> for $U {
            type Output = $U;
            fn shr(self, other: $T) -> $U {
                self.$v2 >> other.$v1
            }
        }
        impl ShrAssign<$T> for $U {
            fn shr_assign(&mut self, other: $T) {
                self.$v2 >>= other.$v1;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_max {
    ($T:ident) => {
        impl $T {
            pub fn max(self, other: $T) -> $T {
                $T(self.0.max(other.0))
            }
        }
    };
    ($T:ident,$v:literal) => {
        impl $T {
            pub fn max(self, other: $T) -> $T {
                $T(self.$v.max(other.$v))
            }
        }
    };
    ($T:ident,$U:ident) => {
        impl $T {
            pub fn max(self, other: $U) -> $T {
                $T(self.0.max(other))
            }
        }
    };
    ($T:ident,$U:ident, $v:literal) => {
        impl $T {
            pub fn max(self, other: $U) -> $T {
                $T(self.$v.max(other))
            }
        }
    };
    ($T:ident,$U:ident,$v1:literal,$v2:literal) => {
        impl $T {
            pub fn max(self, other: $U) -> $T {
                $T(self.$v1.max(other.$v2))
            }
        }
    };
}
#[macro_export]
macro_rules! impl_min {
    ($T:ident) => {
        impl $T {
            pub fn min(self, other: $T) -> $T {
                $T(self.0.min(other.0))
            }
        }
    };
    ($T:ident,$v:literal) => {
        impl $T {
            pub fn min(self, other: $T) -> $T {
                $T(self.$v.min(other.$v))
            }
        }
    };
    ($T:ident,$U:ident) => {
        impl $T {
            pub fn min(self, other: $U) -> $T {
                $T(self.0.min(other))
            }
        }
    };
    ($T:ident,$U:ident, $v:literal) => {
        impl $T {
            pub fn min(self, other: $U) -> $T {
                $T(self.$v.min(other))
            }
        }
    };
    ($T:ident,$U:ident,$v1:literal,$v2:literal) => {
        impl $T {
            pub fn min(self, other: $U) -> $T {
                $T(self.$v1.min(other.$v2))
            }
        }
    };
}
#[macro_export]
macro_rules! impl_clamp {
    ($T:ident) => {
        impl $T {
            pub fn clamp(self, min: $T, max: $T) -> $T {
                $T(self.0.clamp(min.0, max.0))
            }
        }
    };
    ($T:ident,$v:literal) => {
        impl $T {
            pub fn clamp(self, min: $T, max: $T) -> $T {
                $T(self.$v.clamp(min.$v, max.$v))
            }
        }
    };
    ($T:ident,$U:ident) => {
        impl $T {
            pub fn clamp(self, min: $U, max: $U) -> $T {
                $T(self.0.clamp(min, max))
            }
        }
    };
    ($T:ident,$v1:literal,$v2:literal) => {
        impl $T {
            pub fn clamp(self, min: $T, max: $T) -> $T {
                $T(self.$v1.clamp(min.$v2, max.$v2))
            }
        }
    };
    ($T:ident,$U:ident, $v:literal) => {
        impl $T {
            pub fn clamp(self, min: $U, max: $U) -> $T {
                $T(self.$v.clamp(min, max))
            }
        }
    };
    ($T:ident,$U:ident,$v1:literal,$v2:literal) => {
        impl $T {
            pub fn clamp(self, min: $U, max: $U) -> $T {
                $T(self.$v1.clamp(min.$v2, max.$v2))
            }
        }
    };
}
#[macro_export]
macro_rules! impl_size_of {
    ($T:ident) => {
        impl $T {
            pub const fn size_of() -> usize {
                size_of::<$T>()
            }
        }
    };
    ($T:ident,$U:ident) => {
        impl $T {
            pub const fn size_of() -> usize {
                size_of::<$U>()
            }
        }
    };
}
#[macro_export]
macro_rules! impl_abs_diff {
    ($T:ident) => {
        impl $T {
            pub fn abs_diff(self, other: $T) -> $T {
                $T(self.0.abs_diff(other.0))
            }
        }
    };
    ($T:ident,$v:literal) => {
        impl $T {
            pub fn abs_diff(self, other: $T) -> $T {
                $T(self.$v.abs_diff(other.$v))
            }
        }
    };
    ($T:ident,$U:ident) => {
        impl $T {
            pub fn abs_diff(self, other: $U) -> $T {
                $T(self.0.abs_diff(other))
            }
        }
    };
    ($T:ident,$U:ident, $v:literal) => {
        impl $T {
            pub fn abs_diff(self, other: $U) -> $T {
                $T(self.$v.abs_diff(other))
            }
        }
    };
    ($T:ident,$U:ident,$v1:literal,$v2:literal) => {
        impl $T {
            pub fn abs_diff(self, other: $U) -> $T {
                $T(self.$v1.abs_diff(other.$v1), self.$v2.abs_diff(other.$v2))
            }
        }
    };
}
#[macro_export]
macro_rules! impl_index {
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn index(&self, index: usize) -> &$U {
                &self.0[index]
            }
            pub fn index_mut(&mut self, index: usize) -> &mut $U {
                &mut self.0[index]
            }
        }
        impl Index<usize> for $T {
            type Output = $U;
            fn index(&self, index: usize) -> &$U {
                &self.0[index]
            }
        }
        impl IndexMut<usize> for $T {
            fn index_mut(&mut self, index: usize) -> &mut $U {
                &mut self.0[index]
            }
        }
    };
    ($T:ident, $U:ident, $v:literal) => {
        impl $T {
            pub fn index(&self, index: usize) -> &$U {
                &self.$v[index]
            }
            pub fn index_mut(&mut self, index: usize) -> &mut $U {
                &mut self.$v[index]
            }
        }
        impl Index<usize> for $T {
            type Output = $U;
            fn index(&self, index: usize) -> &$U {
                &self.$v[index]
            }
        }
        impl IndexMut<usize> for $T {
            fn index_mut(&mut self, index: usize) -> &mut $U {
                &mut self.$v[index]
            }
        }
    };
}
#[macro_export]
macro_rules! impl_into {
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn into(self) -> $U {
                self.0
            }
        };
        impl Into<$U> for $T {
            fn into(self) -> $U {
                self.0
            }
        }
    };
    ($T:ident, $U:ident, $v:literal) => {
        impl $T {
            pub fn into(self) -> $U {
                self.$v
            }
        }
        impl Into<$U> for $T {
            fn into(self) -> $U {
                self.$v
            }
        }
    };
    ($T:ident, $U:ident, $v1:literal, $v2:literal) => {
        impl $T {
            pub fn into(self) -> $U {
                $U(self.$v1, self.$v2)
            }
        }
        impl Into<$U> for $T {
            fn into(self) -> $U {
                $U(self.$v1, self.$v2)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_as_mut {
    ($T:ident) => {
        impl $T {
            pub fn as_mut(&mut self) -> &mut $T {
                $T(&mut self.0)
            }
        }
        impl AsMut<$T> for $T {
            fn as_mut(&mut self) -> &mut $T {
                $T(&mut self.0)
            }
        }
    };
    ($T:ident, $v:literal) => {
        impl $T {
            pub fn as_mut(&mut self) -> &mut $T {
                $T(&mut self.$v)
            }
        }
        impl AsMut<$T> for $T {
            fn as_mut(&mut self) -> &mut $T {
                $T(&mut self.$v)
            }
        }
    };
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn as_mut(&mut self) -> &mut $U {
                &mut self.0
            }
        }
        impl AsMut<$U> for $T {
            fn as_mut(&mut self) -> &mut $U {
                &mut self.0
            }
        }
    };
    ($T:ident, $U:ident, $v:literal) => {
        impl $T {
            pub fn as_mut(&mut self) -> &mut $U {
                &mut self.$v
            }
        }
        impl AsMut<$U> for $T {
            fn as_mut(&mut self) -> &mut $U {
                &mut self.$v
            }
        }
    };
}
#[macro_export]
macro_rules! impl_display {
    ($T:ident) => {
        impl $T {
            pub fn display(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{}", self.0)
            }
        }
        impl Display for $T {
            fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{}", self.0)
            }
        }
    };
    ($T:ident, $v:literal) => {
        impl $T {
            pub fn display(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{}", self.$v)
            }
        }
        impl Display for $T {
            fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{}", self.$v)
            }
        }
    };
    ($T:ident, $v1:literal, $v2:literal) => {
        impl $T {
            pub fn display(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, $v2, self.$v1)
            }
        }
        impl Display for $T {
            fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, $v2, self.$v1)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_binary {
    ($T:ident) => {
        impl $T {
            pub fn binary(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{:b}", self.0)
            }
        }
        impl Binary for $T {
            fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{:b}", self.0)
            }
        }
    };
    ($T:ident, $v:literal) => {
        impl $T {
            pub fn binary(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{:b}", self.$v)
            }
        }
        impl Binary for $T {
            fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{:b}", self.$v)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_hex {
    ($T:ident) => {
        impl $T {
            pub fn hex(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{:x}", self.0)
            }
        }
        impl Hex for $T {
            fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{:x}", self.0)
            }
        }
    };
    ($T:ident, $v:literal) => {
        impl $T {
            pub fn hex(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{:x}", self.$v)
            }
        }
        impl Hex for $T {
            fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{:x}", self.$v)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_octal {
    ($T:ident) => {
        impl $T {
            pub fn octal(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{:o}", self.0)
            }
        }
        impl Octal for $T {
            fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{:o}", self.0)
            }
        }
    };
    ($T:ident, $v:literal) => {
        impl $T {
            pub fn octal(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{:o}", self.$v)
            }
        }
        impl Octal for $T {
            fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
                write!(f, "{:o}", self.$v)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_from_str {
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
                Ok($T($U::from_str(s)?))
            }
        }
        impl FromStr for $T {
            type Err = ParseIntError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($T($U::from_str(s)?))
            }
        }
    };
    ($T:ident, $U:ident, $v:literal) => {
        impl $T {
            pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
                Ok($T($U::from_str(&s[$v..])?))
            }
        }
        impl FromStr for $T {
            type Err = ParseIntError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($T($U::from_str(&s[$v..])?))
            }
        }
    };
    ($T:ident, $U:ident, $v1:literal, $v2:literal) => {
        impl $T {
            pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
                Ok($T($U::from_str(&s[$v1..$v2])?))
            }
        }
        impl FromStr for $T {
            type Err = ParseIntError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($T($U::from_str(&s[$v1..$v2])?))
            }
        }
    };
}
#[macro_export]
macro_rules! impl_into_iter {
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn into_iter(self) -> IntoIter<$U> {
                self.0.into_iter()
            }
            pub fn iter(&self) -> Iter<$U> {
                self.0.iter()
            }
            pub fn iter_mut(&mut self) -> IterMut<$U> {
                self.0.iter_mut()
            }
        }
        impl IntoIterator for $T {
            type Item = $U;
            type IntoIter = IntoIter<$U>;
            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }
        impl<'a> IntoIterator for &'a $T {
            type Item = &'a $U;
            type IntoIter = Iter<'a, $U>;
            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }
        impl<'a> IntoIterator for &'a mut $T {
            type Item = &'a mut $U;
            type IntoIter = IterMut<'a, $U>;
            fn into_iter(self) -> Self::IntoIter {
                self.0.iter_mut()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_ord {
    ($T:ident) => {
        impl $T {
            pub fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }
        impl Ord for $T {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }
        impl PartialOrd for $T {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
    };
    ($T:ident, $v:literal) => {
        impl $T {
            pub fn cmp(&self, other: &Self) -> Ordering {
                self.$v.cmp(&other.$v)
            }
        }
        impl Ord for $T {
            fn cmp(&self, other: &Self) -> Ordering {
                self.$v.cmp(&other.$v)
            }
        }
        impl PartialOrd for $T {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
    };
    ($T:ident, $v1:literal, $v2:literal) => {
        impl $T {
            pub fn cmp(&self, other: &Self) -> Ordering {
                self.$v1.cmp(&other.$v1) && self.$v2.cmp(&other.$v2)
            }
        }
        impl Ord for $T {
            fn cmp(&self, other: &Self) -> Ordering {
                self.$v1.cmp(&other.$v1) && self.$v2.cmp(&other.$v2)
            }
        }
        impl PartialOrd for $T {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
    };
}
#[macro_export]
macro_rules! impl_eq {
    ($T:ident) => {
        impl $T {
            pub fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }
        impl Eq for $T {}
        impl PartialEq for $T {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }
    };
    ($T:ident, $v:literal) => {
        impl $T {
            pub fn eq(&self, other: &Self) -> bool {
                self.$v.eq(&other.$v)
            }
        }
        impl Eq for $T {}
        impl PartialEq for $T {
            fn eq(&self, other: &Self) -> bool {
                self.$v.eq(&other.$v)
            }
        }
    };
    ($T:ident,$v1:literal, $v2:literal) => {
        impl $T {
            pub fn eq(&self, other: &Self) -> bool {
                self.$v1.eq(&other.$v1) && self.$v2.eq(&other.$v2)
            }
        }
        impl Eq for $T {}
        impl PartialEq for $T {
            fn eq(&self, other: &Self) -> bool {
                self.$v1.eq(&other.$v1) && self.$v2.eq(&other.$v2)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_range_bounds {
    ($T:ident, $U:ident) => {
        impl core::ops::RangeBounds<$U> for $T {
            fn start_bound(&self) -> core::ops::Bound<&$U> {
                core::ops::Bound::Included(&self.0)
            }
            fn end_bound(&self) -> core::ops::Bound<&$U> {
                core::ops::Bound::Excluded(&self.0)
            }
        }
    };
    ($T:ident, $U:ident, $v:literal) => {
        impl $T {
            pub fn start(&self) -> Option<$U> {
                Some(self.$v)
            }
            pub fn end(&self) -> Option<$U> {
                Some(self.$v)
            }
        }
        impl RangeBounds for $T {
            fn start(&self) -> Option<&$U> {
                Some(&self.$v)
            }
            fn end(&self) -> Option<&$U> {
                Some(&self.$v)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_from {
    ($T:ident, $U:ident) => {
        impl From<$T> for $U {
            fn from(value: $T) -> $U {
                value.0
            }
        }
        impl From<$U> for $T {
            fn from(value: $U) -> $T {
                $T(value)
            }
        }
    };
    ($T:ident, $U:ident, $v:literal) => {
        impl From<$T> for $U {
            fn from(value: $T) -> $U {
                value.$v
            }
        }
        impl From<$U> for $T {
            fn from(value: $U) -> $T {
                $T(value)
            }
        }
    };
    ($T:ident, $U:ident, $v1:literal, $v2:literal) => {
        impl From<$T> for $U {
            fn from(value: $T) -> $U {
                $value.$v1
            }
        }
        impl From<$U> for $T {
            fn from(value: $U) -> $T {
                $T(value.$v2)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_from_tuple {
    ($T:ident, $U:ident) => {
        impl From<$T> for $U {
            fn from(value: $T) -> Self {
                let mut result = Self::default();
                for (i, v) in value.iter().enumerate() {
                    result[i] = *v;
                }
                result
            }
        }
    };
}
#[macro_export]
macro_rules! impl_tuple_from {
    ($T:ident, $U:ident) => {
        impl From<$T> for $U {
            fn from(value: $T) -> $U {
                let size_t = size_of::<$T>();
                let size_u = size_of::<$U>();
                match size_t {
                    1 => match size_u {
                        1 => (value.0 as u8),
                        _ => panic!("Invalid size"),
                    },
                    2 => match size_u {
                        1 => ((value.0 as u16 >> 8) as u8, value.1 as u8),
                        2 => (value.0 as u16),
                        _ => panic!("Invalid size"),
                    },
                    4 => match size_u {
                        1 => (
                            (value.0 as u32 >> 24) as u8,
                            (value.1 as u32 >> 16) as u8,
                            (value.2 as u32 >> 8) as u8,
                            value.3 as u8,
                        ),
                        2 => ((value.0 as u32 >> 8) as u16, value.1 as u16),
                        4 => (value.0 as u32),
                        _ => panic!("Invalid size"),
                    },
                    8 => match size_u {
                        1 => (
                            (value.0 as u64 >> 56) as u8,
                            (value.1 as u64 >> 48) as u8,
                            (value.2 as u64 >> 40) as u8,
                            (value.3 as u64 >> 32) as u8,
                            (value.4 as u64 >> 24) as u8,
                            (value.5 as u64 >> 16) as u8,
                            (value.6 as u64 >> 8) as u8,
                            value.7 as u8,
                        ),
                        2 => (
                            (value.0 as u64 >> 48) as u16,
                            (value.1 as u64 >> 32) as u16,
                            (value.2 as u64 >> 16) as u16,
                            value.3 as u16,
                        ),
                        4 => ((value.0 as u64 >> 32) as u32, value.1 as u32),
                        8 => (value.0 as u64),
                        _ => panic!("Invalid size"),
                    },
                    16 => match size_u {
                        1 => (
                            (value.0 as u128 >> 120) as u8,
                            (value.1 as u128 >> 112) as u8,
                            (value.2 as u128 >> 104) as u8,
                            (value.3 as u128 >> 96) as u8,
                            (value.4 as u128 >> 88) as u8,
                            (value.5 as u128 >> 80) as u8,
                            (value.6 as u128 >> 72) as u8,
                            (value.7 as u128 >> 64) as u8,
                            (value.8 as u128 >> 56) as u8,
                            (value.9 as u128 >> 48) as u8,
                            (value.10 as u128 >> 40) as u8,
                            (value.11 as u128 >> 32) as u8,
                            (value.12 as u128 >> 24) as u8,
                            (value.13 as u128 >> 16) as u8,
                            (value.14 as u128 >> 8) as u8,
                            value.0 as u8,
                        ),
                        2 => (
                            (value.0 as u128 >> 112) as u16,
                            (value.1 as u128 >> 96) as u16,
                            (value.2 as u128 >> 80) as u16,
                            (value.3 as u128 >> 64) as u16,
                            (value.4 as u128 >> 48) as u16,
                            (value.5 as u128 >> 32) as u16,
                            (value.6 as u128 >> 16) as u16,
                            value.7 as u16,
                        ),
                        4 => (
                            (value.0 as u128 >> 96) as u32,
                            (value.1 as u128 >> 64) as u32,
                            (value.2 as u128 >> 32) as u32,
                            value.3 as u32,
                        ),
                        8 => ((value.0 as u128 >> 64) as u64, (value.1 as u64)),
                        16 => (value.0 as u128),
                        _ => panic!("Invalid size"),
                    },
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vec {
    ($T:ident,$U:ident) => {
        impl $T {
            pub fn append(&mut self, other: &mut $T) {
                self.0.append(&mut other.0);
            }
            pub fn clear(&mut self) {
                self.0.clear();
            }
            pub fn contains(&self, item: $U) -> bool {
                self.0.contains(&item)
            }
            pub fn dedup(&mut self) {
                self.0.dedup();
            }
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
            pub fn len(&self) -> usize {
                self.0.len()
            }
            pub fn push(&mut self, item: $U) {
                self.0.push(item);
            }
            pub fn pop(&mut self) -> Option<$U> {
                self.0.pop()
            }
            pub fn remove(&mut self, index: usize) -> $U {
                self.0.remove(index)
            }
            pub fn shrink_to_fit(&mut self) {
                self.0.shrink_to_fit();
            }
            pub fn split_at(&self, index: usize) -> ($T, $T) {
                let split = self.0.split_at(index);
                ($T(split.0.to_vec()), $T(split.1.to_vec()))
            }
            pub fn split_at_mut(&mut self, index: usize) -> ($T, $T) {
                let split = self.0.split_at_mut(index);
                ($T(split.0.to_vec()), $T(split.1.to_vec()))
            }
            pub fn sort(&mut self) {
                self.0.sort();
            }
            pub fn sort_by(&mut self, compare: fn(&$U, &$U) -> Ordering) {
                self.0.sort_by(compare);
            }
            pub fn swap(&mut self, index1: usize, index2: usize) {
                self.0.swap(index1, index2);
            }
            pub fn swap_remove(&mut self, index: usize) -> $U {
                self.0.swap_remove(index)
            }
            pub fn with_capacity(capacity: usize) -> Self {
                Self(Vec::with_capacity(capacity))
            }
        }
    };
    ($T:ident, $U:ident, $v:literal) => {
        impl $T {
            pub fn append(&mut self, other: &mut $T) {
                self.$v.append(&mut other.$v);
            }
            pub fn clear(&mut self) {
                self.$v.clear();
            }
            pub fn contains(&self, item: $U) -> bool {
                self.$v.contains(&item)
            }
            pub fn dedup(&mut self) {
                self.$v.dedup();
            }
            pub fn is_empty(&self) -> bool {
                self.$v.is_empty()
            }
            pub fn len(&self) -> usize {
                self.$v.len()
            }
            pub fn push(&mut self, item: $U) {
                self.$v.push(item);
            }
            pub fn pop(&mut self) -> Option<$U> {
                self.$v.pop()
            }
            pub fn remove(&mut self, index: usize) -> $U {
                self.$v.remove(index)
            }
            pub fn shrink_to_fit(&mut self) {
                self.$v.shrink_to_fit();
            }
            pub fn split_at(&self, index: usize) -> ($T, $T) {
                let split = self.$v.split_at(index);
                ($T(split.0.to_vec()), $T(split.1.to_vec()))
            }
            pub fn split_at_mut(&mut self, index: usize) -> ($T, $T) {
                let split = self.$v.split_at_mut(index);
                ($T(split.0.to_vec()), $T(split.1.to_vec()))
            }
            pub fn sort(&mut self) {
                self.$v.sort();
            }
            pub fn sort_by(&mut self, compare: fn(&$U, &$U) -> Ordering) {
                self.$v.sort_by(compare);
            }
            pub fn swap(&mut self, index1: usize, index2: usize) {
                self.$v.swap(index1, index2);
            }
            pub fn swap_remove(&mut self, index: usize) -> $U {
                self.$v.swap_remove(index)
            }
            pub fn with_capacity(capacity: usize) -> Self {
                Self(Vec::with_capacity(capacity))
            }
        }
    };
}
#[macro_export]
macro_rules! impl_octet_quad {
    ($T:ident) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        pub struct $T(u8, u8, u8, u8);
        impl $T {
            pub fn new() -> Self {
                Self(0, 0, 0, 0)
            }
            pub fn octets(&self) -> [u8; 4] {
                [self.0, self.1, self.2, self.3]
            }
            pub fn from_octets(octets: [u8; 4]) -> Self {
                Self(octets[0], octets[1], octets[2], octets[3])
            }
            pub fn size_of() -> usize {
                4
            }
            pub fn to_string(&self) -> String {
                format!("{}.{}.{}.{}", self.0, self.1, self.2, self.3)
            }
            pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
                let mut octets = [0u8; 4];
                for (i, ss) in s.split('.').enumerate() {
                    octets[i] = u8::from_str(ss)?;
                }
                Ok(Self::from_octets(octets))
            }
            pub fn to_vec(&self) -> Vec<u8> {
                self.octets().to_vec()
            }
            pub fn from_vec(bytes: Vec<u8>) -> Self {
                Self::from_octets(bytes.try_into().unwrap())
            }
            pub fn index(&self, index: usize) -> u8 {
                match index {
                    0 => self.0,
                    1 => self.1,
                    2 => self.2,
                    3 => self.3,
                    _ => panic!("index out of bounds"),
                }
            }
            pub fn index_mut(&mut self, index: usize) -> &mut u8 {
                match index {
                    0 => &mut self.0,
                    1 => &mut self.1,
                    2 => &mut self.2,
                    3 => &mut self.3,
                    _ => panic!("index out of bounds"),
                }
            }
        }
        impl Display for $T {
            fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
                write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
            }
        }
        impl FromStr for $T {
            type Err = ParseIntError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::from_str(s)
            }
        }
        impl Ord for $T {
            fn cmp(&self, other: &Self) -> Ordering {
                self.octets().cmp(&other.octets())
            }
        }
        impl PartialOrd for $T {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl From<[u8; 4]> for $T {
            fn from(octets: [u8; 4]) -> Self {
                Self(octets[0], octets[1], octets[2], octets[3])
            }
        }
        impl From<&[u8]> for $T {
            fn from(octets: &[u8]) -> Self {
                Self(octets[0], octets[1], octets[2], octets[3])
            }
        }
        impl From<$T> for u32 {
            fn from(value: $T) -> u32 {
                (value.0 as u32) << 24
                    | (value.1 as u32) << 16
                    | (value.2 as u32) << 8
                    | (value.3 as u32)
            }
        }
        impl Index<usize> for $T {
            type Output = u8;
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.0,
                    1 => &self.1,
                    2 => &self.2,
                    3 => &self.3,
                    _ => panic!("index out of bounds"),
                }
            }
        }
        impl IndexMut<usize> for $T {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.0,
                    1 => &mut self.1,
                    2 => &mut self.2,
                    3 => &mut self.3,
                    _ => panic!("index out of bounds"),
                }
            }
        }
        impl std::ops::RangeBounds<usize> for $T {
            fn start_bound(&self) -> core::ops::Bound<&usize> {
                core::ops::Bound::Included(&0)
            }
            fn end_bound(&self) -> core::ops::Bound<&usize> {
                core::ops::Bound::Included(&3)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_range {
    ($T:ident, $U:ident) => {
        impl $T {
            pub fn start(&self) -> &$U {
                &self.0
            }
            pub fn end(&self) -> &$U {
                &self.1
            }
            pub fn set_start(&mut self, start: $U) {
                self.0 = start;
            }
            pub fn set_end(&mut self, end: $U) {
                self.1 = end;
            }
            pub fn contains(&self, value: $U) -> bool {
                self.0 <= value && value <= self.1
            }
            pub fn len(&self) -> usize {
                (self.1 - self.0) as usize
            }
            pub fn is_empty(&self) -> bool {
                self.0 > self.1
            }
            pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
                let mut parts = s.split('-');
                let start = parts.next().unwrap();
                let end = parts.next().unwrap();
                Ok(Self($U::from_str(start)?, $U::from_str(end)?))
            }
            pub fn to_string(&self) -> String {
                format!("{}-{}", self.0.to_string(), self.1.to_string())
            }
            pub const fn size_of() -> usize {
                size_of::<$U>() * 2
            }
            pub fn from_be_bytes(bytes: [u8; Self::size_of()]) -> Self {
                Self(
                    $U::from_be_bytes(bytes[0..Self::size_of() / 2].try_into().unwrap()),
                    $U::from_be_bytes(
                        bytes[(Self::size_of() / 2)..(Self::size_of())]
                            .try_into()
                            .unwrap(),
                    ),
                )
            }
            pub fn to_be_bytes(&self) -> [u8; Self::size_of()] {
                let mut bytes = [0; Self::size_of()];
                for i in 0..Self::size_of() / 2 {
                    bytes[i] = self.0.to_be_bytes()[i];
                    bytes[i + Self::size_of() / 2] = self.1.to_be_bytes()[i];
                }
                bytes
            }
            pub fn from_vec(vec: Vec<u8>) -> Self {
                Self(
                    $U::from_be_bytes(vec[0..Self::size_of() / 2].try_into().unwrap()),
                    $U::from_be_bytes(
                        vec[(Self::size_of() / 2)..(Self::size_of())]
                            .try_into()
                            .unwrap(),
                    ),
                )
            }
            pub fn to_vec(&self) -> Vec<u8> {
                let mut vec = self.0.to_be_bytes().to_vec();
                vec.extend_from_slice(&self.1.to_be_bytes());
                vec
            }
        }
        impl Ord for $T {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0).then(self.1.cmp(&other.1))
            }
        }
        impl PartialOrd for $T {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(&other))
            }
        }
    };
}
#[macro_export]
macro_rules! impl_vec_op {
    ($T:ident, $U:ident, $tr:tt, $op:tt) => {
        impl $T {
            pub fn $op(&self, other: $T) -> $T {
                let mut result = self.clone();
                for (i, item) in other.0.iter().enumerate() {
                    result.0[i] = self.0[i].$op(item);
                }
                result
            }
        }
        impl $tr for $T {
            type Output = $T;
            fn $op(self, rhs: $T) -> Self::Output {
                let mut result = self.clone();
                for (i, item) in rhs.0.iter().enumerate() {
                    result.0[i] = self.0[i].$op(item);
                }
                result
            }
        }
        impl $tr<$U> for $T {
            type Output = $T;
            fn $op(self, rhs: $U) -> Self::Output {
                let mut result = self.clone();
                for (i, item) in rhs.iter().enumerate() {
                    result.0[i] = self.0[i].$op(item);
                }
                result
            }
        }
    };
}
#[macro_export]
macro_rules! impl_ops {
    ($T:ident) => {
        type T = $T;
        pub struct Op<T> {
            pub add: fn(T, T) -> T,
            pub sub: fn(T, T) -> T,
            pub mul: fn(T, T) -> T,
            pub div: fn(T, T) -> T,
            pub rem: fn(T, T) -> T,
            pub shl: fn(T, T) -> T,
            pub shr: fn(T, T) -> T,
            pub bit_and: fn(T, T) -> T,
            pub bit_or: fn(T, T) -> T,
            pub bit_xor: fn(T, T) -> T,
            pub not: fn(T) -> T,
            pub add_assign: fn(&mut T, T) -> T,
            pub sub_assign: fn(&mut T, T) -> T,
            pub mul_assign: fn(&mut T, T) -> T,
            pub div_assign: fn(&mut T, T) -> T,
            pub rem_assign: fn(&mut T, T) -> T,
            pub shl_assign: fn(&mut T, T) -> T,
            pub shr_assign: fn(&mut T, T) -> T,
            pub bit_and_assign: fn(&mut T, T) -> T,
            pub bit_or_assign: fn(&mut T, T) -> T,
            pub bit_xor_assign: fn(&mut T, T) -> T,
        }
        impl Op<T> {
            pub const fn new() -> Self {
                Self {
                    add: |x, y| x + y,
                    sub: |x, y| x - y,
                    mul: |x, y| x * y,
                    div: |x, y| x / y,
                    rem: |x, y| x % y,
                    shl: |x, y| x << y,
                    shr: |x, y| x >> y,
                    bit_and: |x, y| x & y,
                    bit_or: |x, y| x | y,
                    bit_xor: |x, y| x ^ y,
                    not: |x| !x,
                    add_assign: |x, y| {
                        *x += y;
                        *x
                    },
                    sub_assign: |x, y| {
                        *x -= y;
                        *x
                    },
                    mul_assign: |x, y| {
                        *x *= y;
                        *x
                    },
                    div_assign: |x, y| {
                        *x /= y;
                        *x
                    },
                    rem_assign: |x, y| {
                        *x %= y;
                        *x
                    },
                    shl_assign: |x, y| {
                        *x <<= y;
                        *x
                    },
                    shr_assign: |x, y| {
                        *x >>= y;
                        *x
                    },
                    bit_and_assign: |x, y| {
                        *x &= y;
                        *x
                    },
                    bit_or_assign: |x, y| {
                        *x |= y;
                        *x
                    },
                    bit_xor_assign: |x, y| {
                        *x ^= y;
                        *x
                    },
                }
            }
            pub fn str_op(&self, s: &str, x: T, y: T) -> T {
                match s {
                    "+" => (self.add)(x, y),
                    "-" => (self.sub)(x, y),
                    "*" => (self.mul)(x, y),
                    "/" => (self.div)(x, y),
                    "%" => (self.rem)(x, y),
                    "<<" => (self.shl)(x, y),
                    ">>" => (self.shr)(x, y),
                    "&" => (self.bit_and)(x, y),
                    "|" => (self.bit_or)(x, y),
                    "^" => (self.bit_xor)(x, y),
                    "!" => (self.not)(x),
                    _ => panic!("Invalid operation"),
                }
            }
            pub fn str_op_assign(&self, s: &str, x: &mut T, y: T) -> T {
                match s {
                    "+=" => (self.add_assign)(x, y),
                    "-=" => (self.sub_assign)(x, y),
                    "*=" => (self.mul_assign)(x, y),
                    "/=" => (self.div_assign)(x, y),
                    "%=" => (self.rem_assign)(x, y),
                    "<<=" => (self.shl_assign)(x, y),
                    ">>=" => (self.shr_assign)(x, y),
                    "&=" => (self.bit_and_assign)(x, y),
                    "|=" => (self.bit_or_assign)(x, y),
                    "^=" => (self.bit_xor_assign)(x, y),
                    _ => panic!("Invalid operation"),
                }
            }
            pub fn idx_op(&self, i: usize, x: T, y: T) -> T {
                match i {
                    0 => (self.add)(x, y),
                    1 => (self.sub)(x, y),
                    2 => (self.mul)(x, y),
                    3 => (self.div)(x, y),
                    4 => (self.rem)(x, y),
                    5 => (self.shl)(x, y),
                    6 => (self.shr)(x, y),
                    7 => (self.bit_and)(x, y),
                    8 => (self.bit_or)(x, y),
                    9 => (self.bit_xor)(x, y),
                    10 => (self.not)(x),
                    _ => panic!("Invalid operation"),
                }
            }
            pub fn idx_op_assign(&self, i: usize, x: &mut T, y: T) -> T {
                match i {
                    1 => (self.add_assign)(x, y),
                    2 => (self.sub_assign)(x, y),
                    3 => (self.mul_assign)(x, y),
                    4 => (self.div_assign)(x, y),
                    5 => (self.rem_assign)(x, y),
                    6 => (self.shl_assign)(x, y),
                    7 => (self.shr_assign)(x, y),
                    8 => (self.bit_and_assign)(x, y),
                    9 => (self.bit_or_assign)(x, y),
                    10 => (self.bit_xor_assign)(x, y),
                    _ => panic!("Invalid operation"),
                }
            }
        }
    };
}
