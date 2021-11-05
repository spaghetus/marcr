//! Information about the material's nature.
//! Mostly fills field 6.

use num_enum::FromPrimitive;

/// Extra information about the physical characteristics of the material.
///
/// Original documentation:
///
/// <https://www.loc.gov/marc/bibliographic/concise/bd006.html>
pub enum AdditionalMaterialCharacteristics {
	/// A book or other text.
	Book {
		/// Whether the Book is a Manuscript.
		/// Position 0, a = false, t = true.
		manuscript: bool,
		/// The types of illustrations the book has.
		/// Positions 1-4.
		/// <https://www.loc.gov/marc/bibliographic/concise/bd008b.html>
		illustrations: [Illustration; 4],
		/// The target audience of the book.
		/// Position 5.
		/// <https://www.loc.gov/marc/bibliographic/concise/bd008b.html>
		target_audience: TargetAudience,
		/// The form in which the book is stored.
		/// Position 6.
		/// <https://www.loc.gov/marc/bibliographic/concise/bd008b.html>
		form_of_item: FormOfItem,
		/// The nature of the contents of the book.
		/// Positions 7-10.
		nature_of_contents: [NatureOfContents; 4],
		/// Government publication?
		/// Position 11.
		government_publication: GovernmentPublication,
		/// Is the book a conference publication?
		/// Position 12.
		conference_publication: ConferencePublication,
		/// Is the book a 'festschrift'?
		/// Position 13.
		festschrift: Festschrift,
		/// Does the book have an index?
		/// Position 14.
		index: Index,
		/// The literary form of the book.
		/// Position 16.
		literary_form: LiteraryForm,
		/// The type of biography, if any.
		biography: Biography,
	},
	/// A computer file or electronic resource.
	ComputerFile {
		/// Target audience
		/// Position 5.
		target_audience: TargetAudience,
		/// Form of item
		/// Position 6.
		form_of_item: FormOfItem,
		/// Type of file
		/// Position 9.
		file_type: FileType,
		/// Type of government publication, if any.
		/// Position 11.
		government_publication: GovernmentPublication,
	},
	/// A map.
	Map {
		/// Whether the map is a manuscript.
		/// Position 0, e = false, f = true.
		manuscript: bool,
		/// 'relief'
		/// I'm not sure what this is.
		/// Position 1-4.
		relief: [Relief; 4],
		/// The projection of the map.
		/// Position 5-6.
		projection: Projection,
		/// The type of map.
		/// Position 8.
		cartographic_type: CartographicType,
		/// The type of government publication, if any.
		/// Position 11.
		government_publication: GovernmentPublication,
		/// The form of the map.
		/// Position 12.
		form_of_item: FormOfItem,
		/// Whether an index is included.
		/// Position 14.
		index: Index,
		/// Special format characteristics.
		/// Position 16-17.
		special_format_characteristics: [SpecialFormatCharacteristics; 2],
	},
	/// Many materials.
	MixedMaterials {
		/// The form of the item(s).
		/// Position 6.
		form_of_item: FormOfItem,
	},
	/// Music.
	Music {
		/// Whether the sound is notated.
		/// Implies musical.
		/// Position 0, i+j = false, c+d = true.
		recorded: bool,
		/// Whether the sound is a manuscript.
		/// Implies notated and musical.
		/// Position 0, d = true, otherwise false.
		manuscript: bool,
		/// Whether the sound is musical.
		/// Position 0, i = false, otherwise true.
		musical: bool,
		/// Form of composition.
		/// Position 1-2.
		form_of_composition: FormOfComposition,
		/// Format of the music or sound recording.
		/// Position 3.
		format_of_music: FormatOfMusic,
		/// The parts the music contains.
		/// Position 4.
		parts: MusicParts,
		/// Target audience.
		/// Position 5.
		target_audience: TargetAudience,
		/// Form of item.
		/// Position 6.
		form_of_item: FormOfItem,
		/// Accompanying matter,
		/// Position 7-12.
		accompanying_matter: [AccompanyingMatter; 6],
		/// Literary text.
		/// Position 13-14.
		literary_text: [MusicText; 2],
		/// Transposition and arrangement.
		/// Position 16.
		transposition_and_arrangement: TranspositionArrangement,
	},
	/// Continuing resources.
	ContinuingResources {
		/// The frequency at which the resource is published.
		/// Position 1.
		frequency: Frequency,
		/// The regularity with which the resource is published.
		/// Position 2.
		regularity: Regularity,
		/// Form of the original item.
		/// Position 5.
		form_of_original: FormOfItem,
		/// Form of the current item.
		/// Position 6.
		form_of_current: FormOfItem,
		/// The nature of the work.
		/// Position 7.
		nature_of_work: NatureOfContents,
		/// The nature of the work's contents.
		/// Positions 8-10.
		nature_of_content: [NatureOfContents; 3],
		/// The type of government publication, if any.
		/// Position 11.
		government_publication: GovernmentPublication,
		/// The type of conference publication, if any.
		/// Position 12.
		conference_publication: ConferencePublication,
		/// The original alphabet or script.
		/// Position 16.
		original_alphabet_or_script: AlphabetScript,
	},
}

