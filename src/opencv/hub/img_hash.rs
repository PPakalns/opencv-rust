#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # The module brings implementations of different image hashing algorithms.
//! 
//! Provide algorithms to extract the hash of images and fast way to figure out most similar images in
//! huge data set.
//! 
//! Namespace for all functions is cv::img_hash.
//! 
//! ### Supported Algorithms
//! 
//! - Average hash (also called Different hash)
//! - PHash (also called Perceptual hash)
//! - Marr Hildreth Hash
//! - Radial Variance Hash
//! - Block Mean Hash (modes 0 and 1)
//! - Color Moment Hash (this is the one and only hash algorithm resist to rotation attack(-90~90 degree))
//! 
//! You can study more about image hashing from following paper and websites:
//! 
//! - "Implementation and benchmarking of perceptual image hash functions" [zauner2010implementation](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_zauner2010implementation)
//! - "Looks Like It" [lookslikeit](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_lookslikeit)
//! 
//! ### Code Example
//! 
//! @include samples/hash_samples.cpp
//! 
//! ### Performance under different attacks
//! 
//! ![Performance chart](https://docs.opencv.org/4.5.2/attack_performance.JPG)
//! 
//! ### Speed comparison with PHash library (100 images from ukbench)
//! 
//! ![Hash Computation chart](https://docs.opencv.org/4.5.2/hash_computation_chart.JPG)
//! ![Hash comparison chart](https://docs.opencv.org/4.5.2/hash_comparison_chart.JPG)
//! 
//! As you can see, hash computation speed of img_hash module outperform [PHash library](http://www.phash.org/) a lot.
//! 
//! PS : I do not list out the comparison of Average hash, PHash and Color Moment hash, because I cannot
//! find them in PHash.
//! 
//! ### Motivation
//! 
//! Collects useful image hash algorithms into opencv, so we do not need to rewrite them by ourselves
//! again and again or rely on another 3rd party library(ex : PHash library). BOVW or correlation
//! matching are good and robust, but they are very slow compare with image hash, if you need to deal
//! with large scale CBIR(content based image retrieval) problem, image hash is a more reasonable
//! solution.
//! 
//! ### More info
//! 
//! You can learn more about img_hash modules from following links, these links show you how to find
//! similar image from ukbench dataset, provide thorough benchmark of different attacks(contrast, blur,
//! noise(gaussion,pepper and salt), jpeg compression, watermark, resize).
//! 
//! * [Introduction to image hash module of opencv](http://qtandopencv.blogspot.my/2016/06/introduction-to-image-hash-module-of.html)
//! * [Speed up image hashing of opencv(img_hash) and introduce color moment hash](http://qtandopencv.blogspot.my/2016/06/speed-up-image-hashing-of-opencvimghash.html)
//! 
//! ### Contributors
//! 
//! Tham Ngap Wei, thamngapwei@gmail.com
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::ImgHashBaseTrait, super::AverageHashTrait, super::BlockMeanHashTrait, super::ColorMomentHashTrait, super::MarrHildrethHashTrait, super::PHashTrait, super::RadialVarianceHashTrait };
}

/// use fewer block and generate 16*16/8 uchar hash value
pub const BLOCK_MEAN_HASH_MODE_0: i32 = 0;
/// use block blocks(step sizes/2), generate 31*31/8 + 1 uchar hash value
pub const BLOCK_MEAN_HASH_MODE_1: i32 = 1;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockMeanHashMode {
	/// use fewer block and generate 16*16/8 uchar hash value
	BLOCK_MEAN_HASH_MODE_0 = 0,
	/// use block blocks(step sizes/2), generate 31*31/8 + 1 uchar hash value
	BLOCK_MEAN_HASH_MODE_1 = 1,
}

opencv_type_enum! { crate::img_hash::BlockMeanHashMode }

