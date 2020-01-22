use objc::runtime::Class;

use std::marker::PhantomData;
use INSObject;

pub trait INSOperation : INSObject {

}
pub struct NSBlockOperation {
    //value: PhantomData<T>,
}

pub trait INSBlockOperation: INSOperation {
}

object_impl!(NSBlockOperation);
