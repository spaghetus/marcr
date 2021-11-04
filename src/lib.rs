#![forbid(missing_docs)]
#![feature(proc_macro_hygiene)]
//! A library for converting MARC21 files into an ergonomic struct for easier processing.

#[macro_use]
extern crate num_enum;
pub mod additional_material_characteristics;
use chrono::NaiveDateTime;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}

/// A struct containing data from a MARC21 file.
/// Much of the documentation is taken from https://www.loc.gov/marc/bibliographic/
pub struct Marc {
	/// The control number of the record.
	///
	/// Original documentation:
	///
	/// Contains the control number assigned by the organization creating, using, or distributing the record.
	/// For interchange purposes, documentation of the structure of the control number and input conventions should be provided to exchange partners by the organization initiating the interchange.
	/// The MARC code identifying whose system control number is present in field 001 is contained in field 003 (Control Number Identifier).
	/// An organization receiving a record may move the incoming control number from field 001 (and the control number identifier from field 003) to field 035 (System Control Number),
	/// 010 (Library of Congress Control Number), or 016 (National Bibliographic Agency Control Number), as appropriate, and place its own system control number in field 001 (and its control number identifier in field 003).
	/// ■EXAMPLES
	///
	/// 001 = #880524405##
	/// 003 = CaOONL
	/// 016 = ##$a#880524405##
	/// [LAC generally supplies its control number in both field 001 and 016]
	///
	/// 001 = ###86104385#
	/// 003 = DLC
	/// 010 = ##$a###86104385#
	/// [LC generally supplies its control number in both field 001 and 010]
	///
	/// 001 = ocm14919759
	/// 003 = OCoLC
	///
	/// 001 = #####9007496
	/// 003 = DNLM
	pub control_number: String,
	/// The organization assigning the control number.
	///
	/// Original documentation:
	///
	/// MARC code for the organization whose control number is contained in field 001 (Control Number).
	///
	/// See Appendix I: Organization Code Sources for a listing of sources used in MARC 21 records.
	///
	/// Whenever the number in field 001 is changed, agencies must assure that the MARC code in field 003 applies to the number found in the 001 field.
	/// ■EXAMPLES
	///
	/// 001 = #880524405##
	/// 003 = CaOONL
	///
	/// 001 = ###86104385#
	/// 003 = DLC
	///
	/// 001 = ocm14919759
	/// 003 = OCoLC
	///
	/// 001 = #####9007496
	/// 003 = DNLM
	pub control_number_identifier: String,
	/// The time at which the record was last modified.
	///
	/// Original documentation:
	///
	/// Sixteen characters that indicate the date and time of the latest record transaction and serve as a version identifier for the record.
	/// They are recorded according to Representation of Dates and Times (ISO 8601).
	/// The date requires 8 numeric characters in the pattern yyyymmdd.
	/// The time requires 8 numeric characters in the pattern hhmmss.f, expressed in terms of the 24-hour (00-23) clock.
	/// Example
	///
	/// 005 = 19940223151047.0
	pub date_and_time_of_latest_record_transaction: NaiveDateTime,
	/// Encoded information about the nature of the material.
	///
	/// The original documentation is quite lengthy; here is the URL: https://www.loc.gov/marc/bibliographic/bd006.html
	pub additional_material_characteristics:
		additional_material_characteristics::AdditionalMaterialCharacteristics,
}
