//!
//! # Common Types and Macros
//!

use lazy_static::lazy_static;
use ruc::*;
use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Cow, cmp::Ordering, convert::TryInto, env, fmt, fs, ops::Deref};

lazy_static! {
    ///
    pub static ref CACHE_DIR: String = env::var("FUNDB_DIR").unwrap_or_else(|_| "/tmp".to_owned());
}

///
#[macro_export]
macro_rules! try_twice {
    ($ops: expr) => {
        ruc::pnk!($ops.c(d!()).or_else(|e| {
            e.print();
            $ops.c(d!())
        }))
    };
}

///
#[macro_export]
macro_rules! unique_path {
    () => {
        format!(
            "{}/.fundb/{}/{}_{}_{}_{}",
            *$crate::helper::CACHE_DIR,
            ts!(),
            file!(),
            line!(),
            column!(),
            rand::random::<u32>()
        )
    };
}

///
#[macro_export]
macro_rules! new_vecx {
    ($ty: ty, $in_mem_cnt: expr) => {
        $crate::new_vecx_custom!($ty, $in_mem_cnt, false)
    };
    ($ty: ty) => {
        $crate::new_vecx_custom!($ty, None, false)
    };
    ($in_mem_cnt: expr) => {
        $crate::new_vecx_custom!($in_mem_cnt, false)
    };
    () => {
        $crate::new_vecx_custom!(false)
    };
}

///
#[macro_export]
macro_rules! new_vecx_custom {
    ($ty: ty, $in_mem_cnt: expr, $is_tmp: expr) => {{
        let obj: $crate::Vecx<$ty> = $crate::try_twice!($crate::Vecx::new(
            $crate::unique_path!(),
            Some($in_mem_cnt)
            $is_tmp,
        ));
        obj
    }};
    ($ty: ty, $is_tmp: expr) => {{
        let obj: $crate::Vecx<$ty> =
            $crate::try_twice!($crate::Vecx::new($crate::unique_path!(), None, $is_tmp));
        obj
    }};
    ($in_mem_cnt: expr, $is_tmp: expr) => {
        $crate::try_twice!($crate::Vecx::new($crate::unique_path!(), Some($in_mem_cnt), $is_tmp))
    };
    ($is_tmp: expr) => {
        $crate::try_twice!($crate::Vecx::new($crate::unique_path!(), None, $is_tmp))
    };
}

///
#[macro_export]
macro_rules! new_mapx {
    ($ty: ty, $in_mem_cnt: expr) => {
        $crate::new_mapx_custom!($ty, $in_mem_cnt, false)
    };
    ($ty: ty) => {
        $crate::new_mapx_custom!($ty, None, false)
    };
    ($in_mem_cnt: expr) => {
        $crate::new_mapx_custom!($in_mem_cnt, false)
    };
    () => {
        $crate::new_mapx_custom!(false)
    };
}

///
#[macro_export]
macro_rules! new_mapx_custom {
    ($ty: ty, $in_mem_cnt: expr, $is_tmp: expr) => {{
        let obj: $crate::Mapx<$ty> = $crate::try_twice!($crate::Mapx::new(
            $crate::unique_path!(),
            $in_mem_cnt,
            $is_tmp,
        ));
        obj
    }};
    ($ty: ty, $is_tmp: expr) => {{
        let obj: $crate::Mapx<$ty> =
            $crate::try_twice!($crate::Mapx::new($crate::unique_path!(), None, $is_tmp,));
        obj
    }};
    ($in_mem_cnt: expr, $is_tmp: expr) => {
        $crate::try_twice!($crate::Mapx::new(
            $crate::unique_path!(),
            $in_mem_cnt,
            $is_tmp
        ))
    };
    ($is_tmp: expr) => {
        $crate::try_twice!($crate::Mapx::new($crate::unique_path!(), None, $is_tmp,))
    };
}

////////////////////////////////////////////////////////////////////////////////
// Begin of the implementation of Value(returned by `self.get`) for Vecx/Mapx //
/******************************************************************************/

/// Returned by `.get(...)`
#[derive(Eq, Debug, Clone)]
pub struct Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    value: Cow<'a, V>,
}

impl<'a, V> Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    pub(crate) fn new(value: Cow<'a, V>) -> Self {
        Value { value }
    }

    /// Comsume the ownship and get the inner value.
    pub fn into_inner(self) -> Cow<'a, V> {
        self.value
    }
}

impl<'a, V> Deref for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref()
    }
}

impl<'a, V> PartialEq for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn eq(&self, other: &Value<'a, V>) -> bool {
        self.value.eq(&other.clone().into_inner())
    }
}

impl<'a, V> PartialEq<V> for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn eq(&self, other: &V) -> bool {
        self.value.clone().into_owned().eq(other)
    }
}

impl<'a, V> PartialOrd<V> for Value<'a, V>
where
    V: fmt::Debug + Clone + Eq + PartialEq + Ord + PartialOrd + Serialize + DeserializeOwned,
{
    fn partial_cmp(&self, other: &V) -> Option<Ordering> {
        self.value.clone().into_owned().partial_cmp(other)
    }
}

impl<'a, V> From<V> for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn from(v: V) -> Self {
        Value{value: Cow::Owned(v)}
    }
}

impl<'a, V> From<Cow<'a, V>> for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn from(v: Cow<'a, V>) -> Self {
        Value{value: v}
    }
}

impl<'a, V> From<Value<'a, V>> for Cow<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn from(v: Value<'a, V>) -> Self {
        v.into_inner()
    }
}

impl<'a, V> From<&V> for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn from(v: &V) -> Self {
        Value{value: Cow::Owned(v.clone())}
    }
}

/****************************************************************************/
// End of the implementation of Value(returned by `self.get`) for Vecx/Mapx //
//////////////////////////////////////////////////////////////////////////////

#[inline(always)]
pub(crate) fn sled_open(path: &str, is_tmp: bool) -> Result<sled::Db> {

    let mut new_path = path;
    if is_tmp {
        new_path = CACHE_DIR.as_str();
    }

    sled::open(new_path).map_err(|e| 
            SimpleError::new(SimpleMsg::new(e.to_string(), std::file!(), std::line!(), std::column!()), None)
                .into()
    )
}

#[inline(always)]
pub(crate) fn read_db_len(path: &str) -> Result<usize> {

    let contents = fs::read(path);
    match contents {
        Ok(v) => {
            if v.is_empty() || v.len() != 8 {
                return Ok(0);
            }

            let ay:[u8;8] = v.try_into().expect("unknown error");

            Ok(usize::from_ne_bytes(ay))
        },
        Err(_e) => Ok(0),
    }
}

#[inline(always)]
pub(crate) fn write_db_len(path: &str, len: usize) -> Result<()> {
    let ret = fs::write(path, len.to_ne_bytes());
    match ret {
        Ok(t) => Ok(t),
        Err(e) => Err(SimpleError::new(SimpleMsg::new(e.to_string(), std::file!(), std::line!(), std::column!()), None)
        .into()),
    }
}
