// 包含v3,v4所有Text information frame
// https://docs.mp3tag.de/mapping/

#[derive(Debug)]
pub enum IDFactory {
    T(TextInformationFrameIdentifier), // Start With 'T'
    W(URLLinkFrameIdentifier),         // start with 'W'
    R(RarelyUsedFrameIdentifier), // rarely used, just return the raw bytes, excluding the header
    APIC,                         // attached picture
    COMM,                         // comment
    SYLT,                         // sync lyrics
    USLT,                         // unsync lyrics
    PADDING,
}

impl From<Vec<u8>> for IDFactory {
    fn from(value: Vec<u8>) -> Self {
        if value == [0, 0, 0, 0] {
            return IDFactory::PADDING;
        }
        let id: String = String::from_utf8(value).expect("");
        if id.starts_with("T") {
            return IDFactory::T(TextInformationFrameIdentifier::from(id));
        } else if id.starts_with("W") {
            return IDFactory::W(URLLinkFrameIdentifier::from(id));
        } else if id == "APIC" {
            return IDFactory::APIC;
        } else if id == "USLT" {
            return IDFactory::USLT;
        } else if id == "SYLT" {
            return IDFactory::SYLT;
        } else if id == "COMM" {
            return IDFactory::COMM;
        } else {
            return IDFactory::R(RarelyUsedFrameIdentifier::from(id));
        }
    }
}

