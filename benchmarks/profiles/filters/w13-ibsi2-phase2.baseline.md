# Profile: filters/w13-ibsi2-phase2.baseline

- Command: `target/bench-filters/baseline/w13_ibsi2_features --nocapture`
- Note: baseline binary built from 9e334b7 plus others' uncommitted edits at the time; the 314 s test run is killed after the first seconds of sampling, so the profile covers the opening cases (A configuration)
- Date: 2026-11-22 02:52 UTC
- Sampling: `sample`, 4 s after 1 s, 1 ms interval, all threads; the command was killed after sampling
- Busy samples: 37969 (idle worker-thread samples excluded: 3031)

## Self time (top of stack)

| Samples | Share | Function |
| ---: | ---: | --- |
| 35832 | 94.4% | `core::ops::function::impls::_<impl core::ops::function::FnMut<A> for &F>::call_mut` |
| 2050 | 5.4% | `semaphore_timedwait_trap` |
| 36 | 0.1% | `radiomics::filters::kernels::pad_vol` |
| 13 | 0.0% | `__psynch_mutexwait` |
| 10 | 0.0% | `_<ndarray::iterators::Iter<A,D> as core::iter::traits::iterator::Iterator>::next` |
| 7 | 0.0% | `rayon::iter::plumbing::Producer::fold_with` |
| 6 | 0.0% | `crossbeam_deque::deque::Worker<T>::pop` |
| 3 | 0.0% | `hypot` |
| 2 | 0.0% | `_<alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter` |
| 2 | 0.0% | `rayon_core::join::join_context::_{{closure}}` |
| 1 | 0.0% | `__bzero` |
| 1 | 0.0% | `DYLD-STUB$$hypot` |
| 1 | 0.0% | `ndarray::impl_methods::_<impl ndarray::ArrayBase<S,D>>::zip_mut_with_same_shape` |
| 1 | 0.0% | `_platform_memmove` |
| 1 | 0.0% | `pthread_mutex_unlock` |
| 1 | 0.0% | `rayon_core::registry::in_worker` |
| 1 | 0.0% | `rayon::iter::plumbing::bridge_producer_consumer::helper` |
| 1 | 0.0% | `_tlv_get_addr` |

## Inclusive time, radiomics functions

| Samples | Share | Function |
| ---: | ---: | --- |
| 2049 | 5.4% | `radiomics::workflows::extraction::extract` |
| 2049 | 5.4% | `radiomics::imaging::transforms::compute_feature_images` |
| 2049 | 5.4% | `radiomics::filters::convolution::gabor_responses` |
| 2043 | 5.4% | `radiomics::filters::convolution::convolve_planes` |
| 1996 | 5.3% | `radiomics::filters::kernels::valid_conv_dense` |
| 36 | 0.1% | `radiomics::filters::kernels::pad_vol` |
| 11 | 0.0% | `radiomics::filters::kernels::Vol::from_array` |
| 1 | 0.0% | `radiomics::filters::convolution::move_axis` |
