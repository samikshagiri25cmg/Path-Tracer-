use {
    bytemuck::{Pod, Zeroable},
    std::ops,
};

macro_rules! impl_binary_op {
    ($op:tt : $method:ident => (
           $lhs_i:ident : $lhs_t:path,
           $rhs_i:ident : $rhs_t:path
        ) -> $return_t:path $body:block
    ) => {
        impl ops::$op<$rhs_t> for $lhs_t {
            type Output = $return_t;
            fn $method(self, $rhs_i: $rhs_t) -> $return_t {
                let $lhs_i = self;
                $body
            }
        }
        impl ops::$op<&$rhs_t> for $lhs_t {
            type Output = $return_t;
            fn $method(self, $rhs_i: &$rhs_t) -> $return_t {
                let $lhs_i = self;
                $body
            }
        }
        impl ops::$op<$rhs_t> for &$lhs_t {
            type Output = $return_t;
            fn $method(self, $rhs_i: $rhs_t) -> $return_t {
                let $lhs_i = self;
                $body
            }
        }
        impl ops::$op<&$rhs_t> for &$lhs_t {
            type Output = $return_t;
            fn $method(self, $rhs_i: &$rhs_t) -> $return_t {
                let $lhs_i = self;
                $body
            }
        }
    };
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Vec4([f32; 4]);

impl Default for Vec4 {
    fn default() -> Self {
        Self::zero()
    }
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec4 {
        Vec4([x, y, z, 0.])
    }

    pub fn all(v: f32) -> Vec4 {
        Vec4([v, v, v, 0.])
    }

    pub fn zero() -> Vec4 {
        Vec4([0., 0., 0., 0.])
    }

    pub fn one() -> Vec4 {
        Vec4([1., 1., 1., 0.])
    }

    #[inline(always)]
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    #[inline(always)]
    pub fn y(&self) -> f32 {
        self.0[1]
    }

    #[inline(always)]
    pub fn z(&self) -> f32 {
        self.0[2]
    }

    #[inline(always)]
    pub fn w(&self) -> f32 {
        self.0[3]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn dot(&self, rhs: &Vec4) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn normalized(self) -> Vec4 {
        self * self.length().recip()
    }
    pub fn cross(&self, rhs: &Vec4) -> Vec4 {
        Vec4([
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
            0.0,
        ])
    }
}

impl_binary_op!(Add : add => (lhs: Vec4, rhs: Vec4) -> Vec4 {
    Vec4([
        lhs.x() + rhs.x(),
        lhs.y() + rhs.y(),
        lhs.z() + rhs.z(),
        0.,
    ])
});

impl_binary_op!(Sub : sub => (lhs: Vec4, rhs: Vec4) -> Vec4 {
    Vec4([
        lhs.x() - rhs.x(),
        lhs.y() - rhs.y(),
        lhs.z() - rhs.z(),
        0.,
    ])
});

impl_binary_op!(Mul : mul => (lhs: Vec4, rhs: f32) -> Vec4 {
    Vec4([
        lhs.x() * rhs,
        lhs.y() * rhs,
        lhs.z() * rhs,
        0.,
    ])
});

impl_binary_op!(Div : div => (lhs: Vec4, rhs: f32) -> Vec4 {
    Vec4([
        lhs.x() / rhs,
        lhs.y() / rhs,
        lhs.z() / rhs,
        0.,
    ])
});

impl ops::AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::Neg for Vec4 {
    type Output = Vec4;
    fn neg(self) -> Self::Output {
        Vec4([
            -self.x(),
            -self.y(),
            -self.z(),
            -self.w(),
        ])
    }
}