/// The types of illustrations the book has.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum Illustration {
	/// The book has no more illustrations.
	None = b'#',
	/// The book has illustrations.
	Some = b'a',
	/// The book has maps.
	Maps = b'b',
	/// The book has portraits.
	Portraits = b'c',
	/// The book has charts
	Charts = b'd',
	/// The book has plans
	Plans = b'e',
	/// The book has plates
	Plates = b'f',
	/// The book has music
	Music = b'g',
	/// The book has facsimiles
	Facsimiles = b'h',
	/// The book has coats of arms
	CoatsOfArms = b'i',
	/// The book has genealogical tables
	GenealogicalTables = b'j',
	/// The book has forms
	Forms = b'k',
	/// The book has samples
	Samples = b'l',
	/// The book has phonodiscs, phonowires, etc.
	Phonodiscs = b'm',
	/// The book has photographs
	Photographs = b'o',
	/// The book has illuminations.
	Illuminations = b'p',
	/// No attempt was made to code illustrations.
	#[default]
	NotCoded = b'|',
}

/// The book's target audience.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum TargetAudience {
	/// The book's target audience is unknown.
	Unknown = b'#',
	/// The book's target audience is preschool children.
	Preschool = b'a',
	/// The book's target audience is primary school children.
	Primary = b'b',
	/// The book's target audience is preadolescent children.
	PreAdolescent = b'c',
	/// The book's target audience is adolescent children.
	Adolescent = b'd',
	/// The book's target audience is adult children.
	Adult = b'e',
	/// The book's target audience is specialized.
	Specialized = b'f',
	/// The book's target audience is general.
	General = b'g',
	/// The book's target audience is juvenile.
	Juvenile = b'j',
	/// The book's target audience is not encoded.
	#[default]
	NotCoded = b'|',
}

/// The form in which the book is stored.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum FormOfItem {
	/// The book is not in any of the available forms.
	None = b'#',
	/// The book is a microfilm.
	Microfilm = b'a',
	/// The book is a microfiche.
	Microfiche = b'b',
	/// The book is a microopaque.
	Microopaque = b'c',
	/// The book is in large print.
	LargePrint = b'd',
	/// e - Newspaper format
	Newspaper = b'e',
	/// The book is in Braille.
	Braille = b'f',
	/// The book is available online
	Online = b'o',
	/// The book is 'direct electronic'
	/// (i'm not a librarian, so I don't know what this means)
	DirectElectronic = b'q',
	/// The book is a print reproduction.
	PrintReproduction = b'r',
	/// The book is electronic.
	Electronic = b's',
	/// The book's form is not coded.
	#[default]
	NotCoded = b'|',
}

/// The nature of the book's contents.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum NatureOfContents {
	/// # - No specified nature of contents
	None = b'#',
	/// a - Abstracts/summaries
	Abstracts = b'a',
	/// b - Bibliographies
	Bibliographies = b'b',
	/// c - Catalogs
	Catalogs = b'c',
	/// d - Dictionaries
	Dictionaries = b'd',
	/// e - Encyclopedias
	Encyclopedias = b'e',
	/// f - Handbooks
	Handbooks = b'f',
	/// g - Legal articles
	LegalArticles = b'g',
	/// i - Indexes
	Indexes = b'i',
	/// j - Patent document
	PatentDocument = b'j',
	/// k - Discographies
	Discographies = b'k',
	/// l - Legislation
	Legislation = b'l',
	/// m - Theses
	Theses = b'm',
	/// n - Surveys of literature in a subject area
	SurveysOfLiterature = b'n',
	/// o - Reviews
	Reviews = b'o',
	/// p - Programmed texts
	ProgrammedTexts = b'p',
	/// q - Filmographies
	Filmographies = b'q',
	/// r - Directories
	Directories = b'r',
	/// s - Statistics
	Statistics = b's',
	/// t - Technical reports
	TechnicalReports = b't',
	/// u - Standards/specifications
	Standards = b'u',
	/// v - Legal cases and case notes
	LegalCases = b'v',
	/// w - Law reports and digests
	LawReports = b'w',
	/// y - Yearbooks
	Yearbooks = b'y',
	/// z - Treaties
	Treaties = b'z',
	/// 2 - Offprints
	Offprints = b'2',
	/// 5 - Calendars
	Calendars = b'5',
	/// 6 - Comics/graphic novels
	Comics = b'6',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// What type of government publication the book is, if any.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum GovernmentPublication {
	/// # - Not a government publication
	None = b'#',
	/// a - Autonomous or semi-autonomous component
	Autonomous = b'a',
	/// c - Multilocal
	Multilocal = b'c',
	/// f - Federal/national
	Federal = b'f',
	/// i - International intergovernmental
	International = b'i',
	/// l - Local
	Local = b'l',
	/// m - Multistate
	Multistate = b'm',
	/// o - Government publication-level undetermined
	Undetermined = b'o',
	/// s - State, provincial, territorial, dependent, etc.
	State = b's',
	/// u - Unknown if item is government publication
	Unknown = b'u',
	/// z - Other
	Other = b'z',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// Whether the book is a conference publication.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum ConferencePublication {
	/// 0 - Not a conference publication
	NonConference = b'0',
	/// 1 - Conference publication
	Conference = b'1',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// Whether the book is a 'festschrift'.
