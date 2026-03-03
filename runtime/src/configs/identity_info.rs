use crate::configs::RuntimeDebug;
use crate::frame_support::{CloneNoBound, EqNoBound, PartialEqNoBound, RuntimeDebugNoBound};
use crate::pallet_identity::{Data, IdentityInformationProvider};
use codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use enumflags2::{bitflags, BitFlags};

/// The fields that we use to identify the owner of an account with. Each corresponds to a field
/// in the `IdentityInfo` struct.
#[bitflags]
#[repr(u64)]
#[derive(Clone, Copy, PartialEq, Eq, RuntimeDebug)]
pub enum IdentityField {
    Display,
    Legal,
    Web,
    Matrix,
    Email,
    PgpFingerprint,
    Image,
    Twitter,
    GitHub,
    Discord,
}

#[derive(
    CloneNoBound,
    Encode,
    Decode,
    DecodeWithMemTracking,
    EqNoBound,
    MaxEncodedLen,
    PartialEqNoBound,
    RuntimeDebugNoBound,
    scale_info::TypeInfo,
)]
#[codec(mel_bound())]
pub struct IdentityInformation {
    /// A reasonable display name for the controller of the account. This should be whatever the
    /// account is typically known as and should not be confusable with other entities, given
    /// reasonable context.
    ///
    /// Stored as UTF-8.
    pub display: Data,

    /// The full legal name in the local jurisdiction of the entity. This might be a bit
    /// long-winded.
    ///
    /// Stored as UTF-8.
    pub legal: Data,

    /// A representative website held by the controller of the account.
    ///
    /// NOTE: `https://` is automatically prepended.
    ///
    /// Stored as UTF-8.
    pub web: Data,

    /// The Matrix (e.g. for Element) handle held by the controller of the account. Previously,
    /// this was called `riot`.
    ///
    /// Stored as UTF-8.
    pub matrix: Data,

    /// The email address of the controller of the account.
    ///
    /// Stored as UTF-8.
    pub email: Data,

    /// The PGP/GPG public key of the controller of the account.
    pub pgp_fingerprint: Option<[u8; 20]>,

    /// A graphic image representing the controller of the account. Should be a company,
    /// organization or project logo or a headshot in the case of a human.
    pub image: Data,

    /// The Twitter identity. The leading `@` character may be elided.
    pub twitter: Data,

    /// The GitHub username of the controller of the account.
    pub github: Data,

    /// The Discord username of the controller of the account.
    pub discord: Data,
}

impl IdentityInformationProvider for IdentityInformation {
    type FieldsIdentifier = u64;

    fn has_identity(&self, fields: Self::FieldsIdentifier) -> bool {
        self.fields().bits() & fields == fields
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn create_identity_info() -> Self {
        let data = Data::Raw(vec![0; 32].try_into().unwrap());

        IdentityInformation {
            display: data.clone(),
            legal: data.clone(),
            web: data.clone(),
            matrix: data.clone(),
            email: data.clone(),
            pgp_fingerprint: Some([0; 20]),
            image: data.clone(),
            twitter: data.clone(),
            github: data.clone(),
            discord: data,
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn all_fields() -> Self::FieldsIdentifier {
        use enumflags2::BitFlag;
        IdentityField::all().bits()
    }
}

impl IdentityInformation {
    pub(crate) fn fields(&self) -> BitFlags<IdentityField> {
        let mut res = <BitFlags<IdentityField>>::empty();
        if !self.display.is_none() {
            res.insert(IdentityField::Display);
        }
        if !self.legal.is_none() {
            res.insert(IdentityField::Legal);
        }
        if !self.web.is_none() {
            res.insert(IdentityField::Web);
        }
        if !self.matrix.is_none() {
            res.insert(IdentityField::Matrix);
        }
        if !self.email.is_none() {
            res.insert(IdentityField::Email);
        }
        if self.pgp_fingerprint.is_some() {
            res.insert(IdentityField::PgpFingerprint);
        }
        if !self.image.is_none() {
            res.insert(IdentityField::Image);
        }
        if !self.twitter.is_none() {
            res.insert(IdentityField::Twitter);
        }
        if !self.github.is_none() {
            res.insert(IdentityField::GitHub);
        }
        if !self.discord.is_none() {
            res.insert(IdentityField::Discord);
        }
        res
    }
}

/// A `Default` identity. This is given to users who get a username but have not set an identity.
impl Default for IdentityInformation {
    fn default() -> Self {
        IdentityInformation {
            display: Data::None,
            legal: Data::None,
            web: Data::None,
            matrix: Data::None,
            email: Data::None,
            pgp_fingerprint: None,
            image: Data::None,
            twitter: Data::None,
            github: Data::None,
            discord: Data::None,
        }
    }
}