/// Calculates img_hash::AverageHash in one call
/// ## Parameters
/// * inputArr: input image want to compute hash value, type should be CV_8UC4, CV_8UC3 or CV_8UC1.
/// * outputArr: Hash value of input, it will contain 16 hex decimal number, return type is CV_8U
pub fn average_hash(input_arr: &dyn core::ToInputArray, output_arr: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(input_arr);
	output_array_arg!(output_arr);
	unsafe { sys::cv_img_hash_averageHash_const__InputArrayR_const__OutputArrayR(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray()) }.into_result()
}

/// Computes block mean hash of the input image
/// ## Parameters
/// * inputArr: input image want to compute hash value, type should be CV_8UC4, CV_8UC3 or CV_8UC1.
/// * outputArr: Hash value of input, it will contain 16 hex decimal number, return type is CV_8U
/// * mode: the mode
/// 
/// ## C++ default parameters
/// * mode: BLOCK_MEAN_HASH_MODE_0
pub fn block_mean_hash(input_arr: &dyn core::ToInputArray, output_arr: &mut dyn core::ToOutputArray, mode: i32) -> Result<()> {
	input_array_arg!(input_arr);
	output_array_arg!(output_arr);
	unsafe { sys::cv_img_hash_blockMeanHash_const__InputArrayR_const__OutputArrayR_int(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray(), mode) }.into_result()
}

/// Computes color moment hash of the input, the algorithm
/// is come from the paper "Perceptual  Hashing  for  Color  Images
/// Using  Invariant Moments"
/// ## Parameters
/// * inputArr: input image want to compute hash value,
/// type should be CV_8UC4, CV_8UC3 or CV_8UC1.
/// * outputArr: 42 hash values with type CV_64F(double)
pub fn color_moment_hash(input_arr: &dyn core::ToInputArray, output_arr: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(input_arr);
	output_array_arg!(output_arr);
	unsafe { sys::cv_img_hash_colorMomentHash_const__InputArrayR_const__OutputArrayR(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray()) }.into_result()
}

/// Computes average hash value of the input image
/// ## Parameters
/// * inputArr: input image want to compute hash value,
/// type should be CV_8UC4, CV_8UC3, CV_8UC1.
/// * outputArr: Hash value of input, it will contain 16 hex
/// decimal number, return type is CV_8U
/// * alpha: int scale factor for marr wavelet (default=2).
/// * scale: int level of scale factor (default = 1)
/// 
/// ## C++ default parameters
/// * alpha: 2.0f
/// * scale: 1.0f
pub fn marr_hildreth_hash(input_arr: &dyn core::ToInputArray, output_arr: &mut dyn core::ToOutputArray, alpha: f32, scale: f32) -> Result<()> {
	input_array_arg!(input_arr);
	output_array_arg!(output_arr);
	unsafe { sys::cv_img_hash_marrHildrethHash_const__InputArrayR_const__OutputArrayR_float_float(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray(), alpha, scale) }.into_result()
}

/// Computes pHash value of the input image
/// ## Parameters
/// * inputArr: input image want to compute hash value,
///  type should be CV_8UC4, CV_8UC3, CV_8UC1.
/// * outputArr: Hash value of input, it will contain 8 uchar value
pub fn p_hash(input_arr: &dyn core::ToInputArray, output_arr: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(input_arr);
	output_array_arg!(output_arr);
	unsafe { sys::cv_img_hash_pHash_const__InputArrayR_const__OutputArrayR(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray()) }.into_result()
}

/// Computes radial variance hash of the input image
/// ## Parameters
/// * inputArr: input image want to compute hash value,
/// type should be CV_8UC4, CV_8UC3, CV_8UC1.
/// * outputArr: Hash value of input
/// * sigma: Gaussian kernel standard deviation
/// * numOfAngleLine: The number of angles to consider
/// 
/// ## C++ default parameters
/// * sigma: 1
/// * num_of_angle_line: 180
pub fn radial_variance_hash(input_arr: &dyn core::ToInputArray, output_arr: &mut dyn core::ToOutputArray, sigma: f64, num_of_angle_line: i32) -> Result<()> {
	input_array_arg!(input_arr);
	output_array_arg!(output_arr);
	unsafe { sys::cv_img_hash_radialVarianceHash_const__InputArrayR_const__OutputArrayR_double_int(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray(), sigma, num_of_angle_line) }.into_result()
}