/// I have no idea what this means.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum Festschrift {
	/// 0 - Not a festschrift
	NotFestschrift = b'0',
	/// 1 - Festschrift
	Festschrift = b'1',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// Whether the book contains an index to its own contents.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum Index {
	/// 0 - No index
	None = b'0',
	/// 1 - Index
	Index = b'1',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// The literary form of the book.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum LiteraryForm {
	/// 0 - Not fiction (not further specified)
	NotFiction = b'0',
	/// 1 - Fiction (not further specified)
	Fiction = b'1',
	/// d - Dramas
	Dramas = b'd',
	/// e - Essays
	Essays = b'e',
	/// f - Novels
	Novels = b'f',
	/// h - Humor, satires, etc.
	Humor = b'h',
	/// i - Letters
	Letters = b'i',
	/// j - Short stories
	ShortStories = b'j',
	/// m - Mixed forms
	MixedForms = b'm',
	/// p - Poetry
	Poetry = b'p',
	/// s - Speeches
	Speeches = b's',
	/// u - Unknown
	Unknown = b'u',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// The type of biography, if any.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum Biography {
	/// # - No biographical material
	None = b'#',
	/// a - Autobiography
	Autobiography = b'a',
	/// b - Individual biography
	Individual = b'b',
	/// c - Collective biography
	Collective = b'c',
	/// d - Contains biographical information
	Contains = b'd',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// The type of the file.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum FileType {
	/// a - Numeric data
	Numeric = b'a',
	/// b - Computer program
	ComputerProgram = b'b',
	/// c - Representational
	Representational = b'c',
	/// d - Document
	Document = b'd',
	/// e - Bibliographic data
	Bibliographic = b'e',
	/// f - Font
	Font = b'f',
	/// g - Game
	Game = b'g',
	/// h - Sound
	Sound = b'h',
	/// i - Interactive multimedia
	InteractiveMultimedia = b'i',
	/// j - Online system or service
	OnlineSystem = b'j',
	/// m - Combination
	Combination = b'm',
	/// u - Unknown
	Unknown = b'u',
	/// z - Other
	Other = b'z',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// The type of relief used by the map.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum Relief {
	/// # - No relief shown
	None = b'#',
	/// a - Contours
	Contours = b'a',
	/// b - Shading
	Shading = b'b',
	/// c - Gradient and bathymetric tints
	Gradient = b'c',
	/// d - Hachures
	Hachures = b'd',
	/// e - Bathymetry/soundings
	Bathymetry = b'e',
	/// f - Form lines
	FormLines = b'f',
	/// g - Spot heights
	SpotHeights = b'g',
	/// i - Pictorially
	Pictorially = b'i',
	/// j - Land forms
	LandForms = b'j',
	/// k - Bathymetry/isolines
	Isolines = b'k',
	/// m - Rock drawings
	RockDrawings = b'm',
	/// z - Other
	Other = b'z',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// The projection of the map.
