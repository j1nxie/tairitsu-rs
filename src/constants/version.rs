use semver::Version;

const ARCAEA: [&str; 22] = [
    "Eternal Core",
    "Crimson Solace",
    "Ambivalent Vision",
    "Binary Enfold",
    "Vicious Labyrinth",
    "Luminous Sky",
    "Absolute Reason",
    "Adverse Prelude",
    "Sunset Radiance",
    "Black Fate",
    "Ephemeral Page",
    "The Journey Onwards",
    "Esoteric Order",
    "Pale Tapestry",
    "Light of Salvation",
    "Shared Time",
    "Divided Heart",
    "Final Verdict",
    "Silent Answer",
    "Lasting Eden",
    "Severed Eden",
    "Shifting Veil",
];

pub fn get_version() -> String {
    let semver = env!("CARGO_PKG_VERSION").parse::<Version>();

    if let Ok(semver) = semver {
        let version_name = format!(
            "{} - {} [{}]",
            semver,
            ARCAEA[(semver.major + semver.minor - 1) as usize],
            env!("VERGEN_GIT_SHA")
        );
        version_name
    } else {
        tracing::warn!("couldn't parse a semver out of Cargo.toml? defaulting to 0.0.0-unknown.");
        String::from("0.0.0-unknown - No Version Name")
    }
}