/// Computes average hash value of the input image
/// 
/// This is a fast image hashing algorithm, but only work on simple case. For more details, please
/// refer to [lookslikeit](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_lookslikeit)
pub trait AverageHashTrait: crate::img_hash::ImgHashBaseTrait {
	fn as_raw_AverageHash(&self) -> *const c_void;
	fn as_raw_mut_AverageHash(&mut self) -> *mut c_void;

}

/// Computes average hash value of the input image
/// 
/// This is a fast image hashing algorithm, but only work on simple case. For more details, please
/// refer to [lookslikeit](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_lookslikeit)
pub struct AverageHash {
	ptr: *mut c_void
}

opencv_type_boxed! { AverageHash }

impl Drop for AverageHash {
	fn drop(&mut self) {
		extern "C" { fn cv_AverageHash_delete(instance: *mut c_void); }
		unsafe { cv_AverageHash_delete(self.as_raw_mut_AverageHash()) };
	}
}

impl AverageHash {
	#[inline] pub fn as_raw_AverageHash(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_AverageHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for AverageHash {}

impl core::AlgorithmTrait for AverageHash {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::AverageHashTrait for AverageHash {
	#[inline] fn as_raw_AverageHash(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_AverageHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::ImgHashBaseTrait for AverageHash {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AverageHash {
	pub fn create() -> Result<core::Ptr::<crate::img_hash::AverageHash>> {
		unsafe { sys::cv_img_hash_AverageHash_create() }.into_result().map(|r| unsafe { core::Ptr::<crate::img_hash::AverageHash>::opencv_from_extern(r) } )
	}
	
}

/// Image hash based on block mean.
/// 
/// See [zauner2010implementation](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_zauner2010implementation) for details.
pub trait BlockMeanHashTrait: crate::img_hash::ImgHashBaseTrait {
	fn as_raw_BlockMeanHash(&self) -> *const c_void;
	fn as_raw_mut_BlockMeanHash(&mut self) -> *mut c_void;

	/// Create BlockMeanHash object
	/// ## Parameters
	/// * mode: the mode
	fn set_mode(&mut self, mode: i32) -> Result<()> {
		unsafe { sys::cv_img_hash_BlockMeanHash_setMode_int(self.as_raw_mut_BlockMeanHash(), mode) }.into_result()
	}
	