/// This is quite possibly the most disgusting enum ever made.
/// I had to disable macro error reporting in Rust Analyzer because of this.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u16)]
pub enum Projection {
	/// ## - Projection not specified
	NotSpecified = (b"##"[0] as u16) << 8 | b"##"[1] as u16,
	/// aa - Aitoff
	Aitoff = (b"aa"[0] as u16) << 8 | b"aa"[1] as u16,
	/// ab - Gnomic
	Gnomic = (b"ab"[0] as u16) << 8 | b"ab"[1] as u16,
	/// ac - Lambert's azimuthal equal area
	LambertAzimuthal = (b"ac"[0] as u16) << 8 | b"ac"[1] as u16,
	/// ad - Orthographic
	Orthographic = (b"ad"[0] as u16) << 8 | b"ad"[1] as u16,
	/// ae - Azimuthal equidistant
	AzimuthalEquidistant = (b"ae"[0] as u16) << 8 | b"ae"[1] as u16,
	/// af - Stereographic
	Stereographic = (b"af"[0] as u16) << 8 | b"af"[1] as u16,
	/// ag - General vertical near-sided
	GeneralVerticalNearSided = (b"ag"[0] as u16) << 8 | b"ag"[1] as u16,
	/// am - Modified stereographic for Alaska
	ModifiedStereographicAlaska = (b"am"[0] as u16) << 8 | b"am"[1] as u16,
	/// an - Chamberlin trimetric
	ChamberlinTrimetric = (b"an"[0] as u16) << 8 | b"an"[1] as u16,
	/// ap - Polar stereographic
	PolarStereographic = (b"ap"[0] as u16) << 8 | b"ap"[1] as u16,
	/// au - Azimuthal, specific type unknown
	AzimuthalUnknown = (b"au"[0] as u16) << 8 | b"au"[1] as u16,
	/// az - Azimuthal, other
	AzimuthalOther = (b"az"[0] as u16) << 8 | b"az"[1] as u16,
	/// ba - Gall
	Gall = (b"ba"[0] as u16) << 8 | b"ba"[1] as u16,
	/// bb - Goode's homolographic
	GoodeHomolographic = (b"bb"[0] as u16) << 8 | b"bb"[1] as u16,
	/// bc - Lambert's cylindrical equal area
	LambertCylindrical = (b"bc"[0] as u16) << 8 | b"bc"[1] as u16,
	/// bd - Mercator
	Mercator = (b"bd"[0] as u16) << 8 | b"bd"[1] as u16,
	/// be - Miller
	Miller = (b"be"[0] as u16) << 8 | b"be"[1] as u16,
	/// bf - Mollweide
	Mollweide = (b"bf"[0] as u16) << 8 | b"bf"[1] as u16,
	/// bg - Sinusoidal
	Sinusoidal = (b"bg"[0] as u16) << 8 | b"bg"[1] as u16,
	/// bh - Transverse Mercator
	TransverseMercator = (b"bh"[0] as u16) << 8 | b"bh"[1] as u16,
	/// bi - Gauss-Kruger
	GaussKruger = (b"bi"[0] as u16) << 8 | b"bi"[1] as u16,
	/// bj - Equirectangular
	Equirectangular = (b"bj"[0] as u16) << 8 | b"bj"[1] as u16,
	/// bk - Krovak
	Krovak = (b"bk"[0] as u16) << 8 | b"bk"[1] as u16,
	/// bl - Cassini-Soldner
	CassiniSoldner = (b"bl"[0] as u16) << 8 | b"bl"[1] as u16,
	/// bo - Oblique Mercator
	ObliqueMercator = (b"bo"[0] as u16) << 8 | b"bo"[1] as u16,
	/// br - Robinson
	Robinson = (b"br"[0] as u16) << 8 | b"br"[1] as u16,
	/// bs - Space oblique Mercator
	SpaceObliqueMercator = (b"bs"[0] as u16) << 8 | b"bs"[1] as u16,
	/// bu - Cylindrical, specific type unknown
	CylindricalUnknown = (b"bu"[0] as u16) << 8 | b"bu"[1] as u16,
	/// bz - Cylindrical, other
	CylindricalOther = (b"bz"[0] as u16) << 8 | b"bz"[1] as u16,
	/// ca - Albers equal area
	AlbersEqualArea = (b"ca"[0] as u16) << 8 | b"ca"[1] as u16,
	/// cb - Bonne
	Bonne = (b"cb"[0] as u16) << 8 | b"cb"[1] as u16,
	/// cc - Lambert's conformal conic
	LambertConformalConic = (b"cc"[0] as u16) << 8 | b"cc"[1] as u16,
	/// ce - Equidistant conic
	EquidistantConic = (b"ce"[0] as u16) << 8 | b"ce"[1] as u16,
	/// cp - Polyconic
	Polyconic = (b"cp"[0] as u16) << 8 | b"cp"[1] as u16,
	/// cu - Conic, specific type unknown
	ConicUnknown = (b"cu"[0] as u16) << 8 | b"cu"[1] as u16,
	/// cz - Conic, other
	ConicOther = (b"cz"[0] as u16) << 8 | b"cz"[1] as u16,
	/// da - Armadillo
	Armadillo = (b"da"[0] as u16) << 8 | b"da"[1] as u16,
	/// db - Butterfly
	Butterfly = (b"db"[0] as u16) << 8 | b"db"[1] as u16,
	/// dc - Eckert
	Eckert = (b"dc"[0] as u16) << 8 | b"dc"[1] as u16,
	/// dd - Goode's homolosine
	GoodeHomolosine = (b"dd"[0] as u16) << 8 | b"dd"[1] as u16,
	/// de - Miller's bipolar oblique conformal conic
	MillerBipolarObliqueConformal = (b"de"[0] as u16) << 8 | b"de"[1] as u16,
	/// df - Van Der Grinten
	VanDerGrinten = (b"df"[0] as u16) << 8 | b"df"[1] as u16,
	/// dg - Dimaxion
	Dimaxion = (b"dg"[0] as u16) << 8 | b"dg"[1] as u16,
	/// dh - Cordiform
	Cordiform = (b"dh"[0] as u16) << 8 | b"dh"[1] as u16,
	/// dl - Lambert conformal
	LambertConformal = (b"dl"[0] as u16) << 8 | b"dl"[1] as u16,
	/// zz - Other
	Other = (b"zz"[0] as u16) << 8 | b"zz"[1] as u16,
	/// || - No attempt to code
	#[default]
	NotCoded = (b"||"[0] as u16) << 8 | b"||"[1] as u16,
}