impl ToString for IDFactory {
    fn to_string(&self) -> String {
        match self {
            Self::T(v) => v.to_string(),
            Self::W(v) => v.to_string(),
            Self::R(v) => v.to_string(),
            Self::APIC => "APIC".to_string(),
            Self::COMM => "COMM".to_string(),
            Self::SYLT => "SYLT".to_string(),
            Self::USLT => "USLT".to_string(),
            Self::PADDING => "PADDING".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum TextInformationFrameIdentifier {
    TIT1,
    TIT2,
    TIT3,
    TALB,
    TOAL,
    TRCK,
    TPOS,
    TSST,
    TSRC,
    TPE1,
    TPE2,
    TPE3,
    TPE4,
    TOPE,
    TEXT,
    TOLY,
    TCOM,
    TMCL,
    TIPL,
    TENC,
    TBPM,
    TLEN,
    TKEY,
    TLAN,
    TCON,
    TFLT,
    TMED,
    TMOO,
    TCOP,
    TPRO,
    TPUB,
    TOWN,
    TRSN,
    TRSO,
    TOFN,
    TDLY,
    TDEN,
    TDOR,
    TDRC, // only in v4, equals to TYER in v3
    TDRL,
    TDTG,
    TSSE,
    TSOA,
    TSOP,
    TSOT,
    TDAT,
    TIME,
    TORY,
    TRDA,
    TSIZ,
    TYER, // only in v3, equals to TDRC in v4
    TXXX,
}

impl From<String> for TextInformationFrameIdentifier {
    fn from(id: String) -> Self {
        // if value == vec![0, 0, 0, 0] {
        //     return TextInformationFrameIdentifiers::PADDING;
        // }
        // let id = String::from_utf8(value).expect("");
        match id.as_str() {
            "TIT1" => TextInformationFrameIdentifier::TIT1,
            "TIT2" => TextInformationFrameIdentifier::TIT2,
            "TIT3" => TextInformationFrameIdentifier::TIT3,
            "TALB" => TextInformationFrameIdentifier::TALB,
            "TOAL" => TextInformationFrameIdentifier::TOAL,
            "TRCK" => TextInformationFrameIdentifier::TRCK,
            "TPOS" => TextInformationFrameIdentifier::TPOS,
            "TSST" => TextInformationFrameIdentifier::TSST,
            "TSRC" => TextInformationFrameIdentifier::TSRC,
            "TPE1" => TextInformationFrameIdentifier::TPE1,
            "TPE2" => TextInformationFrameIdentifier::TPE2,
            "TPE3" => TextInformationFrameIdentifier::TPE3,
            "TPE4" => TextInformationFrameIdentifier::TPE4,
            "TOPE" => TextInformationFrameIdentifier::TOPE,
            "TEXT" => TextInformationFrameIdentifier::TEXT,
            "TOLY" => TextInformationFrameIdentifier::TOLY,
            "TCOM" => TextInformationFrameIdentifier::TCOM,
            "TMCL" => TextInformationFrameIdentifier::TMCL,
            "TIPL" => TextInformationFrameIdentifier::TIPL,
            "TENC" => TextInformationFrameIdentifier::TENC,
            "TBPM" => TextInformationFrameIdentifier::TBPM,
            "TLEN" => TextInformationFrameIdentifier::TLEN,
            "TKEY" => TextInformationFrameIdentifier::TKEY,
            "TLAN" => TextInformationFrameIdentifier::TLAN,
            "TCON" => TextInformationFrameIdentifier::TCON,
            "TFLT" => TextInformationFrameIdentifier::TFLT,
            "TMED" => TextInformationFrameIdentifier::TMED,
            "TMOO" => TextInformationFrameIdentifier::TMOO,
            "TCOP" => TextInformationFrameIdentifier::TCOP,
            "TPRO" => TextInformationFrameIdentifier::TPRO,
            "TPUB" => TextInformationFrameIdentifier::TPUB,
            "TOWN" => TextInformationFrameIdentifier::TOWN,
            "TRSN" => TextInformationFrameIdentifier::TRSN,
            "TRSO" => TextInformationFrameIdentifier::TRSO,
            "TOFN" => TextInformationFrameIdentifier::TOFN,
            "TDLY" => TextInformationFrameIdentifier::TDLY,
            "TDEN" => TextInformationFrameIdentifier::TDEN,
            "TDOR" => TextInformationFrameIdentifier::TDOR,
            "TDRC" => TextInformationFrameIdentifier::TDRC,
            "TDRL" => TextInformationFrameIdentifier::TDRL,
            "TDTG" => TextInformationFrameIdentifier::TDTG,
            "TSSE" => TextInformationFrameIdentifier::TSSE,
            "TSOA" => TextInformationFrameIdentifier::TSOA,
            "TSOP" => TextInformationFrameIdentifier::TSOP,
            "TSOT" => TextInformationFrameIdentifier::TSOT,
            "TDAT" => TextInformationFrameIdentifier::TDAT,
            "TIME" => TextInformationFrameIdentifier::TIME,
            "TORY" => TextInformationFrameIdentifier::TORY,
            "TRDA" => TextInformationFrameIdentifier::TRDA,
            "TSIZ" => TextInformationFrameIdentifier::TSIZ,
            "TYER" => TextInformationFrameIdentifier::TYER,
            "TXXX" => TextInformationFrameIdentifier::TXXX,
            _ => unreachable!(),
        }
    }
}

impl ToString for TextInformationFrameIdentifier {
    fn to_string(&self) -> String {
        match self {
            TextInformationFrameIdentifier::TIT1 => String::from("TIT1"),
            TextInformationFrameIdentifier::TIT2 => String::from("TIT2"),
            TextInformationFrameIdentifier::TIT3 => String::from("TIT3"),
            TextInformationFrameIdentifier::TALB => String::from("TALB"),
            TextInformationFrameIdentifier::TOAL => String::from("TOAL"),
            TextInformationFrameIdentifier::TRCK => String::from("TRCK"),
            TextInformationFrameIdentifier::TPOS => String::from("TPOS"),
            TextInformationFrameIdentifier::TSST => String::from("TSST"),
            TextInformationFrameIdentifier::TSRC => String::from("TSRC"),
            TextInformationFrameIdentifier::TPE1 => String::from("TPE1"),
            TextInformationFrameIdentifier::TPE2 => String::from("TPE2"),
            TextInformationFrameIdentifier::TPE3 => String::from("TPE3"),
            TextInformationFrameIdentifier::TPE4 => String::from("TPE4"),
            TextInformationFrameIdentifier::TOPE => String::from("TOPE"),
            TextInformationFrameIdentifier::TEXT => String::from("TEXT"),
            TextInformationFrameIdentifier::TOLY => String::from("TOLY"),
            TextInformationFrameIdentifier::TCOM => String::from("TCOM"),
            TextInformationFrameIdentifier::TMCL => String::from("TMCL"),
            TextInformationFrameIdentifier::TIPL => String::from("TIPL"),
            TextInformationFrameIdentifier::TENC => String::from("TENC"),
            TextInformationFrameIdentifier::TBPM => String::from("TBPM"),
            TextInformationFrameIdentifier::TLEN => String::from("TLEN"),
            TextInformationFrameIdentifier::TKEY => String::from("TKEY"),
            TextInformationFrameIdentifier::TLAN => String::from("TLAN"),
            TextInformationFrameIdentifier::TCON => String::from("TCON"),
            TextInformationFrameIdentifier::TFLT => String::from("TFLT"),
            TextInformationFrameIdentifier::TMED => String::from("TMED"),
            TextInformationFrameIdentifier::TMOO => String::from("TMOO"),
            TextInformationFrameIdentifier::TCOP => String::from("TCOP"),
            TextInformationFrameIdentifier::TPRO => String::from("TPRO"),
            TextInformationFrameIdentifier::TPUB => String::from("TPUB"),
            TextInformationFrameIdentifier::TOWN => String::from("TOWN"),
            TextInformationFrameIdentifier::TRSN => String::from("TRSN"),
            TextInformationFrameIdentifier::TRSO => String::from("TRSO"),
            TextInformationFrameIdentifier::TOFN => String::from("TOFN"),
            TextInformationFrameIdentifier::TDLY => String::from("TDLY"),
            TextInformationFrameIdentifier::TDEN => String::from("TDEN"),
            TextInformationFrameIdentifier::TDOR => String::from("TDOR"),
            TextInformationFrameIdentifier::TDRC => String::from("TDRC"),
            TextInformationFrameIdentifier::TDRL => String::from("TDRL"),
            TextInformationFrameIdentifier::TDTG => String::from("TDTG"),
            TextInformationFrameIdentifier::TSSE => String::from("TSSE"),
            TextInformationFrameIdentifier::TSOA => String::from("TSOA"),
            TextInformationFrameIdentifier::TSOP => String::from("TSOP"),
            TextInformationFrameIdentifier::TSOT => String::from("TSOT"),
            TextInformationFrameIdentifier::TDAT => String::from("TDAT"),
            TextInformationFrameIdentifier::TIME => String::from("TIME"),
            TextInformationFrameIdentifier::TORY => String::from("TORY"),
            TextInformationFrameIdentifier::TRDA => String::from("TRDA"),
            TextInformationFrameIdentifier::TSIZ => String::from("TSIZ"),
            TextInformationFrameIdentifier::TYER => String::from("TYER"),
            TextInformationFrameIdentifier::TXXX => String::from("TXXX"),
        }
    }
}

#[derive(Debug)]
pub enum URLLinkFrameIdentifier {
    WCOM,
    WCOP,
    WOAF,
    WOAR,
    WOAS,
    WORS,
    WPAY,
    WPUB,
    WXXX,
}

impl From<String> for URLLinkFrameIdentifier {
    fn from(id: String) -> Self {
        // let id = String::from_utf8(value).expect("");
        match id.as_str() {
            "WCOM" => URLLinkFrameIdentifier::WCOM,
            "WCOP" => URLLinkFrameIdentifier::WCOP,
            "WOAF" => URLLinkFrameIdentifier::WOAF,
            "WOAR" => URLLinkFrameIdentifier::WOAR,
            "WOAS" => URLLinkFrameIdentifier::WOAS,
            "WORS" => URLLinkFrameIdentifier::WORS,
            "WPAY" => URLLinkFrameIdentifier::WPAY,
            "WPUB" => URLLinkFrameIdentifier::WPUB,
            "WXXX" => URLLinkFrameIdentifier::WXXX,
            _ => {
                unreachable!()
            }
        }
    }
}

impl ToString for URLLinkFrameIdentifier {
    fn to_string(&self) -> String {
        match self {
            URLLinkFrameIdentifier::WCOM => String::from("WCOM"),
            URLLinkFrameIdentifier::WCOP => String::from("WCOP"),
            URLLinkFrameIdentifier::WOAF => String::from("WOAF"),
            URLLinkFrameIdentifier::WOAR => String::from("WOAR"),
            URLLinkFrameIdentifier::WOAS => String::from("WOAS"),
            URLLinkFrameIdentifier::WORS => String::from("WORS"),
            URLLinkFrameIdentifier::WPAY => String::from("WPAY"),
            URLLinkFrameIdentifier::WPUB => String::from("WPUB"),
            URLLinkFrameIdentifier::WXXX => String::from("WXXX"),
        }
    }
}

// These frames are rarely used in practice, and some of them can't even be found on mp3tag websites, but are only defined in protocols
#[derive(Debug)]
pub enum RarelyUsedFrameIdentifier {
    IPLS,
    MCDI,
    ETCO,
    MLLT,
    SYTC, // v4
    RVA2, // v4
    RVAD,
    EQUA,
    EQU2, // v4
    RVRB,
    GEOB,
    PCNT,
    POPM,
    RBUF,
    AENC,
    LINK,
    POSS,
    USER,
    OWNE,
    COMR,
    ENCR,
    GRID,
    PRIV,
    SIGN, // v4
    SEEK, // v4
    ASPI, // v4
    // addendum for v3 and v4
    CHAP, // v4
    CTOC, // v4
    UNIMPLEMENT(String),
}
impl From<String> for RarelyUsedFrameIdentifier {
    fn from(id: String) -> Self {
        // let id = String::from_utf8(value).expect("");
        match id.as_str() {
            "IPLS" => RarelyUsedFrameIdentifier::IPLS,
            "MCDI" => RarelyUsedFrameIdentifier::MCDI,
            "ETCO" => RarelyUsedFrameIdentifier::ETCO,
            "MLLT" => RarelyUsedFrameIdentifier::MLLT,
            "SYTC" => RarelyUsedFrameIdentifier::SYTC,
            "RVA2" => RarelyUsedFrameIdentifier::RVA2,
            "RVAD" => RarelyUsedFrameIdentifier::RVAD,
            "EQUA" => RarelyUsedFrameIdentifier::EQUA,
            "EQU2" => RarelyUsedFrameIdentifier::EQU2,
            "RVRB" => RarelyUsedFrameIdentifier::RVRB,
            "GEOB" => RarelyUsedFrameIdentifier::GEOB,
            "PCNT" => RarelyUsedFrameIdentifier::PCNT,
            "POPM" => RarelyUsedFrameIdentifier::POPM,
            "RBUF" => RarelyUsedFrameIdentifier::RBUF,
            "AENC" => RarelyUsedFrameIdentifier::AENC,
            "LINK" => RarelyUsedFrameIdentifier::LINK,
            "POSS" => RarelyUsedFrameIdentifier::POSS,
            "USER" => RarelyUsedFrameIdentifier::USER,
            "OWNE" => RarelyUsedFrameIdentifier::OWNE,
            "COMR" => RarelyUsedFrameIdentifier::COMR,
            "ENCR" => RarelyUsedFrameIdentifier::ENCR,
            "GRID" => RarelyUsedFrameIdentifier::GRID,
            "PRIV" => RarelyUsedFrameIdentifier::PRIV,
            "SIGN" => RarelyUsedFrameIdentifier::SIGN,
            "SEEK" => RarelyUsedFrameIdentifier::SEEK,
            "ASPI" => RarelyUsedFrameIdentifier::ASPI,
            "CHAP" => RarelyUsedFrameIdentifier::CHAP,
            "CTOC" => RarelyUsedFrameIdentifier::CTOC,
            _ => RarelyUsedFrameIdentifier::UNIMPLEMENT(id),
        }
    }
}
impl ToString for RarelyUsedFrameIdentifier {
    fn to_string(&self) -> String {
        match self {
            RarelyUsedFrameIdentifier::IPLS => String::from("IPLS"),
            RarelyUsedFrameIdentifier::MCDI => String::from("MCDI"),
            RarelyUsedFrameIdentifier::ETCO => String::from("ETCO"),
            RarelyUsedFrameIdentifier::MLLT => String::from("MLLT"),
            RarelyUsedFrameIdentifier::SYTC => String::from("SYTC"),
            RarelyUsedFrameIdentifier::RVA2 => String::from("RVA2"),
            RarelyUsedFrameIdentifier::RVAD => String::from("RVAD"),
            RarelyUsedFrameIdentifier::EQUA => String::from("EQUA"),
            RarelyUsedFrameIdentifier::EQU2 => String::from("EQU2"),
            RarelyUsedFrameIdentifier::RVRB => String::from("RVRB"),
            RarelyUsedFrameIdentifier::GEOB => String::from("GEOB"),
            RarelyUsedFrameIdentifier::PCNT => String::from("PCNT"),
            RarelyUsedFrameIdentifier::POPM => String::from("POPM"),
            RarelyUsedFrameIdentifier::RBUF => String::from("RBUF"),
            RarelyUsedFrameIdentifier::AENC => String::from("AENC"),
            RarelyUsedFrameIdentifier::LINK => String::from("LINK"),
            RarelyUsedFrameIdentifier::POSS => String::from("POSS"),
            RarelyUsedFrameIdentifier::USER => String::from("USER"),
            RarelyUsedFrameIdentifier::OWNE => String::from("OWNE"),
            RarelyUsedFrameIdentifier::COMR => String::from("COMR"),
            RarelyUsedFrameIdentifier::ENCR => String::from("ENCR"),
            RarelyUsedFrameIdentifier::GRID => String::from("GRID"),
            RarelyUsedFrameIdentifier::PRIV => String::from("PRIV"),
            RarelyUsedFrameIdentifier::SIGN => String::from("SIGN"),
            RarelyUsedFrameIdentifier::SEEK => String::from("SEEK"),
            RarelyUsedFrameIdentifier::ASPI => String::from("ASPI"),
            RarelyUsedFrameIdentifier::CHAP => String::from("CHAP"),
            RarelyUsedFrameIdentifier::CTOC => String::from("CTOC"),
            RarelyUsedFrameIdentifier::UNIMPLEMENT(id) => id.into(),
        }
    }
}