	fn get_mean(&self) -> Result<core::Vector::<f64>> {
		unsafe { sys::cv_img_hash_BlockMeanHash_getMean_const(self.as_raw_BlockMeanHash()) }.into_result().map(|r| unsafe { core::Vector::<f64>::opencv_from_extern(r) } )
	}
	
}

/// Image hash based on block mean.
/// 
/// See [zauner2010implementation](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_zauner2010implementation) for details.
pub struct BlockMeanHash {
	ptr: *mut c_void
}

opencv_type_boxed! { BlockMeanHash }

impl Drop for BlockMeanHash {
	fn drop(&mut self) {
		extern "C" { fn cv_BlockMeanHash_delete(instance: *mut c_void); }
		unsafe { cv_BlockMeanHash_delete(self.as_raw_mut_BlockMeanHash()) };
	}
}

impl BlockMeanHash {
	#[inline] pub fn as_raw_BlockMeanHash(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_BlockMeanHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for BlockMeanHash {}

impl core::AlgorithmTrait for BlockMeanHash {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::BlockMeanHashTrait for BlockMeanHash {
	#[inline] fn as_raw_BlockMeanHash(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_BlockMeanHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::ImgHashBaseTrait for BlockMeanHash {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BlockMeanHash {
	/// ## C++ default parameters
	/// * mode: BLOCK_MEAN_HASH_MODE_0
	pub fn create(mode: i32) -> Result<core::Ptr::<crate::img_hash::BlockMeanHash>> {
		unsafe { sys::cv_img_hash_BlockMeanHash_create_int(mode) }.into_result().map(|r| unsafe { core::Ptr::<crate::img_hash::BlockMeanHash>::opencv_from_extern(r) } )
	}
	
}

/// Image hash based on color moments.
/// 
/// See [tang2012perceptual](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_tang2012perceptual) for details.
pub trait ColorMomentHashTrait: crate::img_hash::ImgHashBaseTrait {
	fn as_raw_ColorMomentHash(&self) -> *const c_void;
	fn as_raw_mut_ColorMomentHash(&mut self) -> *mut c_void;

}

/// Image hash based on color moments.
/// 
/// See [tang2012perceptual](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_tang2012perceptual) for details.
pub struct ColorMomentHash {
	ptr: *mut c_void
}

opencv_type_boxed! { ColorMomentHash }

impl Drop for ColorMomentHash {
	fn drop(&mut self) {
		extern "C" { fn cv_ColorMomentHash_delete(instance: *mut c_void); }
		unsafe { cv_ColorMomentHash_delete(self.as_raw_mut_ColorMomentHash()) };
	}
}

impl ColorMomentHash {
	#[inline] pub fn as_raw_ColorMomentHash(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_ColorMomentHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for ColorMomentHash {}

impl core::AlgorithmTrait for ColorMomentHash {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::ColorMomentHashTrait for ColorMomentHash {
	#[inline] fn as_raw_ColorMomentHash(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ColorMomentHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::ImgHashBaseTrait for ColorMomentHash {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ColorMomentHash {
	pub fn create() -> Result<core::Ptr::<crate::img_hash::ColorMomentHash>> {
		unsafe { sys::cv_img_hash_ColorMomentHash_create() }.into_result().map(|r| unsafe { core::Ptr::<crate::img_hash::ColorMomentHash>::opencv_from_extern(r) } )
	}
	
}

/// The base class for image hash algorithms
pub trait ImgHashBaseTrait: core::AlgorithmTrait {
	fn as_raw_ImgHashBase(&self) -> *const c_void;
	fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void;

	/// Computes hash of the input image
	/// ## Parameters
	/// * inputArr: input image want to compute hash value
	/// * outputArr: hash of the image
	fn compute(&mut self, input_arr: &dyn core::ToInputArray, output_arr: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(input_arr);
		output_array_arg!(output_arr);
		unsafe { sys::cv_img_hash_ImgHashBase_compute_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_ImgHashBase(), input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray()) }.into_result()
	}
	
	/// Compare the hash value between inOne and inTwo
	/// ## Parameters
	/// * hashOne: Hash value one
	/// * hashTwo: Hash value two
	/// ## Returns
	/// value indicate similarity between inOne and inTwo, the meaning
	/// of the value vary from algorithms to algorithms
	fn compare(&self, hash_one: &dyn core::ToInputArray, hash_two: &dyn core::ToInputArray) -> Result<f64> {
		input_array_arg!(hash_one);
		input_array_arg!(hash_two);
		unsafe { sys::cv_img_hash_ImgHashBase_compare_const_const__InputArrayR_const__InputArrayR(self.as_raw_ImgHashBase(), hash_one.as_raw__InputArray(), hash_two.as_raw__InputArray()) }.into_result()
	}
	
}

/// The base class for image hash algorithms
pub struct ImgHashBase {
	ptr: *mut c_void
}

opencv_type_boxed! { ImgHashBase }

impl Drop for ImgHashBase {
	fn drop(&mut self) {
		extern "C" { fn cv_ImgHashBase_delete(instance: *mut c_void); }
		unsafe { cv_ImgHashBase_delete(self.as_raw_mut_ImgHashBase()) };
	}
}

impl ImgHashBase {
	#[inline] pub fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for ImgHashBase {}

impl core::AlgorithmTrait for ImgHashBase {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::ImgHashBaseTrait for ImgHashBase {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ImgHashBase {
}

/// Marr-Hildreth Operator Based Hash, slowest but more discriminative.
/// 
/// See [zauner2010implementation](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_zauner2010implementation) for details.
pub trait MarrHildrethHashTrait: crate::img_hash::ImgHashBaseTrait {
	fn as_raw_MarrHildrethHash(&self) -> *const c_void;
	fn as_raw_mut_MarrHildrethHash(&mut self) -> *mut c_void;