/// The type of the map.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum CartographicType {
	/// a - Single map
	SingleMap = b'a',
	/// b - Map series
	MapSeries = b'b',
	/// c - Map serial
	MapSerial = b'c',
	/// d - Globe
	Globe = b'd',
	/// e - Atlas
	Atlas = b'e',
	/// f - Separate supplement to another work
	Supplement = b'f',
	/// g - Bound as part of another work
	Part = b'g',
	/// u - Unknown
	Unknown = b'u',
	/// z - Other
	Other = b'z',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// Special format characteristics.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum SpecialFormatCharacteristics {
	/// # - No specified special format characteristics
	None = b'#',
	/// e - Manuscript
	Manuscript = b'e',
	/// j - Picture card, post card
	PictureCard = b'j',
	/// k - Calendar
	Calendar = b'k',
	/// l - Puzzle
	Puzzle = b'l',
	/// n - Game
	Game = b'n',
	/// o - Wall map
	WallMap = b'o',
	/// p - Playing cards
	PlayingCards = b'p',
	/// r - Loose-leaf
	LooseLeaf = b'r',
	/// z - Other
	Other = b'z',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// The form of the composition.
/// Oh boy, another two-letter code, that means more spaghetti.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u16)]
pub enum FormOfComposition {
	/// an - Anthems
	Anthems = (b"an"[0] as u16) << 8 | b"an"[1] as u16,
	/// bd - Ballads
	Ballads = (b"bd"[0] as u16) << 8 | b"bd"[1] as u16,
	/// bg - Bluegrass music
	Bluegrass = (b"bg"[0] as u16) << 8 | b"bg"[1] as u16,
	/// bl - Blues
	Blues = (b"bl"[0] as u16) << 8 | b"bl"[1] as u16,
	/// bt - Ballets
	Ballet = (b"bt"[0] as u16) << 8 | b"bt"[1] as u16,
	/// ca - Chaconnes
	Chaconne = (b"ca"[0] as u16) << 8 | b"ca"[1] as u16,
	/// cb - Chants, Other religions
	Chant = (b"cb"[0] as u16) << 8 | b"cb"[1] as u16,
	/// cc - Chant, Christian
	ChristianChant = (b"cc"[0] as u16) << 8 | b"cc"[1] as u16,
	/// cg - Concerti grossi
	Concerti = (b"cg"[0] as u16) << 8 | b"cg"[1] as u16,
	/// ch - Chorales
	Chorale = (b"ch"[0] as u16) << 8 | b"ch"[1] as u16,
	/// cl - Chorale preludes
	ChoralePrelude = (b"cl"[0] as u16) << 8 | b"cl"[1] as u16,
	/// cn - Canons and rounds
	Canon = (b"cn"[0] as u16) << 8 | b"cn"[1] as u16,
	/// co - Concertos
	Concerto = (b"co"[0] as u16) << 8 | b"co"[1] as u16,
	/// cp - Chansons, polyphonic
	Chanson = (b"cp"[0] as u16) << 8 | b"cp"[1] as u16,
	/// cr - Carols
	Carols = (b"cr"[0] as u16) << 8 | b"cr"[1] as u16,
	/// cs - Chance compositions
	Chance = (b"cs"[0] as u16) << 8 | b"cs"[1] as u16,
	/// ct - Cantatas
	Cantata = (b"ct"[0] as u16) << 8 | b"ct"[1] as u16,
	/// cy - Country music
	Country = (b"cy"[0] as u16) << 8 | b"cy"[1] as u16,
	/// cz - Canzonas
	Canzona = (b"cz"[0] as u16) << 8 | b"cz"[1] as u16,
	/// df - Dance forms
	Dance = (b"df"[0] as u16) << 8 | b"df"[1] as u16,
	/// dv - Divertimentos, serenades, cassations, divertissements, and notturni
	Divertimento = (b"dv"[0] as u16) << 8 | b"dv"[1] as u16,
	/// fg - Fugues
	Fugue = (b"fg"[0] as u16) << 8 | b"fg"[1] as u16,
	/// fl - Flamenco
	Flamenco = (b"fl"[0] as u16) << 8 | b"fl"[1] as u16,
	/// fm - Folk music
	Folk = (b"fm"[0] as u16) << 8 | b"fm"[1] as u16,
	/// ft - Fantasias
	Fantasia = (b"ft"[0] as u16) << 8 | b"ft"[1] as u16,
	/// gm - Gospel music
	Gospel = (b"gm"[0] as u16) << 8 | b"gm"[1] as u16,
	/// hy - Hymns
	Hymn = (b"hy"[0] as u16) << 8 | b"hy"[1] as u16,
	/// jz - Jazz
	Jazz = (b"jz"[0] as u16) << 8 | b"jz"[1] as u16,
	/// mc - Musical revues and comedies
	Musical = (b"mc"[0] as u16) << 8 | b"mc"[1] as u16,
	/// md - Madrigals
	Madrigal = (b"md"[0] as u16) << 8 | b"md"[1] as u16,
	/// mi - Minuets
	Minuet = (b"mi"[0] as u16) << 8 | b"mi"[1] as u16,
	/// mo - Motets
	Motet = (b"mo"[0] as u16) << 8 | b"mo"[1] as u16,
	/// mp - Motion picture music
	Motion = (b"mp"[0] as u16) << 8 | b"mp"[1] as u16,
	/// mr - Marches
	March = (b"mr"[0] as u16) << 8 | b"mr"[1] as u16,
	/// ms - Masses
	Mass = (b"ms"[0] as u16) << 8 | b"ms"[1] as u16,
	/// mu - Multiple forms
	Multiple = (b"mu"[0] as u16) << 8 | b"mu"[1] as u16,
	/// mz - Mazurkas
	Mazurka = (b"mz"[0] as u16) << 8 | b"mz"[1] as u16,
	/// nc - Nocturnes
	Nocturne = (b"nc"[0] as u16) << 8 | b"nc"[1] as u16,
	/// nn - Not applicable
	NotApplicable = (b"nn"[0] as u16) << 8 | b"nn"[1] as u16,
	/// op - Operas
	Opera = (b"op"[0] as u16) << 8 | b"op"[1] as u16,
	/// or - Oratorios
	Oratorio = (b"or"[0] as u16) << 8 | b"or"[1] as u16,
	/// ov - Overtures
	Overture = (b"ov"[0] as u16) << 8 | b"ov"[1] as u16,
	/// pg - Program music
	Program = (b"pg"[0] as u16) << 8 | b"pg"[1] as u16,
	/// pm - Passion music
	Passion = (b"pm"[0] as u16) << 8 | b"pm"[1] as u16,
	/// po - Polonaises
	Polonaise = (b"po"[0] as u16) << 8 | b"po"[1] as u16,
	/// pp - Popular music
	Popular = (b"pp"[0] as u16) << 8 | b"pp"[1] as u16,
	/// pr - Preludes
	Prelude = (b"pr"[0] as u16) << 8 | b"pr"[1] as u16,
	/// ps - Passacaglias
	Passacaglia = (b"ps"[0] as u16) << 8 | b"ps"[1] as u16,
	/// pt - Part-songs
	Part = (b"pt"[0] as u16) << 8 | b"pt"[1] as u16,
	/// pv - Pavans
	Pavan = (b"pv"[0] as u16) << 8 | b"pv"[1] as u16,
	/// rc - Rock music
	Rock = (b"rc"[0] as u16) << 8 | b"rc"[1] as u16,
	/// rd - Rondos
	Rondo = (b"rd"[0] as u16) << 8 | b"rd"[1] as u16,
	/// rg - Ragtime music
	Ragtime = (b"rg"[0] as u16) << 8 | b"rg"[1] as u16,
	/// ri - Ricercars
	Ricercar = (b"ri"[0] as u16) << 8 | b"ri"[1] as u16,
	/// rp - Rhapsodies
	Rhapsody = (b"rp"[0] as u16) << 8 | b"rp"[1] as u16,
	/// rq - Requiems
	Requiem = (b"rq"[0] as u16) << 8 | b"rq"[1] as u16,
	/// sd - Square dance music
	Square = (b"sd"[0] as u16) << 8 | b"sd"[1] as u16,
	/// sg - Songs
	Songs = (b"sg"[0] as u16) << 8 | b"sg"[1] as u16,
	/// sn - Sonatas
	Sonata = (b"sn"[0] as u16) << 8 | b"sn"[1] as u16,
	/// sp - Symphonic poems
	Symphonic = (b"sp"[0] as u16) << 8 | b"sp"[1] as u16,
	/// st - Studies and exercises
	Study = (b"st"[0] as u16) << 8 | b"st"[1] as u16,
	/// su - Suites
	Suite = (b"su"[0] as u16) << 8 | b"su"[1] as u16,
	/// sy - Symphonies
	Symphony = (b"sy"[0] as u16) << 8 | b"sy"[1] as u16,
	/// tc - Toccatas
	Toccata = (b"tc"[0] as u16) << 8 | b"tc"[1] as u16,
	/// tl - Teatro lirico
	Teatro = (b"tl"[0] as u16) << 8 | b"tl"[1] as u16,
	/// ts - Trio-sonatas
	TrioSonata = (b"ts"[0] as u16) << 8 | b"ts"[1] as u16,
	/// uu - Unknown
	Unknown = (b"uu"[0] as u16) << 8 | b"uu"[1] as u16,
	/// vi - Villancicos
	Villancico = (b"vi"[0] as u16) << 8 | b"vi"[1] as u16,
	/// vr - Variations
	Variation = (b"vr"[0] as u16) << 8 | b"vr"[1] as u16,
	/// wz - Waltzes
	Waltz = (b"wz"[0] as u16) << 8 | b"wz"[1] as u16,
	/// za - Zarzuelas
	Zarzuela = (b"za"[0] as u16) << 8 | b"za"[1] as u16,
	/// zz - Other
	Other = (b"zz"[0] as u16) << 8 | b"zz"[1] as u16,
	/// || - No attempt to code
	#[default]
	NotCoded = (b"||"[0] as u16) << 8 | b"||"[1] as u16,
}

