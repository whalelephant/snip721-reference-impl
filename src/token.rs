use crate::rand::Prng;
use hex::ToHex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Coin};

use crate::expiration::Expiration;
use crate::state::Permission;

/// token
#[derive(Serialize, Deserialize)]
pub struct Token {
    /// owner
    pub owner: CanonicalAddr,
    /// permissions granted for this token
    pub permissions: Vec<Permission>,
    /// true if this token has been unwrapped.  If sealed metadata is not enabled, all
    /// tokens are considered unwrapped
    pub unwrapped: bool,
    /// true if this token has been collateralised, if so then no operation can be performed
    /// apart from UnCollateralise
    pub collateralised: bool,
}

/// CollateralInfo
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct CollateralInfo {
    /// price willing to accept collateral at
    pub price: Coin,
    /// price willing to relay at
    pub repayment: Coin,
    /// expiration is the time after which the collateral can be redeemed by the holder
    /// of the collateral
    pub expiration: Expiration,
    /// collateral holder
    pub holder: Option<CanonicalAddr>,
}

/// token metadata
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct Metadata {
    /// optional uri for off-chain metadata.  This should be prefixed with `http://`, `https://`, `ipfs://`, or
    /// `ar://`.  Only use this if you are not using `extension`
    pub token_uri: Option<String>,
    /// optional on-chain metadata.  Only use this if you are not using `token_uri`
    pub extension: Option<Extension>,
}

/// metadata extension
/// You can add any metadata fields you need here.  These fields are based on
/// https://docs.opensea.io/docs/metadata-standards and are the metadata fields that
/// Stashh uses for robust NFT display.  Urls should be prefixed with `http://`, `https://`, `ipfs://`, or
/// `ar://`
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct Extension {
    /// url to the image
    pub image: Option<String>,
    /// raw SVG image data (not recommended). Only use this if you're not including the image parameter
    pub image_data: Option<String>,
    /// url to allow users to view the item on your site
    pub external_url: Option<String>,
    /// item description
    pub description: Option<String>,
    /// dice experience level
    pub xp: u32,
    /// name of the item
    pub name: Option<String>,
    /// item attributes
    pub attributes: Vec<Trait>,
    /// background color represented as a six-character hexadecimal without a pre-pended #
    pub background_color: Option<String>,
    /// url to a multimedia attachment
    pub animation_url: Option<String>,
    /// url to a YouTube video
    pub youtube_url: Option<String>,
    /// media files as specified on Stashh that allows for basic authenticatiion and decryption keys.
    /// Most of the above is used for bridging public eth NFT metadata easily, whereas `media` will be used
    /// when minting NFTs on Stashh
    pub media: Option<Vec<MediaFile>>,
    /// a select list of trait_types that are in the private metadata.  This will only ever be used
    /// in public metadata
    pub protected_attributes: Option<Vec<String>>,
}

/// attribute trait
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct Trait {
    /// indicates how a trait should be displayed
    pub display_type: Option<String>,
    /// name of the trait
    pub trait_type: Option<String>,
    /// trait value
    pub value: String,
    /// optional max value for numerical traits
    pub max_value: Option<String>,
}

/// media file
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct MediaFile {
    /// file type
    /// Stashh currently uses: "image", "video", "audio", "text", "font", "application"
    pub file_type: Option<String>,
    /// file extension
    pub extension: Option<String>,
    /// authentication information
    pub authentication: Option<Authentication>,
    /// url to the file.  Urls should be prefixed with `http://`, `https://`, `ipfs://`, or `ar://`
    pub url: String,
}

/// media file authentication
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct Authentication {
    /// either a decryption key for encrypted files or a password for basic authentication
    pub key: Option<String>,
    /// username used in basic authentication
    pub user: Option<String>,
}

/// colours
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct Colour(Vec<u8>);

impl Colour {
    pub fn new(p: &mut Prng) -> Self {
        let rand_colour = &p.rand_bytes()[0..3];
        Colour(rand_colour.to_vec())
    }
}

impl Trait {
    pub fn new_dice_colour(p: &mut Prng) -> Self {
        let colour = Colour::new(p).0.encode_hex::<String>();
        Trait {
            display_type: None,
            trait_type: None,
            value: colour,
            max_value: None,
        }
    }
}

impl Default for Extension {
    fn default() -> Self {
        Extension {
            image: None,
            image_data: None,
            external_url: None,
            description: Some("A dice set for web3 gaming".into()),
            xp: 0,
            name: Some("Poker Joke Dice".into()),
            attributes: vec![],
            background_color: None,
            animation_url: None,
            youtube_url: None,
            media: None,
            protected_attributes: None,
        }
    }
}

impl Extension {
    pub fn with_colours(seed: &[u8]) -> Self {
        let mut new_dice_traits = Vec::new();
        let mut p = Prng::new(&[12], seed);
        for _ in 0..3 {
            new_dice_traits.push(Trait::new_dice_colour(&mut p));
        }
        Extension {
            image: None,
            image_data: None,
            external_url: None,
            description: Some("A dice set for web3 gaming".into()),
            xp: 0,
            name: Some("Poker Joke Dice".into()),
            attributes: new_dice_traits,
            background_color: Some(Colour::new(&mut p).0.encode_hex()),
            animation_url: None,
            youtube_url: None,
            media: None,
            protected_attributes: None,
        }
    }
}