	/// self explain
	fn get_alpha(&self) -> Result<f32> {
		unsafe { sys::cv_img_hash_MarrHildrethHash_getAlpha_const(self.as_raw_MarrHildrethHash()) }.into_result()
	}
	
	/// self explain
	fn get_scale(&self) -> Result<f32> {
		unsafe { sys::cv_img_hash_MarrHildrethHash_getScale_const(self.as_raw_MarrHildrethHash()) }.into_result()
	}
	
	/// Set Mh kernel parameters
	/// ## Parameters
	/// * alpha: int scale factor for marr wavelet (default=2).
	/// * scale: int level of scale factor (default = 1)
	fn set_kernel_param(&mut self, alpha: f32, scale: f32) -> Result<()> {
		unsafe { sys::cv_img_hash_MarrHildrethHash_setKernelParam_float_float(self.as_raw_mut_MarrHildrethHash(), alpha, scale) }.into_result()
	}
	
}

/// Marr-Hildreth Operator Based Hash, slowest but more discriminative.
/// 
/// See [zauner2010implementation](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_zauner2010implementation) for details.
pub struct MarrHildrethHash {
	ptr: *mut c_void
}

opencv_type_boxed! { MarrHildrethHash }

impl Drop for MarrHildrethHash {
	fn drop(&mut self) {
		extern "C" { fn cv_MarrHildrethHash_delete(instance: *mut c_void); }
		unsafe { cv_MarrHildrethHash_delete(self.as_raw_mut_MarrHildrethHash()) };
	}
}

impl MarrHildrethHash {
	#[inline] pub fn as_raw_MarrHildrethHash(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_MarrHildrethHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MarrHildrethHash {}

impl core::AlgorithmTrait for MarrHildrethHash {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::ImgHashBaseTrait for MarrHildrethHash {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::MarrHildrethHashTrait for MarrHildrethHash {
	#[inline] fn as_raw_MarrHildrethHash(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MarrHildrethHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MarrHildrethHash {
	/// ## Parameters
	/// * alpha: int scale factor for marr wavelet (default=2).
	/// * scale: int level of scale factor (default = 1)
	/// 
	/// ## C++ default parameters
	/// * alpha: 2.0f
	/// * scale: 1.0f
	pub fn create(alpha: f32, scale: f32) -> Result<core::Ptr::<crate::img_hash::MarrHildrethHash>> {
		unsafe { sys::cv_img_hash_MarrHildrethHash_create_float_float(alpha, scale) }.into_result().map(|r| unsafe { core::Ptr::<crate::img_hash::MarrHildrethHash>::opencv_from_extern(r) } )
	}
	
}

/// pHash
/// 
/// Slower than average_hash, but tolerant of minor modifications
/// 
/// This algorithm can combat more variation than averageHash, for more details please refer to [lookslikeit](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_lookslikeit)
pub trait PHashTrait: crate::img_hash::ImgHashBaseTrait {
	fn as_raw_PHash(&self) -> *const c_void;
	fn as_raw_mut_PHash(&mut self) -> *mut c_void;

}

/// pHash
/// 
/// Slower than average_hash, but tolerant of minor modifications
/// 
/// This algorithm can combat more variation than averageHash, for more details please refer to [lookslikeit](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_lookslikeit)
pub struct PHash {
	ptr: *mut c_void
}

opencv_type_boxed! { PHash }

impl Drop for PHash {
	fn drop(&mut self) {
		extern "C" { fn cv_PHash_delete(instance: *mut c_void); }
		unsafe { cv_PHash_delete(self.as_raw_mut_PHash()) };
	}
}

impl PHash {
	#[inline] pub fn as_raw_PHash(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for PHash {}

impl core::AlgorithmTrait for PHash {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::ImgHashBaseTrait for PHash {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::PHashTrait for PHash {
	#[inline] fn as_raw_PHash(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_PHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PHash {
	pub fn create() -> Result<core::Ptr::<crate::img_hash::PHash>> {
		unsafe { sys::cv_img_hash_PHash_create() }.into_result().map(|r| unsafe { core::Ptr::<crate::img_hash::PHash>::opencv_from_extern(r) } )
	}
	
}

/// Image hash based on Radon transform.
/// 
/// See [tang2012perceptual](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_tang2012perceptual) for details.
pub trait RadialVarianceHashTrait: crate::img_hash::ImgHashBaseTrait {
	fn as_raw_RadialVarianceHash(&self) -> *const c_void;
	fn as_raw_mut_RadialVarianceHash(&mut self) -> *mut c_void;