/// The format of the music.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum FormatOfMusic {
	/// a - Full score
	Full = b'a',
	/// b - Miniature or study score
	Miniature = b'b',
	/// c - Accompaniment reduced for keyboard
	Accompaniment = b'c',
	/// d - Voice score with accompaniment omitted
	Voice = b'd',
	/// e - Condensed score or piano-conductor score
	CondensedOrConductor = b'e',
	/// g - Close score
	Close = b'g',
	/// h - Chorus score
	Chorus = b'h',
	/// i - Condensed score
	Condensed = b'i',
	/// j - Performer-conductor part
	Performer = b'j',
	/// k - Vocal score
	Vocal = b'k',
	/// l - Score
	Score = b'l',
	/// m - Multiple score formats
	Multiple = b'm',
	/// n - Not applicable
	Not = b'n',
	/// p - Piano score
	Piano = b'p',
	/// u - Unknown
	Unknown = b'u',
	/// z - Other
	Other = b'z',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// The parts in the music.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum MusicParts {
	/// # - No parts in hand or not specified
	None = b'#',
	/// d - Instrumental and vocal parts
	InstrumentalAndVocal = b'd',
	/// e - Instrumental parts
	Instrumental = b'e',
	/// f - Vocal parts
	Vocal = b'f',
	/// n - Not applicable
	Not = b'n',
	/// u - Unknown
	Unknown = b'u',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// Matter accompanying the media.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum AccompanyingMatter {
	/// # - No accompanying matter
	None = b'#',
	/// a - Discography
	Discography = b'a',
	/// b - Bibliography
	Bibliography = b'b',
	/// c - Thematic index
	Thematic = b'c',
	/// d - Libretto or text
	Libretto = b'd',
	/// e - Biography of composer or author
	BiographyComposer = b'e',
	/// f - Biography of performer or history of ensemble
	BiographyPerformer = b'f',
	/// g - Technical and/or historical information on instruments
	TechnicalInstruments = b'g',
	/// h - Technical information on music
	TechnicalMusic = b'h',
	/// i - Historical information
	Historical = b'i',
	/// k - Ethnological information
	Ethnological = b'k',
	/// r - Instructional materials
	Instructional = b'r',
	/// s - Music
	Music = b's',
	/// z - Other
	Other = b'z',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// The type of literary text in the recording.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum MusicText {
	/// # - Item is a music sound recording
	Music = b'#',
	/// a - Autobiography
	Autobiography = b'a',
	/// b - Biography
	Biography = b'b',
	/// c - Conference proceedings
	Conference = b'c',
	/// d - Drama
	Drama = b'd',
	/// e - Essays
	Essays = b'e',
	/// f - Fiction
	Fiction = b'f',
	/// g - Reporting
	Reporting = b'g',
	/// h - History
	History = b'h',
	/// i - Instruction
	Instruction = b'i',
	/// j - Language instruction
	Language = b'j',
	/// k - Comedy
	Comedy = b'k',
	/// l - Lectures, speeches
	Lectures = b'l',
	/// m - Memoirs
	Memoirs = b'm',
	/// n - Not applicable
	Not = b'n',
	/// o - Folktales
	Folktales = b'o',
	/// p - Poetry
	Poetry = b'p',
	/// r - Rehearsals
	Rehearsals = b'r',
	/// s - Sounds
	Sounds = b's',
	/// t - Interviews
	Interviews = b't',
	/// z - Other
	Other = b'z',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// Whether the music is transposed or arranged.

