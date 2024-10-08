/*
Copyright (c) 2021 VMware, Inc.
SPDX-License-Identifier: MIT

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

impl<T, FB> FromFlatBuffer<FB> for ddlog_std::Ref<T>
where
    T: FromFlatBuffer<FB>,
{
    fn from_flatbuf(fb: FB) -> ::std::result::Result<Self, String> {
        Ok(ddlog_std::Ref::from(T::from_flatbuf(fb)?))
    }
}

impl<'b, T> ToFlatBuffer<'b> for ddlog_std::Ref<T>
where
    T: ToFlatBuffer<'b>,
{
    type Target = T::Target;

    fn to_flatbuf(&self, fbb: &mut fbrt::FlatBufferBuilder<'b>) -> Self::Target {
        std::ops::Deref::deref(self).to_flatbuf(fbb)
    }
}

impl<'b, T> ToFlatBufferTable<'b> for ddlog_std::Ref<T>
where
    T: ToFlatBufferTable<'b>,
{
    type Target = T::Target;

    fn to_flatbuf_table(
        &self,
        fbb: &mut fbrt::FlatBufferBuilder<'b>,
    ) -> fbrt::WIPOffset<Self::Target> {
        std::ops::Deref::deref(self).to_flatbuf_table(fbb)
    }
}

impl<'b, T> ToFlatBufferVectorElement<'b> for ddlog_std::Ref<T>
where
    T: ToFlatBufferVectorElement<'b>,
{
    type Target = T::Target;

    fn to_flatbuf_vector_element(&self, fbb: &mut fbrt::FlatBufferBuilder<'b>) -> Self::Target {
        std::ops::Deref::deref(self).to_flatbuf_vector_element(fbb)
    }
}

impl<'a, T, F> FromFlatBuffer<fbrt::Vector<'a, F>> for ddlog_std::Vec<T>
where
    T: Ord + FromFlatBuffer<F::Inner>,
    F: fbrt::Follow<'a> + 'a,
    <F as fbrt::Follow<'a>>::Inner: Debug,
{
    fn from_flatbuf(fb: fbrt::Vector<'a, F>) -> ::std::result::Result<Self, String> {
        let mut vec = ddlog_std::Vec::with_capacity(fb.len());
        for x in FBIter::from_vector(fb) {
            vec.push(T::from_flatbuf(x)?);
        }
        Ok(vec)
    }
}

// For scalar types, the FlatBuffers API returns slice instead of 'Vector'.
impl<'a, T> FromFlatBuffer<&'a [T]> for ddlog_std::Vec<T>
where
    T: Clone,
{
    fn from_flatbuf(fb: &'a [T]) -> ::std::result::Result<Self, String> {
        let mut vec = ddlog_std::Vec::with_capacity(fb.len());
        vec.extend_from_slice(fb);
        Ok(vec)
    }
}
impl<'b, T> ToFlatBuffer<'b> for ddlog_std::Vec<T>
where
    T: ToFlatBufferVectorElement<'b>,
{
    type Target = fbrt::WIPOffset<fbrt::Vector<'b, <T::Target as fbrt::Push>::Output>>;

    fn to_flatbuf(&self, fbb: &mut fbrt::FlatBufferBuilder<'b>) -> Self::Target {
        let vec: ::std::vec::Vec<T::Target> = self
            .iter()
            .map(|x| x.to_flatbuf_vector_element(fbb))
            .collect();
        fbb.create_vector(vec.as_slice())
    }
}

impl<'a, T, F> FromFlatBuffer<fbrt::Vector<'a, F>> for ddlog_std::Set<T>
where
    T: Ord + FromFlatBuffer<F::Inner>,
    F: fbrt::Follow<'a> + 'a,
    <F as fbrt::Follow<'a>>::Inner: Debug,
{
    fn from_flatbuf(fb: fbrt::Vector<'a, F>) -> ::std::result::Result<Self, String> {
        let mut set = ddlog_std::Set::new();
        for x in FBIter::from_vector(fb) {
            set.insert(T::from_flatbuf(x)?);
        }
        Ok(set)
    }
}

// For scalar types, the FlatBuffers API returns slice instead of 'Vector'.
impl<'a, T> FromFlatBuffer<&'a [T]> for ddlog_std::Set<T>
where
    T: Ord + Clone,
{
    fn from_flatbuf(fb: &'a [T]) -> ::std::result::Result<Self, String> {
        let mut set = ddlog_std::Set::new();
        for x in fb.iter() {
            set.insert(x.clone());
        }
        Ok(set)
    }
}

impl<'b, T> ToFlatBuffer<'b> for ddlog_std::Set<T>
where
    T: Ord + ToFlatBufferVectorElement<'b>,
{
    type Target = fbrt::WIPOffset<fbrt::Vector<'b, <T::Target as fbrt::Push>::Output>>;

    fn to_flatbuf(&self, fbb: &mut fbrt::FlatBufferBuilder<'b>) -> Self::Target {
        let vec: ::std::vec::Vec<T::Target> = self
            .iter()
            .map(|x| x.to_flatbuf_vector_element(fbb))
            .collect();
        fbb.create_vector(vec.as_slice())
    }
}

impl<'a, K, V, F> FromFlatBuffer<fbrt::Vector<'a, F>> for ddlog_std::Map<K, V>
where
    F: fbrt::Follow<'a> + 'a,
    <F as fbrt::Follow<'a>>::Inner: Debug,
    K: Ord,
    ddlog_std::tuple2<K, V>: FromFlatBuffer<F::Inner>,
{
    fn from_flatbuf(fb: fbrt::Vector<'a, F>) -> ::std::result::Result<Self, String> {
        let mut m = ddlog_std::Map::new();
        for x in FBIter::from_vector(fb) {
            let ddlog_std::tuple2(k, v) = <ddlog_std::tuple2<K, V>>::from_flatbuf(x)?;
            m.insert(k, v);
        }
        Ok(m)
    }
}

impl<'b, K, V, T> ToFlatBuffer<'b> for ddlog_std::Map<K, V>
where
    K: Ord + Clone,
    V: Clone,
    ddlog_std::tuple2<K, V>: ToFlatBufferVectorElement<'b, Target = T>,
    T: 'b + fbrt::Push + Copy,
{
    type Target = fbrt::WIPOffset<fbrt::Vector<'b, <T as fbrt::Push>::Output>>;

    fn to_flatbuf(&self, fbb: &mut fbrt::FlatBufferBuilder<'b>) -> Self::Target {
        let vec: ::std::vec::Vec<
            <ddlog_std::tuple2<K, V> as ToFlatBufferVectorElement<'b>>::Target,
        > = self
            .iter()
            .map(|ddlog_std::tuple2(k, v)| ddlog_std::tuple2(k, v).to_flatbuf_vector_element(fbb))
            .collect();
        fbb.create_vector(vec.as_slice())
    }
}