	fn get_num_of_angle_line(&self) -> Result<i32> {
		unsafe { sys::cv_img_hash_RadialVarianceHash_getNumOfAngleLine_const(self.as_raw_RadialVarianceHash()) }.into_result()
	}
	
	fn get_sigma(&self) -> Result<f64> {
		unsafe { sys::cv_img_hash_RadialVarianceHash_getSigma_const(self.as_raw_RadialVarianceHash()) }.into_result()
	}
	
	fn set_num_of_angle_line(&mut self, value: i32) -> Result<()> {
		unsafe { sys::cv_img_hash_RadialVarianceHash_setNumOfAngleLine_int(self.as_raw_mut_RadialVarianceHash(), value) }.into_result()
	}
	
	fn set_sigma(&mut self, value: f64) -> Result<()> {
		unsafe { sys::cv_img_hash_RadialVarianceHash_setSigma_double(self.as_raw_mut_RadialVarianceHash(), value) }.into_result()
	}
	
	fn get_features(&mut self) -> Result<core::Vector::<f64>> {
		unsafe { sys::cv_img_hash_RadialVarianceHash_getFeatures(self.as_raw_mut_RadialVarianceHash()) }.into_result().map(|r| unsafe { core::Vector::<f64>::opencv_from_extern(r) } )
	}
	
	fn get_hash(&mut self) -> Result<core::Mat> {
		unsafe { sys::cv_img_hash_RadialVarianceHash_getHash(self.as_raw_mut_RadialVarianceHash()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn get_pix_per_line(&mut self, input: &core::Mat) -> Result<core::Mat> {
		unsafe { sys::cv_img_hash_RadialVarianceHash_getPixPerLine_const_MatR(self.as_raw_mut_RadialVarianceHash(), input.as_raw_Mat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn get_projection(&mut self) -> Result<core::Mat> {
		unsafe { sys::cv_img_hash_RadialVarianceHash_getProjection(self.as_raw_mut_RadialVarianceHash()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// Image hash based on Radon transform.
/// 
/// See [tang2012perceptual](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_tang2012perceptual) for details.
pub struct RadialVarianceHash {
	ptr: *mut c_void
}

opencv_type_boxed! { RadialVarianceHash }

impl Drop for RadialVarianceHash {
	fn drop(&mut self) {
		extern "C" { fn cv_RadialVarianceHash_delete(instance: *mut c_void); }
		unsafe { cv_RadialVarianceHash_delete(self.as_raw_mut_RadialVarianceHash()) };
	}
}

impl RadialVarianceHash {
	#[inline] pub fn as_raw_RadialVarianceHash(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_RadialVarianceHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for RadialVarianceHash {}

impl core::AlgorithmTrait for RadialVarianceHash {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::ImgHashBaseTrait for RadialVarianceHash {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::RadialVarianceHashTrait for RadialVarianceHash {
	#[inline] fn as_raw_RadialVarianceHash(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_RadialVarianceHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RadialVarianceHash {
	/// ## C++ default parameters
	/// * sigma: 1
	/// * num_of_angle_line: 180
	pub fn create(sigma: f64, num_of_angle_line: i32) -> Result<core::Ptr::<crate::img_hash::RadialVarianceHash>> {
		unsafe { sys::cv_img_hash_RadialVarianceHash_create_double_int(sigma, num_of_angle_line) }.into_result().map(|r| unsafe { core::Ptr::<crate::img_hash::RadialVarianceHash>::opencv_from_extern(r) } )
	}
	
}