#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum TranspositionArrangement {
	/// # - Not arrangement or transposition or not specified
	None = b'#',
	/// a - Transposition
	Transposition = b'a',
	/// b - Arrangement
	Arrangement = b'b',
	/// c - Both transposed and arranged
	Both = b'c',
	/// n - Not applicable
	NotApplicable = b'n',
	/// u - Unknown
	Unknown = b'u',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// The frequency at which a publication is published.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum Frequency {
	/// # - No determinable frequency
	None = b'#',
	/// a - Annual
	Annual = b'a',
	/// b - Bimonthly
	Bimonthly = b'b',
	/// c - Semiweekly
	Semiweekly = b'c',
	/// d - Daily
	Daily = b'd',
	/// e - Biweekly
	Biweekly = b'e',
	/// f - Semiannual
	Semiannual = b'f',
	/// g - Biennial
	Biennial = b'g',
	/// h - Triennial
	Triennial = b'h',
	/// i - Three times a week
	ThreeWeekly = b'i',
	/// j - Three times a month
	ThreeMonthly = b'j',
	/// k - Continuously updated
	Continuously = b'k',
	/// m - Monthly
	Monthly = b'm',
	/// q - Quarterly
	Quarterly = b'q',
	/// s - Semimonthly
	Semimonthly = b's',
	/// t - Three times a year
	Three = b't',
	/// u - Unknown
	Unknown = b'u',
	/// w - Weekly
	Weekly = b'w',
	/// z - Other
	Other = b'z',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// The regularity with which a publication is published.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum Regularity {
	/// n - Normalized irregular
	Normalized = b'n',
	/// r - Regular
	Regular = b'r',
	/// u - Unknown
	Unknown = b'u',
	/// x - Completely irregular
	Completely = b'x',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// The type of the periodic publication.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum PublicationType {
	/// # - None of the following
	None = b'#',
	/// d - Updating database
	UpdatingDatabase = b'd',
	/// g - Magazine
	Magazine = b'g',
	/// h - Blog
	Blog = b'h',
	/// j - Journal
	Journal = b'j',
	/// l - Updating loose-leaf
	UpdatingLooseLeaf = b'l',
	/// m - Monographic series
	Monographic = b'm',
	/// n - Newspaper
	Newspaper = b'n',
	/// p - Periodical
	Periodical = b'p',
	/// r - Repository
	Repository = b'r',
	/// s - Newsletter
	Newsletter = b's',
	/// t - Directory
	Directory = b't',
	/// w - Updating Web site
	UpdatingWeb = b'w',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// An alphabet or script.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum AlphabetScript {
	/// # - No alphabet or script given/No key title
	None = b'#',
	/// a - Basic Roman
	BasicRoman = b'a',
	/// b - Extended Roman
	ExtendedRoman = b'b',
	/// c - Cyrillic
	Cyrillic = b'c',
	/// d - Japanese
	Japanese = b'd',
	/// e - Chinese
	Chinese = b'e',
	/// f - Arabic
	Arabic = b'f',
	/// g - Greek
	Greek = b'g',
	/// h - Hebrew
	Hebrew = b'h',
	/// i - Thai
	Thai = b'i',
	/// j - Devanagari
	Devanagari = b'j',
	/// k - Korean
	Korean = b'k',
	/// l - Tamil
	Tamil = b'l',
	/// u - Unknown
	Unknown = b'u',
	/// z - Other
	Other = b'z',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}

/// The convention according to which the publication is published.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum EntryConvention {
	/// 0 - Successive entry
	Successive = b'0',
	/// 1 - Latest entry
	Latest = b'1',
	/// 2 - Integrated entry
	Integrated = b'2',
	/// | - No attempt to code
	#[default]
	NotCoded = b'|',
}
